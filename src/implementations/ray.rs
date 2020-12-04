use crate::implementations::{
    tuples::*,
    matrices::*
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4
}

impl Ray {
    pub fn position(&self, t: f32) -> Vec4 {
        &self.origin + &(&self.direction * t) 
    }

    pub fn transform(&self, m: &Matrix4x4) -> Ray {
        Ray {
            origin: m * &self.origin,
            direction: m * &self.direction
        }
    }
}