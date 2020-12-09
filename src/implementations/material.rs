use super::{tuples::*, pattern::*};

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec4,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub pattern: Option<Box<dyn Pattern>>
}

impl PartialEq<Material> for Material {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None
        }
    }

    pub fn set_pattern(&mut self, pattern: Box<dyn Pattern>) {
        self.pattern = Some(pattern);
    }
}