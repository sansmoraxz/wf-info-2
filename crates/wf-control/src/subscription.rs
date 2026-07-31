use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::events::{DaemonEvent, DaemonEventKind};

/// Parameters for subscribe request.
#[derive(Debug, Deserialize, Default)]
pub struct SubscribeParams {
    /// List of event types to subscribe to. If empty or None, subscribes to all events.
    pub events: Option<Vec<String>>,
}

/// Filter for determining which events to send to a subscriber.
#[derive(Debug, Clone)]
pub struct EventFilter {
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
    pub fn matches(&self, event: &DaemonEvent) -> bool {
        match &self.allowed_events {
            Some(allowed) => allowed.contains(&event.kind()),
            None => true,
        }
    }

    /// Returns the list of allowed events, if any filter is set.
    pub fn allowed_events(&self) -> Option<Vec<String>> {
        self.allowed_events
            .as_ref()
            .map(|s| s.iter().map(ToString::to_string).collect())
    }
}

/// Response data for successful subscribe.
#[derive(Debug, Serialize)]
pub struct SubscribeResponse {
    pub subscribed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SubscribeFilterInfo>,
}

#[derive(Debug, Serialize)]
pub struct SubscribeFilterInfo {
    pub allowed_events: Vec<String>,
}

/// Result of handling a subscribe request.
pub struct SubscribeResult {
    pub filter: EventFilter,
    pub response: SubscribeResponse,
}

/// Handle a subscribe request and return the filter and response.
pub fn handle_subscribe(params: SubscribeParams) -> anyhow::Result<SubscribeResult> {
    let filter = EventFilter::from(params);

    let filter_info = filter.allowed_events().map(|events| SubscribeFilterInfo {
        allowed_events: events,
    });

    let response = SubscribeResponse {
        subscribed: true,
        filter: filter_info,
    };

    Ok(SubscribeResult { filter, response })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DaemonEvent, SystemQuitEvent, SystemQuitReason};
    use chrono::Utc;

    #[test]
    fn lifecycle_events_can_be_selected_by_wire_name() {
        let filter = EventFilter::from(SubscribeParams {
            events: Some(vec!["system_quit".to_string()]),
        });
        let event = DaemonEvent::SystemQuit(SystemQuitEvent {
            timestamp: Utc::now(),
            reason: SystemQuitReason::Unexpected,
        });

        assert!(filter.matches(&event));
    }
}
