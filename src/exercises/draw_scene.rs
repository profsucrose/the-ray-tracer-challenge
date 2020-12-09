extern crate image;
use crate::implementations::{camera::*, light::*, matrices::*, patterns::{checker_pattern::CheckerPattern, ring_pattern::RingPattern}, shape::*, tuples::*, world::*};
use image::DynamicImage;
use std::path::Path;

pub fn get_image(path: &str) -> DynamicImage {
    image::open(&Path::new(path)).unwrap()
}

pub fn draw_scene() {
    //let img = get_image("santaclaus100x100.png");
    let mut floor = Shape::new(ShapeType::Plane);
    floor.material.set_pattern(
        Box::new(CheckerPattern {
            a: color(0.8, 0.8, 0.8), 
            b: color(0.0, 0.0, 0.0), 
            transform: Matrix4x4::ident()
        })
    );

    let mut middle = Shape::new(ShapeType::Sphere);
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.set_pattern(
        Box::new(RingPattern {
            a: color(0.0, 1.0, 0.0), 
            b: color(1.0, 0.0, 0.0), 
            transform: Matrix4x4::ident()
        })
    );

    let mut right = Shape::new(ShapeType::Sphere);
    right.transform = translation(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5);
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::new(ShapeType::Sphere);
    left.transform = translation(-1.5, 0.33, -0.75)
        .scale(0.33, 0.33, 0.33);
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.shapes = vec![
        floor,
        middle,
        right,
        left
    ];
    world.light = Light {
        position: point(-10.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let mut camera = Camera::new(300, 150, 60.0);
    camera.transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0)
    );
    let canvas = camera.render(&world, 3);
    canvas.write_to_ppm("scene.ppm");
}
