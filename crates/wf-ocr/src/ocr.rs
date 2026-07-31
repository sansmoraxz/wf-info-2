use std::sync::LazyLock;

use ocr_rs::OcrEngine;

#[derive(rust_embed::Embed)]
#[folder = "models/"]
struct OcrModelAsset;

fn asset(name: &str) -> anyhow::Result<rust_embed::EmbeddedFile> {
    OcrModelAsset::get(name).ok_or_else(|| anyhow::anyhow!("OCR model asset missing: {name}"))
}

pub fn new_default_ocr_engine() -> anyhow::Result<OcrEngine> {
    OcrEngine::from_bytes(
        &asset("ch_PP-OCRv4_det_infer.mnn")?.data,
        &asset("ch_PP-OCRv4_rec_infer.mnn")?.data,
        &asset("ppocr_keys_v4.txt")?.data,
        None,
    )
    .map_err(|e| anyhow::anyhow!("failed to initialize OCR engine: {e}"))
}

pub static DEFAULT_OCR_ENGINE: LazyLock<anyhow::Result<OcrEngine>> =
    LazyLock::new(new_default_ocr_engine);

#[test]
fn engine_load_should_not_panic() {
    new_default_ocr_engine().unwrap();
}
