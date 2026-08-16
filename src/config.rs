use crate::haptic::HapticPattern;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Sensitivity {
    High,
    Medium,
    Low,
}

impl Sensitivity {
    pub fn mouse_threshold_pixels(&self) -> f64 {
        match self {
            Sensitivity::High => 15.0,
            Sensitivity::Medium => 30.0,
            Sensitivity::Low => 60.0,
        }
    }

    pub fn scroll_threshold_units(&self) -> f64 {
        match self {
            Sensitivity::High => 3.0,
            Sensitivity::Medium => 7.0,
            Sensitivity::Low => 18.0,
        }
    }

    pub fn pinch_threshold(&self) -> f64 {
        match self {
            Sensitivity::High => 0.025,
            Sensitivity::Medium => 0.05,
            Sensitivity::Low => 0.10,
        }
    }

    pub fn rotate_threshold_deg(&self) -> f32 {
        match self {
            Sensitivity::High => 10.0,
            Sensitivity::Medium => 20.0,
            Sensitivity::Low => 35.0,
        }
    }

    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => Sensitivity::High,
            1 => Sensitivity::Medium,
            2 => Sensitivity::Low,
            _ => Sensitivity::Medium,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Sensitivity::High => 0,
            Sensitivity::Medium => 1,
            Sensitivity::Low => 2,
        }
    }
}

pub struct AppConfig {
    pub enabled: AtomicBool,
    pub mouse_move_enabled: AtomicBool,
    pub scroll_enabled: AtomicBool,
    pub gestures_enabled: AtomicBool,
    pub pattern: AtomicU8,             // 0 = Generic, 1 = Alignment, 2 = LevelChange
    pub mouse_sensitivity: AtomicU8,   // 0 = High, 1 = Medium, 2 = Low
    pub scroll_sensitivity: AtomicU8,  // 0 = High, 1 = Medium, 2 = Low
    pub min_interval_ms: AtomicU64,    // minimum ms between consecutive haptic pulses
}

impl AppConfig {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            enabled: AtomicBool::new(true),
            mouse_move_enabled: AtomicBool::new(true),
            scroll_enabled: AtomicBool::new(true),
            gestures_enabled: AtomicBool::new(true),
            pattern: AtomicU8::new(0), // Generic
            mouse_sensitivity: AtomicU8::new(1), // Medium
            scroll_sensitivity: AtomicU8::new(1), // Medium
            min_interval_ms: AtomicU64::new(25), // 25ms limit
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_enabled(&self) -> bool {
        let prev = self.enabled.fetch_xor(true, Ordering::SeqCst);
        !prev
    }

    pub fn is_mouse_move_enabled(&self) -> bool {
        self.mouse_move_enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_mouse_move(&self) -> bool {
        let prev = self.mouse_move_enabled.fetch_xor(true, Ordering::SeqCst);
        !prev
    }

    pub fn is_scroll_enabled(&self) -> bool {
        self.scroll_enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_scroll(&self) -> bool {
        let prev = self.scroll_enabled.fetch_xor(true, Ordering::SeqCst);
        !prev
    }

    pub fn is_gestures_enabled(&self) -> bool {
        self.gestures_enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_gestures(&self) -> bool {
        let prev = self.gestures_enabled.fetch_xor(true, Ordering::SeqCst);
        !prev
    }

    pub fn get_pattern(&self) -> HapticPattern {
        match self.pattern.load(Ordering::Relaxed) {
            0 => HapticPattern::Generic,
            1 => HapticPattern::Alignment,
            2 => HapticPattern::LevelChange,
            _ => HapticPattern::Generic,
        }
    }

    pub fn set_pattern(&self, pattern: HapticPattern) {
        self.pattern.store(pattern as u8, Ordering::SeqCst);
    }

    pub fn get_mouse_sensitivity(&self) -> Sensitivity {
        Sensitivity::from_u8(self.mouse_sensitivity.load(Ordering::Relaxed))
    }

    pub fn set_mouse_sensitivity(&self, sens: Sensitivity) {
        self.mouse_sensitivity.store(sens.to_u8(), Ordering::SeqCst);
    }

    pub fn get_scroll_sensitivity(&self) -> Sensitivity {
        Sensitivity::from_u8(self.scroll_sensitivity.load(Ordering::Relaxed))
    }

    pub fn set_scroll_sensitivity(&self, sens: Sensitivity) {
        self.scroll_sensitivity.store(sens.to_u8(), Ordering::SeqCst);
    }

    pub fn get_min_interval_ms(&self) -> u64 {
        self.min_interval_ms.load(Ordering::Relaxed)
    }
}
