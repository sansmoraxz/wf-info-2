use anyhow::{Context, Result, anyhow};

/// Capture the entire screen and return the screenshot content as BMP.
/// TODO: swap to window capture and test on windows
pub(crate) async fn capture_screen() -> Result<(Vec<u8>, String)> {
    use image::ExtendedColorType;
    use image::codecs::bmp::BmpEncoder;
    use image::{ImageBuffer, Rgb};
    use win_screenshot::prelude::*;

    let buf = capture_display().map_err(|e| anyhow!("Failed to capture display: {:?}", e))?;

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width as u32, buf.height as u32, buf.pixels)
            .context("Failed to create image buffer")?;

    let mut bmp_bytes = Vec::new();
    BmpEncoder::new(&mut bmp_bytes).encode(
        img.as_raw(),
        buf.width as u32,
        buf.height as u32,
        ExtendedColorType::Rgb8,
    )?;

    Ok((bmp_bytes, "image/bmp".to_string()))
}
