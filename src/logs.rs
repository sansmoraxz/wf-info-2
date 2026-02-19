use regex::Regex;
use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::account::AccountInfo;

static LOGIN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Sys \[Info\]: Logged in (\S+) \(([A-Fa-f0-9]+)\)").unwrap()
});
static ACCOUNT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Player name changed to (\S+).*AccountId:\s*([A-Fa-f0-9]+)").unwrap()
});
static LOGOUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"IRC out: QUIT :Logged out of game").unwrap()
});

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

/// Returns true if the line starts a new log entry (has a timestamp prefix like "10.227 ").
fn is_log_entry_start(line: &str) -> bool {
    let bytes = line.as_bytes();
    // Must start with at least one digit
    let mut i = 0;
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return false;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Dot
    if i >= bytes.len() || bytes[i] != b'.' {
        return false;
    }
    i += 1;
    // At least one digit after dot
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return false;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Space
    i < bytes.len() && bytes[i] == b' '
}

/// State machine that accumulates multi-line log entries and yields complete ones.
///
/// Transitions:
///   Idle   + timestamp line  → buffer it, go to Buffering
///   Idle   + continuation    → discard (orphaned)
///   Buffering + timestamp    → flush buffered entry, start buffering new one
///   Buffering + continuation → append to buffer
///   flush()                  → yield buffered entry if any, go to Idle
pub struct LogEntryParser {
    buffer: Option<String>,
}

impl LogEntryParser {
    pub fn new() -> Self {
        Self { buffer: None }
    }

    /// Feed a single line. Returns a completed log entry if one was finalized.
    pub fn feed_line(&mut self, line: &str) -> Option<String> {
        if is_log_entry_start(line) {
            // New entry starts — flush previous if any
            let completed = self.buffer.take();
            self.buffer = Some(line.to_string());
            completed
        } else {
            // Continuation line — append to current buffer
            if let Some(buf) = &mut self.buffer {
                buf.push('\n');
                buf.push_str(line);
            }
            None
        }
    }

    /// Flush the current buffer, returning any incomplete entry.
    pub fn flush(&mut self) -> Option<String> {
        self.buffer.take()
    }

    /// Reset state (e.g. on file truncation/recreation).
    pub fn reset(&mut self) {
        self.buffer = None;
    }
}

pub fn parse_log_line(line: &str) -> Option<LogEvent> {
    // Check for "Logged in" pattern
    if let Some(caps) = LOGIN_REGEX.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(LogEvent::Login(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            }));
        }
    }

    // Check for "Player name changed" pattern
    if let Some(caps) = ACCOUNT_REGEX.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(LogEvent::Login(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            }));
        }
    }

    // Check for logout
    if LOGOUT_REGEX.is_match(line) {
        return Some(LogEvent::Logout);
    }

    None
}
