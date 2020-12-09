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
    pub under_point: Vec4,
    pub reflectv: Vec4,
    pub n1: Option<f32>,
    pub n2: Option<f32>
}

impl<'a> Computations<'a> {
    pub fn schlick(&self) -> f32 {
        let mut cos = self.eyev.dot(&self.normalv);

        let n1 = self.n1.unwrap();
        let n2 = self.n2.unwrap();
        if n1 > n2 {
            let n = n1 / n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}