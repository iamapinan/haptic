use crate::config::AppConfig;
use crate::haptic::{perform_haptic, Id, NIL};
use block::ConcreteBlock;
use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use objc::{class, msg_send, sel, sel_impl};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: core_foundation::dictionary::CFDictionaryRef) -> bool;
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
    last_haptic_time: Instant,
}

static MONITOR_REF: AtomicUsize = AtomicUsize::new(0);
static LOCAL_MONITOR_REF: AtomicUsize = AtomicUsize::new(0);

fn process_event(event: Id, state_lock: &Mutex<MonitorState>) {
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
                            perform_haptic(pattern);
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
                            perform_haptic(pattern);
                            state.last_haptic_time = now;
                        }
                        state.accumulated_scroll = 0.0;
                    }
                }
            }
        }
    }
}

/// Sets up system-wide NSEvent global and local monitors
pub fn start_event_tap(config: Arc<AppConfig>) -> Result<(), &'static str> {
    if !is_accessibility_trusted(true) {
        eprintln!("[Haptic] Accessibility permission requested. Please enable in System Settings.");
    }

    let state = Arc::new(Mutex::new(MonitorState {
        config,
        accumulated_mouse_dist: 0.0,
        accumulated_scroll: 0.0,
        last_haptic_time: Instant::now(),
    }));

    unsafe {
        // Mask: MouseMoved (1 << 5), LeftMouseDragged (1 << 6), RightMouseDragged (1 << 7), OtherMouseDragged (1 << 27), ScrollWheel (1 << 22)
        let mask: u64 = (1 << 5) | (1 << 6) | (1 << 7) | (1 << 27) | (1 << 22);

        // 1. Global Monitor (catches events when ANY other app is active in foreground)
        let state_global = Arc::clone(&state);
        let global_block = ConcreteBlock::new(move |event: Id| {
            process_event(event, &state_global);
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
            println!("[Haptic] Global NSEvent monitor active (system-wide background tracking enabled).");
        } else {
            eprintln!("[Haptic] Warning: Failed to add global NSEvent monitor.");
        }

        // 2. Local Monitor (catches events when our menu/app is active)
        let state_local = Arc::clone(&state);
        let local_block = ConcreteBlock::new(move |event: Id| -> Id {
            process_event(event, &state_local);
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
            println!("[Haptic] Local NSEvent monitor active.");
        }
    }

    Ok(())
}
