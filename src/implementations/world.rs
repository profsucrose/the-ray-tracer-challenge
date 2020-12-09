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
    pub light: Light,
    pub quick_rendered: bool
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
            },
            quick_rendered: false
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
            self.get_light_intensity_with_shadow(&comps.over_point)
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = &comps.object.material;
        // combine reflectance and transparency using Schlick's formula 
        // if material is both reflective and transparent
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            return &surface + &(&(&reflected * reflectance) + &(&refracted * (1.0 - reflectance)));
        }
        &(&surface + &reflected) + &refracted
    }

    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Vec4 {
        let intersections = self.intersect(&ray);
        if let Some(hit) = hit(intersections.clone()) {
            let hit_clone = hit.clone();
            let comps = hit_clone.prepare_computations(&ray, intersections);
            if self.quick_rendered {
                let shape = comps.object;
                let material = &comps.object.material;
                let material_color: Vec4;
                if let Some(pattern) = &material.pattern {
                    material_color = pattern.color_at(shape, &comps.over_point);
                } else {
                    material_color = material.color;
                }
                return material_color;
            }
            
            self.shade_hit(&comps, remaining)
        } else {
            color(0.0, 0.0, 0.0)
        }
    }

    fn test_shadow_feeler(&self, light_point: Vec4, point: &Vec4) -> bool {
        let diff = &light_point - point;
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

    pub fn get_light_intensity_with_shadow(&self, p: &Vec4) -> Vec4 {
        let light_point = self.light.position;
        let mut hit_count = 8.0;
        if self.test_shadow_feeler(point(light_point.0 + 0.01, light_point.0 + 0.01, light_point.0 + 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 + 0.01, light_point.0 + 0.01, light_point.0 - 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 + 0.01, light_point.0 - 0.01, light_point.0 + 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 + 0.01, light_point.0 - 0.01, light_point.0 - 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 - 0.01, light_point.0 + 0.01, light_point.0 + 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 - 0.01, light_point.0 + 0.01, light_point.0 - 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 - 0.01, light_point.0 - 0.01, light_point.0 + 0.01), &p) {
            hit_count -= 1.0;
        }
        if self.test_shadow_feeler(point(light_point.0 - 0.01, light_point.0 - 0.01, light_point.0 - 0.01), &p) {
            hit_count -= 1.0;
        }

        let percentage = hit_count / 8.0;
        println!("{}", percentage);
        color(percentage, percentage, percentage)
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

    pub fn refracted_color(&self, comps: &Computations, remaining: u32) -> Vec4 {
        // return black if opaque or refraction call stack cap
        if comps.object.material.transparency == 0.0 || remaining == 0 {
            return color(0.0, 0.0, 0.0)
        }

        // check for total internal reflection using Snell's Law
        let n_ratio = comps.n1.unwrap() / comps.n2.unwrap();
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return color(0.0, 0.0, 0.0)
        }

        // get cos via trig identity
        let cos_t = ((1.0 - sin2_t) as f32).sqrt();
        
        // get direction of refracted ray
        let direction = &(&comps.normalv * (n_ratio * cos_i - cos_t)) - &(&comps.eyev * n_ratio);

        // spawn refracted ray
        let refract_ray = Ray {
            origin: comps.under_point,
            direction
        };

        // get color of refracted ray and account for transparency
        &self.color_at(&refract_ray, remaining - 1) * comps.object.material.transparency
    }
}
