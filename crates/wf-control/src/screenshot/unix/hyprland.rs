use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;

use super::common::{
    ensure_command_available, ensure_png_bytes, ensure_success, run_command, window_matches_hint,
};

pub(super) fn capture_window(pid: u32) -> Result<Vec<u8>> {
    ensure_command_available("hyprctl")?;
    ensure_command_available("grim")?;

    let output = run_command("hyprctl", ["clients", "-j"])?;
    let clients: Vec<HyprlandClient> = serde_json::from_slice(&output.stdout)
        .context("Failed to parse hyprctl clients output as JSON")?;

    let client = clients
        .iter()
        .find(|client| client_matches(client, pid))
        .ok_or_else(|| {
            anyhow!(
                "Could not find a Warframe window in Hyprland; it may not exist yet or the compositor may not be exposing it"
            )
        })?;

    if client.hidden.unwrap_or(false) {
        bail!(
            "Warframe window is hidden in Hyprland; it is likely not renderable on the current workspace, so window-only capture is unavailable"
        );
    }

    let geometry = client_geometry(client)?;
    let output = Command::new("grim")
        .args(["-g", &geometry, "-"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run grim for Hyprland window capture")?;

    ensure_success(&output, "grim Hyprland window capture")?;
    if output.stdout.is_empty() {
        bail!("grim returned no image data for the Hyprland capture");
    }
    ensure_png_bytes(&output.stdout, "grim Hyprland capture")?;
    Ok(output.stdout)
}

#[derive(Debug, Deserialize)]
struct HyprlandClient {
    pid: Option<u64>,
    class: Option<String>,
    title: Option<String>,
    hidden: Option<bool>,
    at: Option<[i64; 2]>,
    size: Option<[i64; 2]>,
}

fn client_matches(client: &HyprlandClient, pid: u32) -> bool {
    if client.pid == Some(pid as u64) {
        return true;
    }

    let class = client.class.as_deref().unwrap_or_default();
    let title = client.title.as_deref().unwrap_or_default();

    window_matches_hint(title, class)
}

fn client_geometry(client: &HyprlandClient) -> Result<String> {
    let [x, y] = client
        .at
        .ok_or_else(|| anyhow!("Hyprland client missing position"))?;
    let [width, height] = client
        .size
        .ok_or_else(|| anyhow!("Hyprland client missing size"))?;

    if width <= 0 || height <= 0 {
        bail!("Hyprland window geometry is invalid: {}x{}", width, height);
    }

    Ok(format!("{},{} {}x{}", x, y, width, height))
}
