use ray_tracer::implementations::{
    camera::Camera, 
    intersection::*, 
    light::*, 
    matrices::*, 
    ray::*, 
    tuples::*, 
    world::*,
    shape::*
};

#[test]
fn intersect_world() {
    let world = World::new();
    let ray = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let intersections = world.intersect(&ray);
    assert_eq!(intersections.len(), 4);
    assert_eq!(intersections[0].t, 4.0);
    assert_eq!(intersections[1].t, 4.5);
    assert_eq!(intersections[2].t, 5.5);
    assert_eq!(intersections[3].t, 6.0);
}

#[test]
fn shade_hit() {
    let mut w = World::new();
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let shape = w.shapes[0];
    let i = Intersection {
        object: shape,
        t: 4.0
    };
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, color(0.38066, 0.47583, 0.2855));

    w.light = Light {
        position: point(0.0, 0.25, 0.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let r = Ray {
        origin: point(0.0, 0.0, 0.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let shape = w.shapes[1];
    let i = Intersection {
        object: shape,
        t: 0.5
    };
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, color(0.1, 0.1, 0.1));
}

#[test]
fn color_at() {
    // ray misses
    let mut w = World::new();
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 1.0, 0.0)
    };
    let c = w.color_at(&r);
    assert_eq!(c, color(0.0, 0.0, 0.0));

    // ray hits
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let c = w.color_at(&r);
    assert_eq!(c, color(0.38066, 0.47583, 0.2855));

    // ray in-between two concentric spheres and pointed at inner
    // color should be color of inner sphere
    let mut outer = &mut w.shapes[0];
    outer.material.ambient = 1.0;
    let mut inner = &mut w.shapes[1];
    inner.material.ambient = 1.0;
    let r = Ray {
        origin: point(0.0, 0.0, 0.75),
        direction: vector(0.0, 0.0, -1.0)
    };
    let c = w.color_at(&r);
    assert_eq!(c, w.shapes[1].material.color);
}

#[test]
fn view_transform_test() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, -1.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(&from, &to, &up);
    assert_eq!(t, Matrix4x4::ident());

    let from = point(1.0, 3.0, 2.0);
    let to = point(4.0, -2.0, 8.0);
    let up = vector(1.0, 1.0, 0.0);
    let t = view_transform(&from, &to, &up);
    assert_eq!(t, Matrix4x4(
        Vec4(-0.50709, 0.50709, 0.67612, -2.36643),
        Vec4(0.76772, 0.60609, 0.12122, -2.82843),
        Vec4(-0.35857, 0.59761, -0.71714, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    ));
}

#[test]
fn pixel_size() {
    let c = Camera::new(200, 125, 90.0);
    assert_eq!(c.pixel_size, 0.01);

    let c = Camera::new(125, 200, 90.0);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn ray_for_pixel() {
    let mut c = Camera::new(201, 101, 90.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.0, 0.0, -1.0));

    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));

    c.transform = rotation_y(45.0)
        .translate(0.0, -2.0, 5.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 2.0, -5.0));
    assert_eq!(r.direction, vector((2.0 as f32).sqrt() / 2.0, 0.0, -(2.0 as f32).sqrt() / 2.0));
}

#[test]
fn render_test() {
    let w = World::new();
    let mut c = Camera::new(11, 11, 90.0);
    let from = point(0.0, 0.0, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(&from, &to, &up);
    let image = c.render(&w);
    assert_eq!(image.get(5, 5), &color(0.38066, 0.47583, 0.2855));
}

#[test]
fn is_shadowed() {
    let w = World::new();
    let p = point(0.0, 10.0, 0.0);
    assert_eq!(w.is_shadowed(&p), false);

    let p = point(10.0, -10.0, 10.0);
    assert_eq!(w.is_shadowed(&p), true);

    let p = point(-20.0, 20.0, -20.0);
    assert_eq!(w.is_shadowed(&p), false);

    let p = point(-2.0, 2.0, -2.0);
    assert_eq!(w.is_shadowed(&p), false);
}

#[test]
fn shade_hit_with_shadow() {
    let mut w = World::new();
    w.light = Light {
        position: point(0.0, 0.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };

    let s1 = Shape::new(ShapeType::Sphere);
    let mut s2 = Shape::new(ShapeType::Sphere);
    s2.transform = translation(0.0, 0.0, 10.0);

    w.shapes = vec![
        s1,
        s2
    ];

    let r = Ray {
        origin: point(0.0, 0.0, 5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let i = Intersection {
        object: s2,
        t: 4.0
    };
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, color(0.1, 0.1, 0.1));
}