use std::env;
use std::ffi::OsStr;
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use rand::random;

pub(super) const WARFRAME_TITLE_HINTS: &[&str] = &["Warframe"];
pub(super) const WARFRAME_CLASS_HINTS: &[&str] = &["steam_app_230410", "warframe"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EnvironmentKind {
    X11,
    KdeWayland,
    Hyprland,
    Niri,
    GnomeWayland,
    OtherWayland,
    Unknown,
}

pub(super) fn detect_wayland_environment() -> EnvironmentKind {
    let wayland = env::var_os("WAYLAND_DISPLAY").is_some();
    if !wayland {
        // not always guaranteed to be X11 session
        return EnvironmentKind::Unknown;
    }

    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let niri_detected = env::var_os("NIRI_SOCKET").is_some()
        || desktop.contains("niri")
        || command_succeeds("niri", ["msg", "version"]);

    if desktop.contains("kde") || desktop.contains("plasma") {
        EnvironmentKind::KdeWayland
    } else if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some() || desktop.contains("hyprland") {
        EnvironmentKind::Hyprland
    } else if niri_detected {
        EnvironmentKind::Niri
    } else if desktop.contains("gnome") {
        EnvironmentKind::GnomeWayland
    } else {
        EnvironmentKind::OtherWayland
    }
}

pub(super) fn ensure_command_available(command: &str) -> Result<()> {
    if command_exists(command) {
        Ok(())
    } else {
        bail!("Required command '{}' is not installed", command);
    }
}

fn command_exists(command: &str) -> bool {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| path.join(command).exists()))
        .unwrap_or(false)
}

fn command_succeeds<I, S>(program: &str, args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    if !command_exists(program) {
        return false;
    }

    Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub(super) fn run_command<I, S>(program: &str, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to run {}", program))?;
    ensure_success(&output, program)?;
    Ok(output)
}

pub(super) fn ensure_success(output: &Output, context: &str) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr = stderr.trim();
    if stderr.is_empty() {
        bail!("{} failed with status {}", context, output.status);
    }
    bail!("{} failed: {}", context, stderr);
}

pub(super) fn temp_path(prefix: &str, extension: &str) -> std::path::PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let name = format!(
        "{}-{}-{}-{}.{}",
        prefix,
        std::process::id(),
        unique_suffix,
        random::<u32>(),
        extension
    );
    env::temp_dir().join(name)
}

pub(super) fn ensure_png_bytes(bytes: &[u8], context: &str) -> Result<()> {
    const PNG_MAGIC: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if bytes.starts_with(PNG_MAGIC) {
        Ok(())
    } else {
        let preview = bytes
            .iter()
            .take(16)
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        bail!("{context} did not return a valid PNG image; first bytes: [{preview}]")
    }
}

pub(super) fn window_matches_hint(title: &str, class_name: &str) -> bool {
    let title = title.to_ascii_lowercase();
    let class_name = class_name.to_ascii_lowercase();
    WARFRAME_CLASS_HINTS
        .iter()
        .any(|hint| class_name.contains(hint))
        || WARFRAME_TITLE_HINTS.iter().any(|hint| title.contains(hint))
}

pub(crate) fn process_output_to_string(
    output: std::process::Output,
    context: &str,
) -> Result<String> {
    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .with_context(|| format!("{} returned invalid UTF-8", context))
}
