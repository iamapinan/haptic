use crate::config::AppConfig;
use crate::haptic::perform_haptic;
use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

pub type CGEventRef = *mut c_void;
pub type CGEventTapProxy = *mut c_void;
pub type CFMachPortRef = *mut c_void;
pub type CFRunLoopRef = *mut c_void;
pub type CFRunLoopSourceRef = *mut c_void;
pub type CFAllocatorRef = *mut c_void;
pub type CFStringRef = *mut c_void;

pub type CGEventTapCallBack = unsafe extern "C" fn(
    proxy: CGEventTapProxy,
    event_type: u32,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: core_foundation::dictionary::CFDictionaryRef) -> bool;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        eventsOfInterest: u64,
        callback: CGEventTapCallBack,
        userInfo: *mut c_void,
    ) -> CFMachPortRef;

    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopCommonModes: CFStringRef;
    static kCFAllocatorDefault: CFAllocatorRef;

    fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        port: CFMachPortRef,
        order: isize,
    ) -> CFRunLoopSourceRef;

    fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    fn CFRunLoopRun();
}

/// Checks if Accessibility permissions are granted.
/// If `prompt` is true and permission is not granted, macOS displays a prompt to open System Settings.
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

// Struct to store accumulated tracking state in user_data
struct TapState {
    config: Arc<AppConfig>,
    tap_port: CFMachPortRef,
    last_mouse_x: f64,
    last_mouse_y: f64,
    mouse_has_prev: bool,
    accumulated_mouse_dist: f64,
    accumulated_scroll: f64,
    last_haptic_time: Instant,
}

// Event tap callback
unsafe extern "C" fn event_tap_callback(
    _proxy: CGEventTapProxy,
    event_type: u32,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef {
    if user_info.is_null() || event.is_null() {
        return event;
    }

    let state = &mut *(user_info as *mut TapState);

    // Handle tap disabled by system timeout or user input - re-enable using actual CFMachPortRef
    if event_type == 0xFFFFFFFE || event_type == 0xFFFFFFFF {
        if !state.tap_port.is_null() {
            CGEventTapEnable(state.tap_port, true);
        }
        return event;
    }

    // If global haptic is disabled, do nothing
    if !state.config.is_enabled() {
        return event;
    }

    let now = Instant::now();
    let min_interval = std::time::Duration::from_millis(state.config.get_min_interval_ms());
    let pattern = state.config.get_pattern();

    // 1. Mouse Moved Event (kCGEventMouseMoved = 5)
    if event_type == 5 {
        if state.config.is_mouse_move_enabled() {
            let point = CGEventGetLocation(event);
            if state.mouse_has_prev {
                let dx = point.x - state.last_mouse_x;
                let dy = point.y - state.last_mouse_y;
                let dist = (dx * dx + dy * dy).sqrt();

                // Ignore sudden giant leaps (e.g. cursor warp across multi-monitors)
                if dist < 500.0 {
                    state.accumulated_mouse_dist += dist;
                    let threshold = state.config.get_mouse_sensitivity().mouse_threshold_pixels();

                    if state.accumulated_mouse_dist >= threshold {
                        if now.duration_since(state.last_haptic_time) >= min_interval {
                            perform_haptic(pattern);
                            state.last_haptic_time = now;
                        }
                        state.accumulated_mouse_dist = 0.0;
                    }
                }
            }
            state.last_mouse_x = point.x;
            state.last_mouse_y = point.y;
            state.mouse_has_prev = true;
        }
    }

    // 2. Scroll Wheel Event (kCGEventScrollWheel = 22)
    if event_type == 22 {
        if state.config.is_scroll_enabled() {
            // kCGScrollWheelEventDeltaAxis1 = 11 (vertical), kCGScrollWheelEventDeltaAxis2 = 12 (horizontal)
            // Or point delta: kCGScrollWheelEventPointDeltaAxis1 = 96, kCGScrollWheelEventPointDeltaAxis2 = 97
            let delta_y = CGEventGetIntegerValueField(event, 11) as f64;
            let delta_x = CGEventGetIntegerValueField(event, 12) as f64;
            let pt_delta_y = CGEventGetIntegerValueField(event, 96) as f64;
            let pt_delta_x = CGEventGetIntegerValueField(event, 97) as f64;

            let scroll_magnitude = if pt_delta_y.abs() > 0.0 || pt_delta_x.abs() > 0.0 {
                pt_delta_y.abs() + pt_delta_x.abs()
            } else {
                (delta_y.abs() + delta_x.abs()) * 5.0
            };

            if scroll_magnitude > 0.0 {
                state.accumulated_scroll += scroll_magnitude;
                let threshold = state.config.get_scroll_sensitivity().scroll_threshold_units();

                if state.accumulated_scroll >= threshold {
                    if now.duration_since(state.last_haptic_time) >= min_interval {
                        perform_haptic(pattern);
                        state.last_haptic_time = now;
                    }
                    state.accumulated_scroll = 0.0;
                }
            }
        }
    }

    event
}

/// Starts the global event tap listener on a dedicated background thread.
pub fn start_event_tap(config: Arc<AppConfig>) -> Result<(), &'static str> {
    if !is_accessibility_trusted(true) {
        eprintln!("[Haptic] Accessibility permission not granted yet. Prompting user...");
    }

    std::thread::Builder::new()
        .name("haptic-event-tap".into())
        .spawn(move || unsafe {
            let state = Box::into_raw(Box::new(TapState {
                config,
                tap_port: std::ptr::null_mut(),
                last_mouse_x: 0.0,
                last_mouse_y: 0.0,
                mouse_has_prev: false,
                accumulated_mouse_dist: 0.0,
                accumulated_scroll: 0.0,
                last_haptic_time: Instant::now(),
            }));

            // kCGEventMouseMoved = 5, kCGEventScrollWheel = 22
            let mask: u64 = (1 << 5) | (1 << 22);

            let tap = CGEventTapCreate(
                1, // kCGSessionEventTap
                0, // kCGHeadInsertEventTap
                1, // kCGEventTapOptionListenOnly (no event blocking)
                mask,
                event_tap_callback,
                state as *mut c_void,
            );

            if tap.is_null() {
                eprintln!("[Haptic] Failed to create CGEventTap. Please grant Accessibility permissions in System Settings.");
                return;
            }

            // Save the tap port in state so event_tap_callback can safely re-enable it on timeout
            (*state).tap_port = tap;

            let loop_source = CFMachPortCreateRunLoopSource(
                kCFAllocatorDefault,
                tap,
                0,
            );

            if loop_source.is_null() {
                eprintln!("[Haptic] Failed to create CFRunLoopSource.");
                return;
            }

            let current_loop = CFRunLoopGetCurrent();
            CFRunLoopAddSource(
                current_loop,
                loop_source,
                kCFRunLoopCommonModes,
            );

            CGEventTapEnable(tap, true);
            println!("[Haptic] Event Tap active and listening for mouse movement & scroll events.");

            // Run the background event loop
            CFRunLoopRun();
        })
        .map_err(|_| "Failed to spawn event tap thread")?;

    Ok(())
}
