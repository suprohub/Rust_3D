use crate::math::Vec3D::Vec3D;
use crate::math::Vec4D::Vec4D;

// Matrix 4x4
// Матрица 4x4
#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    pub set: [[f64; 4]; 4]
}

impl std::ops::Mul<Vec4D> for Matrix {
    type Output = Vec4D;

    fn mul(self, _rhs: Vec4D) -> Vec4D {
        Vec4D {
            x: self.set[0][0] * _rhs.x + self.set[0][1] * _rhs.y + self.set[0][2] * _rhs.z + self.set[0][3] * _rhs.w,
            y: self.set[1][0] * _rhs.x + self.set[1][1] * _rhs.y + self.set[1][2] * _rhs.z + self.set[1][3] * _rhs.w,
            z: self.set[2][0] * _rhs.x + self.set[2][1] * _rhs.y + self.set[2][2] * _rhs.z + self.set[2][3] * _rhs.w,
            w: self.set[3][0] * _rhs.x + self.set[3][1] * _rhs.y + self.set[3][2] * _rhs.z + self.set[3][3] * _rhs.w
        }
    }
}

impl std::ops::Mul<Vec3D> for Matrix {
    type Output = Vec3D;

    fn mul(self, _rhs: Vec3D) -> Vec3D {
        Vec3D {
            x: self.set[0][0] * _rhs.x + self.set[0][1] * _rhs.y + self.set[0][2] * _rhs.z,
            y: self.set[1][0] * _rhs.x + self.set[1][1] * _rhs.y + self.set[1][2] * _rhs.z,
            z: self.set[2][0] * _rhs.x + self.set[2][1] * _rhs.y + self.set[2][2] * _rhs.z
        }
    }
}

impl Matrix {
    pub fn new(self) -> Matrix {
        Matrix {set: [[0.0, 0.0, 0.0, 0.0],
                      [0.0, 0.0, 0.0, 0.0], 
                      [0.0, 0.0, 0.0, 0.0], 
                      [0.0, 0.0, 0.0, 0.0]]}
    }
    pub fn identity() -> Matrix {
        Matrix {set: [[1.0, 0.0, 0.0, 0.0],
                      [0.0, 1.0, 0.0, 0.0], 
                      [0.0, 0.0, 1.0, 0.0], 
                      [0.0, 0.0, 0.0, 1.0]]}
    }
    pub fn constant(c: f64) -> Matrix {
        Matrix {set: [[c, c, c, c],
                      [c, c, c, c], 
                      [c, c, c, c], 
                      [c, c, c, c]]}
    }
    pub fn scale(v: &Vec3D) -> Matrix{
        Matrix {set: [[v.x, 0.0, 0.0, 0.0],
                      [0.0, v.y, 0.0, 0.0], 
                      [0.0, 0.0, v.z, 0.0], 
                      [0.0, 0.0, 0.0, 1.0]]}
    }
    pub fn zero() -> Matrix {
        Matrix {set: [[0.0, 0.0, 0.0, 0.0],
                      [0.0, 0.0, 0.0, 0.0], 
                      [0.0, 0.0, 0.0, 0.0], 
                      [0.0, 0.0, 0.0, 0.0]]}
    }
    pub fn translation(v: &Vec3D) -> Matrix {
        Matrix {set: [[1.0, 0.0, 0.0, v.x],
                      [0.0, 1.0, 0.0, v.y], 
                      [0.0, 0.0, 1.0, v.z], 
                      [0.0, 0.0, 0.0, 1.0]]}
    }
    pub fn RotationX(rx: f64) -> Matrix {
        let c: f64 = rx.cos();
        let s: f64 = rx.sin();
        Matrix {set: [[1.0, 0.0, 0.0, 0.0],
                      [0.0, c, -s, 0.0], 
                      [0.0, s, c, 0.0], 
                      [0.0, 0.0, 0.0, 1.0]]}
    }
    //TODO: доделать повороты
}