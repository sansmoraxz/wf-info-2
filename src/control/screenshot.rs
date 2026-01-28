use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rand::random;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::storage;

use super::utils::parse_params;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct ScreenshotParams {
    pub action: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ScreenshotEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub action: Option<String>,
    pub metadata: Option<Value>,
}

pub(crate) fn handle_screenshot_trigger(params: Option<Value>) -> Result<Value> {
    let params: ScreenshotParams = parse_params(params)?;
    let event = record_screenshot_event(params.action, params.metadata)?;
    Ok(serde_json::to_value(event).context("Failed to serialize screenshot event")?)
}

fn record_screenshot_event(
    action: Option<String>,
    metadata: Option<Value>,
) -> Result<ScreenshotEvent> {
    let event = ScreenshotEvent {
        id: format!("{}-{}", Utc::now().timestamp_millis(), random::<u32>()),
        timestamp: Utc::now(),
        action,
        metadata,
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
