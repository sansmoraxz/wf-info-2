use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::events::{DaemonEvent, DaemonEventKind};

/// Parameters for subscribe request.
#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SubscribeParams {
    /// Comma-separated list of events to subscribe to; all events if omitted
    /// (game_start, account_login, account_logout, system_quit, inventory_fetched,
    /// inventory_stale, profile_updated, screenshot_triggered)
    #[cfg_attr(feature = "cli", arg(long, value_delimiter = ','))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

/// Filter for determining which events to send to a subscriber.
#[derive(Debug, Clone)]
pub(super) struct EventFilter {
    /// If Some, only events matching these kinds are sent.
    /// If None, all events are sent.
    allowed_events: Option<HashSet<DaemonEventKind>>,
}

// Unrecognized event names are silently dropped (lenient wire behavior,
// preserved from the String-based filter).
impl From<SubscribeParams> for EventFilter {
    fn from(params: SubscribeParams) -> Self {
        let allowed_events = params
            .events
            .map(|e| e.iter().filter_map(|s| s.parse().ok()).collect());
        Self { allowed_events }
    }
}

impl EventFilter {
    /// Returns true if the event should be sent to the subscriber.
    pub(crate) fn matches(&self, event: &DaemonEvent) -> bool {
        self.allowed_events
            .as_ref()
            .is_none_or(|allowed| allowed.contains(&event.kind()))
    }

    /// Returns the list of allowed events, if any filter is set.
    pub(crate) fn allowed_events(&self) -> Option<Vec<DaemonEventKind>> {
        self.allowed_events
            .as_ref()
            .map(|s| s.iter().copied().collect())
    }
}

/// Response data for successful subscribe.
#[derive(Debug, Serialize)]
pub(super) struct SubscribeResponse {
    pub subscribed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SubscribeFilterInfo>,
}

#[serde_with::serde_as]
#[derive(Debug, Serialize)]
pub(super) struct SubscribeFilterInfo {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    pub allowed_events: Vec<DaemonEventKind>,
}

/// Result of handling a subscribe request.
pub(super) struct SubscribeResult {
    pub filter: EventFilter,
    pub response: SubscribeResponse,
}

/// Handle a subscribe request and return the filter and response.
pub(super) fn handle_subscribe(params: SubscribeParams) -> SubscribeResult {
    let filter = EventFilter::from(params);

    let filter_info = filter.allowed_events().map(|events| SubscribeFilterInfo {
        allowed_events: events,
    });

    let response = SubscribeResponse {
        subscribed: true,
        filter: filter_info,
    };

    SubscribeResult { filter, response }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DaemonEvent, SystemQuitEvent, SystemQuitReason};
    use chrono::Utc;

    #[test]
    fn lifecycle_events_can_be_selected_by_wire_name() {
        let filter = EventFilter::from(SubscribeParams {
            events: Some(vec!["system_quit".to_owned()]),
        });
        let event = DaemonEvent::SystemQuit(SystemQuitEvent {
            timestamp: Utc::now(),
            reason: SystemQuitReason::Unexpected,
        });

        assert!(filter.matches(&event));
    }
}
