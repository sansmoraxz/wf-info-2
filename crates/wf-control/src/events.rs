use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wf_core::{account::Platform, logs::TradeItem};

/// All daemon events that can be emitted and subscribed to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DaemonEvent {
    AccountLogin(AccountLoginEvent),
    AccountLogout(AccountLogoutEvent),
    InventoryFetched(InventoryFetchedEvent),
    InventoryStale(InventoryStaleEvent),
    ProfileUpdated(ProfileUpdatedEvent),
    ScreenshotTriggered(ScreenshotTriggeredEvent),
    DmTabOpened(DmTabOpenedEvent),
    TradeSuccess(TradeSuccessEvent),
    TradeFailed(TradeFailedEvent),
    RelicSelectionOpen(RelicSelectionPopup),
    RelicSelectionClosed,
}

impl DaemonEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            DaemonEvent::AccountLogin(_) => "account_login",
            DaemonEvent::AccountLogout(_) => "account_logout",
            DaemonEvent::InventoryFetched(_) => "inventory_fetched",
            DaemonEvent::InventoryStale(_) => "inventory_stale",
            DaemonEvent::ProfileUpdated(_) => "profile_updated",
            DaemonEvent::ScreenshotTriggered(_) => "screenshot_triggered",
            DaemonEvent::DmTabOpened(_) => "dm_tab_opened",
            DaemonEvent::TradeSuccess(_) => "trade_success",
            DaemonEvent::TradeFailed(_) => "trade_failed",
            DaemonEvent::RelicSelectionOpen(_) => "relic_opened",
            DaemonEvent::RelicSelectionClosed => "relic_closed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoginEvent {
    pub timestamp: DateTime<Utc>,
    pub account_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLogoutEvent {
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryFetchedEvent {
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub summary: Value,
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
    pub account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotTriggeredEvent {
    pub timestamp: DateTime<Utc>,
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmTabOpenedEvent {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeConfirmPopupEvent {
    pub sent: Vec<TradeItem>,
    pub received: Vec<TradeItem>,
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSuccessEvent(pub TradeConfirmPopupEvent);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelicSelectionPopup {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFailedEvent(pub TradeConfirmPopupEvent, pub String);

/// Wire format for pushing events to subscribed clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event: String,
    pub payload: DaemonEvent,
}

impl EventMessage {
    pub fn from_event(event: DaemonEvent) -> Self {
        Self {
            event: event.event_type().to_string(),
            payload: event,
        }
    }
}
