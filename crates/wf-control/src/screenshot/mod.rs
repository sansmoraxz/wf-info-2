use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use anyhow::{Context, Result};
use base64::Engine;
use chrono::{DateTime, Utc};
use rand::random;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use wf_core::storage;

use super::events::{DaemonEvent, ScreenshotTriggeredEvent};
use super::state::AppState;

#[derive(Debug, Clone, Copy, Default)]
pub struct ScreenshotConfig {
    pub native_wayland_capture: bool,
}

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub(crate) use unix::BackendCacheEntry;
#[cfg(unix)]
pub(crate) use unix::capture_screen;
#[cfg(windows)]
pub(crate) use windows::WindowCacheEntry;
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

#[derive(Debug, Serialize)]
struct ScreenshotEventLogEntry<'a> {
    id: &'a str,
    timestamp: DateTime<Utc>,
    metadata: &'a Option<Value>,
    content_type: &'a str,
    content_len: usize,
}

pub(crate) async fn handle_screenshot_trigger(
    state: &AppState,
    params: ScreenshotParams,
) -> Result<ScreenshotEvent> {
    let total_start = Instant::now();

    let capture_start = Instant::now();
    let (bytes, content_type) = capture_screen(&state.screenshot).await?;
    log::trace!(
        "Screenshot capture returned {} bytes ({}) in {:?}",
        bytes.len(),
        content_type,
        capture_start.elapsed()
    );

    let base64_start = Instant::now();
    let base64_content = base64::engine::general_purpose::STANDARD.encode(&bytes);
    log::trace!(
        "Screenshot base64 encoding produced {} bytes in {:?}",
        base64_content.len(),
        base64_start.elapsed()
    );

    let record_start = Instant::now();
    let event = record_screenshot_event(params.metadata, base64_content, content_type)?;
    log::trace!(
        "Screenshot event persistence completed in {:?}",
        record_start.elapsed()
    );

    state.emit(DaemonEvent::ScreenshotTriggered(ScreenshotTriggeredEvent {
        timestamp: event.timestamp,
        event_id: event.id.clone(),
    }));

    log::trace!("Screenshot handler total time {:?}", total_start.elapsed());

    Ok(event)
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
    let log_path = cache_dir.join("screenshot_events.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .context("Failed to open screenshot events log")?;
    let line = serde_json::to_string(&ScreenshotEventLogEntry {
        id: &event.id,
        timestamp: event.timestamp,
        metadata: &event.metadata,
        content_type: &event.content_type,
        content_len: event.content.len(),
    })
    .context("Failed to serialize screenshot event log entry")?;
    writeln!(file, "{}", line).context("Failed to append screenshot event")?;

    Ok(event)
}
