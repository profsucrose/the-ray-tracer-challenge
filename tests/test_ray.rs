use ray_tracer::implementations::{
    ray::*,
    tuples::*,
    matrices::*,
    material::*,
    shape::*,
    light::*,
    intersection::*,
    world::*
};

#[test]
fn sphere_ray_intersections() {
    let r = Ray {
        origin: point(0.0, 0.0, 0.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let s = Shape::new(ShapeType::Sphere);
    let intersections = s.intersect(&r);
    assert_ne!(intersections.len(), 0);
    let i1 = &intersections[0];
    let i2 = &intersections[1];
    assert_eq!(i1.object.origin, s.origin);
    assert_eq!(i1.t, -1.0);
    assert_eq!(i2.t, 1.0);

    let r = Ray {
        origin: point(0.0, 0.0, 5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let intersections = s.intersect(&r);
    assert_ne!(intersections.len(), 0);
    let i1 = &intersections[0];
    let i2 = &intersections[1];
    assert_eq!(i1.object.origin, s.origin);
    assert_eq!(i1.t, -6.0);
    assert_eq!(i2.t, -4.0);
}

#[test]
fn test_hit() {
    let s = Shape::new(ShapeType::Sphere);
    let i1 = Intersection {
        object: &s,
        t: 1.0
    };
    let i2 = Intersection {
        object: &s,
        t: 2.0
    };
    let intersections = vec![i1.clone(), i2];
    let i = hit(intersections);
    assert_ne!(i.clone(), None);
    assert_eq!(i.unwrap(), i1);

    let i1 = Intersection {
        object: &s,
        t: -1.0
    };
    let i2 = Intersection {
        object: &s,
        t: 1.0
    };
    let intersections = vec![i1, i2.clone()];
    let i = hit(intersections);
    assert_ne!(i, None);
    assert_eq!(i.unwrap(), i2);

    let i1 = Intersection {
        object: &s,
        t: -2.0
    };
    let i2 = Intersection {
        object: &s,
        t: -1.0
    };
    let intersections = vec![i1, i2];
    let i = hit(intersections);
    assert_eq!(i, None);

    let i1 = Intersection {
        object: &s,
        t: 5.0
    };
    let i2 = Intersection {
        object: &s,
        t: 7.0
    };
    let i3 = Intersection {
        object: &s,
        t: -3.0
    };
    let i4 = Intersection {
        object: &s,
        t: 2.0
    };
    let intersections = vec![i1, i2, i3, i4.clone()];
    let i = hit(intersections);
    assert_ne!(i, None);
    assert_eq!(i.unwrap(), i4);
}

#[test]
fn transform_ray() {
    let r = Ray {
        origin: point(1.0, 2.0, 3.0),
        direction: vector(0.0, 1.0, 0.0)
    };
    let m = translation(3.0, 4.0, 5.0);
    let r2 = r.transform(&m);
    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));

    let m = scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(&m);
    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}

#[test]
fn intersect_transformed_spheres() {
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let mut s = Shape::new(ShapeType::Sphere);
    s.transform = scaling(2.0, 2.0, 2.0);
    let intersections = s.intersect(&r);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, 3.0);
    assert_eq!(intersections[1].t, 7.0);

    s.transform = translation(5.0, 0.0, 0.0);
    let intersections = s.intersect(&r);
    assert_eq!(intersections.len(), 0);
}

#[test]
fn sphere_surface_normals() {
    let s = Shape::new(ShapeType::Sphere);
    let n = s.normal_at(&point(1.0, 0.0, 0.0));
    assert_eq!(n, vector(1.0, 0.0, 0.0));

    let n = s.normal_at(&point(0.0, 1.0, 0.0));
    assert_eq!(n, vector(0.0, 1.0, 0.0));

    let n = s.normal_at(&point(0.0, 0.0, 1.0));
    assert_eq!(n, vector(0.0, 0.0, 1.0));

    let coord = (3.0 as f32).cbrt() / 3.0;
    let n = s.normal_at(&point(coord, coord, coord));
    assert_eq!(n, vector(coord, coord, coord).normalize());
}

#[test]
fn transformed_sphere_surface_normals() {
    let mut s = Shape::new(ShapeType::Sphere);
    s.transform = translation(0.0, 1.0, 0.0);
    let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
    assert_eq!(n, vector(0.0, 0.70711, -0.70711));

    s.transform = &scaling(1.0, 0.5, 1.0)
        * &rotation_z(36.0);
    let n = s.normal_at(
        &point(
            0.0, 
            (2.0 as f32).sqrt() / 2.0, 
            -(2.0 as f32).sqrt() / 2.0
        )
    );
    assert_eq!(n, vector(0.0, 0.97014, -0.24254));
}

#[test]
fn reflect_vector() {
    let v = vector(1.0, -1.0, 0.0);
    let n = vector(0.0, 1.0, 0.0);
    let r = v.reflect(&n);
    assert_eq!(r, vector(1.0, 1.0, 0.0));

    let v = vector(0.0, -1.0, 0.0);
    let coord = (2.0 as f32).sqrt() / 2.0;
    let n = vector(coord, coord, 0.0);
    let r = v.reflect(&n);
    assert_eq!(r, vector(1.0, 0.0, 0.0));
}

#[test]
fn lighting_test() {
    let m = Material::new();
    let position = point(0.0, 0.0, 0.0);

    let shape = Shape::new(ShapeType::Sphere);

    // light and eye opposite to normal
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &shape, &light, &position, &eyev, &normalv, false);
    assert_eq!(result, color(1.9, 1.9, 1.9));

    // light opposite to normal, eye at 45 degrees
    let eyev = vector(0.0, (2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let result = lighting(&m, &shape, &light, &position, &eyev, &normalv, false);
    assert_eq!(result, color(1.0, 1.0, 1.0));

    // eye directly opposite to surface normal, light at 45 degrees
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: vector(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &shape, &light, &position, &eyev, &normalv, false);
    assert_eq!(result, color(0.73481, 0.73481, 0.73481));

    // eye and sun at opposing 45 degrees
    let eyev = vector(0.0, -(2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &shape, &light, &position, &eyev, &normalv, false);
    assert_eq!(result, color(1.6363853, 1.6363853, 1.6363853));

    // light behind surface
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 0.0, 10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &shape, &light, &position, &eyev, &normalv, false);
    assert_eq!(result, color(0.1, 0.1, 0.1));
}

#[test]
fn prepare_computations() {
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let shape = Shape::new(ShapeType::Sphere);
    let i = Intersection {
        object: &shape,
        t: 4.0
    };
    let comps = i.prepare_computations(&r, vec![i]);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, point(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, false);

    // intersection inside object
    let r = Ray {
        origin: point(0.0, 0.0, 0.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let i = Intersection {
        object: &shape,
        t: 1.0
    };
    let comps = i.prepare_computations(&r, vec![i]);
    assert_eq!(comps.point, point(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
}

#[test]
fn reflect_vector_test() {
    let shape = Shape::new(ShapeType::Plane);
    let r = Ray {
        origin: point(0.0, 1.0, -1.0),
        direction: vector(0.0, -(2.0 as f32).sqrt() / 2.0, (2.0 as f32).sqrt() / 2.0)
    };
    let i = Intersection {
        t: (2.0 as f32).sqrt(),
        object: &shape
    };
    let comps = i.prepare_computations(&r, vec![i]);
    assert_eq!(&comps.reflectv, &vector(0.0, (2.0 as f32).sqrt() / 2.0, (2.0 as f32).sqrt() / 2.0));
}

#[test]
fn reflective_surface() {
    let mut w = World::new();
    let mut shape = Shape::new(ShapeType::Plane);
    shape.material.reflective = 0.5;
    shape.transform = translation(0.0, -1.0, 0.0);
    w.shapes.push(shape.clone());
    let r = Ray {
        origin: point(0.0, 0.0, -3.0),
        direction: vector(0.0, -(2.0 as f32).sqrt() / 2.0, (2.0 as f32).sqrt() / 2.0)
    };
    let i = Intersection {
        t: (2.0 as f32).sqrt(),
        object: &shape
    };
    let comps = i.prepare_computations(&r, vec![i]);
    let c = w.reflected_color(&comps, 1);
    assert_eq!(&c, &color(0.19205, 0.24006, 0.14404));
}

fn test_refraction_n1_n2_helper(index: usize, n1: f32, n2: f32) {
    let mut a = glass_sphere();
    a.transform = scaling(2.0, 2.0, 2.0);
    a.material.refractive_index = 1.5;

    let mut b = glass_sphere();
    b.transform = translation(0.0, 0.0, -0.25);
    b.material.refractive_index = 2.0;
    
    let mut c = glass_sphere();
    c.transform = translation(0.0, 0.0, 0.25);
    c.material.refractive_index = 2.5;

    let r = Ray {
        origin: point(0.0, 0.0, -4.0),
        direction: vector(0.0, 0.0, 1.0)
    };

    let intersections = vec![
        Intersection {
            t: 2.0,
            object: &a
        },
        Intersection {
            t: 2.75,
            object: &b
        },
        Intersection {
            t: 3.25,
            object: &c
        },
        Intersection {
            t: 4.75,
            object: &b
        },
        Intersection {
            t: 5.25,
            object: &c
        },
        Intersection {
            t: 6.0,
            object: &a
        }        
    ];

    let intersections_head_clone = intersections[index].clone();
    let comps = intersections_head_clone.prepare_computations(&r, intersections);
    println!("{:?} {:?}: {:?} {:?}", n1, n2, comps.n1, comps.n2);
    assert_eq!(comps.n1, Some(n1));
    assert_eq!(comps.n2, Some(n2));
}

#[test]
fn test_refraction_n1_n2() {
    test_refraction_n1_n2_helper(0, 1.0, 1.5);
    test_refraction_n1_n2_helper(1, 1.5, 2.0);
    test_refraction_n1_n2_helper(2, 2.0, 2.5);
    test_refraction_n1_n2_helper(3, 2.5, 2.5);
    test_refraction_n1_n2_helper(4, 2.5, 1.5);
    test_refraction_n1_n2_helper(5, 1.5, 1.0);
}

#[test]
fn under_point_test() {
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let mut shape = glass_sphere();
    shape.transform = translation(0.0, 0.0, 1.0);

    let i = Intersection {
        t: 5.0,
        object: &shape
    };
    let comps = i.prepare_computations(&r, vec![i]);
    assert!(comps.under_point.2 > EPSILON / 2.0);
    println!("Point: {:#?} Under-point: {:#?}", comps.point, comps.under_point);
    assert!(comps.point.2 < comps.under_point.2);
}

#[test]
fn refraction_when_transparent() {
    let w = World::new();
    let s = w.shapes.first().unwrap();
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let intersections = vec![
        Intersection {
            t: 4.0,
            object: s
        },
        Intersection {
            t: 6.0,
            object: s
        }
    ];
    let comps = intersections.first().unwrap().prepare_computations(&r, intersections.clone());
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn total_internal_refraction() {
    let w = World::new();
    let mut s = w.shapes.first().unwrap().clone();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    let r = Ray {
        origin: point(0.0, 0.0, (2.0 as f32).sqrt() / 2.0),
        direction: vector(0.0, 1.0, 0.0)
    };
    let intersections = vec![
        Intersection {
            t: -(2.0 as f32).sqrt() / 2.0,
            object: &s
        },
        Intersection {
            t: (2.0 as f32).sqrt() / 2.0,
            object: &s
        }
    ];
    let intersections_head = intersections.clone()[1];
    let comps = intersections_head.prepare_computations(&r, intersections);
    assert_eq!(w.refracted_color(&comps, 5), color(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color() {
    let w = World::new();
    let mut a = w.shapes[0].clone();
    a.material.ambient = 1.0;
    
    let mut b = w.shapes[1].clone();
    b.material.transparency = 1.0;
    b.material.refractive_index = 1.5;

    let r = Ray {
        origin: point(0.0, 0.0, 0.1),
        direction: vector(0.0, 1.0, 0.0)
    };

    let intersections = vec![
        Intersection {
            t: -0.9899,
            object: &a
        },
        Intersection {
            t: -0.4899,
            object: &b
        },
        Intersection {
            t: 0.4899,
            object: &b
        },
        Intersection {
            t: 0.9899,
            object: &a
        }
    ];
    let i = intersections[2].clone();
    let comps = i.prepare_computations(&r, intersections);
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, color(0.0, 0.99888, 0.04725));
}