use crate::menu::create_ns_string;
use objc::{class, msg_send, sel, sel_impl};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
static IS_UPDATING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReleaseInfo {
    pub version: String,
    pub tag: String,
    pub download_url: String,
    pub release_notes: String,
}

/// Parses semantic version into (major, minor, patch)
fn parse_version(v: &str) -> Option<(u32, u32, u32)> {
    let clean = v.trim().trim_start_matches('v');
    let parts: Vec<&str> = clean.split('.').collect();
    if parts.len() >= 3 {
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()?;
        Some((major, minor, patch))
    } else if parts.len() == 2 {
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        Some((major, minor, 0))
    } else {
        None
    }
}

/// Compares if remote_ver is strictly newer than local_ver
fn is_newer_version(remote_ver: &str, local_ver: &str) -> bool {
    if let (Some(r), Some(l)) = (parse_version(remote_ver), parse_version(local_ver)) {
        r > l
    } else {
        remote_ver.trim_start_matches('v') != local_ver.trim_start_matches('v')
    }
}

/// Fetches the latest release info from GitHub API via macOS native curl
pub fn fetch_latest_release() -> Result<ReleaseInfo, String> {
    let output = Command::new("curl")
        .args([
            "-s",
            "-L",
            "--max-time",
            "10",
            "-H",
            "Accept: application/vnd.github.v3+json",
            "-H",
            "User-Agent: Haptic-macOS-App",
            "https://api.github.com/repos/iamapinan/haptic/releases/latest",
        ])
        .output()
        .map_err(|e| format!("Failed to execute curl: {}", e))?;

    if !output.status.success() {
        return Err("GitHub release check failed".to_string());
    }

    let body = String::from_utf8_lossy(&output.stdout).to_string();
    if body.contains("\"message\": \"Not Found\"") || !body.contains("\"tag_name\"") {
        return Err("No published releases found on GitHub".to_string());
    }

    // Extract tag_name
    let tag = extract_json_field(&body, "tag_name")
        .ok_or_else(|| "Could not parse tag_name from release JSON".to_string())?;

    let version = tag.trim_start_matches('v').to_string();

    // Extract release notes body
    let release_notes = extract_json_field(&body, "body").unwrap_or_default();

    // Look for Haptic.zip or Haptic.dmg download URL
    let download_url = if let Some(url) = extract_asset_download_url(&body, "Haptic.zip") {
        url
    } else if let Some(url) = extract_asset_download_url(&body, "Haptic.dmg") {
        url
    } else {
        format!("https://github.com/iamapinan/haptic/releases/download/{}/Haptic.dmg", tag)
    };

    Ok(ReleaseInfo {
        version,
        tag,
        download_url,
        release_notes,
    })
}

fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let search = format!("\"{}\":", field);
    let start_idx = json.find(&search)? + search.len();
    let remaining = json[start_idx..].trim_start();

    if remaining.starts_with('"') {
        let after_quote = &remaining[1..];
        let mut end_idx = 0;
        let mut escape = false;
        for (i, c) in after_quote.chars().enumerate() {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                end_idx = i;
                break;
            }
        }
        if end_idx > 0 {
            let val = &after_quote[..end_idx];
            return Some(val.replace("\\n", "\n").replace("\\\"", "\""));
        }
    }
    None
}

fn extract_asset_download_url(json: &str, asset_name: &str) -> Option<String> {
    let target = format!("\"name\": \"{}\"", asset_name);
    let idx = json.find(&target)?;
    let chunk = &json[..idx];
    let browser_key = "\"browser_download_url\": \"";
    let url_start = chunk.rfind(browser_key)? + browser_key.len();
    let url_chunk = &chunk[url_start..];
    let url_end = url_chunk.find('"')?;
    Some(url_chunk[..url_end].to_string())
}

/// Displays an alert modal to the user using Cocoa NSAlert
pub fn show_alert(title: &str, message: &str) {
    unsafe {
        let alert: crate::haptic::Id = msg_send![class!(NSAlert), new];
        let ns_title = create_ns_string(title);
        let ns_msg = create_ns_string(message);
        let () = msg_send![alert, setMessageText: ns_title];
        let () = msg_send![alert, setInformativeText: ns_msg];
        let ok_btn = create_ns_string("OK");
        let () = msg_send![alert, addButtonWithTitle: ok_btn];
        let () = msg_send![alert, runModal];
    }
}

/// Prompts user to download and install update
pub fn prompt_update_and_install(release: ReleaseInfo) {
    unsafe {
        let alert: crate::haptic::Id = msg_send![class!(NSAlert), new];
        let title_str = format!("🎉 New Version Available: v{}", release.version);
        let msg_str = format!(
            "A newer version of Haptic is available!\n\n• Current Version: v{}\n• Latest Version: v{}\n\nWould you like to automatically download, update, and restart Haptic now?",
            CURRENT_VERSION, release.version
        );

        let ns_title = create_ns_string(&title_str);
        let ns_msg = create_ns_string(&msg_str);
        let () = msg_send![alert, setMessageText: ns_title];
        let () = msg_send![alert, setInformativeText: ns_msg];

        let update_btn = create_ns_string("Update & Restart");
        let later_btn = create_ns_string("Later");
        let () = msg_send![alert, addButtonWithTitle: update_btn];
        let () = msg_send![alert, addButtonWithTitle: later_btn];

        let response: isize = msg_send![alert, runModal];
        // 1000 = first button ("Update & Restart")
        if response == 1000 {
            perform_in_place_update(&release.download_url, &release.version);
        }
    }
}

/// Downloads, unzips, replaces /Applications/Haptic.app, and restarts
pub fn perform_in_place_update(download_url: &str, new_ver: &str) {
    if IS_UPDATING.swap(true, Ordering::SeqCst) {
        return;
    }

    println!("[Haptic Updater] Downloading update from {}...", download_url);

    let script = format!(
        r#"
TMP_DIR="/tmp/haptic_auto_update"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"

echo "Downloading v{new_ver}..."
curl -s -L --max-time 120 "{download_url}" -o "$TMP_DIR/update_pkg"

if [ -f "$TMP_DIR/update_pkg" ]; then
    # Try zip first
    if unzip -q "$TMP_DIR/update_pkg" -d "$TMP_DIR/extracted" 2>/dev/null; then
        if [ -d "$TMP_DIR/extracted/Haptic.app" ]; then
            APP_SRC="$TMP_DIR/extracted/Haptic.app"
        else
            APP_SRC=$(find "$TMP_DIR/extracted" -name "Haptic.app" -type d | head -n 1)
        fi
    # If DMG, mount and copy
    else
        MOUNT_DIR="$TMP_DIR/mount"
        mkdir -p "$MOUNT_DIR"
        hdiutil attach "$TMP_DIR/update_pkg" -mountpoint "$MOUNT_DIR" -nobrowse -quiet
        APP_SRC="$MOUNT_DIR/Haptic.app"
    fi

    if [ -d "$APP_SRC" ]; then
        echo "Installing to /Applications/Haptic.app..."
        pkill -f haptic-mac || true
        sleep 0.5
        rm -rf "/Applications/Haptic.app"
        cp -R "$APP_SRC" "/Applications/Haptic.app"
        codesign --force --deep --sign - --identifier "com.apinan.haptic-mac" "/Applications/Haptic.app" 2>/dev/null || true
        
        # Clean up mounts
        hdiutil detach "$TMP_DIR/mount" -quiet 2>/dev/null || true
        rm -rf "$TMP_DIR"

        # Relaunch newly updated app
        open -n "/Applications/Haptic.app"
        exit 0
    fi
fi

# Fallback open browser download page if automatic replacement failed
open "https://github.com/iamapinan/haptic/releases/latest"
"#,
        new_ver = new_ver,
        download_url = download_url
    );

    // Launch updater script in background detached process
    let _ = Command::new("bash")
        .arg("-c")
        .arg(&script)
        .spawn();

    std::thread::sleep(std::time::Duration::from_millis(500));
    std::process::exit(0);
}

/// Checks for updates (called from menu or background supervisor)
pub fn check_for_updates(manual: bool) {
    std::thread::spawn(move || {
        match fetch_latest_release() {
            Ok(release) => {
                if is_newer_version(&release.version, CURRENT_VERSION) {
                    println!(
                        "[Haptic Updater] Found newer version: v{} (Current: v{})",
                        release.version, CURRENT_VERSION
                    );
                    prompt_update_and_install(release);
                } else if manual {
                    show_alert(
                        "You're Up to Date! ⚡️",
                        &format!(
                            "Haptic v{} is currently the newest version available.",
                            CURRENT_VERSION
                        ),
                    );
                }
            }
            Err(e) => {
                println!("[Haptic Updater] Check info: {}", e);
                if manual {
                    show_alert(
                        "Haptic Version Check",
                        &format!(
                            "Currently running Haptic v{}.\n(Could not reach GitHub Releases: {})",
                            CURRENT_VERSION, e
                        ),
                    );
                }
            }
        }
    });
}

/// Starts an automatic periodic background check for updates (every 4 hours, and 10s after boot)
pub fn start_auto_update_checker() {
    std::thread::spawn(|| {
        // Initial check 10 seconds after launch
        std::thread::sleep(std::time::Duration::from_secs(10));
        check_for_updates(false);

        // Check periodically every 4 hours
        loop {
            std::thread::sleep(std::time::Duration::from_secs(4 * 3600));
            check_for_updates(false);
        }
    });
}
