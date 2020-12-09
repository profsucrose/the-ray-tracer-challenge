use ray_tracer::implementations::tuples::*;

#[test]
fn create_point() {
    let point = point(1.0, 1.0, 1.0);
    assert_eq!(&point, &Vec4(1.0, 1.0, 1.0, 1.0));
}

#[test]
fn create_vector() {
    let vec = vector(1.0, 1.0, 1.0);
    assert_eq!(&vec, &Vec4(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn add_vectors() {
    let vec1 = vector(1.0, 1.0, 1.0);
    let vec2 = vector(1.0, 1.0, 1.0);
    assert_eq!(&vec1 + &vec2, vector(2.0, 2.0, 2.0));
}

#[test]
#[should_panic(expected = "Attempted to add two points, w component cannot be 2.0")]
fn add_points() {
    let point1 = point(1.0, 1.0, 1.0);
    let point2 = point(1.0, 1.0, 1.0);
    println!("{:?}", &point1 + &point2);
}

#[test]
fn subtract_points() {
    let point1 = point(3.0, 2.0, 1.0);
    let point2 = point(5.0, 6.0, 7.0);
    assert_eq!(&point1 - &point2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtract_vectors() {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    assert_eq!(&v1 - &v2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtract_vector_from_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(&p - &v, point(-2.0, -4.0, -6.0));
}    

#[test]
fn dot_product_vectors() {
    let vec1 = vector(1.0, 2.0, 3.0);
    let vec2 = vector(2.0, 3.0, 4.0);
    assert_eq!(vec1.dot(&vec2), 20.0);
}

#[test]
fn mult_tuple_and_whole_scalar() {
    let t = Vec4(2.0, 1.0, 1.0, 3.0);
    let scalar = 5.0;
    assert_eq!(&t * scalar, Vec4(10.0, 5.0, 5.0, 15.0));
}

#[test]
fn mult_tuple_and_fractional_scalar() {
    let t = Vec4(1.0, -2.0, 3.0, -4.0);
    let scalar = 0.5;
    assert_eq!(&t * scalar, Vec4(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn negate_vector() {
    let v = vector(5.0, 3.0, 1.0);
    assert_eq!(-v, vector(-5.0, -3.0, -1.0));
}

#[test]
fn magnitude() {
    let v = vector(1.0, 0.0, 0.0);
    assert_eq!(v.mag(), 1.0);

    let v = vector(0.0, 1.0, 0.0);
    assert_eq!(v.mag(), 1.0);

    let v = vector(1.0, 2.0, 3.0);
    assert_eq!(v.mag(), (14.0 as f32).sqrt());

    let v = vector(-1.0, -2.0, -3.0);
    assert_eq!(v.mag(), (14.0 as f32).sqrt());
}

#[test]
fn normalize() {
    let v = vector(4.0, 0.0, 0.0);
    assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    
    let v = vector(1.0, 2.0, 3.0);
    let sqrt14 = (14.0 as f32).sqrt();
    assert_eq!(
        v.normalize(), 
        vector(
            1.0 / sqrt14, 
            2.0 / sqrt14, 
            3.0 / sqrt14
        )
    );
    
    assert!(fequals(v.normalize().mag(), 1.0));
}

#[test]
fn cross_vectors() {
    let v1 = vector(1.0, 2.0, 3.0);
    let v2 = vector(2.0, 3.0, 4.0);
    assert_eq!(v1.cross(&v2), vector(-1.0, 2.0, -1.0));
    assert_eq!(v2.cross(&v1), vector(1.0, -2.0, 1.0))
}