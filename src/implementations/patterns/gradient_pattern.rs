use super::super::{tuples::*, pattern::*, matrices::Matrix4x4, shape::*};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GradientPattern {
    pub a: Vec4,
    pub b: Vec4,
    pub transform: Matrix4x4
}

impl Pattern for GradientPattern {
    fn color_at(&self, shape: &Shape, point: &Vec4) -> Vec4 {
        let point = transform_point_to_pattern_space(self, shape, point);
        let distance = &self.b - &self.a;
        let fraction = point.0 - point.0.floor();
        &self.a + &(&distance * fraction)
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}