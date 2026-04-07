use itertools::Itertools;

use image::{DynamicImage, ImageReader};
use ocr_rs::{OcrEngine, OcrResult_};
use std::{
    cmp::{max, min},
    collections::HashMap,
    io::Cursor,
};

mod ocr;

pub use ocr::{new_default_ocr_engine, DEFAULT_OCR_ENGINE};

pub fn load_png_image(bytes: Vec<u8>) -> anyhow::Result<image::DynamicImage> {
    let mut reader = ImageReader::new(Cursor::new(bytes));
    reader.set_format(image::ImageFormat::Png);
    Ok(reader.decode()?)
}

pub struct RelicRecognizer<'a> {
    ocr_engine: &'a OcrEngine,
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

impl<'a> RelicRecognizer<'a> {
    pub fn new(ocr_engine: &'a OcrEngine) -> RelicRecognizer<'a> {
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
        let scaled_and_cropped_img = img
            .resize(1920, 1080, image::imageops::FilterType::Lanczos3)
            .crop(self.start_x, self.start_y, self.box_w, self.box_h);
        let res = self.ocr_engine.recognize(&scaled_and_cropped_img)?;

        // group overlapping horizontal texts
        let mut rg: HashMap<(i32, i32), Vec<&OcrResult_>> = HashMap::new();
        'outer: for a in &res {
            let la = a.bbox.rect.left();
            let ra = a.bbox.rect.right();
            for (bd, v) in rg.iter_mut() {
                let (lb, rb) = (bd.0, bd.1);
                if max(la, lb) < min(ra, rb) {
                    v.push(a);
                    continue 'outer;
                }
            }
            rg.insert((la, ra), vec![a]);
        }

        // merge text and sort by cordinates
        let res = rg
            .iter()
            .map(|(_, v)| {
                let text = v.iter().map(|ores| &ores.text).join(" ").trim().to_string();
                let rec = v.get(0).unwrap();
                let x = rec.bbox.rect.left() as u32;
                let y = rec.bbox.rect.top() as u32;
                RelicRecogizeText { text, x, y }
            })
            .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
            .collect();

        Ok(res)
    }
}
