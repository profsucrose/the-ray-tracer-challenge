use std::fmt::Debug;
use super::{matrices::Matrix4x4, shape::*, tuples::*};

pub trait PatternClone {
    fn clone_box(&self) -> Box<dyn Pattern>;
}

impl<T> PatternClone for T
where
    T: 'static + Pattern + Clone
{
    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }   
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Box<dyn Pattern> {
        self.clone_box()
    }
}

pub trait Pattern: Debug + PatternClone {
    fn color_at(&self, shape: &Shape, point: &Vec4) -> Vec4;
    fn get_transform(&self) -> Matrix4x4;
    fn eq(&self, other: &dyn Pattern) -> bool;
}

pub fn transform_point_to_pattern_space<T>(pattern: &T, shape: &Shape, point: &Vec4) -> Vec4 
    where T: Pattern
{
    let object_point = &shape.transform.invert() * point;
    let pattern_point = &pattern.get_transform().invert() * &object_point;
    pattern_point
}