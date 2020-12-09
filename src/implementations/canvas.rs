use crate::implementations::tuples::*;
use std::fs;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Vec4>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels: Vec<Vec<Vec4>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row: Vec<Vec4> = Vec::with_capacity(width);
            for _ in 0..width {
                let color = color(0.0, 0.0, 0.0);
                row.push(color);
            } 
            pixels.push(row);
        }
        Canvas { width, height, pixels }
    }

    pub fn get(&self, x: usize, y: usize) -> &Vec4 {
        let row = self.pixels.get(y).unwrap_or_else(|| {
            panic!("Unable to get Y: {} from canvas", y);
        });
        row.get(x).unwrap_or_else(|| {
            panic!("Unable to get X: {} from canvas", x);
        })
    }

    pub fn set(&mut self, x: usize, y: usize, color: Vec4) {
        if self.height < y {
            panic!("Y cannot be negative or greater than height when indexing canvas");
        } else if self.width < x {
            panic!("X cannot be negative or greater than width when indexing canvas");
        }

        let row = &mut self.pixels[y];
        row[x] = color;
    }

    pub fn write_to_ppm(&self, filepath: &str) {
        let mut ppm = String::from(format!("P3\n{} {}\n255\n", self.width, self.height));
        for row in self.pixels.iter() {
            let mut row_string = String::new();
            for color in row.iter() {
                let color = color.clamp(255.0);
                let mut color_string = String::new();
                color_string.push_str(&color.0.round().to_string());
                color_string.push(' ');
                color_string.push_str(&color.1.round().to_string());
                color_string.push(' ');
                color_string.push_str(&color.1.round().to_string());
                color_string.push(' ');
                row_string.push_str(&color_string);
            }
            ppm.push_str(&row_string);
            ppm.push('\n');
        }

        fs::write(format!("output/{}", filepath), ppm)
            .expect("Unable to write canvas to file");
    }
}