use crate::implementations::{
    tuples::*, 
    shape::*
};

pub struct Computations<'a> {
    pub t: f32,
    pub object: &'a Shape,
    pub point: Vec4,
    pub eyev: Vec4,
    pub normalv: Vec4,
    pub inside: bool,
    pub over_point: Vec4,
    pub reflectv: Vec4,
    pub n1: Option<f32>,
    pub n2: Option<f32>
}