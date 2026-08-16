use crate::config::{AppConfig, HapticOutputMode, Sensitivity};
use crate::event_tap::is_accessibility_trusted;
use crate::haptic::{perform_haptic, HapticPattern, Id, NIL};
use crate::sound::{init_sound_engine, play_keyboard_sound, SoundProfile};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CString;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once, OnceLock};

pub const NO: i8 = 0;

static INIT_DELEGATE: Once = Once::new();
static GLOBAL_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();

static DELEGATE_REF: AtomicUsize = AtomicUsize::new(0);
static STATUS_ITEM_REF: AtomicUsize = AtomicUsize::new(0);
static MENU_REF: AtomicUsize = AtomicUsize::new(0);
static PATTERN_MENU_REF: AtomicUsize = AtomicUsize::new(0);
static OUTPUT_MODE_MENU_REF: AtomicUsize = AtomicUsize::new(0);
static MOUSE_MENU_REF: AtomicUsize = AtomicUsize::new(0);
static SCROLL_MENU_REF: AtomicUsize = AtomicUsize::new(0);
static SOUND_PROFILE_MENU_REF: AtomicUsize = AtomicUsize::new(0);
static SOUND_VOL_MENU_REF: AtomicUsize = AtomicUsize::new(0);

// Menu Item Tags
const TAG_STATUS_HEADER: isize = 100;
const TAG_ENABLE_ALL: isize = 101;
const TAG_ENABLE_MOUSE: isize = 102;
const TAG_ENABLE_SCROLL: isize = 103;
const TAG_ENABLE_GESTURES: isize = 104;
const TAG_ENABLE_KEYBOARD: isize = 105;

const TAG_MODE_TRACKPAD: isize = 151;
const TAG_MODE_SPEAKER: isize = 152;
const TAG_MODE_BOTH: isize = 153;

const TAG_PAT_GENERIC: isize = 201;
const TAG_PAT_ALIGNMENT: isize = 202;
const TAG_PAT_LEVEL: isize = 203;

const TAG_MOUSE_HIGH: isize = 301;
const TAG_MOUSE_MED: isize = 302;
const TAG_MOUSE_LOW: isize = 303;

const TAG_SCROLL_HIGH: isize = 401;
const TAG_SCROLL_MED: isize = 402;
const TAG_SCROLL_LOW: isize = 403;

const TAG_SND_THOCK: isize = 501;
const TAG_SND_BLUE: isize = 502;
const TAG_SND_TYPEWRITER: isize = 503;

const TAG_VOL_100: isize = 601;
const TAG_VOL_70: isize = 602;
const TAG_VOL_40: isize = 603;
const TAG_VOL_15: isize = 604;
const TAG_VOL_0: isize = 605;

pub fn create_ns_string(s: &str) -> Id {
    unsafe {
        let c_str = CString::new(s).unwrap_or_else(|_| CString::new("").unwrap());
        let ns_str: Id = msg_send![class!(NSString), stringWithUTF8String: c_str.as_ptr()];
        ns_str
    }
}

fn get_config() -> &'static Arc<AppConfig> {
    GLOBAL_CONFIG
        .get()
        .expect("GLOBAL_CONFIG must be initialized")
}

pub fn update_menu_state() {
    unsafe {
        let menu: Id = MENU_REF.load(Ordering::Relaxed) as Id;
        let pattern_menu: Id = PATTERN_MENU_REF.load(Ordering::Relaxed) as Id;
        let output_mode_menu: Id = OUTPUT_MODE_MENU_REF.load(Ordering::Relaxed) as Id;
        let mouse_menu: Id = MOUSE_MENU_REF.load(Ordering::Relaxed) as Id;
        let scroll_menu: Id = SCROLL_MENU_REF.load(Ordering::Relaxed) as Id;
        let sound_profile_menu: Id = SOUND_PROFILE_MENU_REF.load(Ordering::Relaxed) as Id;
        let sound_vol_menu: Id = SOUND_VOL_MENU_REF.load(Ordering::Relaxed) as Id;
        let status_item: Id = STATUS_ITEM_REF.load(Ordering::Relaxed) as Id;

        if menu == NIL {
            return;
        }

        let config = get_config();
        let enabled = config.is_enabled();
        let mouse_enabled = config.is_mouse_move_enabled();
        let scroll_enabled = config.is_scroll_enabled();
        let gestures_enabled = config.is_gestures_enabled();
        let keyboard_sound_enabled = config.is_keyboard_sound_enabled();
        let output_mode = config.get_output_mode();
        let pattern = config.get_pattern();
        let mouse_sens = config.get_mouse_sensitivity();
        let scroll_sens = config.get_scroll_sensitivity();
        let sound_profile = config.get_sound_profile();
        let sound_vol = config.get_sound_volume();

        // Update Status bar title
        if status_item != NIL {
            let button: Id = msg_send![status_item, button];
            if button != NIL {
                let title = if enabled { "⚡️" } else { "💤" };
                let ns_title = create_ns_string(title);
                let () = msg_send![button, setTitle: ns_title];
            }
        }

        // Helper to set item state on a specific menu
        let set_item_state = |target_menu: Id, tag: isize, is_on: bool| {
            if target_menu != NIL {
                let item: Id = msg_send![target_menu, itemWithTag: tag];
                if item != NIL {
                    let state: isize = if is_on { 1 } else { 0 };
                    let () = msg_send![item, setState: state];
                }
            }
        };

        // Helper to set item title on main menu
        let set_item_title = |target_menu: Id, tag: isize, text: &str| {
            if target_menu != NIL {
                let item: Id = msg_send![target_menu, itemWithTag: tag];
                if item != NIL {
                    let ns_text = create_ns_string(text);
                    let () = msg_send![item, setTitle: ns_text];
                }
            }
        };

        let status_text = if enabled {
            "Status: Active (Running)"
        } else {
            "Status: Paused (Off)"
        };
        set_item_title(menu, TAG_STATUS_HEADER, status_text);

        set_item_state(menu, TAG_ENABLE_ALL, enabled);
        set_item_state(menu, TAG_ENABLE_MOUSE, mouse_enabled);
        set_item_state(menu, TAG_ENABLE_SCROLL, scroll_enabled);
        set_item_state(menu, TAG_ENABLE_GESTURES, gestures_enabled);
        set_item_state(menu, TAG_ENABLE_KEYBOARD, keyboard_sound_enabled);

        // Update Output Mode submenu
        set_item_state(output_mode_menu, TAG_MODE_TRACKPAD, output_mode == HapticOutputMode::TrackpadOnly);
        set_item_state(output_mode_menu, TAG_MODE_SPEAKER, output_mode == HapticOutputMode::SpeakerOnly);
        set_item_state(output_mode_menu, TAG_MODE_BOTH, output_mode == HapticOutputMode::Both);

        // Update Pattern submenu
        set_item_state(pattern_menu, TAG_PAT_GENERIC, pattern == HapticPattern::Generic);
        set_item_state(pattern_menu, TAG_PAT_ALIGNMENT, pattern == HapticPattern::Alignment);
        set_item_state(pattern_menu, TAG_PAT_LEVEL, pattern == HapticPattern::LevelChange);

        // Update Mouse Sensitivity submenu
        set_item_state(mouse_menu, TAG_MOUSE_HIGH, mouse_sens == Sensitivity::High);
        set_item_state(mouse_menu, TAG_MOUSE_MED, mouse_sens == Sensitivity::Medium);
        set_item_state(mouse_menu, TAG_MOUSE_LOW, mouse_sens == Sensitivity::Low);

        // Update Scroll Sensitivity submenu
        set_item_state(scroll_menu, TAG_SCROLL_HIGH, scroll_sens == Sensitivity::High);
        set_item_state(scroll_menu, TAG_SCROLL_MED, scroll_sens == Sensitivity::Medium);
        set_item_state(scroll_menu, TAG_SCROLL_LOW, scroll_sens == Sensitivity::Low);

        // Update Sound Profile submenu
        set_item_state(sound_profile_menu, TAG_SND_THOCK, sound_profile == SoundProfile::DeepThock);
        set_item_state(sound_profile_menu, TAG_SND_BLUE, sound_profile == SoundProfile::ClickyBlue);
        set_item_state(sound_profile_menu, TAG_SND_TYPEWRITER, sound_profile == SoundProfile::Typewriter);

        // Update Sound Volume submenu
        set_item_state(sound_vol_menu, TAG_VOL_100, sound_vol >= 90);
        set_item_state(sound_vol_menu, TAG_VOL_70, sound_vol >= 60 && sound_vol < 90);
        set_item_state(sound_vol_menu, TAG_VOL_40, sound_vol >= 30 && sound_vol < 60);
        set_item_state(sound_vol_menu, TAG_VOL_15, sound_vol > 0 && sound_vol < 30);
        set_item_state(sound_vol_menu, TAG_VOL_0, sound_vol == 0);
    }
}

// Action Handlers
extern "C" fn on_toggle_enabled(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        let new_val = config.toggle_enabled();
        if new_val {
            perform_haptic(config.get_pattern(), config.get_output_mode());
        }
        update_menu_state();
    });
}

extern "C" fn on_toggle_mouse(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        let new_val = config.toggle_mouse_move();
        if new_val {
            perform_haptic(config.get_pattern(), config.get_output_mode());
        }
        update_menu_state();
    });
}

extern "C" fn on_toggle_scroll(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        let new_val = config.toggle_scroll();
        if new_val {
            perform_haptic(config.get_pattern(), config.get_output_mode());
        }
        update_menu_state();
    });
}

extern "C" fn on_toggle_gestures(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        let new_val = config.toggle_gestures();
        if new_val {
            perform_haptic(config.get_pattern(), config.get_output_mode());
        }
        update_menu_state();
    });
}

extern "C" fn on_toggle_keyboard(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        let new_val = config.toggle_keyboard_sound();
        if new_val {
            play_keyboard_sound(49, config.get_sound_profile(), config.get_sound_volume());
        }
        update_menu_state();
    });
}

// Output mode handlers
extern "C" fn on_set_mode_trackpad(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_output_mode(HapticOutputMode::TrackpadOnly);
        perform_haptic(config.get_pattern(), HapticOutputMode::TrackpadOnly);
        update_menu_state();
    });
}

extern "C" fn on_set_mode_speaker(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_output_mode(HapticOutputMode::SpeakerOnly);
        perform_haptic(config.get_pattern(), HapticOutputMode::SpeakerOnly);
        update_menu_state();
    });
}

extern "C" fn on_set_mode_both(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_output_mode(HapticOutputMode::Both);
        perform_haptic(config.get_pattern(), HapticOutputMode::Both);
        update_menu_state();
    });
}

extern "C" fn on_set_pattern_generic(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_pattern(HapticPattern::Generic);
        perform_haptic(HapticPattern::Generic, config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_pattern_alignment(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_pattern(HapticPattern::Alignment);
        perform_haptic(HapticPattern::Alignment, config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_pattern_level(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_pattern(HapticPattern::LevelChange);
        perform_haptic(HapticPattern::LevelChange, config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_mouse_sens_high(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_mouse_sensitivity(Sensitivity::High);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_mouse_sens_med(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_mouse_sensitivity(Sensitivity::Medium);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_mouse_sens_low(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_mouse_sensitivity(Sensitivity::Low);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_scroll_sens_high(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_scroll_sensitivity(Sensitivity::High);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_scroll_sens_med(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_scroll_sensitivity(Sensitivity::Medium);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

extern "C" fn on_set_scroll_sens_low(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_scroll_sensitivity(Sensitivity::Low);
        perform_haptic(config.get_pattern(), config.get_output_mode());
        update_menu_state();
    });
}

// Sound handlers
extern "C" fn on_set_sound_thock(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_profile(SoundProfile::DeepThock);
        play_keyboard_sound(49, SoundProfile::DeepThock, config.get_sound_volume());
        update_menu_state();
    });
}

extern "C" fn on_set_sound_blue(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_profile(SoundProfile::ClickyBlue);
        play_keyboard_sound(49, SoundProfile::ClickyBlue, config.get_sound_volume());
        update_menu_state();
    });
}

extern "C" fn on_set_sound_typewriter(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_profile(SoundProfile::Typewriter);
        play_keyboard_sound(49, SoundProfile::Typewriter, config.get_sound_volume());
        update_menu_state();
    });
}

extern "C" fn on_set_vol_100(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_volume(100);
        play_keyboard_sound(49, config.get_sound_profile(), 100);
        update_menu_state();
    });
}

extern "C" fn on_set_vol_70(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_volume(70);
        play_keyboard_sound(49, config.get_sound_profile(), 70);
        update_menu_state();
    });
}

extern "C" fn on_set_vol_40(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_volume(40);
        play_keyboard_sound(49, config.get_sound_profile(), 40);
        update_menu_state();
    });
}

extern "C" fn on_set_vol_15(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_volume(15);
        play_keyboard_sound(49, config.get_sound_profile(), 15);
        update_menu_state();
    });
}

extern "C" fn on_set_vol_0(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        config.set_sound_volume(0);
        update_menu_state();
    });
}

extern "C" fn on_test_haptic(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        let config = get_config();
        perform_haptic(config.get_pattern(), config.get_output_mode());
        play_keyboard_sound(49, config.get_sound_profile(), config.get_sound_volume());
    });
}

extern "C" fn on_check_accessibility(_this: &Object, _cmd: Sel, _sender: Id) {
    let _ = std::panic::catch_unwind(|| {
        if is_accessibility_trusted(false) {
            println!("[Haptic] Accessibility permission is already granted.");
        } else {
            let _ = is_accessibility_trusted(true);
        }
    });
}

extern "C" fn on_menu_will_open(_this: &Object, _cmd: Sel, _menu: Id) {
    let _ = std::panic::catch_unwind(|| {
        update_menu_state();
    });
}

extern "C" fn on_quit(_this: &Object, _cmd: Sel, _sender: Id) {
    unsafe {
        let app: Id = msg_send![class!(NSApplication), sharedApplication];
        let () = msg_send![app, terminate: NIL];
    }
}

fn register_action_handler_class() -> &'static Class {
    INIT_DELEGATE.call_once(|| {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("HapticMenuDelegate", superclass)
            .expect("Failed to declare HapticMenuDelegate class");

        unsafe {
            decl.add_method(sel!(toggleEnabled:), on_toggle_enabled as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(toggleMouseMove:), on_toggle_mouse as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(toggleScroll:), on_toggle_scroll as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(toggleGestures:), on_toggle_gestures as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(toggleKeyboard:), on_toggle_keyboard as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setModeTrackpad:), on_set_mode_trackpad as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setModeSpeaker:), on_set_mode_speaker as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setModeBoth:), on_set_mode_both as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setPatternGeneric:), on_set_pattern_generic as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setPatternAlignment:), on_set_pattern_alignment as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setPatternLevel:), on_set_pattern_level as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setMouseSensHigh:), on_set_mouse_sens_high as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setMouseSensMed:), on_set_mouse_sens_med as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setMouseSensLow:), on_set_mouse_sens_low as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setScrollSensHigh:), on_set_scroll_sens_high as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setScrollSensMed:), on_set_scroll_sens_med as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setScrollSensLow:), on_set_scroll_sens_low as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setSoundThock:), on_set_sound_thock as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setSoundBlue:), on_set_sound_blue as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setSoundTypewriter:), on_set_sound_typewriter as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(setVol100:), on_set_vol_100 as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setVol70:), on_set_vol_70 as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setVol40:), on_set_vol_40 as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setVol15:), on_set_vol_15 as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(setVol0:), on_set_vol_0 as extern "C" fn(&Object, Sel, Id));

            decl.add_method(sel!(testHaptic:), on_test_haptic as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(checkAccessibility:), on_check_accessibility as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(menuWillOpen:), on_menu_will_open as extern "C" fn(&Object, Sel, Id));
            decl.add_method(sel!(quit:), on_quit as extern "C" fn(&Object, Sel, Id));
        }

        decl.register();
    });

    class!(HapticMenuDelegate)
}

pub fn create_status_bar_menu(config: Arc<AppConfig>) -> Result<(), &'static str> {
    unsafe {
        let _ = GLOBAL_CONFIG.set(config.clone());

        // Pre-initialize sound engine
        init_sound_engine(config.get_sound_volume());

        let delegate_cls = register_action_handler_class();
        let delegate: Id = msg_send![delegate_cls, alloc];
        let delegate: Id = msg_send![delegate, init];
        let () = msg_send![delegate, retain];
        DELEGATE_REF.store(delegate as usize, Ordering::Relaxed);

        let status_bar: Id = msg_send![class!(NSStatusBar), systemStatusBar];
        let status_item: Id = msg_send![status_bar, statusItemWithLength: -1.0f64];
        let () = msg_send![status_item, retain];
        STATUS_ITEM_REF.store(status_item as usize, Ordering::Relaxed);

        let button: Id = msg_send![status_item, button];
        if button != NIL {
            let title = create_ns_string("⚡️");
            let () = msg_send![button, setTitle: title];
        }

        let menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![menu, retain];
        let () = msg_send![menu, setDelegate: delegate];
        MENU_REF.store(menu as usize, Ordering::Relaxed);

        let add_item = |target_menu: Id, title: &str, action: Option<Sel>, tag: isize, key: &str| -> Id {
            let ns_title = create_ns_string(title);
            let ns_key = create_ns_string(key);
            let action_sel = action.unwrap_or(Sel::from_ptr(std::ptr::null()));
            let item: Id = msg_send![class!(NSMenuItem), alloc];
            let item: Id = msg_send![
                item,
                initWithTitle: ns_title
                action: action_sel
                keyEquivalent: ns_key
            ];
            if action.is_some() {
                let () = msg_send![item, setTarget: delegate];
            }
            let () = msg_send![item, setTag: tag];
            let () = msg_send![target_menu, addItem: item];
            item
        };

        let add_separator = |target_menu: Id| {
            let sep: Id = msg_send![class!(NSMenuItem), separatorItem];
            let () = msg_send![target_menu, addItem: sep];
        };

        // 1. Status Header
        let header = add_item(menu, "Status: Active", None, TAG_STATUS_HEADER, "");
        let () = msg_send![header, setEnabled: NO];

        add_separator(menu);

        // 2. Enable Toggles
        add_item(menu, "Enable All Effects", Some(sel!(toggleEnabled:)), TAG_ENABLE_ALL, "e");
        add_item(menu, "Mouse Movement Haptic", Some(sel!(toggleMouseMove:)), TAG_ENABLE_MOUSE, "m");
        add_item(menu, "Scroll Wheel Haptic", Some(sel!(toggleScroll:)), TAG_ENABLE_SCROLL, "s");
        add_item(menu, "Multi-Touch Gestures (Pinch/Rotate)", Some(sel!(toggleGestures:)), TAG_ENABLE_GESTURES, "g");
        add_item(menu, "Mechanical Keyboard Sounds", Some(sel!(toggleKeyboard:)), TAG_ENABLE_KEYBOARD, "k");

        add_separator(menu);

        // 3. Haptic Output Mode Submenu
        let output_mode_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![output_mode_menu, retain];
        OUTPUT_MODE_MENU_REF.store(output_mode_menu as usize, Ordering::Relaxed);

        add_item(output_mode_menu, "Trackpad Vibration Only (Default)", Some(sel!(setModeTrackpad:)), TAG_MODE_TRACKPAD, "");
        add_item(output_mode_menu, "Speaker Audio Tick (For Normal Mouse)", Some(sel!(setModeSpeaker:)), TAG_MODE_SPEAKER, "");
        add_item(output_mode_menu, "Both (Trackpad + Speaker Tick)", Some(sel!(setModeBoth:)), TAG_MODE_BOTH, "");

        let output_mode_item = add_item(menu, "Haptic Output Mode", None, 0, "");
        let () = msg_send![output_mode_item, setSubmenu: output_mode_menu];

        // 4. Pattern / Intensity Submenu
        let pattern_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![pattern_menu, retain];
        PATTERN_MENU_REF.store(pattern_menu as usize, Ordering::Relaxed);

        add_item(pattern_menu, "Generic (Light)", Some(sel!(setPatternGeneric:)), TAG_PAT_GENERIC, "");
        add_item(pattern_menu, "Alignment (Medium)", Some(sel!(setPatternAlignment:)), TAG_PAT_ALIGNMENT, "");
        add_item(pattern_menu, "Level Change (Firm)", Some(sel!(setPatternLevel:)), TAG_PAT_LEVEL, "");

        let pattern_item = add_item(menu, "Haptic Pattern / Intensity", None, 0, "");
        let () = msg_send![pattern_item, setSubmenu: pattern_menu];

        // 5. Mouse Sensitivity Submenu
        let mouse_sens_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![mouse_sens_menu, retain];
        MOUSE_MENU_REF.store(mouse_sens_menu as usize, Ordering::Relaxed);

        add_item(mouse_sens_menu, "High (Sensitive)", Some(sel!(setMouseSensHigh:)), TAG_MOUSE_HIGH, "");
        add_item(mouse_sens_menu, "Medium (Normal)", Some(sel!(setMouseSensMed:)), TAG_MOUSE_MED, "");
        add_item(mouse_sens_menu, "Low (Coarse)", Some(sel!(setMouseSensLow:)), TAG_MOUSE_LOW, "");

        let mouse_sens_item = add_item(menu, "Mouse Sensitivity", None, 0, "");
        let () = msg_send![mouse_sens_item, setSubmenu: mouse_sens_menu];

        // 6. Scroll Sensitivity Submenu
        let scroll_sens_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![scroll_sens_menu, retain];
        SCROLL_MENU_REF.store(scroll_sens_menu as usize, Ordering::Relaxed);

        add_item(scroll_sens_menu, "High (Sensitive)", Some(sel!(setScrollSensHigh:)), TAG_SCROLL_HIGH, "");
        add_item(scroll_sens_menu, "Medium (Normal)", Some(sel!(setScrollSensMed:)), TAG_SCROLL_MED, "");
        add_item(scroll_sens_menu, "Low (Coarse)", Some(sel!(setScrollSensLow:)), TAG_SCROLL_LOW, "");

        let scroll_sens_item = add_item(menu, "Scroll Sensitivity", None, 0, "");
        let () = msg_send![scroll_sens_item, setSubmenu: scroll_sens_menu];

        add_separator(menu);

        // 7. Keyboard Sound Switch Profile Submenu
        let sound_profile_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![sound_profile_menu, retain];
        SOUND_PROFILE_MENU_REF.store(sound_profile_menu as usize, Ordering::Relaxed);

        add_item(sound_profile_menu, "Cream / Holy Panda (Thocky)", Some(sel!(setSoundThock:)), TAG_SND_THOCK, "");
        add_item(sound_profile_menu, "Blue Switch (Crisp Click)", Some(sel!(setSoundBlue:)), TAG_SND_BLUE, "");
        add_item(sound_profile_menu, "Vintage Typewriter", Some(sel!(setSoundTypewriter:)), TAG_SND_TYPEWRITER, "");

        let sound_profile_item = add_item(menu, "Keyboard Switch Sound", None, 0, "");
        let () = msg_send![sound_profile_item, setSubmenu: sound_profile_menu];

        // 8. Keyboard Sound Volume Submenu
        let sound_vol_menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![sound_vol_menu, retain];
        SOUND_VOL_MENU_REF.store(sound_vol_menu as usize, Ordering::Relaxed);

        add_item(sound_vol_menu, "100% (High)", Some(sel!(setVol100:)), TAG_VOL_100, "");
        add_item(sound_vol_menu, "70% (Medium)", Some(sel!(setVol70:)), TAG_VOL_70, "");
        add_item(sound_vol_menu, "40% (Quiet)", Some(sel!(setVol40:)), TAG_VOL_40, "");
        add_item(sound_vol_menu, "15% (Subtle)", Some(sel!(setVol15:)), TAG_VOL_15, "");
        add_item(sound_vol_menu, "Mute (Off)", Some(sel!(setVol0:)), TAG_VOL_0, "");

        let sound_vol_item = add_item(menu, "Keyboard Sound Volume", None, 0, "");
        let () = msg_send![sound_vol_item, setSubmenu: sound_vol_menu];

        add_separator(menu);

        // 9. Test & Permissions
        add_item(menu, "Test Haptic & Sound", Some(sel!(testHaptic:)), 0, "t");
        add_item(menu, "Check Accessibility Permissions...", Some(sel!(checkAccessibility:)), 0, "");

        add_separator(menu);

        // 10. Quit
        add_item(menu, "Quit Haptic", Some(sel!(quit:)), 0, "q");

        // Attach menu to status item
        let () = msg_send![status_item, setMenu: menu];

        update_menu_state();
    }

    Ok(())
}
