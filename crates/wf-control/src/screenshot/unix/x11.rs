use std::collections::BTreeSet;
use std::env;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::screenshot::unix::common::process_output_to_string;

use super::common::{
    WARFRAME_CLASS_HINTS, WARFRAME_TITLE_HINTS, ensure_command_available, ensure_png_bytes,
    ensure_success, run_command,
};

pub(super) fn find_window(pid: u32) -> Result<String> {
    if env::var_os("DISPLAY").is_none() {
        bail!("DISPLAY is not set");
    }
    ensure_command_available("xdotool")?;
    ensure_command_available("xwd")?;
    ensure_command_available("magick")?;

    let pid_matches = search_window_ids(["search", "--pid", &pid.to_string()])?;
    if let Some(window_id) = pid_matches.into_iter().next() {
        return Ok(window_id);
    }

    let mut heuristic_matches = BTreeSet::new();
    for title_hint in WARFRAME_TITLE_HINTS {
        for window_id in search_window_ids(["search", "--name", title_hint])? {
            heuristic_matches.insert(window_id);
        }
    }
    for class_hint in WARFRAME_CLASS_HINTS {
        for window_id in search_window_ids(["search", "--classname", class_hint])? {
            heuristic_matches.insert(window_id);
        }
        for window_id in search_window_ids(["search", "--class", class_hint])? {
            heuristic_matches.insert(window_id);
        }
    }
    if let Some(window_id) = heuristic_matches.into_iter().next() {
        return Ok(window_id);
    }

    bail!("No X11/XWayland window found for Warframe; PID and title/class lookup both failed");
}

fn search_window_ids<const N: usize>(args: [&str; N]) -> Result<Vec<String>> {
    let mut window_ids = BTreeSet::new();

    if let Ok(output) = run_command("xdotool", args) {
        let stdout = process_output_to_string(output, "xdotool search")?;
        for line in stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            window_ids.insert(line.to_string());
        }
    }

    Ok(window_ids.into_iter().collect())
}

pub(super) fn capture_window(window_id: &str) -> Result<Vec<u8>> {
    let mut xwd = Command::new("xwd")
        .args(["-silent", "-id", window_id])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to run xwd for X11 window {}", window_id))?;

    let xwd_stdout = xwd
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to capture xwd stdout"))?;
    let magick = Command::new("magick")
        .args(["xwd:-", "png:-"])
        .stdin(Stdio::from(xwd_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run magick to convert X11 xwd capture to PNG")?;

    let xwd_output = xwd
        .wait_with_output()
        .with_context(|| format!("Failed to wait for xwd on X11 window {}", window_id))?;

    let xwd_stderr = String::from_utf8_lossy(&xwd_output.stderr)
        .trim()
        .to_string();
    if !xwd_stderr.is_empty() {
        log::warn!("X11 xwd stderr: {}", xwd_stderr);
    }
    let magick_stderr = String::from_utf8_lossy(&magick.stderr).trim().to_string();
    if !magick_stderr.is_empty() {
        log::warn!("X11 magick stderr: {}", magick_stderr);
    }

    ensure_success(&xwd_output, "xwd X11 window capture")?;
    ensure_success(&magick, "magick X11 window capture conversion")?;

    if magick.stdout.is_empty() {
        bail!("X11 window capture returned no image data");
    }
    ensure_png_bytes(&magick.stdout, "X11 window capture")?;
    Ok(magick.stdout)
}
