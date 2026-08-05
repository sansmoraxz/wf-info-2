use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use wf_core::{
    account::{Platform, Username},
    logs::TradeInfo,
    process::AccountId,
};
use wf_inventory::FractionSyndicates;

const CHANNEL_CAPACITY: usize = 256;

/// Cheaply-cloneable handle to the daemon event broadcast channel. Every
/// module that emits or subscribes to [`DaemonEvent`]s holds one of these
/// instead of the whole application state.
#[derive(Clone)]
pub struct EventBus(broadcast::Sender<DaemonEvent>);

impl Default for EventBus {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self(sender)
    }
}

impl EventBus {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Broadcast an event to all subscribers. A send error only means there
    /// are currently no subscribers, which is fine.
    pub fn emit(&self, event: DaemonEvent) {
        self.0.send(event).ok();
    }

    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<DaemonEvent> {
        self.0.subscribe()
    }
}

/// All daemon events that can be emitted and subscribed to.
///
/// The generated [`DaemonEventKind`] discriminant enum carries the
/// subscription wire names; note that the relic kinds intentionally differ
/// from the serde `type` tags ("relic_opened"/"relic_closed" vs
/// "relic_selection_open"/"relic_selection_closed") — a pre-existing wire
/// contract that must be preserved.
#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumDiscriminants)]
#[serde(tag = "type", rename_all = "snake_case")]
#[strum_discriminants(
    name(DaemonEventKind),
    derive(Hash, strum::Display, strum::EnumString),
    strum(serialize_all = "snake_case")
)]
pub enum DaemonEvent {
    GameStart(GameStartEvent),
    AccountLogin(AccountLoginEvent),
    AccountLogout(AccountLogoutEvent),
    SystemQuit(SystemQuitEvent),
    InventoryFetched(InventoryFetchedEvent),
    InventoryStale(InventoryStaleEvent),
    ProfileUpdated(ProfileUpdatedEvent),
    ScreenshotTriggered(ScreenshotTriggeredEvent),
    DmTabOpened(DmTabOpenedEvent),
    TradeSuccess(TradeSuccessEvent),
    TradeFailed(TradeFailedEvent),
    #[strum_discriminants(strum(serialize = "relic_opened"))]
    RelicSelectionOpen(RelicSelectionPopup),
    #[strum_discriminants(strum(serialize = "relic_closed"))]
    RelicSelectionClosed,
}

impl DaemonEvent {
    #[must_use]
    pub fn kind(&self) -> DaemonEventKind {
        self.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStartEvent {
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoginEvent {
    pub timestamp: DateTime<Utc>,
    pub username: Username,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLogoutEvent {
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemQuitEvent {
    pub timestamp: DateTime<Utc>,
    pub reason: SystemQuitReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemQuitReason {
    Requested,
    Unexpected,
}

/// Per-category counts and trade info summarizing a fetched inventory.
// serde(default) so events recorded by older daemons (which emitted a 4-key
// subset from the watcher path) still deserialize.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InventorySummary {
    pub suits: usize,
    pub long_guns: usize,
    pub pistols: usize,
    pub melee: usize,
    pub space_suits: usize,
    pub space_guns: usize,
    pub space_melee: usize,
    pub raw_upgrades: usize,
    pub upgrades: usize,
    pub recipes: usize,
    pub pending_recipes: usize,
    pub trades_remaining: Option<i64>,
    pub supported_syndicates: Option<FractionSyndicates>,
}

/// Where an inventory fetch originated. Free-form user-supplied sources are
/// preserved verbatim in `Other`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    strum::Display,
    strum::EnumString,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Source {
    #[default]
    Manual,
    LiveRefresh,
    Auto,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryFetchedEvent {
    pub timestamp: DateTime<Utc>,
    pub source: Source,
    pub summary: InventorySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryStaleEvent {
    pub timestamp: DateTime<Utc>,
    pub stale_since: DateTime<Utc>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdatedEvent {
    pub timestamp: DateTime<Utc>,
    pub account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotTriggeredEvent {
    pub timestamp: DateTime<Utc>,
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmTabOpenedEvent {
    pub timestamp: DateTime<Utc>,
    pub username: Username,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSuccessEvent(pub TradeInfo);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelicSelectionPopup {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFailedEvent(pub TradeInfo, pub String);

/// Wire format for pushing events to subscribed clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event: String,
    pub payload: DaemonEvent,
}

impl From<DaemonEvent> for EventMessage {
    fn from(event: DaemonEvent) -> Self {
        Self {
            event: event.kind().to_string(),
            payload: event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_events_have_stable_wire_names_and_reason_values() {
        let start = DaemonEvent::GameStart(GameStartEvent {
            timestamp: Utc::now(),
        });
        assert_eq!(start.kind().to_string(), "game_start");

        let quit = DaemonEvent::SystemQuit(SystemQuitEvent {
            timestamp: Utc::now(),
            reason: SystemQuitReason::Requested,
        });
        let message = EventMessage::from(quit);
        let value = serde_json::to_value(message).unwrap();

        assert_eq!(value["event"], "system_quit");
        assert_eq!(value["payload"]["type"], "system_quit");
        assert_eq!(value["payload"]["reason"], "requested");
    }

    #[test]
    fn dm_tab_opened_platform_serializes_as_variant_name() {
        let event = DmTabOpenedEvent {
            timestamp: Utc::now(),
            username: "player".into(),
            platform: Platform::Playstation,
        };
        let value = serde_json::to_value(&event).unwrap();
        assert_eq!(value["platform"], "PLAYSTATION");

        let unknown = serde_json::to_value(Platform::Unknown).unwrap();
        assert_eq!(unknown, "UNKNOWN");
    }
}
