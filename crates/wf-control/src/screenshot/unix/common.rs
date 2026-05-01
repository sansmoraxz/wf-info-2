use std::env;

use anyhow::{Result, bail};

pub(super) const WARFRAME_TITLE_HINTS: &[&str] = &["Warframe"];
pub(super) const WARFRAME_CLASS_HINTS: &[&str] = &["steam_app_230410", "warframe"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EnvironmentKind {
    X11,
    XWayland,
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
