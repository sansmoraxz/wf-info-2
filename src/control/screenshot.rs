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

use crate::storage;

use super::broadcaster;
use super::events::{DaemonEvent, ScreenshotTriggeredEvent};
use super::utils::parse_params;

#[cfg(unix)]
async fn capture_screen() -> Result<(String, String)> {
    use ashpd::desktop::screenshot::Screenshot;

    let response = Screenshot::request()
        .interactive(false)
        .modal(false)
        .send()
        .await
        .context("Failed to request screenshot")?
        .response()
        .context("Screenshot request failed")?;

    let path = response
        .uri()
        .to_file_path()
        .map_err(|_| anyhow::anyhow!("Invalid screenshot URI"))?;
    let png_bytes = fs::read(&path).context("Failed to read screenshot file")?;

    let base64_content = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Ok((base64_content, "image/png".to_string()))
}

#[cfg(windows)]
async fn capture_screen() -> Result<(String, String)> {
    use image::{ImageBuffer, Rgb};
    use win_screenshot::prelude::*;

    let buf = capture_display().context("Failed to capture display")?;

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width as u32, buf.height as u32, buf.pixels)
            .context("Failed to create image buffer")?;

    let mut png_bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )?;

    let base64_content = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Ok((base64_content, "image/png".to_string()))
}

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

    // Capture screenshot first
    let (content, content_type) = capture_screen().await?;

    let event = record_screenshot_event(params.metadata, content, content_type)?;

    // Emit screenshot triggered event
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
