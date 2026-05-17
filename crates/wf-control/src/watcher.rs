use chrono::Utc;
#[cfg(feature = "memory")]
use serde_json::json;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::time::Duration;
use wf_ocr::{RelicRecognizer, load_image};

use crate::screenshot::capture_screen;
use crate::{
    AccountLoginEvent, AccountLogoutEvent, DaemonEvent, DmTabOpenedEvent, ProfileUpdatedEvent,
};
use wf_core::account::AccountInfo;
use wf_core::logs::pattern::LogProcessingEngine;
use wf_core::logs::{self, LineAssembler, LogEvent, LogSource};
use wf_core::{api, storage};

#[cfg(feature = "memory")]
use crate::InventoryFetchedEvent;
#[cfg(feature = "memory")]
use wf_core::{inventory_refresh, process};

struct WatchState {
    current_account_id: Option<String>,
    /// Usernames for which we issued `IRC out: WHO` (self-initiated DMs).
    /// Used to suppress DmTabOpened events for tabs we opened ourselves.
    self_initiated_dms: HashSet<String>,
    /// Pending trade stored from recent confirmations
    trades: Option<logs::TradeInfo>,
    // Relic selection open
    relic_countdown: bool,
}

impl WatchState {
    fn new() -> Self {
        Self {
            current_account_id: None,
            self_initiated_dms: HashSet::new(),
            trades: None,
            relic_countdown: false,
        }
    }
}

fn event_emitter_fn(
    mut state: WatchState,
    entries: Vec<LogEvent>,
    warframe_pid: Option<u32>,
    skip_cb: bool,
) -> WatchState {
    for entry in entries {
        match entry {
            LogEvent::Login(AccountInfo {
                username,
                account_id,
                ..
            }) => {
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
                tokio::spawn(handle_login_event(
                    account_id,
                    username,
                    warframe_pid,
                    skip_cb,
                ));
            }
            LogEvent::Logout => {
                state.current_account_id = None;
                crate::set_current_account(None);
                log::info!("User logged out");
                crate::emit(DaemonEvent::AccountLogout(AccountLogoutEvent {
                    timestamp: Utc::now(),
                }));
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
                if let Some(trades) = state.trades.take() {
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
                if let Some(trades) = state.trades.take() {
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
                state.trades = Some(info);
            }
            LogEvent::RelicOpen => {
                log::info!("Relic selection window opened");
                state.relic_countdown = true;
                tokio::spawn(handle_relic_selection_popup());
            }
            LogEvent::RelicClose => {
                state.relic_countdown = false;
                log::info!("Relic selection window closed");
            }
        }
    }
    state
}

#[allow(unused)]
async fn handle_login_event(
    acc_id: String,
    user_name: String,
    known_pid: Option<u32>,
    skip_cb: bool,
) {
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

    #[cfg(feature = "memory")]
    if skip_cb {
        log::info!("Skipping auto fetch inventory. Fetch manually if required.");
    } else if let Some(pid) = known_pid.or_else(process::get_warframe_pid) {
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
    mut source: S,
    warframe_pid: Option<u32>,
    skip_cb: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Watching for Warframe activity...");
    let log_processor = LogProcessingEngine::new()?;
    let mut state = WatchState::new();
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
                state = event_emitter_fn(state, entries, warframe_pid, skip_cb);
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

    const ACCOUNT_ID: &str = "AREDN0T1CE672";
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
             Clan: TestC#963 AccountId: AREDN0T1CE672\r\n",
        );
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

        let events = harness.feed(
            "167.073 Sys [Info]: Discord Service has begun shut down.\r\n\
             167.073 Sys [Info]: ===[ Exiting main loop ]===\r\n\
             167.073 Net [Info]: IRC out: QUIT :Logged out of game\r\n",
        );
        assert_eq!(events.len(), 1, "expected exactly one logout event");
        assert!(
            matches!(events[0], LogEvent::Logout),
            "expected Logout event"
        );
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
            "123\u{E000} Clan: TestC#963 AccountId: AREDN0T1CE672\r\n\
             167.073 Net [Info]: IRC out: QUIT :Logged out of ga",
        );
        assert_eq!(second.len(), 1);
        assert!(matches!(second[0], LogEvent::Login(_)));

        let third = harness.feed("me\r\n");
        assert_eq!(third.len(), 1);
        assert!(matches!(third[0], LogEvent::Logout));
    }

    #[tokio::test]
    async fn test_observe_warframe_activity_stops_when_source_closes() {
        let source = MockLogSource {
            chunks: VecDeque::from([Ok(None)]),
        };

        observe_warframe_activity(source, Some(1234), true)
            .await
            .unwrap();
    }
}
