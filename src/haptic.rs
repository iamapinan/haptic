use crate::config::HapticOutputMode;
use crate::sound::play_haptic_audio_tick;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::{c_void, CString};
use std::sync::Mutex;

pub type Id = *mut Object;
pub const NIL: Id = std::ptr::null_mut();

type MTDeviceRef = *mut c_void;
type MTActuatorRef = *mut c_void;

type MTDeviceCreateListFn = unsafe extern "C" fn() -> *mut c_void;
type MTDeviceCreateDefaultFn = unsafe extern "C" fn() -> MTDeviceRef;
type MTDeviceGetDeviceIDFn = unsafe extern "C" fn(MTDeviceRef, *mut u64) -> i32;
type MTActuatorCreateFromDeviceIDFn = unsafe extern "C" fn(u64) -> MTActuatorRef;
type MTActuatorOpenFn = unsafe extern "C" fn(MTActuatorRef) -> i32;
type MTActuatorActuateFn = unsafe extern "C" fn(MTActuatorRef, i32, u32) -> i32;

extern "C" {
    fn dlopen(filename: *const i8, flag: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const i8) -> *mut c_void;
    fn CFArrayGetCount(theArray: *mut c_void) -> isize;
    fn CFArrayGetValueAtIndex(theArray: *mut c_void, idx: isize) -> *mut c_void;
}

#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HapticPattern {
    Generic = 0,     // Subtle tick
    Alignment = 1,   // Medium notch / snap
    LevelChange = 2, // Firm bump
}

impl HapticPattern {
    pub fn actuator_id(&self) -> i32 {
        match self {
            HapticPattern::Generic => 1,     // Light / subtle tick
            HapticPattern::Alignment => 2,   // Medium click
            HapticPattern::LevelChange => 3, // Firm bump
        }
    }
}

struct MultitouchHapticEngine {
    actuators: Vec<MTActuatorRef>,
    actuate_fn: Option<MTActuatorActuateFn>,
    initialized: bool,
}

unsafe impl Send for MultitouchHapticEngine {}
unsafe impl Sync for MultitouchHapticEngine {}

static GLOBAL_ENGINE: Mutex<Option<MultitouchHapticEngine>> = Mutex::new(None);

fn get_or_init_engine() -> bool {
    let mut guard = GLOBAL_ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        if engine.initialized && !engine.actuators.is_empty() {
            return true;
        }
    }

    unsafe {
        let path = CString::new(
            "/System/Library/PrivateFrameworks/MultitouchSupport.framework/MultitouchSupport",
        )
        .unwrap();
        let handle = dlopen(path.as_ptr(), 1);
        if handle.is_null() {
            return false;
        }

        let create_list_sym = CString::new("MTDeviceCreateList").unwrap();
        let create_default_sym = CString::new("MTDeviceCreateDefault").unwrap();
        let get_device_id_sym = CString::new("MTDeviceGetDeviceID").unwrap();
        let create_actuator_sym = CString::new("MTActuatorCreateFromDeviceID").unwrap();
        let open_actuator_sym = CString::new("MTActuatorOpen").unwrap();
        let actuate_sym = CString::new("MTActuatorActuate").unwrap();

        let create_list_ptr = dlsym(handle, create_list_sym.as_ptr());
        let create_default_ptr = dlsym(handle, create_default_sym.as_ptr());
        let get_device_id_ptr = dlsym(handle, get_device_id_sym.as_ptr());
        let create_actuator_ptr = dlsym(handle, create_actuator_sym.as_ptr());
        let open_actuator_ptr = dlsym(handle, open_actuator_sym.as_ptr());
        let actuate_ptr = dlsym(handle, actuate_sym.as_ptr());

        if get_device_id_ptr.is_null() || create_actuator_ptr.is_null() || open_actuator_ptr.is_null() || actuate_ptr.is_null() {
            return false;
        }

        let create_list: Option<MTDeviceCreateListFn> = if !create_list_ptr.is_null() {
            Some(std::mem::transmute(create_list_ptr))
        } else {
            None
        };
        let create_default: Option<MTDeviceCreateDefaultFn> = if !create_default_ptr.is_null() {
            Some(std::mem::transmute(create_default_ptr))
        } else {
            None
        };
        let get_device_id: MTDeviceGetDeviceIDFn = std::mem::transmute(get_device_id_ptr);
        let create_actuator: MTActuatorCreateFromDeviceIDFn = std::mem::transmute(create_actuator_ptr);
        let open_actuator: MTActuatorOpenFn = std::mem::transmute(open_actuator_ptr);
        let actuate_fn: MTActuatorActuateFn = std::mem::transmute(actuate_ptr);

        let mut actuators = Vec::new();

        // 1. Try enumerating all connected multitouch devices (Trackpads)
        if let Some(create_list_fn) = create_list {
            let list = create_list_fn();
            if !list.is_null() {
                let count = CFArrayGetCount(list);
                for i in 0..count {
                    let dev = CFArrayGetValueAtIndex(list, i) as MTDeviceRef;
                    if !dev.is_null() {
                        let mut dev_id: u64 = 0;
                        if get_device_id(dev, &mut dev_id) == 0 && dev_id != 0 {
                            let actuator = create_actuator(dev_id);
                            if !actuator.is_null() {
                                open_actuator(actuator);
                                actuators.push(actuator);
                            }
                        }
                    }
                }
            }
        }

        // 2. Fallback to default device if list was empty
        if actuators.is_empty() {
            if let Some(create_default_fn) = create_default {
                let dev = create_default_fn();
                if !dev.is_null() {
                    let mut dev_id: u64 = 0;
                    if get_device_id(dev, &mut dev_id) == 0 && dev_id != 0 {
                        let actuator = create_actuator(dev_id);
                        if !actuator.is_null() {
                            open_actuator(actuator);
                            actuators.push(actuator);
                        }
                    }
                }
            }
        }

        println!("[Haptic] Initialized {} global hardware actuator(s).", actuators.len());

        *guard = Some(MultitouchHapticEngine {
            actuators,
            actuate_fn: Some(actuate_fn),
            initialized: true,
        });

        true
    }
}

/// Triggers haptic feedback based on output mode (Trackpad / Speaker / Both)
pub fn perform_haptic(pattern: HapticPattern, mode: HapticOutputMode) {
    // 1. Speaker audio tick simulation (if SpeakerOnly or Both)
    if mode == HapticOutputMode::SpeakerOnly || mode == HapticOutputMode::Both {
        play_haptic_audio_tick(pattern);
    }

    // 2. Physical Trackpad actuation (if TrackpadOnly or Both)
    if mode == HapticOutputMode::TrackpadOnly || mode == HapticOutputMode::Both {
        if get_or_init_engine() {
            let guard = GLOBAL_ENGINE.lock().unwrap();
            if let Some(engine) = guard.as_ref() {
                if let Some(actuate) = engine.actuate_fn {
                    let act_id = pattern.actuator_id();
                    let mut actuated = false;
                    for &actuator in &engine.actuators {
                        unsafe {
                            if actuate(actuator, act_id, 0) == 0 {
                                actuated = true;
                            }
                        }
                    }
                    if actuated {
                        return;
                    }
                }
            }
        }

        // Fallback to AppKit NSHapticFeedbackManager
        unsafe {
            let cls = class!(NSHapticFeedbackManager);
            let performer: Id = msg_send![cls, defaultPerformer];
            if performer != NIL {
                let pattern_val: isize = pattern as isize;
                let time_val: usize = 1;
                let () = msg_send![performer, performFeedbackPattern:pattern_val performanceTime:time_val];
            }
        }
    }
}
