use crate::implementations::{
    tuples::*,
    matrices::*,
    material::*,
    intersection::*,
    ray::*
};

pub fn glass_sphere() -> Shape {
    let mut s = Shape::new(ShapeType::Sphere);
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeType {
    Plane,
    Sphere
}

#[derive(Debug, PartialEq, Clone)]
pub struct Shape {
    pub origin: Vec4,
    pub transform: Matrix4x4,
    pub material: Material,
    pub shape_type: ShapeType
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Shape {
        // unit sphere with identity transform matrix
        Shape {
            origin: point(0.0, 0.0, 0.0),
            transform: Matrix4x4::ident(),
            material: Material::new(),
            shape_type
        }
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match self.shape_type {
            ShapeType::Sphere => {
                // calculate ray intersection
                // solutions via quadratic formula
                // get discriminant to get number of solutions 
                // to return
                let sphere_to_ray = &ray.origin - &self.origin;
                let a = ray.direction.dot(&ray.direction);
                let b = &ray.direction.dot(&sphere_to_ray) * 2.0;
                let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
                let discriminant = b.powi(2) as f32 - 4.0 * a * c;
                let discrim_sqrt = discriminant.sqrt();
                // TODO: Make more DRY with closures
                if fequals(discriminant, 0.0) {
                    let i1 = Intersection {
                        object: self,
                        t: (-b - discrim_sqrt) / (2.0 * a)
                    };
                    vec![i1]
                } else {
                    if discriminant > 0.0 {
                        let i1 = Intersection {
                            object: self,
                            t: (-b - discrim_sqrt) / (2.0 * a)
                        };
                        let i2 = Intersection {
                            object: self,
                            t: (-b + discrim_sqrt) / (2.0 * a)
                        };
                        vec![i1, i2]
                    } else {
                        vec![] as Vec<Intersection>
                    }         
                }
            },
            ShapeType::Plane => {
                if ray.direction.1.abs() < EPSILON {
                    return vec![]
                }
        
                let t = -ray.origin.1 / ray.direction.1;
                vec![
                    Intersection {
                        object: self,
                        t
                    }
                ]
            }
        }
        
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.transform.invert());
        self.local_intersect(&local_ray)
    }

    fn local_normal_at(&self, point: &Vec4) -> Vec4 {
        match self.shape_type {
            ShapeType::Plane => vector(0.0, 1.0, 0.0),
            ShapeType::Sphere => (point - &self.origin).normalize()
        }
    }

    pub fn normal_at(&self, point: &Vec4) -> Vec4 {
        let local_point = &self.transform.invert() * point;
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = &self.transform.invert().transpose() * &local_normal;
        world_normal.3 = 0.0;
        world_normal.normalize()
    }
}