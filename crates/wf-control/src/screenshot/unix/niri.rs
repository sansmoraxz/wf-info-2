use std::fs;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;

use super::common::{
    ensure_command_available, ensure_png_bytes, ensure_success, run_command, temp_path,
    window_matches_hint,
};

pub(super) fn capture_window(pid: u32) -> Result<Vec<u8>> {
    ensure_command_available("niri")?;

    let window_id = find_window_id(pid)?;
    let output_path = temp_path("wf-info-niri", "png");
    let output_path_str = output_path
        .to_str()
        .ok_or_else(|| anyhow!("Temporary screenshot path is not valid UTF-8"))?;

    let result = Command::new("niri")
        .args([
            "msg",
            "action",
            "screenshot-window",
            "--id",
            &window_id,
            "--write-to-disk",
            "true",
            "--path",
            output_path_str,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run niri screenshot-window action")?;

    let stderr = String::from_utf8_lossy(&result.stderr).trim().to_string();
    if !stderr.is_empty() {
        log::warn!("niri stderr: {}", stderr);
    }
    ensure_success(&result, "niri screenshot-window action")?;

    let bytes = fs::read(&output_path).with_context(|| {
        format!(
            "niri did not produce a screenshot file at {}",
            output_path.display()
        )
    })?;
    let _ = fs::remove_file(&output_path);

    if bytes.is_empty() {
        bail!("niri returned an empty screenshot");
    }
    ensure_png_bytes(&bytes, "niri screenshot-window capture")?;

    Ok(bytes)
}

fn find_window_id(pid: u32) -> Result<String> {
    let output = run_command("niri", ["msg", "-j", "windows"])?;
    let windows: Vec<NiriWindow> = serde_json::from_slice(&output.stdout)
        .context("Failed to parse niri windows output as JSON")?;

    let window = windows
        .iter()
        .find(|window| window_matches(window, pid))
        .ok_or_else(|| {
            anyhow!(
                "Could not find a Warframe window in niri; it may not exist yet or the compositor may not be exposing it"
            )
        })?;

    extract_window_id(window)
}

#[derive(Debug, Deserialize)]
struct NiriWindow {
    id: NiriWindowId,
    #[serde(default, alias = "window_pid")]
    pid: Option<u64>,
    #[serde(default, alias = "name")]
    title: Option<String>,
    #[serde(default, alias = "app-id", alias = "class")]
    app_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NiriWindowId {
    String(String),
    Number(u64),
}

fn window_matches(window: &NiriWindow, pid: u32) -> bool {
    if window.pid == Some(pid as u64) {
        return true;
    }

    let title = window.title.as_deref().unwrap_or_default();
    let class_name = window.app_id.as_deref().unwrap_or_default();
    window_matches_hint(title, class_name)
}

fn extract_window_id(window: &NiriWindow) -> Result<String> {
    match &window.id {
        NiriWindowId::String(id) => Ok(id.clone()),
        NiriWindowId::Number(id) => Ok(id.to_string()),
    }
}
