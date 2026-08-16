use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

pub type Id = *mut Object;
pub const NIL: Id = std::ptr::null_mut();

#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HapticPattern {
    Generic = 0,     // Subtle tick
    Alignment = 1,   // Medium notch / snap
    LevelChange = 2, // Firm bump
}

impl HapticPattern {
    pub fn name(&self) -> &'static str {
        match self {
            HapticPattern::Generic => "Generic (Light)",
            HapticPattern::Alignment => "Alignment (Medium)",
            HapticPattern::LevelChange => "Level Change (Firm)",
        }
    }
}

/// Triggers macOS Force Touch / Taptic Engine haptic feedback on the trackpad
pub fn perform_haptic(pattern: HapticPattern) {
    unsafe {
        let cls = class!(NSHapticFeedbackManager);
        let performer: Id = msg_send![cls, defaultPerformer];
        if performer != NIL {
            let pattern_val: isize = pattern as isize;
            let time_val: usize = 1; // NSHapticFeedbackPerformanceTimeNow = 1
            let () = msg_send![performer, performFeedbackPattern:pattern_val performanceTime:time_val];
        }
    }
}
