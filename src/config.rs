use crate::haptic::HapticPattern;
use crate::sound::SoundProfile;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HapticOutputMode {
    TrackpadOnly = 0,
    SpeakerOnly = 1,
    Both = 2,
}

impl HapticOutputMode {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => HapticOutputMode::TrackpadOnly,
            1 => HapticOutputMode::SpeakerOnly,
            2 => HapticOutputMode::Both,
            _ => HapticOutputMode::TrackpadOnly,
        }
    }
}

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

fn get_launch_agent_path() -> Option<std::path::PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(std::path::PathBuf::from(home).join("Library/LaunchAgents/com.apinan.haptic.plist"))
}

pub fn is_launch_at_login_installed() -> bool {
    if let Some(path) = get_launch_agent_path() {
        path.exists()
    } else {
        false
    }
}

pub fn set_launch_at_login_system(enabled: bool) {
    if let Some(path) = get_launch_agent_path() {
        if enabled {
            let exe_path = if std::path::Path::new("/Applications/Haptic.app/Contents/MacOS/haptic-mac").exists() {
                "/Applications/Haptic.app/Contents/MacOS/haptic-mac".to_string()
            } else if let Ok(current_exe) = std::env::current_exe() {
                current_exe.to_string_lossy().to_string()
            } else {
                "/Applications/Haptic.app/Contents/MacOS/haptic-mac".to_string()
            };

            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.apinan.haptic</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>
"#,
                exe_path
            );

            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, plist_content);
            println!("[Haptic] Launch at login enabled: {:?}", path);
        } else {
            let _ = std::fs::remove_file(&path);
            println!("[Haptic] Launch at login disabled.");
        }
    }
}

pub struct AppConfig {
    pub enabled: AtomicBool,
    pub mouse_move_enabled: AtomicBool,
    pub scroll_enabled: AtomicBool,
    pub gestures_enabled: AtomicBool,
    pub keyboard_sound_enabled: AtomicBool,
    pub launch_at_login: AtomicBool,
    pub output_mode: AtomicU8,          // 0 = Trackpad, 1 = Speaker Tick, 2 = Both
    pub pattern: AtomicU8,              // 0 = Generic, 1 = Alignment, 2 = LevelChange
    pub mouse_sensitivity: AtomicU8,    // 0 = High, 1 = Medium, 2 = Low
    pub scroll_sensitivity: AtomicU8,   // 0 = High, 1 = Medium, 2 = Low
    pub sound_profile: AtomicU8,        // 0 = Grand Piano, 1 = Drum Kit, 2 = Marimba, 3 = Thock, 4 = Blue, 5 = Typewriter
    pub sound_volume: AtomicU8,         // 0 - 100 (Default: 70)
    pub min_interval_ms: AtomicU64,     // minimum ms between consecutive haptic pulses
}

impl AppConfig {
    pub fn new() -> Arc<Self> {
        let launch_login = is_launch_at_login_installed();
        Arc::new(Self {
            enabled: AtomicBool::new(true),
            mouse_move_enabled: AtomicBool::new(true),
            scroll_enabled: AtomicBool::new(true),
            gestures_enabled: AtomicBool::new(true),
            keyboard_sound_enabled: AtomicBool::new(true),
            launch_at_login: AtomicBool::new(launch_login),
            output_mode: AtomicU8::new(0), // Trackpad Only
            pattern: AtomicU8::new(0), // Generic
            mouse_sensitivity: AtomicU8::new(1), // Medium
            scroll_sensitivity: AtomicU8::new(1), // Medium
            sound_profile: AtomicU8::new(0), // Grand Piano (Concert Acoustic)
            sound_volume: AtomicU8::new(75), // 75% volume
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

    pub fn is_keyboard_sound_enabled(&self) -> bool {
        self.keyboard_sound_enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_keyboard_sound(&self) -> bool {
        let prev = self.keyboard_sound_enabled.fetch_xor(true, Ordering::SeqCst);
        !prev
    }

    pub fn is_launch_at_login(&self) -> bool {
        self.launch_at_login.load(Ordering::Relaxed)
    }

    pub fn toggle_launch_at_login(&self) -> bool {
        let next = !self.is_launch_at_login();
        self.launch_at_login.store(next, Ordering::SeqCst);
        set_launch_at_login_system(next);
        next
    }

    pub fn get_output_mode(&self) -> HapticOutputMode {
        HapticOutputMode::from_u8(self.output_mode.load(Ordering::Relaxed))
    }

    pub fn set_output_mode(&self, mode: HapticOutputMode) {
        self.output_mode.store(mode as u8, Ordering::Relaxed);
    }

    pub fn get_pattern(&self) -> HapticPattern {
        HapticPattern::from_u8(self.pattern.load(Ordering::Relaxed))
    }

    pub fn set_pattern(&self, pattern: HapticPattern) {
        self.pattern.store(pattern.to_u8(), Ordering::Relaxed);
    }

    pub fn get_mouse_sensitivity(&self) -> Sensitivity {
        Sensitivity::from_u8(self.mouse_sensitivity.load(Ordering::Relaxed))
    }

    pub fn set_mouse_sensitivity(&self, sens: Sensitivity) {
        self.mouse_sensitivity.store(sens.to_u8(), Ordering::Relaxed);
    }

    pub fn get_scroll_sensitivity(&self) -> Sensitivity {
        Sensitivity::from_u8(self.scroll_sensitivity.load(Ordering::Relaxed))
    }

    pub fn set_scroll_sensitivity(&self, sens: Sensitivity) {
        self.scroll_sensitivity.store(sens.to_u8(), Ordering::Relaxed);
    }

    pub fn get_sound_profile(&self) -> SoundProfile {
        SoundProfile::from_u8(self.sound_profile.load(Ordering::Relaxed))
    }

    pub fn set_sound_profile(&self, profile: SoundProfile) {
        self.sound_profile.store(profile as u8, Ordering::Relaxed);
    }

    pub fn get_sound_volume(&self) -> u8 {
        self.sound_volume.load(Ordering::Relaxed)
    }

    pub fn set_sound_volume(&self, vol: u8) {
        self.sound_volume.store(vol.min(100), Ordering::Relaxed);
    }

    pub fn get_min_interval_ms(&self) -> u64 {
        self.min_interval_ms.load(Ordering::Relaxed)
    }
}
