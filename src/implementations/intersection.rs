use crate::implementations::{
    computations::Computations, 
    shape::*,
    ray::*
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub object: &'a Shape,
    pub t: f32
}

static BUMP_EPSILON: f32 = 0.01;
impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: &Ray, intersections: Vec<Intersection>) -> Computations {
        let hit = hit(intersections.clone());
        let mut containers: Vec<Shape> = Vec::new();
        let mut n1: Option<f32> = None;
        let mut n2: Option<f32> = None;
        for i in intersections.iter() {
            if let Some(hit) = hit.clone() {
                if hit == *i {
                    if containers.is_empty() {
                        n1 = Some(1.0);
                    } else {
                        n1 = Some(containers.last().unwrap().material.refractive_index);
                    }
                }
            }

            println!("Object: {:?} Containers: {:?}", i.object, containers);
            if containers.contains(i.object) { 
                println!("Removing object");
                for (index, object) in containers.iter().enumerate() {
                    if object == i.object {
                        containers.remove(index);
                        println!("Removed object");
                        break;
                    }
                }
            } else {
                containers.push(i.object.clone());
            }

            if let Some(hit) = hit {
                if hit == *i {
                    if containers.is_empty() {
                        n2 = Some(1.0);
                    } else {
                        n2 = Some(containers.last().unwrap().material.refractive_index);
                    }
                }

                break;
            }
        }

        let ray_position = ray.position(self.t);
        let normal = self.object.normal_at(&ray_position);
        let over_point = &ray_position + &(&normal * BUMP_EPSILON);
        let mut comps = Computations {
            t: self.t,
            object: self.object,
            point: ray_position,
            eyev: -ray.direction,
            normalv: normal,
            inside: false,
            over_point,
            reflectv: ray.direction.reflect(&normal),
            n1,
            n2
        };
        // account for if ray is inside object
        if comps.normalv.dot(&comps.eyev) < 0.0 {
            comps.inside = true;
            comps.normalv = -comps.normalv
        }
        comps
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections.iter()
        .fold(None, |acc, el| {
            match el.t {
                _ if el.t < 0.0 => acc,
                _ => {
                    if let Some(acc) = acc {
                        if el.t < acc.t {
                            Some(el.clone())
                        } else {
                            Some(acc)
                        }
                    } else {
                        Some(el.clone())
                    }
                }
            }
        })
}
