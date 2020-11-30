use crate::canvas::*;
use crate::tuples::*;

struct Environment {
    gravity: Vec4,
    wind: Vec4
}

#[derive(Debug)]
struct Projectile {
    position: Vec4,
    velocity: Vec4
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}

// draw_tick usage example
// draw_tick(300, 100, &point(0.0, 1.0, 0.0), &vector(5.0, 2.0, 0.0), &vector(0.0, -0.1, 0.0), &vector(-0.01, 0.0, 0.0));
pub fn draw_tick(width: usize, height: usize, position: &Vec4, velocity: &Vec4, gravity: &Vec4, wind: &Vec4) {
    let mut p = Projectile {
        position: *position,
        velocity: *velocity
    };

    let e = Environment {
        gravity: *gravity,
        wind: *wind
    };

    let mut canvas = Canvas::new(width, height);
    while p.position.1 > 0.0 {
        p = tick(&e, &p);
        let x = p.position.0.round();
        let y = p.position.1.round();
        if x >= width as f32
                || y >= height as f32 {
            continue;
        }
        canvas.set(x as usize, height - 1 - y as usize, color(1.0, 0.0, 0.0));
    }
    canvas.write_to_ppm("tick.ppm");
}
