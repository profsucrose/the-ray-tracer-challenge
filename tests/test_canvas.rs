use ray_tracer::canvas::*;
use ray_tracer::tuples::*;

#[test]
fn create_canvas() {
    // canvas should be 10 by 10 pixels wide with each pixel being black
    let canvas = Canvas::new(10, 10);
    for y in 0..10 {
        for x in 0..10 {
            let pixel = canvas.get(x, y);
            assert_eq!(pixel, &color(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn write_to_canvas() {
    let mut canvas = Canvas::new(10, 10);
    canvas.set(5, 5, color(1.0, 1.0, 1.0));
    let c = &color(5000.0, 1.0, 1.0);
    let c1 = canvas.get(2, 5);
    println!("{}", fequals(c.0, c1.0) && fequals(c.1, c1.1) && fequals(c.2, c1.2) && fequals(c.3, c1.2));
    assert_eq!(canvas.get(2, 5), &color(5000000.0, 1.0, 1.0));
}