use super::super::{tuples::*, pattern::*, matrices::Matrix4x4, shape::*};
use std::fmt::Debug;
use image::{RgbImage};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct TexturePattern {
    pub width: u32,
    pub height: u32,
    pub image_pixels: RgbImage,
    pub transform: Matrix4x4,
    pub image_scale_x: f32,
    pub image_scale_y: f32,
    pub z_oriented: bool,
    pub flipped: bool,
    pub offset_x: f32
}

impl TexturePattern {
    pub fn new(image_path: &str, image_scale_x: f32, image_scale_y: f32, offset_x: f32, z_oriented: bool, flipped: bool) -> TexturePattern {
        let image = image::open(&Path::new(image_path));
        let image = image.unwrap_or_else(|_| {
            panic!("Image at {} could not be found!", image_path);
        });
        let rgb_image = image.into_rgb8();
        TexturePattern {
            width: rgb_image.width(),
            height: rgb_image.height(),
            image_pixels: rgb_image,
            transform: Matrix4x4::ident(),
            image_scale_x,
            image_scale_y,
            z_oriented,
            flipped,
            offset_x
        }
    }
}

impl Pattern for TexturePattern {
    fn color_at(&self, _shape: &Shape, point: &Vec4) -> Vec4 {
        let mut first_coord = ((point.0.abs() + self.offset_x) * self.image_scale_x) as u32 % self.width;
        let mut second_coord = match self.z_oriented {
            true => (point.2.abs() * self.image_scale_y) as u32 % self.height,
            false => (point.1.abs() * self.image_scale_y) as u32 % self.height
        };
        if self.flipped {
            second_coord = self.height - 1 - second_coord;
            first_coord = self.width - 1 - first_coord;
        }
        let pixel = self.image_pixels
            .get_pixel(first_coord, second_coord);
        color(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0)
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}