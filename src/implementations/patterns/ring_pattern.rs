use super::super::{tuples::*, pattern::*, matrices::Matrix4x4, shape::*};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RingPattern {
    pub a: Vec4,
    pub b: Vec4,
    pub transform: Matrix4x4
}

impl Pattern for RingPattern {
    fn color_at(&self, shape: &Shape, point: &Vec4) -> Vec4 {
        let point = transform_point_to_pattern_space(self, shape, point);
        if (point.0.powi(2) + point.2.powi(2)) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}