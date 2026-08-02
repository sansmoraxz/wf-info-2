use image::{DynamicImage, ImageReader};
use ocr_rs::{OcrEngine, engine::OcrResult_};
use std::io::Cursor;

mod ocr;

pub use ocr::{OcrInitError, new_default_ocr_engine};

pub fn load_image(bytes: &[u8]) -> Result<image::DynamicImage, image::ImageError> {
    ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()
}

pub struct RelicRecognizer {
    ocr_engine: OcrEngine,
    pub start_x: u32,
    pub start_y: u32,
    pub box_w: u32,
    pub box_h: u32,
}

impl From<OcrEngine> for RelicRecognizer {
    fn from(ocr_engine: OcrEngine) -> Self {
        // TODO: handle non default scales + widescreen etc.
        Self {
            ocr_engine,
            start_x: 477,
            start_y: 373,
            box_w: 1435 - 477,
            box_h: 91,
        }
    }
}

#[derive(Debug)]
pub struct RelicRecogizeText {
    pub x: u32,
    pub y: u32,
    pub text: String,
}

/// Saturating f64 -> u32: inputs are u32/positive-scale quotients, so the
/// clamp only guards degenerate (tiny-image) scales.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn to_px(v: f64) -> u32 {
    v.clamp(0.0, f64::from(u32::MAX)) as u32
}

impl RelicRecognizer {
    pub fn recognize_and_list(
        &self,
        img: &DynamicImage,
    ) -> Result<Vec<RelicRecogizeText>, ocr_rs::OcrError> {
        let reward_box = self.crop_reward_box(img);
        let hits = self.ocr_engine.recognize(&reward_box)?;

        let mut merged = merge_overlapping_columns(hits);
        for group in &mut merged {
            trim_in_place(&mut group.text);
        }
        merged.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        Ok(merged)
    }

    /// Crop the reward-box rect (defined in 1920x1080 space) out of the source
    /// image, then resize it to the rect's reference size. Crop first so the
    /// expensive resample only touches the region of interest.
    fn crop_reward_box(&self, img: &DynamicImage) -> DynamicImage {
        let scale = f64::min(
            1920.0 / f64::from(img.width()),
            1080.0 / f64::from(img.height()),
        );
        let src_x = to_px((f64::from(self.start_x) / scale).floor());
        let src_y = to_px((f64::from(self.start_y) / scale).floor());
        let src_w = to_px((f64::from(self.box_w) / scale).ceil());
        let src_h = to_px((f64::from(self.box_h) / scale).ceil());
        img.crop_imm(
            src_x,
            src_y,
            src_w.min(img.width().saturating_sub(src_x)),
            src_h.min(img.height().saturating_sub(src_y)),
        )
        .resize_exact(
            self.box_w,
            self.box_h,
            image::imageops::FilterType::Lanczos3,
        )
    }
}

/// Group horizontally overlapping OCR hits into one entry per column,
/// concatenating text into the first overlapping group as we go. n is a
/// handful of OCR hits, so the linear group scan is fine.
fn merge_overlapping_columns(hits: Vec<OcrResult_>) -> Vec<RelicRecogizeText> {
    let mut bounds: Vec<(i32, i32)> = Vec::new();
    let mut merged: Vec<RelicRecogizeText> = Vec::new();
    for hit in hits {
        let (left, right) = (hit.bbox.rect.left(), hit.bbox.rect.right());
        let overlapping = bounds
            .iter()
            .position(|(l, r)| left.max(*l) < right.min(*r));
        match overlapping {
            Some(i) => {
                merged[i].text.push(' ');
                merged[i].text.push_str(&hit.text);
            }
            None => {
                bounds.push((left, right));
                merged.push(RelicRecogizeText {
                    text: hit.text,
                    x: u32::try_from(left).unwrap_or(0),
                    y: u32::try_from(hit.bbox.rect.top()).unwrap_or(0),
                });
            }
        }
    }
    merged
}

fn trim_in_place(s: &mut String) {
    s.truncate(s.trim_end().len());
    s.drain(..s.len() - s.trim_start().len());
}
