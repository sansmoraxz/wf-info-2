use std::fs;
use std::num::NonZeroI32;
use std::os::fd::{AsRawFd, OwnedFd};
use std::time::{Duration, Instant};

use ashpd::desktop::{
    PersistMode,
    screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType, Stream},
};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use serde::{Deserialize, Serialize};

use wf_core::storage;

use super::common::{BmpError, BmpRgb24};

#[derive(Debug, thiserror::Error)]
pub(crate) enum PortalError {
    #[error("Failed to connect to xdg-desktop-portal ScreenCast interface")]
    Connect(#[source] ashpd::Error),
    #[error("Failed to create Wayland ScreenCast portal session")]
    CreateSession(#[source] ashpd::Error),
    #[error(
        "Failed to select Wayland ScreenCast portal window source; ensure your xdg-desktop-portal backend supports WINDOW capture"
    )]
    SelectSources(#[source] ashpd::Error),
    #[error("Failed to start Wayland ScreenCast portal session")]
    Start(#[source] ashpd::Error),
    #[error("Wayland ScreenCast portal did not grant window capture access")]
    AccessDenied(#[source] ashpd::Error),
    #[error(
        "Wayland ScreenCast portal returned no PipeWire stream; your portal backend may not support window capture"
    )]
    NoStream,
    #[error("Failed to open Wayland ScreenCast PipeWire remote")]
    OpenRemote(#[source] ashpd::Error),
    #[error("Wayland ScreenCast frame capture task failed")]
    CaptureTask(#[source] tokio::task::JoinError),
    #[error("Failed to initialize GStreamer")]
    GstInit(#[source] gst::glib::Error),
    #[error(
        "Failed to create GStreamer pipewiresrc element; install the PipeWire GStreamer plugin"
    )]
    PipewireSrc(#[source] gst::glib::BoolError),
    #[error("Failed to create GStreamer videoconvert element")]
    VideoConvert(#[source] gst::glib::BoolError),
    #[error("Failed to build Wayland ScreenCast GStreamer pipeline")]
    BuildPipeline(#[source] gst::glib::BoolError),
    #[error("Failed to link Wayland ScreenCast GStreamer pipeline")]
    LinkPipeline(#[source] gst::glib::BoolError),
    #[error("Failed to start Wayland ScreenCast GStreamer pipeline")]
    StartPipeline(#[source] gst::StateChangeError),
    #[error("Timed out waiting for a Wayland ScreenCast frame")]
    FrameTimeout,
    #[error("Wayland ScreenCast frame missing caps")]
    MissingCaps,
    #[error("Failed to read Wayland ScreenCast frame video info")]
    VideoInfo(#[source] gst::glib::BoolError),
    #[error("Wayland ScreenCast sample missing buffer")]
    MissingBuffer,
    #[error("Failed to map Wayland ScreenCast frame buffer")]
    MapBuffer(#[source] gst::glib::BoolError),
    #[error("Wayland ScreenCast returned an invalid frame size {width}x{height}")]
    InvalidFrameSize { width: u32, height: u32 },
    #[error("Wayland ScreenCast frame buffer is smaller than expected")]
    BufferTooSmall,
    #[error("Wayland ScreenCast frame dimensions overflow")]
    FrameOverflow,
    #[error(transparent)]
    Bmp(#[from] BmpError),
}

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

pub(super) async fn capture_window() -> Result<Vec<u8>, PortalError> {
    let start = Instant::now();
    let stored_token = read_restore_token();
    let result = match open_portal_stream(stored_token.as_deref()).await {
        Ok(stream) => capture_portal_stream(stream).await,
        Err(err) if stored_token.is_some() => {
            log::warn!(
                "Wayland ScreenCast portal restore token failed; retrying without token: {err}"
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

async fn open_portal_stream(restore_token: Option<&str>) -> Result<PortalStream, PortalError> {
    let proxy = Screencast::new().await.map_err(PortalError::Connect)?;
    let session = proxy
        .create_session(Default::default())
        .await
        .map_err(PortalError::CreateSession)?;

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
        .map_err(PortalError::SelectSources)?;

    let response = proxy
        .start(&session, None, Default::default())
        .await
        .map_err(PortalError::Start)?
        .response()
        .map_err(PortalError::AccessDenied)?;

    if let Some(token) = response.restore_token() {
        write_restore_token(token);
    }

    let stream = response
        .streams()
        .first()
        .cloned()
        .ok_or(PortalError::NoStream)?;
    let fd = proxy
        .open_pipe_wire_remote(&session, Default::default())
        .await
        .map_err(PortalError::OpenRemote)?;

    Ok(PortalStream { stream, fd })
}

async fn capture_portal_stream(portal_stream: PortalStream) -> Result<Vec<u8>, PortalError> {
    tokio::task::spawn_blocking(move || capture_pipewire_frame(portal_stream))
        .await
        .map_err(PortalError::CaptureTask)?
}

fn capture_pipewire_frame(portal_stream: PortalStream) -> Result<Vec<u8>, PortalError> {
    let total_start = Instant::now();
    gst::init().map_err(PortalError::GstInit)?;

    let pipewire_src = gst::ElementFactory::make("pipewiresrc")
        .property("fd", portal_stream.fd.as_raw_fd())
        .property("path", portal_stream.stream.pipe_wire_node_id().to_string())
        .build()
        .map_err(PortalError::PipewireSrc)?;
    let convert = gst::ElementFactory::make("videoconvert")
        .build()
        .map_err(PortalError::VideoConvert)?;
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
        .map_err(PortalError::BuildPipeline)?;
    gst::Element::link_many([
        &pipewire_src,
        &convert,
        appsink.upcast_ref::<gst::Element>(),
    ])
    .map_err(PortalError::LinkPipeline)?;

    pipeline
        .set_state(gst::State::Playing)
        .map_err(PortalError::StartPipeline)?;

    let sample_start = Instant::now();
    let sample_result = appsink
        .try_pull_sample(gst::ClockTime::from_nseconds(
            u64::try_from(FRAME_TIMEOUT.as_nanos()).unwrap_or(u64::MAX),
        ))
        .ok_or(PortalError::FrameTimeout)
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

fn sample_to_bmp(sample: gst::Sample) -> Result<Vec<u8>, PortalError> {
    let total_start = Instant::now();
    let caps = sample.caps().ok_or(PortalError::MissingCaps)?;
    let info = gstreamer_video::VideoInfo::from_caps(caps).map_err(PortalError::VideoInfo)?;
    let buffer = sample.buffer().ok_or(PortalError::MissingBuffer)?;
    let map = buffer.map_readable().map_err(PortalError::MapBuffer)?;

    let width = info.width();
    let height = info.height();
    let frame_dim = |dim: u32| {
        i32::try_from(dim)
            .ok()
            .and_then(NonZeroI32::new)
            .ok_or(PortalError::InvalidFrameSize { width, height })
    };

    let source_row_len = width as usize * 4;
    let stride = usize::try_from(info.stride()[0])
        .map_err(|_| PortalError::InvalidFrameSize { width, height })?;
    let mut bmp = BmpRgb24::new(frame_dim(width)?, frame_dim(height)?)?;

    let convert_start = Instant::now();
    for row in 0..height as usize {
        let start = row.checked_mul(stride).ok_or(PortalError::FrameOverflow)?;
        let end = start
            .checked_add(source_row_len)
            .ok_or(PortalError::FrameOverflow)?;
        let row_bytes = map
            .as_slice()
            .get(start..end)
            .ok_or(PortalError::BufferTooSmall)?;
        bmp.copy_bgrx_row(row, row_bytes);
    }
    log::trace!(
        "Screenshot Wayland portal BGRA-to-BMP pixel conversion completed in {:?}",
        convert_start.elapsed()
    );
    log::trace!(
        "Screenshot Wayland portal sample_to_bmp completed in {:?}",
        total_start.elapsed()
    );

    Ok(bmp.into_bytes())
}

fn restore_token_path() -> Result<std::path::PathBuf, storage::StorageError> {
    Ok(storage::app_cache_dir()?.join(RESTORE_TOKEN_FILE))
}

fn read_restore_token() -> Option<String> {
    read_restore_token_from_path(restore_token_path().ok()?)
}

fn read_restore_token_from_path(path: impl AsRef<std::path::Path>) -> Option<String> {
    let raw = fs::read_to_string(path).ok()?;
    let stored: StoredPortalToken = serde_json::from_str(&raw).ok()?;
    if stored.restore_token.is_empty() {
        None
    } else {
        Some(stored.restore_token)
    }
}

fn write_restore_token(token: &str) {
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let path = restore_token_path()?;
        let raw = serde_json::to_string_pretty(&StoredPortalToken {
            restore_token: token.to_string(),
        })?;
        fs::write(&path, raw)?;
        Ok(())
    })();

    if let Err(err) = result {
        log::warn!("Failed to persist Wayland ScreenCast restore token: {err}");
    }
}

fn delete_restore_token() {
    if let Ok(path) = restore_token_path() {
        let _ = fs::remove_file(path);
    }
}
