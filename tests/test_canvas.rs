use ray_tracer::implementations::{
    canvas::*,
    tuples::*
};

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
    assert_eq!(canvas.get(5, 5), &color(1.0, 1.0, 1.0));
}