use crate::matrices::*;
use crate::tuples::*;
use crate::canvas::*;

pub fn draw_clock(width: usize, height: usize, x: f32, y: f32) {
    let p = point(x, 0.0, y);    
    let mut c = Canvas::new(width, height);
    for i in 0..12 {
        let p = &rotation_y((i as f32) * 30.0) * &p;
        println!("{:?}", p);
        let x = (p.0.round() + 50.0) as usize;
        let y = (p.2.round() + 50.0) as usize;
        c.set(x, y, color(1.0, 1.0, 1.0));
    }
    c.write_to_ppm("clock.ppm");
}