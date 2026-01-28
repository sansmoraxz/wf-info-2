use regex::Regex;
use std::env;
use std::path::PathBuf;

use crate::account::AccountInfo;

pub enum LogEvent {
    Login(AccountInfo),
    Logout,
}

#[cfg(target_os = "linux")]
fn platform_default_app_config() -> Option<PathBuf> {
    // Common Warframe installation paths on Linux (Steam/Proton)
    let home = env::var("HOME").ok()?;

    // Try Steam Proton path
    let steam_path = PathBuf::from(&home).join(
        ".steam/steam/steamapps/compatdata/230410/pfx/drive_c/users/steamuser/AppData/Local/Warframe/",
    );

    if steam_path.exists() {
        return Some(steam_path);
    }

    None
}

#[cfg(target_os = "windows")]
fn platform_default_app_config() -> Option<PathBuf> {
    // Native Windows install writes logs to %LOCALAPPDATA%\Warframe\EE.log
    if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
        let path = PathBuf::from(local_app_data).join("Warframe");
        if path.exists() {
            return Some(path);
        }
    }

    // Fallback using USERPROFILE if LOCALAPPDATA isn't set
    if let Ok(user_profile) = env::var("USERPROFILE") {
        let fallback = PathBuf::from(user_profile)
            .join("AppData")
            .join("Local")
            .join("Warframe");
        if fallback.exists() {
            return Some(fallback);
        }
    }

    None
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn platform_default_app_config() -> Option<PathBuf> {
    None
}

pub fn find_wf_app_config() -> Option<PathBuf> {
    // Try custom path from environment variable
    if let Ok(custom_path) = env::var("WARFRAME_APP_CONFIG") {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            return Some(path);
        }
    }

    platform_default_app_config()
}

pub fn parse_log_line(line: &str) -> Option<LogEvent> {
    // Regex patterns to find account ID and username
    // Pattern 1: "Logged in Username (accountid)"
    let login_regex = Regex::new(r"Sys \[Info\]: Logged in (\S+) \(([A-Fa-f0-9]+)\)").ok()?;
    // Pattern 2: "Player name changed to Username ... AccountId: accountid"
    let account_regex =
        Regex::new(r"Player name changed to (\S+).*AccountId:\s*([A-Fa-f0-9]+)").ok()?;
    // Pattern 3: Logout
    let logout_regex = Regex::new(r"IRC out: QUIT :Logged out of game").ok()?;

    // Check for "Logged in" pattern
    if let Some(caps) = login_regex.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(LogEvent::Login(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            }));
        }
    }

    // Check for "Player name changed" pattern
    if let Some(caps) = account_regex.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(LogEvent::Login(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            }));
        }
    }

    // Check for logout
    if logout_regex.is_match(line) {
        return Some(LogEvent::Logout);
    }

    None
}
