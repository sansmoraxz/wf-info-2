mod common;
mod gnome;
mod hyprland;
mod kde;
mod niri;
mod x11;

use std::sync::{LazyLock, Mutex};

use anyhow::{Result, anyhow, bail};

use wf_core::process;

use self::common::{EnvironmentKind, detect_wayland_environment};

static BACKEND_CACHE: LazyLock<Mutex<Option<BackendCacheEntry>>> =
    LazyLock::new(|| Mutex::new(None));

#[derive(Debug, Clone, PartialEq, Eq)]
enum CaptureBackend {
    X11Window { window_id: String },
    KdeSpectacle,
    GnomeScreenCastPortal,
    NiriScreenshot,
    Grim { locator: WindowLocator },
    Unsupported { reason: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindowLocator {
    Hyprland,
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
    warframe_pid: u32,
    resolution: &BackendResolution,
) -> Result<(Vec<u8>, String)> {
    match &resolution.capture_backend {
        CaptureBackend::X11Window { window_id } => {
            log::info!("Capturing Warframe via X11/XWayland window {}", window_id);
            let bytes = x11::capture_window(window_id)?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::KdeSpectacle => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=KdeSpectacle",
                resolution.environment
            );
            let bytes = kde::capture_active_window(warframe_pid).inspect_err(|err| {
                log::error!("KDE Wayland capture failed: {}", err);
            })?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::GnomeScreenCastPortal => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=GnomeScreenCastPortal",
                resolution.environment
            );
            let bytes = gnome::capture_window().await.inspect_err(|err| {
                log::error!("GNOME ScreenCast portal capture failed: {}", err);
            })?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::NiriScreenshot => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=NiriScreenshot",
                resolution.environment
            );
            let bytes = niri::capture_window(warframe_pid).inspect_err(|err| {
                log::error!("niri capture failed: {}", err);
            })?;
            Ok((bytes, "image/png".to_string()))
        }
        CaptureBackend::Grim { locator } => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=Grim({:?})",
                resolution.environment,
                locator
            );
            let bytes = match locator {
                WindowLocator::Hyprland => hyprland::capture_window(warframe_pid),
            }
            .inspect_err(|err| {
                log::error!("grim capture failed: {}", err);
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

    // try wayland environments and supported capture tools if no x11 game window was found
    let environment = detect_wayland_environment();
    let capture_backend = match environment {
        EnvironmentKind::KdeWayland => CaptureBackend::KdeSpectacle,
        EnvironmentKind::Hyprland => CaptureBackend::Grim {
            locator: WindowLocator::Hyprland,
        },
        EnvironmentKind::Niri => CaptureBackend::NiriScreenshot,
        EnvironmentKind::X11 => CaptureBackend::Unsupported {
            reason: "X11 was detected but no Warframe X11/XWayland window was found",
        },
        EnvironmentKind::GnomeWayland => CaptureBackend::GnomeScreenCastPortal,
        EnvironmentKind::OtherWayland => CaptureBackend::Unsupported {
            reason: "Wayland window-only capture is unsupported for this desktop with the installed tools; run Warframe under XWayland/X11 or use a supported compositor backend",
        },
        EnvironmentKind::Unknown => CaptureBackend::Unsupported {
            reason: "Unsupported compositor environment; run Warframe under XWayland/X11 or use a supported compositor backend",
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
