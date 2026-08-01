use anyhow::{Context, Result, anyhow, bail};
use wf_core::process;

use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;
use winapi::um::winuser::GetWindowThreadProcessId;

use crate::state::ScreenshotState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WindowCacheEntry {
    warframe_pid: u32,
    hwnd: isize,
    window_name: String,
}

/// Capture the Warframe window and return the screenshot content as BMP.
pub(crate) async fn capture_screen(state: &ScreenshotState) -> Result<(Vec<u8>, String)> {
    use image::ExtendedColorType;
    use image::codecs::bmp::BmpEncoder;
    use image::{DynamicImage, RgbaImage};
    use win_screenshot::prelude::*;

    let warframe_pid =
        cached_warframe_pid(state).ok_or_else(|| anyhow!("Warframe process not detected"))?;
    let cached = cached_window(state, warframe_pid);
    let window = match cached.clone() {
        Some(window) => window,
        None => resolve_warframe_window(state, warframe_pid)?,
    };

    let buf = match capture_window(window.hwnd) {
        Ok(buf) => buf,
        Err(err) if cached.is_some() => {
            log::warn!(
                "Cached Windows screenshot target {} ({}) failed; re-resolving: {:?}",
                window.hwnd,
                window.window_name,
                err
            );
            clear_cached_window(state);
            clear_cached_warframe_pid(state);
            let warframe_pid =
                cached_warframe_pid(state).ok_or_else(|| anyhow!("Warframe process not detected"))?;
            let refreshed = resolve_warframe_window(state, warframe_pid)?;
            match capture_window(refreshed.hwnd) {
                Ok(buf) => buf,
                Err(retry_err) => {
                    return Err(anyhow!(
                        "Failed to capture Warframe window {} ({}): {:?}",
                        refreshed.hwnd,
                        refreshed.window_name,
                        retry_err
                    ));
                }
            }
        }
        Err(err) => {
            return Err(anyhow!(
                "Failed to capture Warframe window {} ({}): {:?}",
                window.hwnd,
                window.window_name,
                err
            ));
        }
    };

    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels)
        .context("Failed to create RGBA image buffer from Windows screenshot")?;
    let rgb = DynamicImage::ImageRgba8(img).to_rgb8();

    let mut bmp_bytes = Vec::new();
    BmpEncoder::new(&mut bmp_bytes).encode(
        rgb.as_raw(),
        buf.width,
        buf.height,
        ExtendedColorType::Rgb8,
    )?;

    Ok((bmp_bytes, "image/bmp".to_string()))
}

fn cached_warframe_pid(state: &ScreenshotState) -> Option<u32> {
    if let Some(pid) = state.warframe_pid.lock().ok().and_then(|cache| *cache) {
        if process::is_warframe_pid(pid) {
            return Some(pid);
        }
        clear_cached_warframe_pid(state);
    }

    let pid = process::get_warframe_pid()?;
    if let Ok(mut cache) = state.warframe_pid.lock() {
        *cache = Some(pid);
    }
    Some(pid)
}

fn clear_cached_warframe_pid(state: &ScreenshotState) {
    if let Ok(mut cache) = state.warframe_pid.lock() {
        *cache = None;
    }
}

fn cached_window(state: &ScreenshotState, warframe_pid: u32) -> Option<WindowCacheEntry> {
    state
        .window_cache
        .lock()
        .ok()
        .and_then(|cache| cache.as_ref().cloned())
        .filter(|entry| {
            entry.warframe_pid == warframe_pid && window_process_id(entry.hwnd) == warframe_pid
        })
}

fn clear_cached_window(state: &ScreenshotState) {
    if let Ok(mut cache) = state.window_cache.lock() {
        *cache = None;
    }
}

fn resolve_warframe_window(
    state: &ScreenshotState,
    warframe_pid: u32,
) -> Result<WindowCacheEntry> {
    let candidates: Vec<_> = win_screenshot::utils::window_list()
        .map_err(|e| anyhow!("Failed to enumerate windows: {:?}", e))?
        .into_iter()
        .filter(|window| window_process_id(window.hwnd) == warframe_pid)
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        bail!("Visible Warframe window not found for PID {}", warframe_pid);
    }

    let selected = candidates
        .iter()
        .find(|window| window.window_name.to_ascii_lowercase().contains("warframe"))
        .unwrap_or(&candidates[0]);

    log::info!(
        "Capturing Warframe window {} for PID {} ({})",
        selected.hwnd,
        warframe_pid,
        selected.window_name
    );

    let entry = WindowCacheEntry {
        warframe_pid,
        hwnd: selected.hwnd,
        window_name: selected.window_name.clone(),
    };

    if let Ok(mut cache) = state.window_cache.lock() {
        *cache = Some(entry.clone());
    }

    Ok(entry)
}

fn window_process_id(hwnd: isize) -> u32 {
    let mut pid: DWORD = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd as HWND, &mut pid);
    }
    pid
}
