use anyhow::{Context, Result, anyhow};

/// Capture the entire screen and return the screenshot content as png.
/// TODO: swap to window capture and test on windows
pub(crate) async fn capture_screen() -> Result<(Vec<u8>, String)> {
    use image::{ImageBuffer, Rgb};
    use win_screenshot::prelude::*;

    let buf = capture_display().map_err(|e| anyhow!("Failed to capture display: {:?}", e))?;

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width as u32, buf.height as u32, buf.pixels)
            .context("Failed to create image buffer")?;

    let mut png_bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )?;

    Ok((png_bytes, "image/png".to_string()))
}
