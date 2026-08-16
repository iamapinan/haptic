mod config;
mod event_tap;
mod haptic;
mod menu;
mod sound;

use config::AppConfig;
use event_tap::start_event_tap;
use haptic::Id;
use menu::create_status_bar_menu;
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Arc;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

fn main() {
    unsafe {
        let _pool: Id = msg_send![class!(NSAutoreleasePool), new];

        // 1. Initialize NSApplication
        let app: Id = msg_send![class!(NSApplication), sharedApplication];

        // Set activation policy to Accessory = 1 (runs in background without Dock icon, only menu bar)
        let () = msg_send![app, setActivationPolicy: 1isize];

        // 2. Initialize App Configuration & Sound Engine
        let config = AppConfig::new();
        sound::init_sound_engine(config.get_sound_volume());

        // 3. Start Global Event Tap (Mouse move, Scroll wheel, Gestures & Hardware Keyboard)
        if let Err(e) = start_event_tap(Arc::clone(&config)) {
            eprintln!("[Haptic] Warning: {}", e);
        }

        // 4. Create Status Bar Menu Item
        if let Err(e) = create_status_bar_menu(Arc::clone(&config)) {
            eprintln!("[Haptic] Error creating status bar menu: {}", e);
            return;
        }

        println!("⚡️ Haptic Touch & Scroll for macOS is running!");
        println!("Look for the ⚡️ icon in your macOS menu bar (top-right).");

        // 5. Run macOS Application Event Loop
        let () = msg_send![app, run];
    }
}
