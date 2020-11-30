use ray_tracer::ray::*;
use ray_tracer::tuples::*;
use ray_tracer::matrices::*;
use ray_tracer::material::*;
use ray_tracer::sphere::*;
use ray_tracer::light::*;
use ray_tracer::intersection::*;

#[test]
fn sphere_ray_intersections() {
    let r = Ray {
        origin: point(0.0, 0.0, 0.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let sphere = Sphere::new();
    let intersections = r.intersects(&sphere);
    assert_ne!(intersections.len(), 0);
    let i1 = &intersections[0];
    let i2 = &intersections[1];
    assert_eq!(i1.object.origin, sphere.origin);
    assert_eq!(i1.t, -1.0);
    assert_eq!(i2.t, 1.0);

    let r = Ray {
        origin: point(0.0, 0.0, 5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let intersections = r.intersects(&sphere);
    assert_ne!(intersections.len(), 0);
    let i1 = &intersections[0];
    let i2 = &intersections[1];
    assert_eq!(i1.object.origin, sphere.origin);
    assert_eq!(i1.t, -6.0);
    assert_eq!(i2.t, -4.0);
}

#[test]
fn test_hit() {
    let s = Sphere::new();
    let i1 = Intersection {
        object: s,
        t: 1.0
    };
    let i2 = Intersection {
        object: s,
        t: 2.0
    };
    let intersections = vec![i1, i2];
    let i = hit(intersections);
    assert_ne!(i, None);
    assert_eq!(i.unwrap(), i1);

    let i1 = Intersection {
        object: s,
        t: -1.0
    };
    let i2 = Intersection {
        object: s,
        t: 1.0
    };
    let intersections = vec![i1, i2];
    let i = hit(intersections);
    assert_ne!(i, None);
    assert_eq!(i.unwrap(), i2);

    let i1 = Intersection {
        object: s,
        t: -2.0
    };
    let i2 = Intersection {
        object: s,
        t: -1.0
    };
    let intersections = vec![i1, i2];
    let i = hit(intersections);
    assert_eq!(i, None);

    let i1 = Intersection {
        object: s,
        t: 5.0
    };
    let i2 = Intersection {
        object: s,
        t: 7.0
    };
    let i3 = Intersection {
        object: s,
        t: -3.0
    };
    let i4 = Intersection {
        object: s,
        t: 2.0
    };
    let intersections = vec![i1, i2, i3, i4];
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
    let mut s = Sphere::new();
    s.transform = scaling(2.0, 2.0, 2.0);
    let intersections = r.intersects(&s);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, 3.0);
    assert_eq!(intersections[1].t, 7.0);

    s.transform = translation(5.0, 0.0, 0.0);
    let intersections = r.intersects(&s);
    assert_eq!(intersections.len(), 0);
}

#[test]
fn sphere_surface_normals() {
    let s = Sphere::new();
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
    let mut s = Sphere::new();
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

    // light and eye opposite to normal
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &light, &position, &eyev, &normalv);
    assert_eq!(result, color(1.9, 1.9, 1.9));

    // light opposite to normal, eye at 45 degrees
    let eyev = vector(0.0, (2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let result = lighting(&m, &light, &position, &eyev, &normalv);
    assert_eq!(result, color(1.0, 1.0, 1.0));

    // eye directly opposite to surface normal, light at 45 degrees
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: vector(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &light, &position, &eyev, &normalv);
    assert_eq!(result, color(0.7364, 0.7364, 0.7364));

    // eye and sun at opposing 45 degrees
    let eyev = vector(0.0, -(2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &light, &position, &eyev, &normalv);
    assert_eq!(result, color(1.6364, 1.6364, 1.6364));

    // light behind surface
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light {
        position: point(0.0, 0.0, 10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let result = lighting(&m, &light, &position, &eyev, &normalv);
    assert_eq!(result, color(0.1, 0.1, 0.1));
}