use chrono::Utc;
use std::collections::HashSet;
use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
use wf_ocr::{RelicRecognizer, load_image};

use crate::screenshot::capture_screen;
use crate::{
    AccountLoginEvent, AccountLogoutEvent, DaemonEvent, DmTabOpenedEvent, SystemQuitReason,
};
use wf_core::account::AccountInfo;
use wf_core::logs::pattern::LogProcessingEngine;
use wf_core::logs::{self, LineAssembler, LogEvent, LogSource};

#[cfg(feature = "memory")]
use crate::{InventoryFetchedEvent, ProfileUpdatedEvent};
#[cfg(feature = "memory")]
use wf_core::{api, inventory_refresh, process, storage};

#[derive(Debug, Clone, Default)]
pub struct GameLifecycleTracker {
    quit_requested: Arc<AtomicBool>,
}

impl GameLifecycleTracker {
    fn mark_quit_requested(&self) {
        self.quit_requested.store(true, Ordering::SeqCst);
    }

    pub fn exit_reason(&self) -> SystemQuitReason {
        if self.quit_requested.load(Ordering::SeqCst) {
            SystemQuitReason::Requested
        } else {
            SystemQuitReason::Unexpected
        }
    }
}

/// Whether login events trigger the automatic profile/inventory fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoCallbacks {
    Enabled,
    Skip,
}

/// Whether the relic reward-selection window is on screen.
#[derive(Debug, Default, PartialEq, Eq)]
enum RelicState {
    #[default]
    Closed,
    Open,
}

impl RelicState {
    /// Transition to open. Returns `false` on a duplicate open (window
    /// already showing), so the caller can skip re-triggering OCR.
    fn open(&mut self) -> bool {
        if *self == Self::Open {
            return false;
        }
        *self = Self::Open;
        true
    }

    /// Transition to closed. Returns `false` when the window was never
    /// observed open (stale close event).
    fn close(&mut self) -> bool {
        if *self == Self::Closed {
            return false;
        }
        *self = Self::Closed;
        true
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum SessionState {
    #[default]
    LoggedOut,
    LoggedIn {
        username: String,
    },
}

impl SessionState {
    /// Transition to logged-in. Returns `false` when this is a duplicate
    /// login for the same username (dedup lives in the transition).
    fn login(&mut self, username: &str) -> bool {
        if matches!(self, Self::LoggedIn { username: u } if u == username) {
            return false;
        }
        *self = Self::LoggedIn {
            username: username.to_string(),
        };
        true
    }

    fn logout(&mut self) {
        *self = Self::LoggedOut;
    }
}

#[derive(Debug, Default)]
enum TradeState {
    #[default]
    Idle,
    Pending(logs::TradeInfo),
}

impl TradeState {
    fn confirm_popup(&mut self, info: logs::TradeInfo) {
        *self = Self::Pending(info);
    }

    /// Take the pending trade on success/failure; `None` when no trade was
    /// pending (log stream out of sync).
    fn resolve(&mut self) -> Option<logs::TradeInfo> {
        match std::mem::take(self) {
            Self::Pending(info) => Some(info),
            Self::Idle => None,
        }
    }
}

struct WatchState {
    #[cfg_attr(not(feature = "memory"), allow(dead_code))]
    warframe_pid: Option<u32>,
    #[cfg_attr(not(feature = "memory"), allow(dead_code))]
    auto_callbacks: AutoCallbacks,
    session: SessionState,
    /// Usernames for which we issued `IRC out: WHO` (self-initiated DMs).
    /// Used to suppress DmTabOpened events for tabs we opened ourselves.
    self_initiated_dms: HashSet<String>,
    trade: TradeState,
    relic: RelicState,
}

impl WatchState {
    fn new(warframe_pid: Option<u32>, auto_callbacks: AutoCallbacks) -> Self {
        Self {
            warframe_pid,
            auto_callbacks,
            session: SessionState::default(),
            self_initiated_dms: HashSet::new(),
            trade: TradeState::default(),
            relic: RelicState::default(),
        }
    }
}

fn event_emitter_fn(
    mut state: WatchState,
    entries: Vec<LogEvent>,
    lifecycle: &GameLifecycleTracker,
) -> WatchState {
    for entry in entries {
        match entry {
            LogEvent::Login(AccountInfo { username, .. }) => {
                if !state.session.login(&username) {
                    log::debug!("Duplicate login event for username={}", username);
                    continue;
                }
                log::info!("User logged in: username={}", username);
                crate::emit(DaemonEvent::AccountLogin(AccountLoginEvent {
                    timestamp: Utc::now(),
                    username: username.clone(),
                }));
                #[cfg(feature = "memory")]
                tokio::spawn(handle_login_event(
                    username,
                    state.warframe_pid,
                    state.auto_callbacks,
                ));
            }
            LogEvent::Logout => {
                state.session.logout();
                log::info!("User logged out");
                crate::emit(DaemonEvent::AccountLogout(AccountLogoutEvent {
                    timestamp: Utc::now(),
                }));
            }
            LogEvent::QuitRequested => {
                log::info!("Warframe quit command observed");
                lifecycle.mark_quit_requested();
            }
            LogEvent::WhoQuery(username) => {
                log::debug!("Self-initiated DM WHO query for {}", username);
                state.self_initiated_dms.insert(username);
            }
            LogEvent::DmTabOpened(info) => {
                if state.self_initiated_dms.remove(&info.username) {
                    log::debug!("Ignoring self-initiated DM tab for {}", info.username);
                } else {
                    log::info!(
                        "DM tab opened: username={}, platform={:?}",
                        info.username,
                        info.platform
                    );
                    crate::emit(DaemonEvent::DmTabOpened(DmTabOpenedEvent {
                        timestamp: Utc::now(),
                        username: info.username,
                        platform: info.platform,
                    }));
                }
            }
            LogEvent::TradeSuccess => {
                if let Some(trades) = state.trade.resolve() {
                    log::info!("Trade confirmed: {:?}", &trades);
                    let popup = crate::events::TradeConfirmPopupEvent {
                        sent: trades.sent,
                        received: trades.received,
                        name: trades.name,
                        platform: trades.platform,
                    };
                    crate::emit(DaemonEvent::TradeSuccess(crate::events::TradeSuccessEvent(
                        popup,
                    )));
                } else {
                    log::error!("No trade activity in watch buffer. Something's probably wrong");
                }
            }
            LogEvent::TradeFail(reason) => {
                if let Some(trades) = state.trade.resolve() {
                    log::info!("Trade failed: {:?}, reason: {}", &trades, reason);
                    let popup = crate::events::TradeConfirmPopupEvent {
                        sent: trades.sent,
                        received: trades.received,
                        name: trades.name,
                        platform: trades.platform,
                    };
                    crate::emit(DaemonEvent::TradeFailed(crate::events::TradeFailedEvent(
                        popup, reason,
                    )));
                } else {
                    log::error!("No trade in watch buffer. Something's probably wrong");
                }
            }
            LogEvent::TradeConfirmPopup(info) => {
                log::info!("Got trade request confirmation: {:?}", info);
                state.trade.confirm_popup(info);
            }
            LogEvent::RelicOpen => {
                if !state.relic.open() {
                    log::debug!("Duplicate relic open event; window already showing");
                    continue;
                }
                log::info!("Relic selection window opened");
                tokio::spawn(handle_relic_selection_popup());
            }
            LogEvent::RelicClose => {
                if !state.relic.close() {
                    log::debug!("Relic close event without an observed open");
                    continue;
                }
                log::info!("Relic selection window closed");
                crate::emit(DaemonEvent::RelicSelectionClosed);
            }
        }
    }
    state
}

#[cfg(feature = "memory")]
async fn handle_login_event(
    user_name: String,
    known_pid: Option<u32>,
    auto_callbacks: AutoCallbacks,
) {
    let Some(pid) = known_pid.or_else(process::get_warframe_pid) else {
        log::info!("Warframe not running - skipping profile and inventory fetch");
        return;
    };

    log::info!(
        "Warframe running (PID: {}), attempting to resolve account authorization...",
        pid
    );
    let auth = match process::scan_memory_for_auth_with_retry(pid, 5, Duration::from_secs(3)).await
    {
        Ok(Some(auth)) => auth,
        Ok(None) => {
            log::warn!("Could not resolve account authorization from process memory");
            log::info!("Tip: Make sure you're logged into Warframe");
            return;
        }
        Err(e) => {
            log::error!("Memory scan error: {}", e);
            log::info!("Tip: Grant necessary permissions or try running with sudo");
            return;
        }
    };

    log::info!("Resolved account authorization from process memory");
    match api::fetch_player_profile(&auth.account_id).await {
        Ok(profile) => {
            log::info!("Fetched profile for {}: {:?}", user_name, profile);
            if let Err(e) = storage::save_encrypted_profile(&profile) {
                log::error!("Failed to save profile for {}: {}", user_name, e);
            } else {
                crate::emit(DaemonEvent::ProfileUpdated(ProfileUpdatedEvent {
                    timestamp: Utc::now(),
                    account_id: auth.account_id.clone(),
                }));
            }
        }
        Err(e) => {
            log::error!("Failed to fetch profile for {}: {}", user_name, e);
        }
    }

    if auto_callbacks == AutoCallbacks::Skip {
        log::info!("Skipping auto fetch inventory. Fetch manually if required.");
        return;
    }

    match inventory_refresh::fetch_inventory_with_auth_from_process(
        pid,
        auth,
        5,
        Duration::from_secs(3),
    )
    .await
    {
        Ok(Some(result)) => {
            log::info!("Successfully fetched live inventory");
            if let Err(e) = storage::save_inventory(&result.inventory) {
                log::error!("Failed to save inventory: {}", e);
            } else {
                if let Err(e) =
                    storage::touch_inventory_updated(Some(&crate::events::Source::Auto.to_string()))
                {
                    log::warn!("Failed to update inventory metadata: {}", e);
                }
                crate::emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
                    timestamp: Utc::now(),
                    source: crate::events::Source::Auto,
                    summary: crate::inventory::inventory_summary(&result.inventory),
                }));
            }
        }
        Ok(None) => {
            log::warn!("Could not extract auth data from process memory");
            log::info!("Tip: Make sure you're logged into Warframe");
        }
        Err(e) => {
            log::error!("Live inventory fetch failed: {}", e);
        }
    }
}

static RELIC_RECOG_ENGINE: LazyLock<wf_ocr::RelicRecognizer> =
    LazyLock::new(|| RelicRecognizer::new(&wf_ocr::DEFAULT_OCR_ENGINE));

async fn handle_relic_selection_popup() {
    tokio::time::sleep(Duration::from_millis(500)).await;

    let res = capture_screen().await;
    match res {
        Ok((image_bytes, _)) => match load_image(image_bytes) {
            Ok(img) => match RELIC_RECOG_ENGINE.recognize_and_list(&img) {
                Ok(mut v) => {
                    log::info!("Got relic items: {:?}", v);
                    let filtered: Vec<String> = v.drain(..).map(|e| e.text).collect();
                    let popup = crate::events::RelicSelectionPopup { items: filtered };
                    crate::emit(DaemonEvent::RelicSelectionOpen(popup));
                }
                Err(e) => log::error!("OCR failed on screenshot image {}", e),
            },
            Err(e) => log::error!("Failed to parse screenshot image {}", e),
        },
        Err(e) => log::error!("Failed to capture screenshot for relic ocr {}", e),
    }
}

pub async fn observe_warframe_activity<S: LogSource>(
    source: S,
    warframe_pid: Option<u32>,
    auto_callbacks: AutoCallbacks,
) -> Result<(), Box<dyn std::error::Error>> {
    observe_warframe_activity_with_lifecycle(
        source,
        warframe_pid,
        auto_callbacks,
        GameLifecycleTracker::default(),
    )
    .await
}

pub async fn observe_warframe_activity_with_lifecycle<S: LogSource>(
    mut source: S,
    warframe_pid: Option<u32>,
    auto_callbacks: AutoCallbacks,
    lifecycle: GameLifecycleTracker,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Watching for Warframe activity...");
    let log_processor = LogProcessingEngine::new()?;
    let mut state = WatchState::new(warframe_pid, auto_callbacks);
    let mut assembler = LineAssembler::default();

    loop {
        match source.recv_chunk().await? {
            Some(chunk) => {
                let lines = assembler.push_chunk(&chunk);
                if lines.is_empty() {
                    continue;
                }
                let entries = log_processor.extract_events(&lines);
                log::debug!("Observed entries: {:?}", entries);
                state = event_emitter_fn(state, entries, &lifecycle);
            }
            None => {
                log::info!("Log source closed");
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, VecDeque};
    use std::io;

    const USERNAME: &str = "Jasper123";

    struct ChunkHarness {
        assembler: LineAssembler,
        processor: LogProcessingEngine,
    }

    impl ChunkHarness {
        fn new() -> Self {
            Self {
                assembler: LineAssembler::default(),
                processor: LogProcessingEngine::new().unwrap(),
            }
        }

        fn feed(&mut self, chunk: &str) -> Vec<LogEvent> {
            let lines = self.assembler.push_chunk(chunk);
            if lines.is_empty() {
                return Vec::new();
            }
            self.processor.extract_events(&lines)
        }
    }

    struct MockLogSource {
        chunks: VecDeque<io::Result<Option<String>>>,
    }

    impl LogSource for MockLogSource {
        fn recv_chunk(
            &mut self,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = io::Result<Option<String>>> + Send + '_>,
        > {
            Box::pin(async move { self.chunks.pop_front().unwrap_or(Ok(None)) })
        }
    }

    #[test]
    fn test_incremental_login_logout_detection() {
        let mut harness = ChunkHarness::new();

        let events = harness.feed(
            "0.049 Sys [Diag]: Build Label: 2026.02.13.16.03 Retail Windows x64 [Stripped]\r\n\
             0.100 Sys [Info]: Loading packages took 0.0ms\r\n\
             2.272 Net [Info]: RMI::Initialize - Methods: 431\r\n\
             71.730 Gfx [Error]: Flushed 63 active-prefetch PSO jobs\r\n",
        );
        assert!(events.is_empty(), "no events expected during startup");

        let events = harness.feed("72.458 Sys [Info]: Logged in Jasper123 (AREDN0T1CE672)\r\n");
        assert_eq!(
            events.len(),
            0,
            "login event not tracked anymore, rely on profile activity"
        );

        let events = harness.feed(
            "72.459 Sys [Info]: Using profile dir C:\\Warframe\\3684EDC75CAB924E0418513469C6EE3B\r\n\
             72.460 Sys [Info]: Profile hash on read: 6501EF2950164301C055C2A2EC6AD536\r\n",
        );
        assert!(
            events.is_empty(),
            "no events expected during mid-session activity"
        );

        let events = harness.feed(
            "84.333 Sys [Info]: Player name changed to Jasper123\u{E000} \
             Clan: TestC#963\r\n",
        );
        assert_eq!(
            events.len(),
            1,
            "expected exactly one login event from name-change"
        );
        match &events[0] {
            LogEvent::Login(info) => {
                assert_eq!(info.username, USERNAME);
                assert_eq!(info.platform, wf_core::account::Platform::PC);
                assert_eq!(info.clan, "TestC#963");
            }
            _ => panic!("expected Login from name-change"),
        }

        let events = harness.feed("167.073 Sys [Info]: Logout confirmed\r\n");
        assert_eq!(events.len(), 1, "expected exactly one logout event");
        assert!(
            matches!(events[0], LogEvent::Logout),
            "expected Logout event"
        );
    }

    #[test]
    fn test_three_login_two_logout_cycles_then_requested_quit() {
        let mut harness = ChunkHarness::new();
        let events = harness.feed(
            "19.467 Sys [Info]: Player name changed to SamplePlayer\u{E000} Clan: Test Clan#903\r\n\
             20.000 Sys [Info]: Logout confirmed\r\n\
             100.059 Sys [Info]: Player name changed to SamplePlayer\u{E000} Clan: Test Clan#903\r\n\
             150.567 Sys [Info]: Logout confirmed\r\n\
             180.000 Sys [Info]: Player name changed to SamplePlayer\u{E000} Clan: Test Clan#903\r\n\
             300.123 Sys [Info]: Executing command: /EE/Editor/ToolMenus/Commands/CmdQuit\r\n\
             300.300 Sys [Info]: ===[ Exiting main loop ]===\r\n\
             301.201 Sys [Info]: Main Shutdown Initiated.\r\n\
             301.500 Sys [Info]: Main Shutdown Complete.\r\n\
             301.600 Net [Info]: IRC out: QUIT :Logged out of game\r\n",
        );

        assert_eq!(events.len(), 6);
        assert!(matches!(events[0], LogEvent::Login(_)));
        assert!(matches!(events[1], LogEvent::Logout));
        assert!(matches!(events[2], LogEvent::Login(_)));
        assert!(matches!(events[3], LogEvent::Logout));
        assert!(matches!(events[4], LogEvent::Login(_)));
        assert!(matches!(events[5], LogEvent::QuitRequested));
    }

    #[test]
    fn test_legacy_login_suffix_is_accepted_and_ignored() {
        let mut harness = ChunkHarness::new();
        let events = harness.feed(
            "84.333 Sys [Info]: Player name changed to Jasper123\u{E002} \
             Clan: Test Clan#963 AccountId: 2baaaaaaaaaaaaaaaaaaaaaa\r\n",
        );

        assert_eq!(events.len(), 1);
        match &events[0] {
            LogEvent::Login(info) => {
                assert_eq!(info.username, USERNAME);
                assert_eq!(info.platform, wf_core::account::Platform::PLAYSTATION);
                assert_eq!(info.clan, "Test Clan#963");
            }
            _ => panic!("expected Login from legacy name-change line"),
        }
    }

    #[test]
    fn test_duplicate_login_suppression_resets_on_logout() {
        let mut session = SessionState::default();

        assert!(session.login(USERNAME));
        assert!(!session.login(USERNAME));
        assert!(session.login("AnotherPlayer"));
        assert!(!session.login("AnotherPlayer"));

        session.logout();
        assert!(session.login("AnotherPlayer"));
    }

    #[test]
    fn test_quit_request_changes_process_exit_reason() {
        let lifecycle = GameLifecycleTracker::default();
        assert_eq!(lifecycle.exit_reason(), SystemQuitReason::Unexpected);

        event_emitter_fn(
            WatchState::new(None, AutoCallbacks::Skip),
            vec![LogEvent::QuitRequested],
            &lifecycle,
        );

        assert_eq!(lifecycle.exit_reason(), SystemQuitReason::Requested);
    }

    #[test]
    fn test_incremental_dm_tab_detection() {
        let mut harness = ChunkHarness::new();

        let _ = harness.feed(
            "0.049 Sys [Diag]: Build Label: 2026.02.13.16.03\r\n\
             72.458 Sys [Info]: Logged in sample_account (2baaaaaaaaaaaaaaaaaaaaaa)\r\n",
        );

        let events = harness.feed(
            "88.663 Net [Info]: IRC out: WHOIS `redacted_alpha\r\n\
             88.906 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_alpha\u{E000} to index 6\r\n\
             88.907 Script [Info]: Chat: Filters for Fredacted_alpha\u{E000}:\r\n",
        );
        assert_eq!(events.len(), 1, "expected one DM event for redacted_alpha");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_alpha");
                assert_eq!(info.platform, wf_core::account::Platform::PC);
            }
            _ => panic!("expected DirectMessage"),
        }

        let events = harness.feed(
            "113.428 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_bravo\u{E000} to index 6\r\n",
        );
        assert_eq!(events.len(), 1, "expected one DM event for redacted_bravo");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_bravo");
                assert_eq!(info.platform, wf_core::account::Platform::PC);
            }
            _ => panic!("expected DirectMessage"),
        }

        let events = harness.feed(
            "125.000 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Q_EN_AS to index 3\r\n\
             125.994 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_charlie\u{E000} to index 7\r\n",
        );
        assert_eq!(
            events.len(),
            1,
            "non-DM tab (Q_EN_AS) should not produce an event"
        );
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_charlie");
                assert_eq!(info.platform, wf_core::account::Platform::PC);
            }
            _ => panic!("expected DirectMessage"),
        }

        let events = harness.feed(
            "161.805 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_delta\u{E001} to index 8\r\n",
        );
        assert_eq!(events.len(), 1, "expected one DM event for redacted_delta");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_delta");
                assert_eq!(info.platform, wf_core::account::Platform::XBOX);
            }
            _ => panic!("expected DirectMessage"),
        }
    }

    #[test]
    fn test_self_initiated_dm_filtered_out() {
        let mut harness = ChunkHarness::new();
        let mut self_initiated: HashSet<String> = HashSet::new();

        let filter_events =
            |events: Vec<LogEvent>, initiated: &mut HashSet<String>| -> Vec<LogEvent> {
                events
                    .into_iter()
                    .filter(|e| match e {
                        LogEvent::WhoQuery(username) => {
                            initiated.insert(username.clone());
                            false
                        }
                        LogEvent::DmTabOpened(info) => !initiated.remove(&info.username),
                        _ => true,
                    })
                    .collect()
            };

        let events = filter_events(
            harness.feed("0.049 Sys [Diag]: Build Label: 2026.02.13.16.03\r\n"),
            &mut self_initiated,
        );
        assert!(events.is_empty());

        let events = filter_events(
            harness.feed(
                "163.252 Net [Info]: Received IT_FROM_PEER introduction request\r\n\
                 163.502 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 9\r\n",
            ),
            &mut self_initiated,
        );
        assert_eq!(events.len(), 1, "incoming DM should produce an event");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_echo");
                assert_eq!(info.platform, wf_core::account::Platform::PC);
            }
            _ => panic!("expected DmTabOpened"),
        }

        let events = filter_events(
            harness.feed(
                "344.886 Script [Info]: ChatRedux.lua: ChatRedux::RemoveTab: Removing tab with name Fredacted_echo\r\n",
            ),
            &mut self_initiated,
        );
        assert!(events.is_empty());

        let events = filter_events(
            harness.feed(
                "353.340 Net [Info]: IRC out: WHO redacted_echo??? n%nu\r\n\
                 353.596 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 8\r\n\
                 353.599 Net [Info]: IRC out: PRIVMSG redacted_echo :hello\r\n",
            ),
            &mut self_initiated,
        );
        assert!(
            events.is_empty(),
            "self-initiated DM (preceded by WHO) should be filtered out"
        );

        let events = filter_events(
            harness.feed(
                "360.100 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_foxtrot\u{E002} to index 9\r\n",
            ),
            &mut self_initiated,
        );
        assert_eq!(events.len(), 1, "incoming DM should produce an event");
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_foxtrot");
                assert_eq!(info.platform, wf_core::account::Platform::PLAYSTATION);
            }
            _ => panic!("expected DmTabOpened"),
        }

        let events = filter_events(
            harness.feed(
                "400.000 Script [Info]: ChatRedux.lua: ChatRedux::RemoveTab: Removing tab with name Fredacted_echo\r\n",
            ),
            &mut self_initiated,
        );
        assert!(events.is_empty());

        let events = filter_events(
            harness.feed(
                "420.000 Script [Info]: ChatRedux.lua: ChatRedux::AddTab: Adding tab with channel name: Fredacted_echo\u{E000} to index 8\r\n",
            ),
            &mut self_initiated,
        );
        assert_eq!(
            events.len(),
            1,
            "redacted_echo re-initiating after our earlier WHO should still emit"
        );
        match &events[0] {
            LogEvent::DmTabOpened(info) => {
                assert_eq!(info.username, "redacted_echo");
                assert_eq!(info.platform, wf_core::account::Platform::PC);
            }
            _ => panic!("expected DmTabOpened"),
        }
    }

    #[test]
    fn test_trade_success() {
        let mut harness = ChunkHarness::new();

        let events = harness.feed(
            "0.049 Sys [Diag]: 2026.03.25.16.45 Retail Windows x64 [Stripped]\r\n\
             72.458 Sys [Info]: Logged in sample_account (2baaaaaaaaaaaaaaaaaaaaaa)\r\n",
        );
        assert!(events.is_empty());

        let events = harness.feed(
            "478.779 Script [Info]: Dialog.lua: Dialog::CreateOkCancel(description=Are you sure you want to accept this trade? You are offering:\r\n\
            \r\n\
            Platinum x 30\r\n\
            \r\n\
            \r\n\
            \r\n\
            and will receive from redacted_alpha\u{E000} the following:\r\n\
            \r\n\
            Kestrel Prime Blueprint\r\n\
            \r\n\
            Kestrel Prime Grip\r\n\
            \r\n\
            Kestrel Prime Blade\r\n\
            \r\n\
            Kestrel Prime Blade\r\n\
            \r\n\
            Kestrel Prime Blade, title= leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)\r\n",
        );
        assert_eq!(events.len(), 1, "confirm popup only captured");
        match &events[0] {
            LogEvent::TradeConfirmPopup(trade_info) => {
                assert_eq!(trade_info.name, "redacted_alpha");
                assert_eq!(trade_info.sent.len(), 1);
                assert_eq!(trade_info.received.len(), 3);
            }
            _ => panic!("expected TradeConfirmPopup"),
        }

        let events = harness.feed(
            "484.224 Script [Info]: Dialog.lua: Dialog::CreateOk(description=The trade was successful!, title= leftItem=/Menu/Confirm_Item_Ok)\r\n",
        );
        assert_eq!(events.len(), 1, "trade success");
        assert!(matches!(events[0], LogEvent::TradeSuccess));
    }

    #[test]
    fn test_chunk_boundaries_do_not_need_to_align_to_lines() {
        let mut harness = ChunkHarness::new();

        let first = harness.feed("84.333 Sys [Info]: Player name changed to Jasper");
        assert!(first.is_empty(), "partial line must be buffered");

        let second = harness.feed(
            "123\u{E000} Clan: TestC#963\r\n\
             167.073 Sys [Info]: Logout conf",
        );
        assert_eq!(second.len(), 1);
        assert!(matches!(second[0], LogEvent::Login(_)));

        let third = harness.feed("irmed\r\n");
        assert_eq!(third.len(), 1);
        assert!(matches!(third[0], LogEvent::Logout));
    }

    #[tokio::test]
    async fn test_observe_warframe_activity_stops_when_source_closes() {
        let source = MockLogSource {
            chunks: VecDeque::from([Ok(None)]),
        };

        observe_warframe_activity(source, Some(1234), AutoCallbacks::Skip)
            .await
            .unwrap();
    }
}
