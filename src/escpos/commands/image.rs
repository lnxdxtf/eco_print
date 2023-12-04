#![allow(non_snake_case)]
use image::{imageops, DynamicImage, GrayImage, ImageBuffer, ImageError, Luma};
use std::path::Path;

pub struct ESCPOSImage {
    pub data: Vec<u8>,
    pub max_width_escpos: u32,
    pub width: u32,
    pub height: u32,
}

impl ESCPOSImage {
    pub fn new<P: AsRef<Path>>(path: P, max_width_escpos: u32) -> Result<Self, ImageError> {
        let img = image::open(path)?.to_luma8();
        let resized_img = ESCPOSImage::resize_image(&img, max_width_escpos);
        let (width, height) = resized_img.dimensions();

        Ok(ESCPOSImage {
            data: resized_img.into_raw(),
            max_width_escpos,
            width,
            height,
        })
    }

    /// Resize image to fit in ESC/POS printer
    fn resize_image(img: &GrayImage, max_width_escpos: u32) -> GrayImage {
        if img.width() > max_width_escpos {
            let new_height = img.height() * max_width_escpos / img.width();
            imageops::resize(
                img,
                max_width_escpos,
                new_height,
                imageops::FilterType::Lanczos3,
            )
        } else {
            img.clone()
        }
    }

    // TODO: Fix error when printing image
    // 1. Return error when printing image 
    // (Error: Error { kind: Other, source: Some(Error { code: HRESULT(0x80070057), message: "The parameter is incorrect." }), message: "" })
    // Maybe the command is wrong, driver problem, or the image is not in the correct format
    pub fn to_escpos(&self) -> Vec<u8> {
        let threshold = 128u8;
        let mut bitmap_data = Vec::new();

        // convert image to bitmap
        for y in 0..self.height {
            for x in 0..self.width {
                if x % 8 == 0 {
                    bitmap_data.push(0);
                }
                let index = (y * self.width + x) as usize;
                let pixel = self.data[index];
                if pixel < threshold {
                    let byte_index = bitmap_data.len() - 1;
                    bitmap_data[byte_index] |= 1 << (7 - x % 8);
                }
            }
        }

        let width_bytes = (self.width + 7) / 8;
        let xL = (width_bytes & 0xFF) as u8;
        let xH = ((width_bytes >> 8) & 0xFF) as u8;
        let yL = (self.height & 0xFF) as u8;
        let yH = ((self.height >> 8) & 0xFF) as u8;

        // 4 First bytes are ESC/POS command to print image
        let mut escpos_command = vec![0x1D, 0x76, 0x30, 0x00]; // GS v 0 m
        escpos_command.extend_from_slice(&[xL, xH, yL, yH]);
        escpos_command.extend(bitmap_data);

        escpos_command
    }

    pub fn to_ascii_art(&self) -> String {
        let ascii_chars = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];
        let mut ascii_art = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let luminance = self.data[index] as f32 / 255.0;
                let char_index = (luminance * (ascii_chars.len() - 1) as f32) as usize;
                ascii_art.push_str(ascii_chars[char_index]);
            }
            ascii_art.push('\n');
        }

        ascii_art
    }
}
