use std::fs;
use std::io::Cursor;
use std::os::fd::{AsRawFd, OwnedFd};
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use ashpd::desktop::{
    PersistMode,
    screencast::{CursorMode, Screencast, SourceType, Stream},
};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use image::{ColorType, ImageEncoder, codecs::png::PngEncoder};
use serde::{Deserialize, Serialize};

use wf_core::storage;

use super::common::ensure_png_bytes;

const RESTORE_TOKEN_FILE: &str = "gnome_screencast_token.json";
const FRAME_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Default, Serialize, Deserialize)]
struct StoredPortalToken {
    restore_token: String,
}

struct PortalStream {
    stream: Stream,
    fd: OwnedFd,
}

pub(super) async fn capture_window() -> Result<Vec<u8>> {
    let stored_token = read_restore_token();
    match open_portal_stream(stored_token.as_deref()).await {
        Ok(stream) => capture_portal_stream(stream).await,
        Err(err) if stored_token.is_some() => {
            log::warn!(
                "GNOME ScreenCast portal restore token failed; retrying without token: {}",
                err
            );
            delete_restore_token();
            let stream = open_portal_stream(None).await?;
            capture_portal_stream(stream).await
        }
        Err(err) => Err(err),
    }
}

async fn open_portal_stream(restore_token: Option<&str>) -> Result<PortalStream> {
    let proxy = Screencast::new()
        .await
        .context("Failed to connect to xdg-desktop-portal ScreenCast interface")?;
    let session = proxy
        .create_session()
        .await
        .context("Failed to create GNOME ScreenCast portal session")?;

    proxy
        .select_sources(
            &session,
            CursorMode::Hidden,
            SourceType::Window.into(),
            false,
            restore_token,
            PersistMode::ExplicitlyRevoked,
        )
        .await
        .context("Failed to select GNOME ScreenCast portal window source")?;

    let response = proxy
        .start(&session, None)
        .await
        .context("Failed to start GNOME ScreenCast portal session")?
        .response()
        .context("GNOME ScreenCast portal did not grant capture access")?;

    if let Some(token) = response.restore_token() {
        write_restore_token(token);
    }

    let stream = response
        .streams()
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("GNOME ScreenCast portal returned no PipeWire stream"))?;
    let fd = proxy
        .open_pipe_wire_remote(&session)
        .await
        .context("Failed to open GNOME ScreenCast PipeWire remote")?;

    Ok(PortalStream { stream, fd })
}

async fn capture_portal_stream(portal_stream: PortalStream) -> Result<Vec<u8>> {
    tokio::task::spawn_blocking(move || capture_pipewire_frame(portal_stream))
        .await
        .context("GNOME ScreenCast frame capture task failed")?
}

fn capture_pipewire_frame(portal_stream: PortalStream) -> Result<Vec<u8>> {
    gst::init().context("Failed to initialize GStreamer")?;

    let pipewire_src = gst::ElementFactory::make("pipewiresrc")
        .property("fd", portal_stream.fd.as_raw_fd())
        .property("path", portal_stream.stream.pipe_wire_node_id().to_string())
        .build()
        .context(
            "Failed to create GStreamer pipewiresrc element; install the PipeWire GStreamer plugin",
        )?;
    let convert = gst::ElementFactory::make("videoconvert")
        .build()
        .context("Failed to create GStreamer videoconvert element")?;
    let appsink = gst_app::AppSink::builder()
        .caps(
            &gst::Caps::builder("video/x-raw")
                .field("format", "RGBA")
                .build(),
        )
        .max_buffers(1)
        .drop(true)
        .sync(false)
        .build();

    let pipeline = gst::Pipeline::default();
    pipeline
        .add_many([
            &pipewire_src,
            &convert,
            appsink.upcast_ref::<gst::Element>(),
        ])
        .context("Failed to build GNOME ScreenCast GStreamer pipeline")?;
    gst::Element::link_many([
        &pipewire_src,
        &convert,
        appsink.upcast_ref::<gst::Element>(),
    ])
    .context("Failed to link GNOME ScreenCast GStreamer pipeline")?;

    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to start GNOME ScreenCast GStreamer pipeline")?;

    let sample_result = appsink
        .try_pull_sample(gst::ClockTime::from_nseconds(
            FRAME_TIMEOUT.as_nanos().min(u64::MAX as u128) as u64,
        ))
        .ok_or_else(|| anyhow!("Timed out waiting for a GNOME ScreenCast frame"))
        .and_then(sample_to_png);

    let _ = pipeline.set_state(gst::State::Null);
    sample_result
}

fn sample_to_png(sample: gst::Sample) -> Result<Vec<u8>> {
    let caps = sample
        .caps()
        .ok_or_else(|| anyhow!("GNOME ScreenCast frame missing caps"))?;
    let info = gstreamer_video::VideoInfo::from_caps(caps)
        .context("Failed to read GNOME ScreenCast frame video info")?;
    let buffer = sample
        .buffer()
        .ok_or_else(|| anyhow!("GNOME ScreenCast sample missing buffer"))?;
    let map = buffer
        .map_readable()
        .context("Failed to map GNOME ScreenCast frame buffer")?;

    let width = info.width();
    let height = info.height();
    if width == 0 || height == 0 {
        bail!("GNOME ScreenCast returned an invalid frame size: {width}x{height}");
    }

    let expected_stride = width as usize * 4;
    let stride = info.stride()[0] as usize;
    let frame_len = expected_stride
        .checked_mul(height as usize)
        .ok_or_else(|| anyhow!("GNOME ScreenCast frame dimensions overflow"))?;
    let mut rgba = Vec::with_capacity(frame_len);

    for row in 0..height as usize {
        let start = row
            .checked_mul(stride)
            .ok_or_else(|| anyhow!("GNOME ScreenCast frame stride overflow"))?;
        let end = start
            .checked_add(expected_stride)
            .ok_or_else(|| anyhow!("GNOME ScreenCast frame row overflow"))?;
        let row_bytes = map
            .as_slice()
            .get(start..end)
            .ok_or_else(|| anyhow!("GNOME ScreenCast frame buffer is smaller than expected"))?;
        rgba.extend_from_slice(row_bytes);
    }

    let mut png = Vec::new();
    PngEncoder::new(Cursor::new(&mut png))
        .write_image(&rgba, width, height, ColorType::Rgba8.into())
        .context("Failed to encode GNOME ScreenCast frame as PNG")?;
    ensure_png_bytes(&png, "GNOME ScreenCast portal capture")?;

    Ok(png)
}

fn restore_token_path() -> Result<std::path::PathBuf> {
    Ok(storage::app_cache_dir()?.join(RESTORE_TOKEN_FILE))
}

fn read_restore_token() -> Option<String> {
    let path = restore_token_path().ok()?;
    let raw = fs::read_to_string(&path).ok()?;
    let stored: StoredPortalToken = serde_json::from_str(&raw).ok()?;
    if stored.restore_token.is_empty() {
        None
    } else {
        Some(stored.restore_token)
    }
}

fn write_restore_token(token: &str) {
    let result = (|| -> Result<()> {
        let path = restore_token_path()?;
        let raw = serde_json::to_string_pretty(&StoredPortalToken {
            restore_token: token.to_string(),
        })?;
        fs::write(&path, raw).with_context(|| {
            format!(
                "Failed to write GNOME ScreenCast restore token to {}",
                path.display()
            )
        })?;
        Ok(())
    })();

    if let Err(err) = result {
        log::warn!("Failed to persist GNOME ScreenCast restore token: {}", err);
    }
}

fn delete_restore_token() {
    if let Ok(path) = restore_token_path() {
        let _ = fs::remove_file(path);
    }
}
