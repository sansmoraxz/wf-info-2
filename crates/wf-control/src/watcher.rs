use chrono::Utc;
use notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
#[cfg(feature = "memory")]
use serde_json::json;
use std::collections::HashSet;
use std::fs::{File, metadata};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::{
    AccountLoginEvent, AccountLogoutEvent, DaemonEvent, DmTabOpenedEvent, ProfileUpdatedEvent,
};
use wf_core::account::AccountInfo;
use wf_core::logs::{self, LogEntryParser, LogEvent};
use wf_core::{api, storage};

#[cfg(feature = "memory")]
use crate::InventoryFetchedEvent;
#[cfg(feature = "memory")]
use wf_core::{inventory_refresh, process};

struct WatchState {
    last_size: u64,
    last_position: u64,
    current_account_id: Option<String>,
    log_parser: LogEntryParser,
    read_file: File,
    /// Usernames for which we issued `IRC out: WHO` (self-initiated DMs).
    /// Used to suppress DmTabOpened events for tabs we opened ourselves.
    self_initiated_dms: HashSet<String>,
}

impl WatchState {
    fn new(log_path: &PathBuf) -> Result<Self, std::io::Error> {
        let initial_size = metadata(log_path)?.len();
        let mut read_file = File::open(log_path)?;
        read_file.seek(SeekFrom::Start(initial_size))?;
        Ok(Self {
            last_size: initial_size,
            last_position: initial_size,
            current_account_id: None,
            log_parser: LogEntryParser::new(),
            read_file,
            self_initiated_dms: HashSet::new(),
        })
    }

    fn reset(&mut self, log_path: &PathBuf) -> Result<(), std::io::Error> {
        self.last_size = 0;
        self.last_position = 0;
        self.current_account_id = None;
        self.log_parser.reset();
        self.self_initiated_dms.clear();
        self.read_file = File::open(log_path)?;
        Ok(())
    }
}

// --- Helpers ---

async fn wait_for_log_file(app_config_path: &PathBuf) -> PathBuf {
    if !app_config_path.exists() {
        log::info!("Waiting for Warframe config folder to be created...");
        while !app_config_path.exists() {
            sleep(Duration::from_millis(500)).await;
        }
    }

    log::info!("Waiting for EE.log to be created...");
    let log_path = app_config_path.join("EE.log");
    while !log_path.exists() {
        sleep(Duration::from_millis(500)).await;
    }
    log::info!("EE.log found at {:?}", log_path);
    log_path
}

fn read_new_entries(
    read_file: &mut File,
    last_position: u64,
    log_parser: &mut LogEntryParser,
) -> Result<Vec<String>, std::io::Error> {
    read_file.seek(SeekFrom::Start(last_position))?;
    let reader = BufReader::new(&*read_file);

    let mut complete_entries: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            log::trace!("New line: {}", line);
            if let Some(entry) = log_parser.feed_line(&line) {
                complete_entries.push(entry);
            }
        }
    }
    if let Some(entry) = log_parser.flush() {
        complete_entries.push(entry);
    }

    Ok(complete_entries)
}

#[cfg_attr(not(feature = "memory"), allow(unused_variables))]
async fn handle_login_event(acc_id: String, user_name: String, known_pid: Option<u32>) {
    // 1. Fetch profile
    match api::fetch_player_profile(&acc_id).await {
        Ok(profile) => {
            log::info!("Fetched profile for {}: {:?}", user_name, profile);
            if let Err(e) = storage::save_encrypted_profile(&profile) {
                log::error!("Failed to save profile for {}: {}", user_name, e);
            } else {
                crate::emit(DaemonEvent::ProfileUpdated(ProfileUpdatedEvent {
                    timestamp: Utc::now(),
                    account_id: acc_id.clone(),
                }));
            }
        }
        Err(e) => {
            log::error!("Failed to fetch profile for {}: {}", user_name, e);
        }
    }

    // 2. Scan memory & fetch inventory (if memory feature enabled)
    #[cfg(feature = "memory")]
    if let Some(pid) = known_pid.or_else(process::get_warframe_pid) {
        log::info!(
            "Warframe running (PID: {}), attempting to extract inventory auth...",
            pid
        );
        match inventory_refresh::fetch_inventory_from_process(
            &acc_id,
            pid,
            5,
            Duration::from_secs(3),
        )
        .await
        {
            Ok(Some(result)) => {
                log::info!(
                    "Successfully extracted auth: {}",
                    result.auth.to_query_string()
                );
                if let Err(e) = storage::save_inventory(&result.inventory) {
                    log::error!("Failed to save inventory: {}", e);
                } else {
                    if let Err(e) = storage::touch_inventory_updated(Some("auto")) {
                        log::warn!("Failed to update inventory metadata: {}", e);
                    }
                    let summary = json!({
                        "suits": result.inventory.suits.len(),
                        "long_guns": result.inventory.long_guns.len(),
                        "pistols": result.inventory.pistols.len(),
                        "melee": result.inventory.melee.len(),
                    });
                    crate::emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
                        timestamp: Utc::now(),
                        source: "auto".to_string(),
                        summary,
                    }));
                }
            }
            Ok(None) => {
                log::warn!("Could not extract auth data from process memory");
                log::info!("Tip: Make sure you're logged into Warframe");
            }
            Err(e) => {
                log::error!("Memory scan error: {}", e);
                log::info!("Tip: Grant necessary permissions or try running with sudo");
            }
        }
    } else {
        log::info!("Warframe not running - skipping inventory fetch");
    }
}

pub async fn observe_warframe_activity(
    app_config_path: PathBuf,
    warframe_pid: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Watching for Warframe activity...");

    let log_path = wait_for_log_file(&app_config_path).await;
    let log_filename = log_path.file_name().ok_or("Invalid log path")?.to_owned();
    let mut state = WatchState::new(&log_path)?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: DebounceEventResult| {
            if let Ok(events) = res {
                for event in events {
                    let _ = tx.blocking_send(event);
                }
            }
        },
    )?;
    debouncer
        .watcher()
        .watch(&app_config_path, RecursiveMode::NonRecursive)?;

    loop {
        let Some(event) = rx.recv().await else {
            continue;
        };

        let is_our_file = event
            .path
            .file_name()
            .map(|name| name == log_filename.as_os_str())
            .unwrap_or(false);

        if !is_our_file {
            continue;
        }

        log::trace!("Event for EE.log: {:?}", event);

        // Handle deletion — wait with backoff for recreation
        if !log_path.exists() {
            log::info!("File deleted, waiting for recreation");
            let mut backoff = Duration::from_millis(100);
            let max_backoff = Duration::from_secs(15);
            while !log_path.exists() {
                sleep(backoff).await;
                backoff = (backoff * 2).min(max_backoff);
            }
            log::info!("File recreated, game restarted");
            state.reset(&log_path)?;
            continue;
        }

        let current_size = match metadata(&log_path) {
            Ok(meta) => meta.len(),
            Err(_) => continue,
        };

        // Handle truncation (game restart without deletion)
        if current_size < state.last_size {
            log::info!("File truncated, game restarted");
            state.reset(&log_path)?;
            continue;
        }

        state.last_size = current_size;

        if current_size <= state.last_position {
            continue;
        }

        log::debug!(
            "Reading from position {} to {}",
            state.last_position,
            current_size
        );

        let entries = match read_new_entries(
            &mut state.read_file,
            state.last_position,
            &mut state.log_parser,
        ) {
            Ok(e) => e,
            Err(e) => {
                log::error!("Failed to read log entries: {}", e);
                continue;
            }
        };
        state.last_position = current_size;

        for entry in entries {
            log::trace!("Complete log entry: {}", entry);
            match logs::parse_log_line(&entry) {
                Some(LogEvent::Login(AccountInfo {
                    username,
                    account_id,
                })) => {
                    if state.current_account_id.as_deref() == Some(&account_id) {
                        log::debug!("Duplicate login event for account_id={}", account_id);
                        continue;
                    }
                    state.current_account_id = Some(account_id.clone());
                    crate::set_current_account(Some(account_id.clone()));
                    log::info!(
                        "User logged in: username={}, account_id={}",
                        username,
                        account_id
                    );
                    crate::emit(DaemonEvent::AccountLogin(AccountLoginEvent {
                        timestamp: Utc::now(),
                        account_id: account_id.clone(),
                        username: username.clone(),
                    }));
                    tokio::spawn(handle_login_event(account_id, username, warframe_pid));
                }
                Some(LogEvent::Logout) => {
                    state.current_account_id = None;
                    crate::set_current_account(None);
                    log::info!("User logged out");
                    crate::emit(DaemonEvent::AccountLogout(AccountLogoutEvent {
                        timestamp: Utc::now(),
                    }));
                    if let Err(e) = storage::delete_profile() {
                        log::error!("Failed to delete profile: {}", e);
                    }
                }
                Some(LogEvent::DmWhoQuery(username)) => {
                    log::debug!("Self-initiated DM WHO query for {}", username);
                    state.self_initiated_dms.insert(username);
                }
                Some(LogEvent::DmTabOpened(info)) => {
                    if state.self_initiated_dms.remove(&info.username) {
                        log::debug!("Ignoring self-initiated DM tab for {}", info.username);
                    } else {
                        log::info!(
                            "DM tab opened: username={}, platform={}",
                            info.username,
                            info.platform
                        );
                        crate::emit(DaemonEvent::DmTabOpened(DmTabOpenedEvent {
                            timestamp: Utc::now(),
                            username: info.username,
                            platform: info.platform.to_string(),
                        }));
                    }
                }
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;

    const ACCOUNT_ID: &str = "2baaaaaaaaaaaaaaaaaaaaaa";
    const USERNAME: &str = "sample_account";

    fn parse_events(entries: Vec<String>) -> Vec<LogEvent> {
        entries
            .into_iter()
            .filter_map(|e| logs::parse_log_line(&e))
            .collect()
    }

    /// Simulates real game session by appending log chunks to a temp file and
    /// reading incrementally — matching exactly what the watcher does on each
    /// debounce event.
    ///
    /// Timeline (timestamps are seconds from game start, from
    /// testdata/logs/login-logout-shutdown.log):
    ///   T=0s   startup diagnostics  → no events
    ///   T=72s  "Logged in"          → Login event
    ///   T=72-84s mid-session lines  → no events
    ///   T=84s  "Player name changed"→ Login event (account confirmation)
    ///   T=167s shutdown + QUIT      → Logout event
    #[test]
    fn test_incremental_login_logout_detection() {
        let path = std::env::temp_dir().join(format!(
            "wf_watcher_test_{}.log",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        scopeguard::defer! {
            let _ = std::fs::remove_file(&path);
        }

        let append = |content: &str| {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
        };

        let mut parser = LogEntryParser::new();
        let mut read_file = File::open({
            append("");
            &path
        })
        .unwrap();
        let mut last_pos = 0u64;

        // ── T=0s: startup diagnostics ────────────────────────────────────────
        append(
            "0.049 Sys [Diag]: Build Label: 2026.02.13.16.03 Retail Windows x64 [Stripped]\r\n\
             0.100 Sys [Info]: Loading packages took 0.0ms\r\n\
             2.272 Net [Info]: RMI::Initialize - Methods: 431\r\n\
             71.730 Gfx [Error]: Flushed 63 active-prefetch PSO jobs\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        assert!(
            parse_events(entries).is_empty(),
            "no events expected during startup"
        );

        // ── T=72s: account login ──────────────────────────────────────────────
        append("72.458 Sys [Info]: Logged in sample_account (2baaaaaaaaaaaaaaaaaaaaaa)\r\n");
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1, "expected exactly one login event");
        match &events[0] {
            LogEvent::Login(info) => {
                assert_eq!(info.account_id, ACCOUNT_ID);
                assert_eq!(info.username, USERNAME);
            }
            _ => panic!("expected Login"),
        }

        // ── T=72-84s: mid-session activity ───────────────────────────────────
        append(
            "72.459 Sys [Info]: Using profile dir C:\\Warframe\\3684EDC75CAB924E0418513469C6EE3B\r\n\
             72.460 Sys [Info]: Profile hash on read: 6501EF2950164301C055C2A2EC6AD536\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        assert!(
            parse_events(entries).is_empty(),
            "no events expected during mid-session activity"
        );

        // ── T=84s: player name change (account confirmation) ──────────────────
        append(
            "84.333 Sys [Info]: Player name changed to sample_account \
             Clan: TestC#963 AccountId: 2baaaaaaaaaaaaaaaaaaaaaa\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(
            events.len(),
            1,
            "expected exactly one login event from name-change"
        );
        match &events[0] {
            LogEvent::Login(info) => {
                assert_eq!(info.account_id, ACCOUNT_ID);
                assert_eq!(info.username, USERNAME);
            }
            _ => panic!("expected Login from name-change"),
        }

        // ── T=167s: shutdown sequence + logout ────────────────────────────────
        append(
            "167.073 Sys [Info]: Discord Service has begun shut down.\r\n\
             167.073 Sys [Info]: ===[ Exiting main loop ]===\r\n\
             167.073 Net [Info]: IRC out: QUIT :Logged out of game\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1, "expected exactly one logout event");
        assert!(
            matches!(events[0], LogEvent::Logout),
            "expected Logout event"
        );
    }

    /// Simulates DM tab events arriving during a session.
    ///
    /// Timeline:
    ///   T=0s    startup + login
    ///   T=88s   first DM tab (redacted_alpha, PC)
    ///   T=113s  second DM tab (redacted_bravo, PC)
    ///   T=125s  third DM tab (redacted_charlie, PC) + non-DM chat noise
    ///   T=161s  fourth DM tab (redacted_delta, PC)
    #[test]
    fn test_incremental_dm_tab_detection() {
        let path = std::env::temp_dir().join(format!(
            "wf_dm_test_{}.log",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        scopeguard::defer! {
            let _ = std::fs::remove_file(&path);
        }

        let append = |content: &str| {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
        };

        let mut parser = LogEntryParser::new();
        let mut read_file = File::open({
            append("");
            &path
        })
        .unwrap();
        let mut last_pos = 0u64;

        // ── T=0s: startup + login ────────────────────────────────────────────
        append(
            "0.049 Sys [Diag]: Build Label: 2026.02.13.16.03\r\n\
             72.458 Sys [Info]: Logged in sample_account (2baaaaaaaaaaaaaaaaaaaaaa)\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], LogEvent::Login(_)));

        // ── T=88s: first DM (PC platform \u{E000}) ──────────────────────────
        append(
            "88.663 Net [Info]: IRC out: WHOIS `redacted_alpha\r\n\
             88.906 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_alpha\u{E000} to index 6\r\n\
             88.907 Script [Info]: ChatRedux.lua: Chat: Filters for Fredacted_alpha\u{E000}:\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1, "expected one DM event for redacted_alpha");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_alpha");
                assert_eq!(info.platform, "pc");
            }
            _ => panic!("expected DirectMessage"),
        }

        // ── T=113s: second DM (PC) ──────────────────────────────────────────
        append(
            "113.428 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_bravo\u{E000} to index 6\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1, "expected one DM event for redacted_bravo");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_bravo");
                assert_eq!(info.platform, "pc");
            }
            _ => panic!("expected DirectMessage"),
        }

        // ── T=125s: third DM + non-DM AddTab noise ──────────────────────────
        append(
            "125.000 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Q_EN_AS to index 3\r\n\
             125.994 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_charlie\u{E000} to index 7\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = parse_events(entries);
        assert_eq!(
            events.len(),
            1,
            "non-DM tab (Q_EN_AS) should not produce an event"
        );
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_charlie");
                assert_eq!(info.platform, "pc");
            }
            _ => panic!("expected DirectMessage"),
        }

        // ── T=161s: fourth DM (Xbox platform \u{E001}) ──────────────────────
        append(
            "161.805 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_delta\u{E001} to index 8\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        let events = parse_events(entries);
        assert_eq!(events.len(), 1, "expected one DM event for redacted_delta");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_delta");
                assert_eq!(info.platform, "xbox");
            }
            _ => panic!("expected DirectMessage"),
        }
    }

    /// Verifies that self-initiated DM tabs (preceded by `IRC out: WHO`)
    /// are filtered out by watcher state, while incoming DMs are emitted.
    #[test]
    fn test_self_initiated_dm_filtered_out() {
        let path = std::env::temp_dir().join(format!(
            "wf_dm_who_test_{}.log",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        scopeguard::defer! {
            let _ = std::fs::remove_file(&path);
        }

        let append = |content: &str| {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
        };

        let mut parser = LogEntryParser::new();
        let mut read_file = File::open({
            append("");
            &path
        })
        .unwrap();
        let mut last_pos = 0u64;
        let mut self_initiated: HashSet<String> = HashSet::new();

        // Helper: simulate watcher state filtering on parsed events
        let filter_events =
            |events: Vec<LogEvent>, initiated: &mut HashSet<String>| -> Vec<LogEvent> {
                events
                    .into_iter()
                    .filter(|e| match e {
                        LogEvent::DmWhoQuery(username) => {
                            initiated.insert(username.clone());
                            false
                        }
                        LogEvent::DmTabOpened(info) => !initiated.remove(&info.username),
                        _ => true,
                    })
                    .collect()
            };

        // ── T=0s: startup ────────────────────────────────────────────────────
        append("0.049 Sys [Diag]: Build Label: 2026.02.13.16.03\r\n");
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert!(events.is_empty());

        // ── T=163s: incoming DM from redacted_echo (no preceding WHO) ───────────────
        append(
            "163.252 Net [Info]: Received IT_FROM_PEER introduction request\r\n\
             163.502 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 9\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert_eq!(events.len(), 1, "incoming DM should produce an event");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_echo");
                assert_eq!(info.platform, "pc");
            }
            _ => panic!("expected DmTabOpened"),
        }

        // ── T=344s: tabs closed (irrelevant noise) ──────────────────────────
        append(
            "344.886 Script [Info]: ChatRedux.lua: ChatRedux::RemoveTab: Removing tab with name Fredacted_echo\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert!(events.is_empty());

        // ── T=353s: self-initiated DM to redacted_echo (WHO → AddTab) ──────────────
        append(
            "353.340 Net [Info]: IRC out: WHO redacted_echo??? n%nu\r\n\
             353.596 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 8\r\n\
             353.599 Net [Info]: IRC out: PRIVMSG redacted_echo :hello\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert!(
            events.is_empty(),
            "self-initiated DM (preceded by WHO) should be filtered out"
        );

        // ── T=360s: another incoming DM from a different user ────────────────
        append(
            "360.100 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_foxtrot\u{E002} to index 9\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert_eq!(events.len(), 1, "incoming DM should produce an event");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_foxtrot");
                assert_eq!(info.platform, "playstation");
            }
            _ => panic!("expected DmTabOpened"),
        }

        // ── T=400s: close redacted_echo tab again, then redacted_echo initiates ───────────
        append(
            "400.000 Script [Info]: ChatRedux.lua: ChatRedux::RemoveTab: Removing tab with name Fredacted_echo\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        last_pos = metadata(&path).unwrap().len();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert!(events.is_empty());

        // ── T=420s: redacted_echo DMs us again (no WHO — they initiated) ───────────
        append(
            "420.000 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 8\r\n",
        );
        let entries = read_new_entries(&mut read_file, last_pos, &mut parser).unwrap();
        let events = filter_events(parse_events(entries), &mut self_initiated);
        assert_eq!(
            events.len(),
            1,
            "redacted_echo re-initiating after our earlier WHO should still emit"
        );
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_echo");
                assert_eq!(info.platform, "pc");
            }
            _ => panic!("expected DmTabOpened"),
        }
    }
}
