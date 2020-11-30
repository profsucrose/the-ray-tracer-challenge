use crate::{intersection::*, light::*, matrices::*, ray::*, sphere::*, tuples::*};

pub struct World {
    pub shapes: Vec<Sphere>,
    pub light: Light
}

impl World {
    pub fn new() -> World {
        let mut s1 = Sphere::new();
        s1.material.color = color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
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
            for intersection in ray.intersects(&shape) {
                intersections.push(intersection);
            }
        }
        intersections
    }
}