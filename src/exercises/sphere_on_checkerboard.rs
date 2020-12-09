use texture_pattern::TexturePattern;

use crate::implementations::{camera::*, patterns::texture_pattern};
use crate::implementations::light::*;
use crate::implementations::matrices::*;
use crate::implementations::shape::*;
use crate::implementations::tuples::*;
use crate::implementations::world::*;

pub fn draw_scene() {
    let mut floor = Shape::new(ShapeType::Plane);
    floor.material.set_pattern(
        Box::new(TexturePattern::new("santaclaus100x100.png", 30.0, 30.0, 0.0, true, false))
    );

    let mut sphere = Shape::new(ShapeType::Sphere);
    sphere.material.set_pattern(
        Box::new(TexturePattern::new("santaclaus100x100.png", 50.0, 50.0, 0.0, false, true))
    );
    sphere.transform = translation(11.4, 1.0, 0.0);
    sphere.material.reflective = 0.8;

    let mut world = World::new();
    world.shapes = vec![
        floor,
        sphere
    ];
    world.light = Light {
        position: point(-9.6, 10.0, -30.0),
        intensity: color(1.0, 1.0, 1.0)
    };
    let mut camera = Camera::new(300, 200, 60.0);
    camera.transform = view_transform(
        &point(0.0, 1.5, -3.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0)
    ).translate(-11.4, 0.0, 0.0);
    let canvas = camera.render(&world, 3);
    canvas.write_to_ppm("sphere_on_checkerboard.ppm");
}
