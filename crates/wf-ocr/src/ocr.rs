use std::sync::LazyLock;

use ocr_rs::OcrEngine;

#[derive(rust_embed::Embed)]
#[folder = "models/"]
struct OcrModelAsset;

pub fn new_default_ocr_engine() -> OcrEngine {
    OcrEngine::from_bytes(
        &OcrModelAsset::get("ch_PP-OCRv4_det_infer.mnn")
            .unwrap()
            .data,
        &OcrModelAsset::get("ch_PP-OCRv4_rec_infer.mnn")
            .unwrap()
            .data,
        &OcrModelAsset::get("ppocr_keys_v4.txt").unwrap().data,
        None,
    )
    .unwrap()
}

pub static DEFAULT_OCR_ENGINE: LazyLock<OcrEngine> = LazyLock::new(|| new_default_ocr_engine());

#[test]
fn engine_load_should_not_panic() {
    new_default_ocr_engine();
}
