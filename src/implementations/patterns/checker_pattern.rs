use super::super::{tuples::*, pattern::*, matrices::Matrix4x4, shape::*};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CheckerPattern {
    pub a: Vec4,
    pub b: Vec4,
    pub transform: Matrix4x4
}

impl Pattern for CheckerPattern {
    fn color_at(&self, shape: &Shape, point: &Vec4) -> Vec4 {
        let point = transform_point_to_pattern_space(self, shape, point);
        if ((point.0.abs() + point.1.abs() + point.2.abs()) as u32) % 2 == 0 {
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