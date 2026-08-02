use std::env;
use std::num::NonZeroI32;

#[derive(Debug, thiserror::Error)]
pub(crate) enum BmpError {
    #[error("BMP width must be positive: {0}")]
    NegativeWidth(i32),
    #[error("BMP height must be positive: {0}")]
    NegativeHeight(i32),
    #[error("BMP dimensions overflow")]
    Overflow,
}

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

/// A top-down 24-bit BMP under construction. Owns the header layout (magic,
/// 54-byte pixel offset, 4-byte row stride) so callers only write pixels.
pub(super) struct BmpRgb24 {
    bytes: Vec<u8>,
    stride: usize,
    row_len: usize,
}

impl BmpRgb24 {
    const HEADER_LEN: usize = 54;

    /// BMP headers store signed dimensions, so this takes the header's native
    /// type (nonzero, since an empty image has no valid pixel array); height
    /// is negated in the header for top-down rows.
    pub(super) fn new(width: NonZeroI32, height: NonZeroI32) -> Result<Self, BmpError> {
        let width = width.get();
        let height = height.get();
        let width_u32 = u32::try_from(width).map_err(|_| BmpError::NegativeWidth(width))?;
        let height_u32 = u32::try_from(height).map_err(|_| BmpError::NegativeHeight(height))?;
        let row_len = width_u32.checked_mul(3).ok_or(BmpError::Overflow)?;
        let stride = row_len
            .checked_next_multiple_of(4)
            .ok_or(BmpError::Overflow)?;
        let pixel_bytes = stride.checked_mul(height_u32).ok_or(BmpError::Overflow)?;
        let file_size = 54u32.checked_add(pixel_bytes).ok_or(BmpError::Overflow)?;

        let mut bytes = vec![0; file_size as usize];
        bytes[0..2].copy_from_slice(b"BM");
        bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
        bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
        bytes[14..18].copy_from_slice(&40u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&width.to_le_bytes());
        bytes[22..26].copy_from_slice(&(-height).to_le_bytes());
        bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
        bytes[34..38].copy_from_slice(&pixel_bytes.to_le_bytes());

        Ok(Self {
            bytes,
            stride: stride as usize,
            row_len: row_len as usize,
        })
    }

    /// Copy one row of 4-byte-per-pixel BGRX/BGRA source data into row `y`,
    /// dropping the fourth channel.
    pub(super) fn copy_bgrx_row(&mut self, y: usize, source_row: &[u8]) {
        let start = Self::HEADER_LEN + y * self.stride;
        let row = &mut self.bytes[start..start + self.row_len];
        for (out, pixel) in row.chunks_exact_mut(3).zip(source_row.chunks_exact(4)) {
            out.copy_from_slice(&pixel[..3]);
        }
    }

    pub(super) fn set_pixel_bgr(&mut self, x: usize, y: usize, bgr: [u8; 3]) {
        let offset = Self::HEADER_LEN + y * self.stride + x * 3;
        self.bytes[offset..offset + 3].copy_from_slice(&bgr);
    }

    pub(super) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
