use ocr_rs::OcrEngine;

#[derive(Debug, thiserror::Error)]
pub enum OcrInitError {
    #[error("OCR model asset missing: {0}")]
    MissingAsset(&'static str),
    #[error("failed to initialize OCR engine: {0}")]
    Engine(#[from] ocr_rs::OcrError),
}

#[derive(rust_embed::Embed)]
#[folder = "models/"]
struct OcrModelAsset;

fn asset(name: &'static str) -> Result<rust_embed::EmbeddedFile, OcrInitError> {
    OcrModelAsset::get(name).ok_or(OcrInitError::MissingAsset(name))
}

pub fn new_default_ocr_engine() -> Result<OcrEngine, OcrInitError> {
    Ok(OcrEngine::from_bytes(
        &asset("ch_PP-OCRv4_det_infer.mnn")?.data,
        &asset("ch_PP-OCRv4_rec_infer.mnn")?.data,
        &asset("ppocr_keys_v4.txt")?.data,
        None,
    )?)
}

#[test]
fn engine_load_should_not_panic() {
    new_default_ocr_engine().unwrap();
}
