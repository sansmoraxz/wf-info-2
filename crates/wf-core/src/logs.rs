use std::env;
use std::path::PathBuf;

use crate::account::AccountInfo;

pub enum LogEvent {
    Login(AccountInfo),
    Logout,
    DmTabOpened(DirectMessageInfo),
    /// The local client issued an `IRC out: WHO <username>` query,
    /// indicating the user initiated a DM conversation.
    DmWhoQuery(String),
}

pub struct DirectMessageInfo {
    pub username: String,
    pub platform: &'static str,
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

/// Parses: `Sys [Info]: Logged in <username> (<hex_id>)`
fn parse_login(line: &str) -> Option<AccountInfo> {
    let rest = line.split_once("Sys [Info]: Logged in ")?.1;
    let (username, rest) = rest.split_once(' ')?;
    let rest = rest.strip_prefix('(')?.strip_suffix(')')?;
    if !rest.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(AccountInfo {
        username: username.to_string(),
        account_id: rest.to_string(),
    })
}

/// Parses: `Sys [Info]: Player name changed to <username> ... AccountId: <hex_id>`
fn parse_account_change(line: &str) -> Option<AccountInfo> {
    let rest = line.split_once("Sys [Info]: Player name changed to ")?.1;
    let username = rest.split_whitespace().next()?;
    let id = rest
        .split_once("AccountId:")?
        .1
        .trim()
        .split_whitespace()
        .next()?;
    if !id.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(AccountInfo {
        username: username.to_string(),
        account_id: id.to_string(),
    })
}

/// Platform suffix mapping for chat tab names.
fn platform_from_suffix(ch: char) -> Option<&'static str> {
    match ch {
        '\u{e000}' => Some("pc"),
        '\u{e001}' => Some("xbox"),
        '\u{e002}' => Some("playstation"),
        '\u{e003}' => Some("nintendo"),
        '\u{e004}' => Some("ios"),
        '\u{e005}' => Some("android"),
        _ => None,
    }
}

/// Parses: `Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: F<username><platform_suffix> to index <N>`
fn parse_dm_tab(line: &str) -> Option<DirectMessageInfo> {
    let rest = line
        .split_once(
            "Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: F",
        )?
        .1;
    // Strip trailing " to index <N>"
    let channel = rest.rsplit_once(" to index ")?.0;
    // The last character is the platform suffix
    let last_char = channel.chars().last()?;
    let platform = platform_from_suffix(last_char)?;
    // Username is everything before the platform suffix
    let username: String = channel.chars().take(channel.chars().count() - 1).collect();
    if username.is_empty() {
        return None;
    }
    Some(DirectMessageInfo { username, platform })
}

pub fn parse_log_line(line: &str) -> Option<LogEvent> {
    if line.contains("Logged in") {
        if let Some(info) = parse_login(line) {
            return Some(LogEvent::Login(info));
        }
    } else if line.contains("name changed to") {
        if let Some(info) = parse_account_change(line) {
            return Some(LogEvent::Login(info));
        }
    } else if line.contains("QUIT :Logged out") {
        return Some(LogEvent::Logout);
    } else if line.contains("ChatRedux::AddTab: Adding tab with channel name: F") {
        if let Some(info) = parse_dm_tab(line) {
            return Some(LogEvent::DmTabOpened(info));
        }
    } else if let Some(rest) = line.split_once("Net [Info]: IRC out: WHO ").map(|x| x.1) {
        // `IRC out: WHO <username>??? n%nu` — self-initiated DM query
        if let Some(username) = rest.split('?').next() {
            if !username.is_empty() {
                return Some(LogEvent::DmWhoQuery(username.to_string()));
            }
        }
    }
    None
}
