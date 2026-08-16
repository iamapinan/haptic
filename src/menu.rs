use crate::config::{AppConfig, Sensitivity};
use crate::event_tap::is_accessibility_trusted;
use crate::haptic::{perform_haptic, HapticPattern, Id, NIL};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once, OnceLock};

pub const NO: i8 = 0;

static INIT_DELEGATE: Once = Once::new();
static GLOBAL_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();
static MENU_REF: AtomicUsize = AtomicUsize::new(0);
static STATUS_ITEM_REF: AtomicUsize = AtomicUsize::new(0);

// Menu Item Tags
const TAG_STATUS_HEADER: isize = 100;
const TAG_ENABLE_ALL: isize = 101;
const TAG_ENABLE_MOUSE: isize = 102;
const TAG_ENABLE_SCROLL: isize = 103;

const TAG_PAT_GENERIC: isize = 201;
const TAG_PAT_ALIGNMENT: isize = 202;
const TAG_PAT_LEVEL: isize = 203;

const TAG_MOUSE_HIGH: isize = 301;
const TAG_MOUSE_MED: isize = 302;
const TAG_MOUSE_LOW: isize = 303;

const TAG_SCROLL_HIGH: isize = 401;
const TAG_SCROLL_MED: isize = 402;
const TAG_SCROLL_LOW: isize = 403;

pub fn create_ns_string(s: &str) -> Id {
    unsafe {
        let cls = class!(NSString);
        let bytes = s.as_ptr() as *const std::os::raw::c_char;
        let obj: Id = msg_send![cls, alloc];
        msg_send![obj, initWithBytes:bytes length:s.len() encoding:4usize] // 4 = NSUTF8StringEncoding
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
        let status_item: Id = STATUS_ITEM_REF.load(Ordering::Relaxed) as Id;

        if menu == NIL {
            return;
        }

        let config = get_config();
        let enabled = config.is_enabled();
        let mouse_enabled = config.is_mouse_move_enabled();
        let scroll_enabled = config.is_scroll_enabled();
        let pattern = config.get_pattern();
        let mouse_sens = config.get_mouse_sensitivity();
        let scroll_sens = config.get_scroll_sensitivity();

        // Update Status bar title
        if status_item != NIL {
            let button: Id = msg_send![status_item, button];
            if button != NIL {
                let title = if enabled { "⚡️" } else { "💤" };
                let ns_title = create_ns_string(title);
                let () = msg_send![button, setTitle: ns_title];
            }
        }

        // Helper to set item state
        let set_item_state = |tag: isize, is_on: bool| {
            let item: Id = msg_send![menu, itemWithTag: tag];
            if item != NIL {
                let state: isize = if is_on { 1 } else { 0 };
                let () = msg_send![item, setState: state];
            }
        };

        // Helper to set item title
        let set_item_title = |tag: isize, text: &str| {
            let item: Id = msg_send![menu, itemWithTag: tag];
            if item != NIL {
                let ns_text = create_ns_string(text);
                let () = msg_send![item, setTitle: ns_text];
            }
        };

        let status_text = if enabled {
            "Status: Active (Haptics On)"
        } else {
            "Status: Paused (Haptics Off)"
        };
        set_item_title(TAG_STATUS_HEADER, status_text);

        set_item_state(TAG_ENABLE_ALL, enabled);
        set_item_state(TAG_ENABLE_MOUSE, mouse_enabled);
        set_item_state(TAG_ENABLE_SCROLL, scroll_enabled);

        // Update Pattern submenu
        set_item_state(TAG_PAT_GENERIC, pattern == HapticPattern::Generic);
        set_item_state(TAG_PAT_ALIGNMENT, pattern == HapticPattern::Alignment);
        set_item_state(TAG_PAT_LEVEL, pattern == HapticPattern::LevelChange);

        // Update Mouse Sensitivity submenu
        set_item_state(TAG_MOUSE_HIGH, mouse_sens == Sensitivity::High);
        set_item_state(TAG_MOUSE_MED, mouse_sens == Sensitivity::Medium);
        set_item_state(TAG_MOUSE_LOW, mouse_sens == Sensitivity::Low);

        // Update Scroll Sensitivity submenu
        set_item_state(TAG_SCROLL_HIGH, scroll_sens == Sensitivity::High);
        set_item_state(TAG_SCROLL_MED, scroll_sens == Sensitivity::Medium);
        set_item_state(TAG_SCROLL_LOW, scroll_sens == Sensitivity::Low);
    }
}

// Objective-C Action Handlers
extern "C" fn on_toggle_enabled(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    let new_val = config.toggle_enabled();
    println!("[Haptic] Enabled: {}", new_val);
    if new_val {
        perform_haptic(config.get_pattern());
    }
    update_menu_state();
}

extern "C" fn on_toggle_mouse(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    let new_val = config.toggle_mouse_move();
    println!("[Haptic] Mouse move feedback: {}", new_val);
    if new_val {
        perform_haptic(config.get_pattern());
    }
    update_menu_state();
}

extern "C" fn on_toggle_scroll(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    let new_val = config.toggle_scroll();
    println!("[Haptic] Scroll feedback: {}", new_val);
    if new_val {
        perform_haptic(config.get_pattern());
    }
    update_menu_state();
}

extern "C" fn on_set_pattern_generic(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_pattern(HapticPattern::Generic);
    println!("[Haptic] Pattern set to: Generic");
    perform_haptic(HapticPattern::Generic);
    update_menu_state();
}

extern "C" fn on_set_pattern_alignment(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_pattern(HapticPattern::Alignment);
    println!("[Haptic] Pattern set to: Alignment");
    perform_haptic(HapticPattern::Alignment);
    update_menu_state();
}

extern "C" fn on_set_pattern_level(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_pattern(HapticPattern::LevelChange);
    println!("[Haptic] Pattern set to: LevelChange");
    perform_haptic(HapticPattern::LevelChange);
    update_menu_state();
}

extern "C" fn on_set_mouse_sens_high(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_mouse_sensitivity(Sensitivity::High);
    println!("[Haptic] Mouse sensitivity set to: High");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_set_mouse_sens_med(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_mouse_sensitivity(Sensitivity::Medium);
    println!("[Haptic] Mouse sensitivity set to: Medium");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_set_mouse_sens_low(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_mouse_sensitivity(Sensitivity::Low);
    println!("[Haptic] Mouse sensitivity set to: Low");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_set_scroll_sens_high(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_scroll_sensitivity(Sensitivity::High);
    println!("[Haptic] Scroll sensitivity set to: High");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_set_scroll_sens_med(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_scroll_sensitivity(Sensitivity::Medium);
    println!("[Haptic] Scroll sensitivity set to: Medium");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_set_scroll_sens_low(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    config.set_scroll_sensitivity(Sensitivity::Low);
    println!("[Haptic] Scroll sensitivity set to: Low");
    perform_haptic(config.get_pattern());
    update_menu_state();
}

extern "C" fn on_test_haptic(_this: &Object, _cmd: Sel, _sender: Id) {
    let config = get_config();
    println!("[Haptic] Testing haptic click...");
    perform_haptic(config.get_pattern());
}

extern "C" fn on_check_accessibility(_this: &Object, _cmd: Sel, _sender: Id) {
    println!("[Haptic] Checking accessibility permissions...");
    let trusted = is_accessibility_trusted(true);
    if trusted {
        println!("[Haptic] Accessibility is granted.");
    } else {
        println!("[Haptic] Prompted user to enable Accessibility in System Settings.");
    }
}

extern "C" fn on_menu_will_open(_this: &Object, _cmd: Sel, _menu: Id) {
    update_menu_state();
}

extern "C" fn on_quit(_this: &Object, _cmd: Sel, _sender: Id) {
    println!("[Haptic] Exiting app...");
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
            decl.add_method(
                sel!(toggleEnabled:),
                on_toggle_enabled as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(toggleMouseMove:),
                on_toggle_mouse as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(toggleScroll:),
                on_toggle_scroll as extern "C" fn(&Object, Sel, Id),
            );

            decl.add_method(
                sel!(setPatternGeneric:),
                on_set_pattern_generic as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setPatternAlignment:),
                on_set_pattern_alignment as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setPatternLevel:),
                on_set_pattern_level as extern "C" fn(&Object, Sel, Id),
            );

            decl.add_method(
                sel!(setMouseSensHigh:),
                on_set_mouse_sens_high as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setMouseSensMed:),
                on_set_mouse_sens_med as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setMouseSensLow:),
                on_set_mouse_sens_low as extern "C" fn(&Object, Sel, Id),
            );

            decl.add_method(
                sel!(setScrollSensHigh:),
                on_set_scroll_sens_high as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setScrollSensMed:),
                on_set_scroll_sens_med as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(setScrollSensLow:),
                on_set_scroll_sens_low as extern "C" fn(&Object, Sel, Id),
            );

            decl.add_method(
                sel!(testHaptic:),
                on_test_haptic as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(checkAccessibility:),
                on_check_accessibility as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(
                sel!(menuWillOpen:),
                on_menu_will_open as extern "C" fn(&Object, Sel, Id),
            );
            decl.add_method(sel!(quit:), on_quit as extern "C" fn(&Object, Sel, Id));
        }

        decl.register();
    });

    class!(HapticMenuDelegate)
}

pub fn create_status_bar_menu(config: Arc<AppConfig>) -> Result<(), &'static str> {
    unsafe {
        let _ = GLOBAL_CONFIG.set(config);

        let delegate_cls = register_action_handler_class();
        let delegate: Id = msg_send![delegate_cls, alloc];
        let delegate: Id = msg_send![delegate, init];

        let status_bar: Id = msg_send![class!(NSStatusBar), systemStatusBar];
        // -1.0 is NSVariableStatusItemLength
        let status_item: Id = msg_send![status_bar, statusItemWithLength: -1.0f64];
        STATUS_ITEM_REF.store(status_item as usize, Ordering::Relaxed);

        let button: Id = msg_send![status_item, button];
        if button != NIL {
            let title = create_ns_string("⚡️");
            let () = msg_send![button, setTitle: title];
        }

        let menu: Id = msg_send![class!(NSMenu), new];
        let () = msg_send![menu, setDelegate: delegate];
        MENU_REF.store(menu as usize, Ordering::Relaxed);

        // Helper to add item
        let add_item = |menu: Id, title: &str, action: Option<Sel>, tag: isize, key: &str| -> Id {
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
            let () = msg_send![menu, addItem: item];
            item
        };

        let add_separator = |menu: Id| {
            let sep: Id = msg_send![class!(NSMenuItem), separatorItem];
            let () = msg_send![menu, addItem: sep];
        };

        // 1. Status Header
        let header = add_item(menu, "Status: Active", None, TAG_STATUS_HEADER, "");
        let () = msg_send![header, setEnabled: NO];

        add_separator(menu);

        // 2. Enable Toggles
        add_item(
            menu,
            "Enable Haptics",
            Some(sel!(toggleEnabled:)),
            TAG_ENABLE_ALL,
            "e",
        );
        add_item(
            menu,
            "Mouse Movement Haptic",
            Some(sel!(toggleMouseMove:)),
            TAG_ENABLE_MOUSE,
            "m",
        );
        add_item(
            menu,
            "Scroll Wheel Haptic",
            Some(sel!(toggleScroll:)),
            TAG_ENABLE_SCROLL,
            "s",
        );

        add_separator(menu);

        // 3. Pattern / Intensity Submenu
        let pattern_menu: Id = msg_send![class!(NSMenu), new];
        add_item(
            pattern_menu,
            "Generic (Light)",
            Some(sel!(setPatternGeneric:)),
            TAG_PAT_GENERIC,
            "",
        );
        add_item(
            pattern_menu,
            "Alignment (Medium)",
            Some(sel!(setPatternAlignment:)),
            TAG_PAT_ALIGNMENT,
            "",
        );
        add_item(
            pattern_menu,
            "Level Change (Firm)",
            Some(sel!(setPatternLevel:)),
            TAG_PAT_LEVEL,
            "",
        );

        let pattern_item = add_item(menu, "Haptic Pattern / Intensity", None, 0, "");
        let () = msg_send![pattern_item, setSubmenu: pattern_menu];

        // 4. Mouse Sensitivity Submenu
        let mouse_sens_menu: Id = msg_send![class!(NSMenu), new];
        add_item(
            mouse_sens_menu,
            "High (25 px)",
            Some(sel!(setMouseSensHigh:)),
            TAG_MOUSE_HIGH,
            "",
        );
        add_item(
            mouse_sens_menu,
            "Medium (50 px)",
            Some(sel!(setMouseSensMed:)),
            TAG_MOUSE_MED,
            "",
        );
        add_item(
            mouse_sens_menu,
            "Low (100 px)",
            Some(sel!(setMouseSensLow:)),
            TAG_MOUSE_LOW,
            "",
        );

        let mouse_sens_item = add_item(menu, "Mouse Sensitivity", None, 0, "");
        let () = msg_send![mouse_sens_item, setSubmenu: mouse_sens_menu];

        // 5. Scroll Sensitivity Submenu
        let scroll_sens_menu: Id = msg_send![class!(NSMenu), new];
        add_item(
            scroll_sens_menu,
            "High (Sensitive)",
            Some(sel!(setScrollSensHigh:)),
            TAG_SCROLL_HIGH,
            "",
        );
        add_item(
            scroll_sens_menu,
            "Medium (Normal)",
            Some(sel!(setScrollSensMed:)),
            TAG_SCROLL_MED,
            "",
        );
        add_item(
            scroll_sens_menu,
            "Low (Coarse)",
            Some(sel!(setScrollSensLow:)),
            TAG_SCROLL_LOW,
            "",
        );

        let scroll_sens_item = add_item(menu, "Scroll Sensitivity", None, 0, "");
        let () = msg_send![scroll_sens_item, setSubmenu: scroll_sens_menu];

        add_separator(menu);

        // 6. Test Haptic & Permissions
        add_item(menu, "Test Haptic Click", Some(sel!(testHaptic:)), 0, "t");
        add_item(
            menu,
            "Check Accessibility Permissions...",
            Some(sel!(checkAccessibility:)),
            0,
            "",
        );

        add_separator(menu);

        // 7. Quit
        add_item(menu, "Quit Haptic", Some(sel!(quit:)), 0, "q");

        // Attach menu to status item
        let () = msg_send![status_item, setMenu: menu];

        update_menu_state();
    }

    Ok(())
}
