use std::env;

use anyhow::{Result, anyhow, bail};

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

pub(super) fn ensure_bmp_bytes(bytes: &[u8], context: &str) -> Result<()> {
    const BMP_MAGIC: &[u8; 2] = b"BM";
    if bytes.starts_with(BMP_MAGIC) {
        Ok(())
    } else {
        let preview = bytes
            .iter()
            .take(16)
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        bail!("{context} did not return a valid BMP image; first bytes: [{preview}]")
    }
}

pub(super) fn new_bmp_rgb24(width: u32, height: u32) -> Result<(Vec<u8>, usize)> {
    let row_len = width
        .checked_mul(3)
        .ok_or_else(|| anyhow!("BMP row dimensions overflow"))?;
    let stride = row_len
        .checked_next_multiple_of(4)
        .ok_or_else(|| anyhow!("BMP row stride overflow"))?;
    let pixel_bytes = stride
        .checked_mul(height)
        .ok_or_else(|| anyhow!("BMP pixel data size overflow"))?;
    let file_size = 54u32
        .checked_add(pixel_bytes)
        .ok_or_else(|| anyhow!("BMP file size overflow"))?;

    let mut bytes = vec![0; file_size as usize];
    bytes[0..2].copy_from_slice(b"BM");
    bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
    bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
    bytes[14..18].copy_from_slice(&40u32.to_le_bytes());
    bytes[18..22].copy_from_slice(&(width as i32).to_le_bytes());
    bytes[22..26].copy_from_slice(&(-(height as i32)).to_le_bytes());
    bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
    bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
    bytes[34..38].copy_from_slice(&pixel_bytes.to_le_bytes());

    Ok((bytes, stride as usize))
}
