use ray_tracer::tuples::*;
use ray_tracer::matrices::*;
#[test]
fn construct_matrices() {
    let m = Matrix4x4 (
        Vec4(1.0, 2.0, 3.0, 4.0),
        Vec4(5.5, 6.5, 7.5, 8.5),
        Vec4(9.0, 10.0, 11.0, 12.0),
        Vec4(13.5, 14.5, 15.5, 16.5),
    );
    assert_eq!((m.0).0, 1.0);
    assert_eq!((m.0).3, 4.0);
    assert_eq!((m.1).0, 5.5);
    assert_eq!((m.1).2, 7.5);
    assert_eq!((m.2).2, 11.0);
    assert_eq!((m.3).0, 13.5);
    assert_eq!((m.3).2, 15.5);

    let m = Matrix2x2 (
        Vec2(-3.0, 5.0),
        Vec2(1.0, -2.0)
    );
    assert_eq!((m.0).0, -3.0);
    assert_eq!((m.0).1, 5.0);
    assert_eq!((m.1).0, 1.0);
    assert_eq!((m.1).1, -2.0);

    let m = Matrix3x3 (
        Vec3(-3.0, 5.0, 0.0),
        Vec3(-1.0, -2.0, -7.0),
        Vec3(0.0, 1.0, 1.0)
    );
    assert_eq!((m.0).0, -3.0);
    assert_eq!((m.1).1, -2.0);
    assert_eq!((m.2).2, 1.0);
}
#[test]
fn matrix_equalities() {
    let m = Matrix4x4(
        Vec4(1.0, 2.0, 3.0, 4.0),
        Vec4(5.0, 6.0, 7.0, 8.0),
        Vec4(9.0, 8.0, 7.0, 6.0),
        Vec4(5.0, 4.0, 3.0, 2.0)
    );
    let m1 = Matrix4x4(
        Vec4(1.0, 2.0, 3.0, 4.0),
        Vec4(5.0, 6.0, 7.0, 8.0),
        Vec4(9.0, 8.0, 7.0, 6.0),
        Vec4(5.0, 4.0, 3.0, 2.0)
    );
    assert_eq!(&m, &m1);

    let m1 = Matrix4x4(
        Vec4(5.0, 2.0, 3.0, 4.0),
        Vec4(5.0, 6.0, 7.0, 8.0),
        Vec4(9.0, 3.0, 7.0, 6.0),
        Vec4(5.0, 4.0, 8.0, 2.0)
    );
    assert_ne!(&m, &m1);
}
#[test]
fn multiply_matrices() {
    let m1 = Matrix4x4(
        Vec4(1.0, 2.0, 3.0, 4.0),
        Vec4(5.0, 6.0, 7.0, 8.0),
        Vec4(9.0, 8.0, 7.0, 6.0),
        Vec4(5.0, 4.0, 3.0, 2.0)
    );
    let m2 = Matrix4x4(
        Vec4(-2.0, 1.0, 2.0, 3.0),
        Vec4(3.0, 2.0, 1.0, -1.0),
        Vec4(4.0, 3.0, 6.0, 5.0),
        Vec4(1.0, 2.0, 7.0, 8.0)
    );
    assert_eq!(&m1 * &m2, Matrix4x4(
        Vec4(20.0, 22.0, 50.0, 48.0),
        Vec4(44.0, 54.0, 114.0, 108.0),
        Vec4(40.0, 58.0, 110.0, 102.0),
        Vec4(16.0, 26.0, 46.0, 42.0)
    ));
}

#[test]
fn multiply_matrix_and_vector() {
    let m = Matrix4x4(
        Vec4(1.0, 2.0, 3.0, 4.0),
        Vec4(2.0, 4.0, 4.0, 2.0),
        Vec4(8.0, 6.0, 4.0, 1.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    );
    let v = Vec4(1.0, 2.0, 3.0, 1.0);
    assert_eq!(&m * &v, Vec4(18.0, 24.0, 33.0, 1.0));
    assert_eq!(&m * &Matrix4x4::ident(), m);
}

#[test]
fn transpose_matrix() {
    let m = Matrix4x4(
        Vec4(0.0, 9.0, 3.0, 0.0),
        Vec4(9.0, 8.0, 0.0, 8.0),
        Vec4(1.0, 8.0, 5.0, 3.0),
        Vec4(0.0, 0.0, 5.0, 8.0)
    );
    assert_eq!(m.transpose(), Matrix4x4(
        Vec4(0.0, 9.0, 1.0, 0.0),
        Vec4(9.0, 8.0, 8.0, 0.0),
        Vec4(3.0, 0.0, 5.0, 5.0),
        Vec4(0.0, 8.0, 3.0, 8.0)
    ));
    assert_eq!(Matrix4x4::ident().transpose(), Matrix4x4::ident());
}

#[test]
fn determ_matrix() {
    let m = Matrix2x2(
        Vec2(1.0, 5.0),
        Vec2(-3.0, 2.0)
    );
    assert_eq!(m.determ(), 17.0);
}

#[test]
fn submatrices() {
    let m = Matrix3x3(
        Vec3(1.0, 5.0, 0.0),
        Vec3(-3.0, 2.0, 7.0),
        Vec3(0.0, 6.0, -3.0)
    );
    assert_eq!(m.sub(0, 2), Matrix2x2(
        Vec2(-3.0, 2.0),
        Vec2(0.0, 6.0)
    ));
    let m = Matrix4x4(
        Vec4(-6.0, 1.0, 1.0, 6.0),
        Vec4(-8.0, 5.0, 8.0, 6.0),
        Vec4(-1.0, 0.0, 8.0, 2.0),
        Vec4(-7.0, 1.0, -1.0, 1.0)
    );
    assert_eq!(m.sub(2, 1), Matrix3x3(
        Vec3(-6.0, 1.0, 6.0),
        Vec3(-8.0, 8.0, 6.0),
        Vec3(-7.0, -1.0, 1.0)
    ))
}

#[test]
fn minor_and_cofactor() {
    let m = Matrix3x3(
        Vec3(3.0, 5.0, 0.0),
        Vec3(2.0, -1.0, -7.0),
        Vec3(6.0, -1.0, 5.0)
    );
    assert_eq!(m.minor(0, 0), -12.0);
    assert_eq!(m.cofactor(0, 0), -12.0);
    assert_eq!(m.minor(1, 0), 25.0);
    assert_eq!(m.cofactor(1, 0), -25.0);
}

#[test]
fn determinants() {
    let m = Matrix3x3(
        Vec3(1.0, 2.0, 6.0),
        Vec3(-5.0, 8.0, -4.0),
        Vec3(2.0, 6.0, 4.0)
    );
    assert_eq!(m.cofactor(0, 0), 56.0);
    assert_eq!(m.cofactor(0, 1), 12.0);
    assert_eq!(m.cofactor(0, 2), -46.0);
    assert_eq!(m.determ(), -196.0);

    let m = Matrix4x4(
        Vec4(-2.0, -8.0, 3.0, 5.0),
        Vec4(-3.0, 1.0, 7.0, 3.0),
        Vec4(1.0, 2.0, -9.0, 6.0),
        Vec4(-6.0, 7.0, 7.0, -9.0)
    );
    assert_eq!(m.cofactor(0, 0), 690.0);
    assert_eq!(m.cofactor(0, 1), 447.0);
    assert_eq!(m.cofactor(0, 2), 210.0);
    assert_eq!(m.cofactor(0, 3), 51.0);
    assert_eq!(m.determ(), -4071.0);
}

#[test]
fn inverses() {
    let m = Matrix4x4(
        Vec4(-5.0, 2.0, 6.0, -8.0),
        Vec4(1.0, -5.0, 1.0, 8.0),
        Vec4(7.0, 7.0, -6.0, -7.0),
        Vec4(1.0, -3.0, 7.0, 4.0)
    );
    assert_eq!(m.determ(), 532.0);
    assert_eq!(m.cofactor(2, 3), -160.0);
    assert_eq!(m.cofactor(3, 2), 105.0);
    let inverse = m.invert();
    assert_eq!(inverse, Matrix4x4(
        Vec4(0.21805, 0.45113, 0.24060, -0.04511),
        Vec4(-0.80827, -1.45677, -0.44361, 0.52068),
        Vec4(-0.07895, -0.22368, -0.05263, 0.19737),
        Vec4(-0.52256, -0.81391, -0.30075, 0.30639)
    ));
    println!("{:#?}", &m * &inverse);
}

#[test]
fn translation_matrix() {
    let transform = translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    assert_eq!(&transform * &p, point(2.0, 1.0, 7.0));

    let inv = transform.invert();
    assert_eq!(&inv * &p, point(-8.0, 7.0, 3.0));

    let v = vector(-3.0, 4.0, 5.0);
    assert_eq!(&transform * &v, v);
}

#[test]
fn scaling_matrix() {
    let transform = scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);
    assert_eq!(&transform * &p, point(-8.0, 18.0, 32.0));

    let v = vector(-4.0, 6.0, 8.0);
    assert_eq!(&transform * &v, vector(-8.0, 18.0, 32.0));

    let inv = transform.invert();
    assert_eq!(&inv * &v, vector(-2.0, 2.0, 2.0));

    let transform = scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(&transform * &p, point(-2.0, 3.0, 4.0));
}

#[test]
fn rotation_x_test() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(45.0);
    let full_quarter = rotation_x(90.0);
    assert_eq!(&half_quarter * &p, point(0.0, (2.0 as f32).sqrt() / 2.0, (2.0 as f32).sqrt() / 2.0));
    assert_eq!(&full_quarter * &p, point(0.0, 0.0, 1.0));

    let inv = half_quarter.invert();
    assert_eq!(&inv * &p, point(0.0, (2.0 as f32).sqrt() / 2.0, -(2.0 as f32).sqrt() / 2.0))
}

#[test]
fn rotation_y_test() {
    let p = point(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(45.0);
    let full_quarter = rotation_y(90.0);
    assert_eq!(&half_quarter * &p, point((2.0 as f32).sqrt() / 2.0, 0.0, (2.0 as f32).sqrt() / 2.0));
    assert_eq!(&full_quarter * &p, point(1.0, 0.0, 0.0));
}

#[test]
fn rotation_z_test() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(45.0);
    let full_quarter = rotation_z(90.0);
    assert_eq!(&half_quarter * &p, point(-(2.0 as f32).sqrt() / 2.0, (2.0 as f32).sqrt() / 2.0, 0.0));
    assert_eq!(&full_quarter * &p, point(-1.0, 0.0, 0.0));
}

#[test]
fn shearing_test() {{s}
    let p = point(2.0, 3.0, 4.0);

    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(&transform * &p, point(5.0, 3.0, 4.0));

    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(&transform * &p, point(6.0, 3.0, 4.0));
    
    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(&transform * &p, point(2.0, 5.0 , 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(&transform * &p, point(2.0, 7.0, 4.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(&transform * &p, point(2.0, 3.0, 6.0));

    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(&transform * &p, point(2.0, 3.0, 7.0));
}