use std::env;
use std::ffi::OsStr;
use std::process::{Command, Output, Stdio};

use anyhow::{Context, Result, bail};

pub(super) const WARFRAME_TITLE_HINTS: &[&str] = &["Warframe"];
pub(super) const WARFRAME_CLASS_HINTS: &[&str] = &["steam_app_230410", "warframe"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EnvironmentKind {
    X11,
    Wayland,
    Unknown,
}

pub(super) fn detect_unix_environment() -> EnvironmentKind {
    if env::var_os("WAYLAND_DISPLAY").is_some() {
        EnvironmentKind::Wayland
    } else if env::var_os("DISPLAY").is_some() {
        EnvironmentKind::X11
    } else {
        EnvironmentKind::Unknown
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

pub(crate) fn process_output_to_string(
    output: std::process::Output,
    context: &str,
) -> Result<String> {
    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .with_context(|| format!("{} returned invalid UTF-8", context))
}
