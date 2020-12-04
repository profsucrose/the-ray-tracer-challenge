use std::cmp::Ordering;

use crate::implementations::{
    computations::Computations, 
    intersection::*, 
    light::*, 
    matrices::*, 
    ray::*, 
    shape::*, 
    tuples::*
};

pub struct World {
    pub shapes: Vec<Shape>,
    pub light: Light
}

impl World {
    pub fn new() -> World {
        let mut s1 = Shape::new(ShapeType::Sphere);
        s1.material.color = color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Shape::new(ShapeType::Sphere);
        s2.transform = scaling(0.5, 0.5, 0.5);
        World {
            shapes: vec![
                s1,
                s2
            ],
            light: Light {
                position: point(-10.0, 10.0, -10.0),
                intensity: color(1.0, 1.0, 1.0)
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        // add any intersections from ray to each shape in world
        for shape in &self.shapes {
            for intersection in shape.intersect(&ray) {
                intersections.push(intersection);
            }
        }
        intersections.sort_by(|a, b| 
            if fequals(a.t, b.t) {
                Ordering::Equal
            } else if a.t > b.t {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        );
        intersections
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: u32) -> Vec4 {
        let surface = lighting(
            &comps.object.material, 
            &comps.object,
            &self.light, 
            &comps.point, 
            &comps.eyev, 
            &comps.normalv,
            self.is_shadowed(&comps.over_point)
        );

        let reflected = self.reflected_color(comps, remaining);
        &surface + &reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Vec4 {
        let intersections = self.intersect(&ray);
        if let Some(hit) = hit(intersections) {
            let hit_clone = hit.clone();
            let comps = hit_clone.prepare_computations(&ray, vec![hit]);
            self.shade_hit(&comps, remaining)
        } else {
            color(0.0, 0.0, 0.0)
        }
    }

    pub fn is_shadowed(&self, point: &Vec4) -> bool {
        let diff = &self.light.position - point;
        let distance = diff.mag();
        let ray = Ray {
            origin: *point,
            direction: diff.normalize()
        };
        if let Some(hit) = hit(self.intersect(&ray)) {
            if hit.t < distance {
                return true
            }
        }
        false
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: u32) -> Vec4 {
        if remaining <= 0 || fequals(comps.object.material.reflective, 0.0) {
            return color(0.0, 0.0, 0.0)
        }

        let reflect_ray = Ray {
            origin: comps.over_point,
            direction: comps.reflectv
        };
        let color = self.color_at(&reflect_ray, remaining - 1);
        &color * comps.object.material.reflective
    }
}
