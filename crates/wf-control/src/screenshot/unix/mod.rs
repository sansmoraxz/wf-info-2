mod common;
#[cfg(feature = "native-wayland-screenshot")]
mod portal;
mod x11;

use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use anyhow::{Result, anyhow, bail};

use wf_core::process;

use self::common::{EnvironmentKind, detect_unix_environment};
use super::screenshot_config;

static BACKEND_CACHE: LazyLock<Mutex<Option<BackendCacheEntry>>> =
    LazyLock::new(|| Mutex::new(None));
static WARFRAME_PID_CACHE: LazyLock<Mutex<Option<u32>>> = LazyLock::new(|| Mutex::new(None));

#[derive(Debug, Clone, PartialEq, Eq)]
enum CaptureBackend {
    X11Window {
        window_id: String,
    },
    #[cfg(feature = "native-wayland-screenshot")]
    WaylandScreenCastPortal,
    Unsupported {
        reason: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BackendResolution {
    environment: EnvironmentKind,
    capture_backend: CaptureBackend,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BackendCacheEntry {
    warframe_pid: u32,
    native_wayland_capture: bool,
    resolution: BackendResolution,
}

pub(crate) async fn capture_screen() -> Result<(Vec<u8>, String)> {
    let total_start = Instant::now();
    let pid_start = Instant::now();
    let (warframe_pid, pid_cache_status) = cached_warframe_pid()
        .ok_or_else(|| anyhow!("Warframe process not detected; relaunch the game and try again"))?;
    log::trace!(
        "Screenshot Warframe PID lookup ({}) found {} in {:?}",
        pid_cache_status,
        warframe_pid,
        pid_start.elapsed()
    );

    let resolution_start = Instant::now();
    let native_wayland_capture = screenshot_config().native_wayland_capture;
    let cached = cached_resolution(warframe_pid, native_wayland_capture);
    let resolution = cached.clone().unwrap_or_else(|| {
        let resolution = resolve_backend(warframe_pid, native_wayland_capture);
        store_resolution(warframe_pid, native_wayland_capture, &resolution);
        resolution
    });
    log::trace!(
        "Screenshot backend resolution ({}) selected {:?} in {:?}",
        if cached.is_some() { "cached" } else { "fresh" },
        resolution,
        resolution_start.elapsed()
    );

    let capture_start = Instant::now();
    match capture_with_backend(warframe_pid, &resolution).await {
        Err(err) if cached.is_some() && resolution.is_x11_window() => {
            log::warn!(
                "Cached X11/XWayland capture backend failed; resolving screenshot backend again: {}",
                err
            );
            clear_cached_resolution();
            clear_cached_warframe_pid();
            let refreshed = resolve_backend(warframe_pid, native_wayland_capture);
            store_resolution(warframe_pid, native_wayland_capture, &refreshed);
            if refreshed == resolution {
                Err(err)
            } else {
                let result = capture_with_backend(warframe_pid, &refreshed).await;
                log::trace!(
                    "Screenshot Unix capture retry completed in {:?}; total capture_screen time {:?}",
                    capture_start.elapsed(),
                    total_start.elapsed()
                );
                result
            }
        }
        result => {
            log::trace!(
                "Screenshot Unix capture completed in {:?}; total capture_screen time {:?}",
                capture_start.elapsed(),
                total_start.elapsed()
            );
            result
        }
    }
}

async fn capture_with_backend(
    _warframe_pid: u32,
    resolution: &BackendResolution,
) -> Result<(Vec<u8>, String)> {
    match &resolution.capture_backend {
        CaptureBackend::X11Window { window_id } => {
            log::info!("Capturing Warframe via X11/XWayland window {}", window_id);
            let start = Instant::now();
            let bytes = x11::capture_window(window_id)?;
            log::trace!(
                "Screenshot X11/XWayland backend captured {} bytes in {:?}",
                bytes.len(),
                start.elapsed()
            );
            Ok((bytes, "image/bmp".to_string()))
        }
        #[cfg(feature = "native-wayland-screenshot")]
        CaptureBackend::WaylandScreenCastPortal => {
            log::info!(
                "Capturing Warframe on Unix backend: environment={:?}, capture=WaylandScreenCastPortal",
                resolution.environment
            );
            let start = Instant::now();
            let bytes = portal::capture_window().await.inspect_err(|err| {
                log::error!("Wayland ScreenCast portal capture failed: {}", err);
            })?;
            log::trace!(
                "Screenshot Wayland portal backend captured {} bytes in {:?}",
                bytes.len(),
                start.elapsed()
            );
            Ok((bytes, "image/bmp".to_string()))
        }
        CaptureBackend::Unsupported { reason } => bail!("{}", reason),
    }
}

fn resolve_backend(warframe_pid: u32, force_native_wayland: bool) -> BackendResolution {
    let start = Instant::now();
    let environment = detect_unix_environment();
    let x11_window_id = if force_native_wayland && environment == EnvironmentKind::Wayland {
        log::info!(
            "Native Wayland screenshot capture is enabled; using ScreenCast portal instead of X11/XWayland capture"
        );
        None
    } else {
        let x11_probe_start = Instant::now();
        let x11_window_id = match x11::find_window(warframe_pid) {
            Ok(window_id) => {
                log::info!(
                    "Detected Warframe X11/XWayland window {} for PID {}",
                    window_id,
                    warframe_pid
                );
                Some(window_id)
            }
            Err(err) => {
                log::debug!(
                    "Warframe X11/XWayland window probe failed for PID {}: {}",
                    warframe_pid,
                    err
                );
                None
            }
        };
        log::trace!(
            "Screenshot X11/XWayland window probe completed in {:?}",
            x11_probe_start.elapsed()
        );
        x11_window_id
    };
    let resolution = resolve_backend_from_probe(environment, x11_window_id);
    log::trace!(
        "Screenshot backend resolve completed in {:?}",
        start.elapsed()
    );
    resolution
}

fn resolve_backend_from_probe(
    detected_environment: EnvironmentKind,
    x11_window_id: Option<String>,
) -> BackendResolution {
    if let Some(window_id) = x11_window_id {
        let environment = match detected_environment {
            EnvironmentKind::Wayland => EnvironmentKind::XWayland,
            environment => environment,
        };
        return BackendResolution {
            environment,
            capture_backend: CaptureBackend::X11Window { window_id },
        };
    }

    // Use the generic ScreenCast portal for native Wayland. Do not fall back to
    // monitor capture; the portal must provide a window PipeWire stream.
    let capture_backend = match detected_environment {
        EnvironmentKind::X11 => CaptureBackend::Unsupported {
            reason: "X11 was detected but no Warframe X11/XWayland window was found",
        },
        EnvironmentKind::XWayland => CaptureBackend::Unsupported {
            reason: "XWayland was detected but no Warframe X11/XWayland window was found",
        },
        EnvironmentKind::Wayland => {
            #[cfg(feature = "native-wayland-screenshot")]
            {
                CaptureBackend::WaylandScreenCastPortal
            }
            #[cfg(not(feature = "native-wayland-screenshot"))]
            {
                CaptureBackend::Unsupported {
                    reason: "Native Wayland screenshot capture requires building with the 'native-wayland-screenshot' feature, or run Warframe under XWayland/X11",
                }
            }
        }
        EnvironmentKind::Unknown => CaptureBackend::Unsupported {
            reason: "Unsupported Unix display environment; run Warframe under XWayland/X11 or use a Wayland session with xdg-desktop-portal ScreenCast window capture",
        },
    };

    BackendResolution {
        environment: detected_environment,
        capture_backend,
    }
}

impl BackendResolution {
    fn is_x11_window(&self) -> bool {
        matches!(self.capture_backend, CaptureBackend::X11Window { .. })
    }
}

fn cached_resolution(warframe_pid: u32, native_wayland_capture: bool) -> Option<BackendResolution> {
    BACKEND_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.as_ref().cloned())
        .filter(|entry| {
            entry.warframe_pid == warframe_pid
                && entry.native_wayland_capture == native_wayland_capture
        })
        .map(|entry| entry.resolution)
}

fn store_resolution(
    warframe_pid: u32,
    native_wayland_capture: bool,
    resolution: &BackendResolution,
) {
    if let Ok(mut cache) = BACKEND_CACHE.lock() {
        *cache = Some(BackendCacheEntry {
            warframe_pid,
            native_wayland_capture,
            resolution: resolution.clone(),
        });
    }
}

fn clear_cached_resolution() {
    if let Ok(mut cache) = BACKEND_CACHE.lock() {
        *cache = None;
    }
}

fn cached_warframe_pid() -> Option<(u32, &'static str)> {
    if let Some(pid) = WARFRAME_PID_CACHE.lock().ok().and_then(|cache| *cache) {
        if !process::is_warframe_pid(pid) {
            clear_cached_warframe_pid();
        } else {
            return Some((pid, "cached"));
        }
    }

    let pid = process::get_warframe_pid()?;
    if let Ok(mut cache) = WARFRAME_PID_CACHE.lock() {
        *cache = Some(pid);
    }
    Some((pid, "fresh"))
}

fn clear_cached_warframe_pid() {
    if let Ok(mut cache) = WARFRAME_PID_CACHE.lock() {
        *cache = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wayland_with_x11_window_uses_xwayland_x11_capture() {
        let resolution =
            resolve_backend_from_probe(EnvironmentKind::Wayland, Some("123".to_string()));

        assert_eq!(resolution.environment, EnvironmentKind::XWayland);
        assert_eq!(
            resolution.capture_backend,
            CaptureBackend::X11Window {
                window_id: "123".to_string()
            }
        );
    }

    #[cfg(feature = "native-wayland-screenshot")]
    #[test]
    fn wayland_without_x11_window_uses_portal_capture() {
        let resolution = resolve_backend_from_probe(EnvironmentKind::Wayland, None);

        assert_eq!(resolution.environment, EnvironmentKind::Wayland);
        assert_eq!(
            resolution.capture_backend,
            CaptureBackend::WaylandScreenCastPortal
        );
    }

    #[cfg(not(feature = "native-wayland-screenshot"))]
    #[test]
    fn wayland_without_x11_window_is_unsupported_without_native_wayland_feature() {
        let resolution = resolve_backend_from_probe(EnvironmentKind::Wayland, None);

        assert_eq!(resolution.environment, EnvironmentKind::Wayland);
        assert!(matches!(
            resolution.capture_backend,
            CaptureBackend::Unsupported { .. }
        ));
    }

    #[test]
    fn x11_with_x11_window_uses_x11_capture() {
        let resolution = resolve_backend_from_probe(EnvironmentKind::X11, Some("456".to_string()));

        assert_eq!(resolution.environment, EnvironmentKind::X11);
        assert_eq!(
            resolution.capture_backend,
            CaptureBackend::X11Window {
                window_id: "456".to_string()
            }
        );
    }

    #[test]
    fn x11_without_x11_window_is_unsupported() {
        let resolution = resolve_backend_from_probe(EnvironmentKind::X11, None);

        assert_eq!(resolution.environment, EnvironmentKind::X11);
        assert!(matches!(
            resolution.capture_backend,
            CaptureBackend::Unsupported { .. }
        ));
    }

    #[test]
    fn unknown_without_x11_window_is_unsupported() {
        let resolution = resolve_backend_from_probe(EnvironmentKind::Unknown, None);

        assert_eq!(resolution.environment, EnvironmentKind::Unknown);
        assert!(matches!(
            resolution.capture_backend,
            CaptureBackend::Unsupported { .. }
        ));
    }
}
