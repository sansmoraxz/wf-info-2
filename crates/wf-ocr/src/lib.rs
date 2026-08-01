use image::{DynamicImage, ImageReader};
use ocr_rs::OcrEngine;
use std::{
    cmp::{max, min},
    io::Cursor,
};

mod ocr;

pub use ocr::{DEFAULT_OCR_ENGINE, new_default_ocr_engine};

pub fn load_image(bytes: &[u8]) -> anyhow::Result<image::DynamicImage> {
    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?)
}

pub struct RelicRecognizer {
    ocr_engine: &'static OcrEngine,
    pub start_x: u32,
    pub start_y: u32,
    pub box_w: u32,
    pub box_h: u32,
}

#[derive(Debug)]
pub struct RelicRecogizeText {
    pub x: u32,
    pub y: u32,
    pub text: String,
}

impl RelicRecognizer {
    pub fn new(ocr_engine: &'static OcrEngine) -> RelicRecognizer {
        // TODO: handle non default scales + widescreen etc.
        RelicRecognizer {
            ocr_engine,
            start_x: 477,
            start_y: 373,
            box_w: 1435 - 477,
            box_h: 91,
        }
    }

    pub fn recognize_and_list(
        &self,
        img: &DynamicImage,
    ) -> anyhow::Result<Vec<RelicRecogizeText>, anyhow::Error> {
        // Crop first, then resize. Map the reward-box rect
        // (defined in 1920x1080 space) back to source coordinates.
        let scale = f64::min(
            1920.0 / f64::from(img.width()),
            1080.0 / f64::from(img.height()),
        );
        let src_x = (f64::from(self.start_x) / scale).floor() as u32;
        let src_y = (f64::from(self.start_y) / scale).floor() as u32;
        let src_w = (f64::from(self.box_w) / scale).ceil() as u32;
        let src_h = (f64::from(self.box_h) / scale).ceil() as u32;
        let scaled_and_cropped_img = img
            .crop_imm(
                src_x,
                src_y,
                src_w.min(img.width().saturating_sub(src_x)),
                src_h.min(img.height().saturating_sub(src_y)),
            )
            .resize_exact(
                self.box_w,
                self.box_h,
                image::imageops::FilterType::Lanczos3,
            );
        let res = self.ocr_engine.recognize(&scaled_and_cropped_img)?;

        // Group horizontally overlapping texts, merging into the first
        // overlapping group's entry as we go. n is a handful of OCR hits,
        // so the linear group scan is fine.
        let mut bounds: Vec<(i32, i32)> = Vec::new();
        let mut merged: Vec<RelicRecogizeText> = Vec::new();
        'outer: for a in res {
            let (la, ra) = (a.bbox.rect.left(), a.bbox.rect.right());
            for ((lb, rb), group) in bounds.iter().zip(merged.iter_mut()) {
                if max(la, *lb) < min(ra, *rb) {
                    group.text.push(' ');
                    group.text.push_str(&a.text);
                    continue 'outer;
                }
            }
            bounds.push((la, ra));
            merged.push(RelicRecogizeText {
                text: a.text,
                x: la as u32,
                y: a.bbox.rect.top() as u32,
            });
        }

        for group in &mut merged {
            group.text.truncate(group.text.trim_end().len());
            let leading = group.text.len() - group.text.trim_start().len();
            group.text.drain(..leading);
        }
        merged.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        Ok(merged)
    }
}
