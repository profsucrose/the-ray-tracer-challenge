use ray_tracer::{world::*, ray::*, tuples::*};

#[test]
fn intersect_world() {
    let world = World::new();
    let ray = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0)
    };
    let intersections = world.intersect(&ray);
    assert_eq!(intersections.len(), 4);
    for i in intersections {
        println!("{}", i.t);
    }
    // assert_eq!(intersections[0].t, 4.0);
    // assert_eq!(intersections[1].t, 4.5);
    // assert_eq!(intersections[2].t, 5.5);
    // assert_eq!(intersections[3].t, 6.0);
}