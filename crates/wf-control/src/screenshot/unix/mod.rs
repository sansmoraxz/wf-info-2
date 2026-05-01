mod common;
mod portal;
mod x11;

use std::sync::{LazyLock, Mutex};

use anyhow::{Result, anyhow, bail};

use wf_core::process;

use self::common::{EnvironmentKind, detect_unix_environment};

static BACKEND_CACHE: LazyLock<Mutex<Option<BackendCacheEntry>>> =
    LazyLock::new(|| Mutex::new(None));

#[derive(Debug, Clone, PartialEq, Eq)]
enum CaptureBackend {
    X11Window { window_id: String },
    WaylandScreenCastPortal,
    Unsupported { reason: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BackendResolution {
    environment: EnvironmentKind,
    capture_backend: CaptureBackend,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BackendCacheEntry {
    warframe_pid: u32,
    resolution: BackendResolution,
}

pub(crate) async fn capture_screen() -> Result<(Vec<u8>, String)> {
    let warframe_pid = process::get_warframe_pid()
        .ok_or_else(|| anyhow!("Warframe process not detected; relaunch the game and try again"))?;

    let resolution = cached_resolution(warframe_pid).unwrap_or_else(|| {
        let resolution = resolve_backend(warframe_pid);
        store_resolution(warframe_pid, &resolution);
        resolution
    });

    capture_with_backend(warframe_pid, &resolution).await
}

async fn capture_with_backend(
    _warframe_pid: u32,
    resolution: &BackendResolution,
) -> Result<(Vec<u8>, String)> {
    match &resolution.capture_backend {
        CaptureBackend::X11Window { window_id } => {
            log::info!("Capturing Warframe via X11/XWayland window {}", window_id);
            let bytes = x11::capture_window(window_id)?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::WaylandScreenCastPortal => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=WaylandScreenCastPortal",
                resolution.environment
            );
            let bytes = portal::capture_window().await.inspect_err(|err| {
                log::error!("Wayland ScreenCast portal capture failed: {}", err);
            })?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::Unsupported { reason } => bail!("{}", reason),
    }
}

fn resolve_backend(warframe_pid: u32) -> BackendResolution {
    // first try with x11 since that's most likely what's most users play on
    if let Ok(window_id) = x11::find_window(warframe_pid) {
        return BackendResolution {
            environment: EnvironmentKind::X11,
            capture_backend: CaptureBackend::X11Window { window_id },
        };
    }

    // Use the generic ScreenCast portal for native Wayland. Do not fall back to
    // monitor capture; the portal must provide a window PipeWire stream.
    let environment = detect_unix_environment();
    let capture_backend = match environment {
        EnvironmentKind::X11 => CaptureBackend::Unsupported {
            reason: "X11 was detected but no Warframe X11/XWayland window was found",
        },
        EnvironmentKind::Wayland => CaptureBackend::WaylandScreenCastPortal,
        EnvironmentKind::Unknown => CaptureBackend::Unsupported {
            reason: "Unsupported Unix display environment; run Warframe under XWayland/X11 or use a Wayland session with xdg-desktop-portal ScreenCast window capture",
        },
    };

    BackendResolution {
        environment,
        capture_backend,
    }
}

fn cached_resolution(warframe_pid: u32) -> Option<BackendResolution> {
    BACKEND_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.as_ref().cloned())
        .filter(|entry| entry.warframe_pid == warframe_pid)
        .map(|entry| entry.resolution)
}

fn store_resolution(warframe_pid: u32, resolution: &BackendResolution) {
    if let Ok(mut cache) = BACKEND_CACHE.lock() {
        *cache = Some(BackendCacheEntry {
            warframe_pid,
            resolution: resolution.clone(),
        });
    }
}
