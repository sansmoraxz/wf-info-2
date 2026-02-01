use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::events::DaemonEvent;
use super::utils::parse_params;

/// Parameters for subscribe request.
#[derive(Debug, Deserialize, Default)]
pub struct SubscribeParams {
    /// List of event types to subscribe to. If empty or None, subscribes to all events.
    pub events: Option<Vec<String>>,
}

/// Filter for determining which events to send to a subscriber.
#[derive(Debug, Clone)]
pub struct EventFilter {
    /// If Some, only events matching these types are sent.
    /// If None, all events are sent.
    allowed_events: Option<HashSet<String>>,
}

impl EventFilter {
    pub fn new(events: Option<Vec<String>>) -> Self {
        let allowed_events = events.map(|e| e.into_iter().collect());
        Self { allowed_events }
    }

    /// Returns true if the event should be sent to the subscriber.
    pub fn matches(&self, event: &DaemonEvent) -> bool {
        match &self.allowed_events {
            Some(allowed) => allowed.contains(event.event_type()),
            None => true,
        }
    }

    /// Returns the list of allowed events, if any filter is set.
    pub fn allowed_events(&self) -> Option<Vec<String>> {
        self.allowed_events
            .as_ref()
            .map(|s| s.iter().cloned().collect())
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
    pub response: Value,
}

/// Handle a subscribe request and return the filter and response.
pub fn handle_subscribe(params: Option<Value>) -> anyhow::Result<SubscribeResult> {
    let params: SubscribeParams = parse_params(params)?;
    let filter = EventFilter::new(params.events);

    let filter_info = filter.allowed_events().map(|events| SubscribeFilterInfo {
        allowed_events: events,
    });

    let response = SubscribeResponse {
        subscribed: true,
        filter: filter_info,
    };

    Ok(SubscribeResult {
        filter,
        response: json!(response),
    })
}
