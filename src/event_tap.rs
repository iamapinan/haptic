use crate::config::AppConfig;
use crate::haptic::{perform_haptic, Id, NIL};
use crate::sound::play_keyboard_sound;
use block::ConcreteBlock;
use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoopAddSource, CFRunLoopGetMain};
use core_foundation::string::CFString;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

type IOHIDManagerRef = *mut c_void;
type IOHIDValueRef = *mut c_void;
type IOHIDElementRef = *mut c_void;
type CFMachPortRef = *mut c_void;
type CGEventRef = *mut c_void;
type CGEventTapProxy = *mut c_void;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: core_foundation::dictionary::CFDictionaryRef) -> bool;
}

#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn IOHIDManagerCreate(allocator: *mut c_void, options: u32) -> IOHIDManagerRef;
    fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: *mut c_void);
    fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: unsafe extern "C" fn(*mut c_void, i32, *mut c_void, IOHIDValueRef),
        context: *mut c_void,
    );
    fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        runLoop: *mut c_void,
        runLoopMode: *mut c_void,
    );
    fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: u32) -> i32;
    fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> isize;
    fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: unsafe extern "C" fn(CGEventTapProxy, u32, CGEventRef, *mut c_void) -> CGEventRef,
        userInfo: *mut c_void,
    ) -> CFMachPortRef;
    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
}

pub fn is_accessibility_trusted(prompt: bool) -> bool {
    unsafe {
        let key = CFString::new("AXTrustedCheckOptionPrompt");
        let value = if prompt {
            CFBoolean::true_value()
        } else {
            CFBoolean::false_value()
        };
        let dict = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), value.as_CFType())]);
        AXIsProcessTrustedWithOptions(dict.as_concrete_TypeRef())
    }
}

struct MonitorState {
    config: Arc<AppConfig>,
    accumulated_mouse_dist: f64,
    accumulated_scroll: f64,
    accumulated_pinch: f64,
    accumulated_rotate: f32,
    last_haptic_time: Instant,
}

static MONITOR_REF: AtomicUsize = AtomicUsize::new(0);
static LOCAL_MONITOR_REF: AtomicUsize = AtomicUsize::new(0);
static KEYBOARD_TAP_REF: AtomicUsize = AtomicUsize::new(0);
static LAST_KEY_TIME: AtomicUsize = AtomicUsize::new(0);

fn process_mouse_gesture_event(event: Id, state_lock: &Mutex<MonitorState>) {
    unsafe {
        if event == NIL {
            return;
        }

        let event_type: usize = msg_send![event, type];
        let mut state = state_lock.lock().unwrap();

        if !state.config.is_enabled() {
            return;
        }

        let now = Instant::now();
        let min_interval = std::time::Duration::from_millis(state.config.get_min_interval_ms());
        let pattern = state.config.get_pattern();
        let output_mode = state.config.get_output_mode();

        // 1. Mouse Moved (5), LeftMouseDragged (6), RightMouseDragged (7), OtherMouseDragged (27)
        if event_type == 5 || event_type == 6 || event_type == 7 || event_type == 27 {
            if state.config.is_mouse_move_enabled() {
                let dx: f64 = msg_send![event, deltaX];
                let dy: f64 = msg_send![event, deltaY];
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > 0.0 && dist < 300.0 {
                    state.accumulated_mouse_dist += dist;
                    let threshold = state.config.get_mouse_sensitivity().mouse_threshold_pixels();

                    if state.accumulated_mouse_dist >= threshold {
                        if now.duration_since(state.last_haptic_time) >= min_interval {
                            perform_haptic(pattern, output_mode);
                            state.last_haptic_time = now;
                        }
                        state.accumulated_mouse_dist = 0.0;
                    }
                }
            }
        }

        // 2. Scroll Wheel (22)
        if event_type == 22 {
            if state.config.is_scroll_enabled() {
                let delta_y: f64 = msg_send![event, scrollingDeltaY];
                let delta_x: f64 = msg_send![event, scrollingDeltaX];
                let has_precise: bool = msg_send![event, hasPreciseScrollingDeltas];

                let scroll_magnitude = if has_precise {
                    delta_y.abs() + delta_x.abs()
                } else {
                    let dy: f64 = msg_send![event, deltaY];
                    let dx: f64 = msg_send![event, deltaX];
                    (dy.abs() + dx.abs()) * 6.0
                };

                if scroll_magnitude > 0.0 {
                    state.accumulated_scroll += scroll_magnitude;
                    let threshold = state.config.get_scroll_sensitivity().scroll_threshold_units();

                    if state.accumulated_scroll >= threshold {
                        if now.duration_since(state.last_haptic_time) >= min_interval {
                            perform_haptic(pattern, output_mode);
                            state.last_haptic_time = now;
                        }
                        state.accumulated_scroll = 0.0;
                    }
                }
            }
        }

        // 3. Multi-Touch Pinch to Zoom (Magnify = 30)
        if event_type == 30 {
            if state.config.is_gestures_enabled() {
                let mag: f64 = msg_send![event, magnification];
                state.accumulated_pinch += mag.abs();
                let threshold = state.config.get_mouse_sensitivity().pinch_threshold();

                if state.accumulated_pinch >= threshold {
                    if now.duration_since(state.last_haptic_time) >= min_interval {
                        perform_haptic(pattern, output_mode);
                        state.last_haptic_time = now;
                    }
                    state.accumulated_pinch = 0.0;
                }
            }
        }

        // 4. Multi-Touch Rotate (18)
        if event_type == 18 {
            if state.config.is_gestures_enabled() {
                let rot: f32 = msg_send![event, rotation];
                state.accumulated_rotate += rot.abs();
                let threshold = state.config.get_mouse_sensitivity().rotate_threshold_deg();

                if state.accumulated_rotate >= threshold {
                    if now.duration_since(state.last_haptic_time) >= min_interval {
                        perform_haptic(pattern, output_mode);
                        state.last_haptic_time = now;
                    }
                    state.accumulated_rotate = 0.0;
                }
            }
        }

        // 5. Multi-Touch Swipe (31)
        if event_type == 31 {
            if state.config.is_gestures_enabled() {
                if now.duration_since(state.last_haptic_time) >= min_interval {
                    perform_haptic(pattern, output_mode);
                    state.last_haptic_time = now;
                }
            }
        }
    }
}

fn trigger_keyboard_sound(key_code: u16, config: &AppConfig) {
    if !config.is_enabled() || !config.is_keyboard_sound_enabled() {
        return;
    }

    // Debounce duplicate events within 5ms (e.g. if both IOHID and CGEventTap trigger)
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as usize;

    let last = LAST_KEY_TIME.load(Ordering::Relaxed);
    if now_ms.saturating_sub(last) < 5 {
        return;
    }
    LAST_KEY_TIME.store(now_ms, Ordering::Relaxed);

    let profile = config.get_sound_profile();
    let vol = config.get_sound_volume();
    play_keyboard_sound(key_code, profile, vol);
}

unsafe extern "C" fn hid_keyboard_callback(
    context: *mut c_void,
    _result: i32,
    _sender: *mut c_void,
    value: IOHIDValueRef,
) {
    if context.is_null() || value.is_null() {
        return;
    }

    let elem = IOHIDValueGetElement(value);
    if elem.is_null() {
        return;
    }

    let page = IOHIDElementGetUsagePage(elem);
    let int_val = IOHIDValueGetIntegerValue(value);

    // 0x07 = kHIDPage_KeyboardOrKeypad, int_val == 1 is KeyDown
    if page == 0x07 && int_val == 1 {
        let usage = IOHIDElementGetUsage(elem);
        let config = &*(context as *const AppConfig);
        trigger_keyboard_sound(usage as u16, config);
    }
}

unsafe extern "C" fn cg_keyboard_callback(
    _proxy: CGEventTapProxy,
    event_type: u32,
    event: CGEventRef,
    user_data: *mut c_void,
) -> CGEventRef {
    if event.is_null() || user_data.is_null() {
        return event;
    }

    if event_type == 0xFFFFFFFE || event_type == 0xFFFFFFFF {
        let tap = KEYBOARD_TAP_REF.load(Ordering::Relaxed) as CFMachPortRef;
        if !tap.is_null() {
            CGEventTapEnable(tap, true);
        }
        return event;
    }

    if event_type == 10 { // 10 = kCGEventKeyDown
        let config = &*(user_data as *const AppConfig);
        let key_code = CGEventGetIntegerValueField(event, 9) as u16; // 9 = kCGKeyboardEventKeycode
        trigger_keyboard_sound(key_code, config);
    }

    event
}

pub fn check_and_request_accessibility() -> bool {
    // Check silently first without showing dialog
    if is_accessibility_trusted(false) {
        return true;
    }
    // Only prompt user if not already granted
    is_accessibility_trusted(true)
}

/// Sets up system-wide NSEvent monitors for gestures/mouse + IOHIDManager & CGEventTap for keyboard
pub fn start_event_tap(config: Arc<AppConfig>) -> Result<(), &'static str> {
    if !check_and_request_accessibility() {
        eprintln!("[Haptic] Accessibility permission requested. Please enable in System Settings.");
    }

    let raw_config = Arc::into_raw(Arc::clone(&config)) as *mut c_void;

    unsafe {
        let main_run_loop = CFRunLoopGetMain();

        // 1. Setup IOHIDManager directly on Main Run Loop (kCFRunLoopCommonModes)
        let hid_mgr = IOHIDManagerCreate(std::ptr::null_mut(), 0);
        if !hid_mgr.is_null() {
            IOHIDManagerSetDeviceMatchingMultiple(hid_mgr, std::ptr::null_mut());
            IOHIDManagerRegisterInputValueCallback(hid_mgr, hid_keyboard_callback, raw_config);
            IOHIDManagerScheduleWithRunLoop(
                hid_mgr,
                main_run_loop as _,
                kCFRunLoopCommonModes as _,
            );
            let open_res = IOHIDManagerOpen(hid_mgr, 0);
            println!("[Haptic] IOHIDManager scheduled on Main RunLoop (result: {}).", open_res);
        }

        // 2. Setup CGEventTap fallback directly on Main Run Loop
        let tap = CGEventTapCreate(
            1, // kCGSessionEventTap
            0, // kCGHeadInsertEventTap
            1, // kCGEventTapOptionListenOnly
            1 << 10, // kCGEventKeyDown
            cg_keyboard_callback,
            raw_config,
        );

        if !tap.is_null() {
            KEYBOARD_TAP_REF.store(tap as usize, Ordering::Relaxed);
            let loop_source = core_foundation::mach_port::CFMachPortCreateRunLoopSource(
                std::ptr::null_mut(),
                tap as _,
                0,
            );
            CFRunLoopAddSource(main_run_loop, loop_source, kCFRunLoopCommonModes);
            CGEventTapEnable(tap, true);
            println!("[Haptic] CGEventTap scheduled on Main RunLoop.");
        }

        // 3. NSEvent Monitors for Mouse and Multi-Touch Gestures
        let state = Arc::new(Mutex::new(MonitorState {
            config,
            accumulated_mouse_dist: 0.0,
            accumulated_scroll: 0.0,
            accumulated_pinch: 0.0,
            accumulated_rotate: 0.0,
            last_haptic_time: Instant::now(),
        }));

        let mask: u64 = (1 << 5)
            | (1 << 6)
            | (1 << 7)
            | (1 << 27)
            | (1 << 22)
            | (1 << 18)
            | (1 << 30)
            | (1 << 31);

        // Global Monitor
        let state_global = Arc::clone(&state);
        let global_block = ConcreteBlock::new(move |event: Id| {
            process_mouse_gesture_event(event, &state_global);
        });
        let global_block = global_block.copy();

        let global_monitor: Id = msg_send![
            class!(NSEvent),
            addGlobalMonitorForEventsMatchingMask: mask
            handler: &*global_block
        ];

        if global_monitor != NIL {
            let () = msg_send![global_monitor, retain];
            MONITOR_REF.store(global_monitor as usize, Ordering::Relaxed);
            println!("[Haptic] Global Multi-Touch & Mouse monitor active.");
        }

        // Local Monitor
        let state_local = Arc::clone(&state);
        let local_block = ConcreteBlock::new(move |event: Id| -> Id {
            process_mouse_gesture_event(event, &state_local);
            event
        });
        let local_block = local_block.copy();

        let local_monitor: Id = msg_send![
            class!(NSEvent),
            addLocalMonitorForEventsMatchingMask: mask
            handler: &*local_block
        ];

        if local_monitor != NIL {
            let () = msg_send![local_monitor, retain];
            LOCAL_MONITOR_REF.store(local_monitor as usize, Ordering::Relaxed);
            println!("[Haptic] Local monitor active.");
        }
    }

    Ok(())
}
