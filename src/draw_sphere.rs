//extern crate image;

//use image::{GenericImageView, DynamicImage};
//use std::fs::File;
//use std::path::Path;

use crate::ray::*;
use crate::tuples::*;
use crate::canvas::*;
use crate::sphere::*;
use crate::intersection::*;
use crate::light::*;

// pub fn get_image(path: &str) -> DynamicImage {
//     image::open(&Path::new(path)).unwrap()
// }

pub fn draw_sphere() {
    //let img = get_image("santaclaus100x100.png");

    let ray_origin = point(0.0, 0.0, -5.0);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);   
    let mut shape = Sphere::new();
    shape.material.color = color(0.0, 1.0, 0.1);

    let light = Light {
        position: point(-10.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };

    for y in 0..100 {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..100 {
            let world_x = -half + pixel_size * (x as f32);
            //println!("{} {} {}", world_x, world_y, wall_z);
            let position = point(world_x as f32, world_y as f32, wall_z as f32);
            let r = Ray {
                origin: ray_origin,
                direction: (&position - &ray_origin).normalize()
            };
            let intersections = r.intersects(&shape);

           // println!("{:?}", r);
            if intersections.len() > 0 {
                if let Some(hit) = hit(intersections.clone()) {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(&point);
                    let eye = -r.direction;
                    let color = lighting(
                        &hit.object.material,
                        &light,
                        &point,
                        &eye,
                        &normal
                    );
                    println!("{:?}", color);
                    canvas.set(x as usize, y as usize, color);
                }
            }
        }
    }
    canvas.write_to_ppm("sphere.ppm");
}