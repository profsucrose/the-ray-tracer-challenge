use std::ops;
use std::slice;

#[derive(Debug, Copy, Clone)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

pub fn color(r: f32, g: f32, b: f32) -> Vec4 {
    Vec4(r, g, b, 0.0)
}

pub fn point(x: f32, y: f32, z: f32) -> Vec4 {
    Vec4(x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> Vec4 {
    Vec4(x, y, z, 0.0)
}

pub unsafe fn tuple_to_vec<T>(first_in_tuple: &T, tuple_len: usize) -> Vec<T> 
    where T: Clone 
{
    let ptr = first_in_tuple as *const T;
    let slice = slice::from_raw_parts(ptr, tuple_len);
    let mut v = Vec::new();
    for item in slice.iter() {
        v.push(item.clone());
    }
    v
}

// account for round-off error in floating-point subtraction
static EPSILON: f32 = 0.00001;
pub fn fequals(f1: f32, f2: f32) -> bool {
    (f1 - f2) < EPSILON
}

// Vec4 implementations

impl Vec4 {
    pub fn reflect(&self, normal: &Vec4) -> Vec4 {
        self - &(&(normal * 2.0) * self.dot(normal))
    }

    pub fn mag(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec4 {
        let mag = self.mag();
        Vec4(self.0 / mag, self.1 / mag, self.2 / mag, self.3 / mag)
    }

    pub fn dot(self, other: &Vec4) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }

    pub fn clamp(&self, limit: f32) -> Vec4 {
        Vec4(self.0 * limit, self.1 * limit, self.2 * limit, self.3 * limit)
    }

    pub fn from(vec: &Vec<f32>) -> Vec4 {
        Vec4(vec[0], vec[1], vec[2], vec[3])
    }
}

impl Vec3 {
    pub fn from(vec: &Vec<f32>) -> Vec3 {
        Vec3(vec[0], vec[1], vec[2])
    }
}

impl Vec2 {
    pub fn from(vec: &Vec<f32>) -> Vec2 {
        Vec2(vec[0], vec[1])
    }
}

// &Vec4 + &Vec4
impl<'a> ops::Add<&'a Vec4> for &'a Vec4 {
    type Output = Vec4;

    fn add(self, other: &'a Vec4) -> Vec4 {
        if self.3 + other.3 == 2.0 {
            panic!("Attempted to add two points, w component cannot be 2.0")
        }
        Vec4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}

// &Vec4 - &Vec4
impl<'a> ops::Sub<&'a Vec4> for &'a Vec4 {
    type Output = Vec4;

    fn sub(self, other: &'a Vec4) -> Vec4 {
        Vec4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}

// &Vec4 * f32
impl<'a> ops::Mul<f32> for &'a Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: f32) -> Vec4 {
        Vec4(self.0 * scalar, self.1 * scalar, self.2 * scalar, self.3 * scalar)
    }
}

// &Vec4 * &Vec4 (for colors)
impl<'a> ops::Mul<&'a Vec4> for &'a Vec4 {
    type Output = Vec4;

    fn mul(self, other: &'a Vec4) -> Vec4 {
        Vec4(self.0 * other.0, self.1 * other.1, self.2 * other.2, self.3 * other.3)
    }
}

// -Vec4
impl ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec4(-self.0, -self.1, -self.2, -self.3)
    }
}

// Vec4 == Vec4
impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {

        fequals(self.0, other.0)
            && fequals(self.1, other.1)
            && fequals(self.2, other.2)
            && fequals(self.3, other.3)
    }
}

// Vec3 implementations

// &Vec3 + &Vec3
impl<'a> ops::Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// &Vec3 - &Vec3
impl<'a> ops::Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// &Vec3 * &f32
impl<'a> ops::Mul<&'a f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: &'a f32) -> Vec3 {
        Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

// -Vec3
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Vec3 == Vec3
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        fequals(self.0, other.0)
            && fequals(self.1, other.1)
            && fequals(self.2, other.2)
    }
}

// Vec2 implementations

// &Vec2 + &Vec2
impl<'a> ops::Add<&'a Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: &'a Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

// &Vec2 - &Vec2
impl<'a> ops::Sub<&'a Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn sub(self, other: &'a Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

// &Vec2 * &f32
impl<'a> ops::Mul<&'a f32> for &'a Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: &'a f32) -> Vec2 {
        Vec2(self.0 * scalar, self.1 * scalar)
    }
}

// -Vec2
impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2(-self.0, -self.1)
    }
}

// Vec2 == Vec2
impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        fequals(self.0, other.0)
            && fequals(self.1, other.1)
    }
}