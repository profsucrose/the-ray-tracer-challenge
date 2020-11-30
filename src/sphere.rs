use crate::tuples::*;
use crate::matrices::*;
use crate::material::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vec4,
    pub transform: Matrix4x4,
    pub material: Material
}

impl Sphere {
    pub fn new() -> Sphere {
        // unit sphere with identity transform matrix
        Sphere {
            origin: point(0.0, 0.0, 0.0),
            transform: Matrix4x4::ident(),
            material: Material::new()
        }
    }

    pub fn normal_at(&self, point: &Vec4) -> Vec4 {
        // get point in object space first
        let inverted_transform = &self.transform.invert();
        let object_point = inverted_transform * point;
        let object_normal = (&object_point - &self.origin).normalize();
        // get world normal
        let mut world_normal = &inverted_transform.transpose() * &object_normal;
        // account for w coordinate changing with 
        // the transpose of a translation matrix
        world_normal.3 = 0.0;
        world_normal.normalize()
    }
}