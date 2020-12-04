use super::super::{tuples::*, pattern::*, matrices::Matrix4x4, shape::*};
use std::fmt::Debug;
use image::{RgbImage};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct TexturePattern {
    pub image_pixels: RgbImage,
    pub transform: Matrix4x4
}

impl TexturePattern {
    pub fn new(image_path: &str) -> TexturePattern {
        let image = image::open(&Path::new(image_path));
        let image = image.unwrap_or_else(|_| {
            panic!("Image at {} could not be found!", image_path);
        });
        TexturePattern {
            image_pixels: image.into_rgb8(),
            transform: Matrix4x4::ident()
        }
    }
}

impl Pattern for TexturePattern {
    fn color_at(&self, _shape: &Shape, point: &Vec4) -> Vec4 {
        let pixel = self.image_pixels
            .get_pixel(point.0.abs() as u32, point.1.abs() as u32);
        color(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0)
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}