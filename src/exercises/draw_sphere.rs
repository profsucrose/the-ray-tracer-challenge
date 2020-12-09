use crate::implementations::{canvas::*, intersection::*, light::*, ray::*, shape::*, tuples::*};

pub fn draw_sphere() {
    //let img = get_image("santaclaus100x100.png");

    let ray_origin = point(0.0, 0.0, -5.0);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);   
    let mut shape = Shape::new(ShapeType::Sphere);
    shape.material.color = color(1.0, 0.0, 0.0);

    let light = Light {
        position: point(-10.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0)
    };

    for y in 0..100 {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..100 {
            //let pixel = img.get_pixel(x, y);
            //shape.material.color = color(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);
            let world_x = -half + pixel_size * (x as f32);
            let position = point(world_x as f32, world_y as f32, wall_z as f32);
            let r = Ray {
                origin: ray_origin,
                direction: (&position - &ray_origin).normalize()
            };
            let intersections = shape.intersect(&r);

            if intersections.len() > 0 {
                if let Some(hit) = hit(intersections) {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(&point);
                    let eye = -r.direction;
                    let color = lighting(
                        &hit.object.material,
                        &hit.object,
                        &light,
                        &point,
                        &eye,
                        &normal,
                        false
                    );
                    canvas.set(x as usize, y as usize, color);
                }
            }
        }
    }
    canvas.write_to_ppm("sphere.ppm");
}
