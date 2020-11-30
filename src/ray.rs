use crate::tuples::*;
use crate::matrices::*;
use crate::intersection::*;
use crate::sphere::*;

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

    pub fn intersects(&self, sphere: &Sphere) -> Vec<Intersection> {
        // calculate ray intersection
        // solutions via quadratic formula
        // get discriminant to get number of solutions 
        // to return
        let ray = self.transform(&sphere.transform.invert());
        let sphere_to_ray = &ray.origin - &sphere.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = &ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) as f32 - 4.0 * a * c;
        if fequals(discriminant, 0.0) {
            vec![] as Vec<Intersection>
        } else {
            let i1 = Intersection {
                object: *sphere,
                t: (-b - discriminant.sqrt()) / (2.0 * a)
            };
            if discriminant > 0.0 {
                let i2 = Intersection {
                    object: *sphere,
                    t: (-b + discriminant.sqrt()) / (2.0 * a)
                };
                vec![i1, i2]
            } else {
                vec![i1] 
            }         
        }
    }
}