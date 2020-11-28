use crate::tuples::*;
use std::ops;
use std::f32::consts;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3(pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Copy, Clone)]
pub struct Matrix2x2(pub Vec2, pub Vec2);

pub fn shearing(xy: f32, xx: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4x4 {
    Matrix4x4(
        Vec4(1.0, xy, xx, 0.0),
        Vec4(yx, 1.0, yz, 0.0),
        Vec4(zx, zy, 1.0, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (degrees / 180.0) * consts::PI
}

pub fn rotation_x(degrees: f32) -> Matrix4x4 {
    let radians = degrees_to_radians(degrees);
    let cos = radians.cos();
    let sin = radians.sin();
    Matrix4x4(
        Vec4(1.0, 0.0, 0.0, 0.0),
        Vec4(0.0, cos, -sin, 0.0),
        Vec4(0.0, sin, cos, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

pub fn rotation_y(degrees: f32) -> Matrix4x4 {
    let radians = degrees_to_radians(degrees);
    let cos = radians.cos();
    let sin = radians.sin();
    Matrix4x4(
        Vec4(cos, 0.0, sin, 0.0),
        Vec4(0.0, 1.0, 0.0, 0.0),
        Vec4(-sin, 0.0, cos, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

pub fn rotation_z(degrees: f32) -> Matrix4x4 {
    let radians = degrees_to_radians(degrees);
    let cos = radians.cos();
    let sin = radians.sin();
    Matrix4x4(
        Vec4(cos, -sin, 0.0, 0.0),
        Vec4(sin, cos, 0.0, 0.0),
        Vec4(0.0, 0.0, 1.0, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4(
        Vec4(1.0, 0.0, 0.0, x),
        Vec4(0.0, 1.0, 0.0, y),
        Vec4(0.0, 0.0, 1.0, z),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4(
        Vec4(x, 0.0, 0.0, 0.0),
        Vec4(0.0, y, 0.0, 0.0),
        Vec4(0.0, 0.0, z, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0)
    )
}

impl Matrix2x2 {
    pub fn determ(&self) -> f32 {
        (self.0).0 * (self.1).1 - (self.0).1 * (self.1).0
    }
}

impl Matrix3x3 {
    pub fn to_vectors(&self) -> Vec<Vec<f32>> {
        let mut m_vecs: Vec<Vec<f32>> = vec![Vec::new(), Vec::new(), Vec::new()];
        let row_vecs = unsafe { tuple_to_vec(&self.0, 3) };
        for row in 0..3 {
            m_vecs[row] = unsafe { tuple_to_vec(&row_vecs[row].0, 3) };
        }
        m_vecs
    }  

    pub fn sub(&self, x: usize, y: usize) -> Matrix2x2 {
        let mut m_vecs = self.to_vectors();
        m_vecs.remove(x);
        for row in 0..2 {
            m_vecs[row].remove(y);
        }
        Matrix2x2(
            Vec2::from(&m_vecs[0]),
            Vec2::from(&m_vecs[1])
        )
    }

    pub fn minor(&self, x: usize, y: usize) -> f32 {
        self.sub(x, y).determ()
    }

    pub fn cofactor(&self, x: usize, y: usize) -> f32 {
        self.minor(x, y) * if (x + y) % 2 == 0 { 1.0 } else { -1.0 }
    }

    pub fn determ(&self) -> f32 {
        let row = unsafe { tuple_to_vec(&(self.0).0, 3) };
        let mut result = 0.0;
        for (x, row) in row.iter().enumerate() {
            result += row * self.cofactor(0, x);
        }
        result
    }
}

impl Matrix4x4 {
    pub fn sub(&self, x: usize, y: usize) -> Matrix3x3 {
        let mut m_vecs = self.to_vectors();
        m_vecs.remove(x);
        for row in 0..3 {
            m_vecs[row].remove(y);
        }
        Matrix3x3(
            Vec3::from(&m_vecs[0]),
            Vec3::from(&m_vecs[1]),
            Vec3::from(&m_vecs[2])
        )
    }

    pub fn to_vectors(&self) -> Vec<Vec<f32>> {
        let mut m_vecs: Vec<Vec<f32>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let row_vecs = unsafe { tuple_to_vec(&self.0, 4) };
        for row in 0..4 {
            m_vecs[row] = unsafe { tuple_to_vec(&row_vecs[row].0, 4) };
        }
        m_vecs
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut m_vecs = self.to_vectors();
        let original_m_vecs = m_vecs.clone();
        for row in 0..4 {
            for col in 0..4 {
                m_vecs[row][col] = original_m_vecs[col][row];
            }
        }
        Matrix4x4::from(&m_vecs)
    }

    pub fn ident() -> Matrix4x4 {
        Matrix4x4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn from(v: &Vec<Vec<f32>>) -> Matrix4x4 {
        Matrix4x4(
            Vec4::from(&v[0]),
            Vec4::from(&v[1]),
            Vec4::from(&v[2]),
            Vec4::from(&v[3])
        )
    }

    pub fn minor(&self, x: usize, y: usize) -> f32 {
        self.sub(x, y).determ()
    }

    pub fn cofactor(&self, x: usize, y: usize) -> f32 {
        self.minor(x, y) * if (x + y) % 2 == 0 { 1.0 } else { -1.0 }
    }

    pub fn determ(&self) -> f32 {
        let row = unsafe { tuple_to_vec(&(self.0).0, 4) };
        let mut result = 0.0;
        for (x, row) in row.iter().enumerate() {
            result += row * self.cofactor(0, x);
        }
        result
    }

    pub fn invert(&self) -> Matrix4x4 {
        let determ = self.determ();
        if determ == 0.0 {
            panic!("Cannot invert a matrix with a determinant of 0")
        }
        let mut m_vecs = self.to_vectors();
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m_vecs[col][row] = c / determ; 
            }
        }
        Matrix4x4::from(&m_vecs)
    }
}

impl ops::Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: &Matrix4x4) -> Matrix4x4 {
        let mut m_vecs = self.to_vectors();
        let m_original_vecs = m_vecs.clone();
        let other_vecs = other.to_vectors();
        for row in 0..4 {
            for col in 0..4 {
                m_vecs[row][col] = m_original_vecs[row][0] * other_vecs[0][col]
                    + m_original_vecs[row][1] * other_vecs[1][col]
                    + m_original_vecs[row][2] * other_vecs[2][col]
                    + m_original_vecs[row][3] * other_vecs[3][col]; 
            }
        }
        Matrix4x4::from(&m_vecs)
    }
}

impl ops::Mul<&Vec4> for &Matrix4x4 {
    type Output = Vec4;

    fn mul(self, other: &Vec4) -> Vec4 {
        let m_vecs = self.to_vectors();
        let mut result: Vec<f32> = Vec::new();
        for y in 0..4 {
            let row = &m_vecs[y];
            result.push(
                row[0] * other.0 
                    + row[1] * other.1
                    + row[2] * other.2
                    + row[3] * other.3
            );
        }
        Vec4::from(&result)
    }
}

impl PartialEq<Matrix4x4> for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
    }
}

impl PartialEq<Matrix2x2> for Matrix2x2 {
    fn eq(&self, other: &Matrix2x2) -> bool {
        self.0 == other.0
            && self.1 == other.1
    }
}

impl PartialEq<Matrix3x3> for Matrix3x3 {
    fn eq(&self, other: &Matrix3x3) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
    }
}