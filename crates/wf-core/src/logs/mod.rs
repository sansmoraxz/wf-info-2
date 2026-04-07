use std::env;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::account::{AccountInfo, Platform};

pub mod pattern;

#[derive(Debug)]
pub enum LogEvent {
    Login(AccountInfo),
    Logout,
    DmTabOpened(DirectMessageInfo),
    /// The local client issued an `IRC out: WHO <username>` query,
    /// indicating the user initiated a DM conversation.
    WhoQuery(String),
    TradeConfirmPopup(TradeInfo),
    TradeSuccess,
    /// Trade failed with wrapped reason
    TradeFail(String),
    /// Relic crack countdown start (user should take screenshot and run ocr)
    /// There's no other known way to extract rewards directly yet
    RelicOpen,
    // Relic crack countdown end
    RelicClose
}

#[derive(Debug)]
pub struct TradeInfo {
    pub sent: Vec<TradeItem>,
    pub received: Vec<TradeItem>,
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub name: String,
    pub count: u32,
}

#[derive(Debug)]
pub struct DirectMessageInfo {
    pub username: String,
    pub platform: Platform,
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
