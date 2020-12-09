use crate::implementations::{
    canvas::Canvas, 
    matrices::*, 
    ray::*, 
    tuples::*, 
    world::World
};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub transform: Matrix4x4,
    pub half_height: f32,
    pub half_width: f32,
    pub pixel_size: f32
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (degrees_to_radians(field_of_view) / 2.0).tan();
        let aspect = (hsize as f32) / (vsize as f32);
        let half_width: f32;
        let half_height: f32;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4x4::ident(),
            half_height,
            half_width,
            pixel_size: (half_width * 2.0) / (hsize as f32)
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = ((px as f32) + 0.5) * self.pixel_size;
        let yoffset = ((py as f32) + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = &self.transform.invert() * &point(world_x, world_y, -1.0);
        let origin = &self.transform.invert() * &point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();
        
        Ray {
            origin,
            direction
        }
    }

    pub fn render(&self, world: &World, reflection_limit: u32) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        let mut last_percentage_done = 1;
        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let percentage_done = ((x + (y * self.hsize)) * 10) / ((self.hsize - 1) * (self.vsize - 1));
                
                if last_percentage_done != percentage_done {
                    let mut arrow_string = String::new();
                    for _ in 0..percentage_done {
                        arrow_string.push('=');
                    }
                    if percentage_done != 10 {
                        arrow_string.push('>');
                        for _ in 0..9 - percentage_done {
                            arrow_string.push(' ');
                        }
                    }
                    println!(
                        "\u{001b}[37;1mRender is  \u{001b}[33;1m{}0%\u{001b}[37;1m {}complete \u{001b}[32;1m[{}]", 
                        if percentage_done == 0 {
                            String::from(" ")
                        } else {
                            percentage_done.to_string()
                        },
                        if percentage_done != 10 {
                            " "
                        } else {
                            ""
                        },
                        arrow_string
                    );
                }
                last_percentage_done = percentage_done;

                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, reflection_limit);
                canvas.set(x, y, color);
            }
        }
        canvas
    }
}
