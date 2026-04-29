use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use anyhow::{Context, Result};
use base64::Engine;
use chrono::{DateTime, Utc};
use rand::random;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use wf_core::storage;

use super::broadcaster;
use super::events::{DaemonEvent, ScreenshotTriggeredEvent};
use super::utils::parse_params;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub(crate) use unix::capture_screen;
#[cfg(windows)]
pub(crate) use windows::capture_screen;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct ScreenshotParams {
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ScreenshotEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<Value>,
    pub content: String,
    pub content_type: String,
}

pub(crate) async fn handle_screenshot_trigger(params: Option<Value>) -> Result<Value> {
    let params: ScreenshotParams = parse_params(params)?;

    let (bytes, content_type) = capture_screen().await?;
    let base64_content = base64::engine::general_purpose::STANDARD.encode(&bytes);

    let event = record_screenshot_event(params.metadata, base64_content, content_type)?;

    broadcaster::emit(DaemonEvent::ScreenshotTriggered(ScreenshotTriggeredEvent {
        timestamp: event.timestamp,
        event_id: event.id.clone(),
    }));

    Ok(serde_json::to_value(event).context("Failed to serialize screenshot event")?)
}

fn record_screenshot_event(
    metadata: Option<Value>,
    content: String,
    content_type: String,
) -> Result<ScreenshotEvent> {
    let event = ScreenshotEvent {
        id: format!("{}-{}", Utc::now().timestamp_millis(), random::<u32>()),
        timestamp: Utc::now(),
        metadata,
        content,
        content_type,
    };

    let cache_dir = storage::app_cache_dir()?;
    let last_path = cache_dir.join("last_screenshot.json");
    let raw =
        serde_json::to_string_pretty(&event).context("Failed to serialize screenshot event")?;
    fs::write(&last_path, raw).context("Failed to write last screenshot event")?;

    let log_path = cache_dir.join("screenshot_events.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .context("Failed to open screenshot events log")?;
    let line = serde_json::to_string(&event).context("Failed to serialize screenshot event")?;
    writeln!(file, "{}", line).context("Failed to append screenshot event")?;

    Ok(event)
}
