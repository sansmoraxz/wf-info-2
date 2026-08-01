use std::fs;
use std::os::fd::{AsRawFd, OwnedFd};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail};
use ashpd::desktop::{
    PersistMode,
    screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType, Stream},
};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use serde::{Deserialize, Serialize};

use wf_core::storage;

use super::common::{ensure_bmp_bytes, new_bmp_rgb24};

const RESTORE_TOKEN_FILE: &str = "unix_screencast_token.json";
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
    let start = Instant::now();
    let stored_token = read_restore_token();
    let result = match open_portal_stream(stored_token.as_deref()).await {
        Ok(stream) => capture_portal_stream(stream).await,
        Err(err) if stored_token.is_some() => {
            log::warn!(
                "Wayland ScreenCast portal restore token failed; retrying without token: {}",
                err
            );
            delete_restore_token();
            let stream = open_portal_stream(None).await?;
            capture_portal_stream(stream).await
        }
        Err(err) => Err(err),
    };
    log::trace!(
        "Screenshot Wayland portal capture_window completed in {:?}",
        start.elapsed()
    );
    result
}

async fn open_portal_stream(restore_token: Option<&str>) -> Result<PortalStream> {
    let proxy = Screencast::new()
        .await
        .context("Failed to connect to xdg-desktop-portal ScreenCast interface")?;
    let session = proxy
        .create_session(Default::default())
        .await
        .context("Failed to create Wayland ScreenCast portal session")?;

    proxy
        .select_sources(
            &session,
            SelectSourcesOptions::default()
                .set_cursor_mode(CursorMode::Hidden)
                .set_sources(ashpd::enumflags2::BitFlags::from(SourceType::Window))
                .set_multiple(false)
                .set_restore_token(restore_token)
                .set_persist_mode(PersistMode::ExplicitlyRevoked),
        )
        .await
        .context("Failed to select Wayland ScreenCast portal window source; ensure your xdg-desktop-portal backend supports WINDOW capture")?;

    let response = proxy
        .start(&session, None, Default::default())
        .await
        .context("Failed to start Wayland ScreenCast portal session")?
        .response()
        .context("Wayland ScreenCast portal did not grant window capture access")?;

    if let Some(token) = response.restore_token() {
        write_restore_token(token);
    }

    let stream = response
        .streams()
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("Wayland ScreenCast portal returned no PipeWire stream; your portal backend may not support window capture"))?;
    let fd = proxy
        .open_pipe_wire_remote(&session, Default::default())
        .await
        .context("Failed to open Wayland ScreenCast PipeWire remote")?;

    Ok(PortalStream { stream, fd })
}

async fn capture_portal_stream(portal_stream: PortalStream) -> Result<Vec<u8>> {
    tokio::task::spawn_blocking(move || capture_pipewire_frame(portal_stream))
        .await
        .context("Wayland ScreenCast frame capture task failed")?
}

fn capture_pipewire_frame(portal_stream: PortalStream) -> Result<Vec<u8>> {
    let total_start = Instant::now();
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
                // BGRA matches BMP's byte order, so rows copy straight
                // through; videoconvert does any swizzling upstream.
                .field("format", "BGRA")
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
        .context("Failed to build Wayland ScreenCast GStreamer pipeline")?;
    gst::Element::link_many([
        &pipewire_src,
        &convert,
        appsink.upcast_ref::<gst::Element>(),
    ])
    .context("Failed to link Wayland ScreenCast GStreamer pipeline")?;

    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to start Wayland ScreenCast GStreamer pipeline")?;

    let sample_start = Instant::now();
    let sample_result = appsink
        .try_pull_sample(gst::ClockTime::from_nseconds(
            FRAME_TIMEOUT.as_nanos().min(u64::MAX as u128) as u64,
        ))
        .ok_or_else(|| anyhow!("Timed out waiting for a Wayland ScreenCast frame"))
        .inspect(|_| {
            log::trace!(
                "Screenshot Wayland portal frame sample pulled in {:?}",
                sample_start.elapsed()
            );
        })
        .and_then(sample_to_bmp);

    let _ = pipeline.set_state(gst::State::Null);
    log::trace!(
        "Screenshot Wayland portal PipeWire frame capture completed in {:?}",
        total_start.elapsed()
    );
    sample_result
}

fn sample_to_bmp(sample: gst::Sample) -> Result<Vec<u8>> {
    let total_start = Instant::now();
    let caps = sample
        .caps()
        .ok_or_else(|| anyhow!("Wayland ScreenCast frame missing caps"))?;
    let info = gstreamer_video::VideoInfo::from_caps(caps)
        .context("Failed to read Wayland ScreenCast frame video info")?;
    let buffer = sample
        .buffer()
        .ok_or_else(|| anyhow!("Wayland ScreenCast sample missing buffer"))?;
    let map = buffer
        .map_readable()
        .context("Failed to map Wayland ScreenCast frame buffer")?;

    let width = info.width();
    let height = info.height();
    if width == 0 || height == 0 {
        bail!("Wayland ScreenCast returned an invalid frame size: {width}x{height}");
    }

    let source_row_len = width as usize * 4;
    let stride = info.stride()[0] as usize;
    let (mut bmp, bmp_stride) = new_bmp_rgb24(width, height)?;
    let pixel_offset = 54usize;

    let convert_start = Instant::now();
    for row in 0..height as usize {
        let start = row
            .checked_mul(stride)
            .ok_or_else(|| anyhow!("Wayland ScreenCast frame stride overflow"))?;
        let end = start
            .checked_add(source_row_len)
            .ok_or_else(|| anyhow!("Wayland ScreenCast frame row overflow"))?;
        let row_bytes = map
            .as_slice()
            .get(start..end)
            .ok_or_else(|| anyhow!("Wayland ScreenCast frame buffer is smaller than expected"))?;
        let bmp_row_start = pixel_offset + row * bmp_stride;
        let bmp_row = &mut bmp[bmp_row_start..bmp_row_start + width as usize * 3];
        for (out, pixel) in bmp_row.chunks_exact_mut(3).zip(row_bytes.chunks_exact(4)) {
            out.copy_from_slice(&pixel[..3]);
        }
    }
    log::trace!(
        "Screenshot Wayland portal BGRA-to-BMP pixel conversion completed in {:?}",
        convert_start.elapsed()
    );

    ensure_bmp_bytes(&bmp, "Wayland ScreenCast portal capture")?;
    log::trace!(
        "Screenshot Wayland portal sample_to_bmp completed in {:?}",
        total_start.elapsed()
    );

    Ok(bmp)
}

fn restore_token_path() -> Result<std::path::PathBuf> {
    Ok(storage::app_cache_dir()?.join(RESTORE_TOKEN_FILE))
}

fn read_restore_token() -> Option<String> {
    read_restore_token_from_path(restore_token_path().ok()?)
}

fn read_restore_token_from_path(path: std::path::PathBuf) -> Option<String> {
    let raw = fs::read_to_string(path).ok()?;
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
                "Failed to write Wayland ScreenCast restore token to {}",
                path.display()
            )
        })?;
        Ok(())
    })();

    if let Err(err) = result {
        log::warn!(
            "Failed to persist Wayland ScreenCast restore token: {}",
            err
        );
    }
}

fn delete_restore_token() {
    if let Ok(path) = restore_token_path() {
        let _ = fs::remove_file(path);
    }
}
