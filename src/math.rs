use std::ops;

#[derive(Clone)]
#[derive(Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Vec4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

// Matrix 4x4
// Матрица 4x4
#[derive(Clone)]
#[derive(Debug)]
pub struct Matrix {
    pub set: [[f64; 4]; 4]
}

impl ops::Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Vec2D {
        Vec2D {x: -self.x, y: -self.y}
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, check: &Vec2D) -> bool {
        self.x == check.x && self.y == check.y
    }
}

impl ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, _rhs: Vec2D) -> Vec2D {
        Vec2D {x: _rhs.x + self.x, y: _rhs.y + self.y}
    }
}

impl ops::Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, _rhs: Vec2D) -> Vec2D {
        Vec2D {x: self.x - _rhs.x, y: self.y - _rhs.y}
    }
}

impl ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, _rhs: f64) -> Vec2D {
        Vec2D {x: _rhs * self.x, y: _rhs * self.y}
    }
}

impl ops::Div<f64> for Vec2D {
    type Output = Vec2D;

    fn div(self, _rhs: f64) -> Vec2D {
        if _rhs != 0.0{
            Vec2D {x: self.x / _rhs, y: self.y / _rhs}
        } else {
            panic!("impl ops::Div<f64> for Vec2D: Division by zero.")
        }
    }
}

impl Vec2D {
    pub fn new() -> Vec2D {
        Vec2D {x: 0.0, y: 0.0}
    }
    pub fn sqr_abs(self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn abs(self) -> f64 {
        self.sqr_abs().sqrt()
    }
    pub fn dot(self, vector: &Vec2D) -> f64 {
        self.x * vector.x + self.y * vector.y
    }
    pub fn normalized(self) -> Vec2D {
        let vec_abs: f64 = self.clone().sqr_abs();

        if vec_abs != 0.0 {
            self.clone() / self.abs()
        } else {
            panic!("Impl Vec2D: fn normalized(): Division by zero.")
        }
    }
}

impl ops::Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Vec3D {
        Vec3D {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, check: &Vec3D) -> bool {
        self.x == check.x && self.y == check.y && self.z == check.z
    }
}

impl ops::Add for Vec3D {
    type Output = Vec3D;

    fn add(self, _rhs: Vec3D) -> Vec3D {
        Vec3D {x: _rhs.x + self.x, y: _rhs.y + self.y, z: _rhs.z + self.z}
    }
}

impl ops::Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, _rhs: Vec3D) -> Vec3D {
        Vec3D {x: self.x - _rhs.x, y: _rhs.y - self.y, z: self.z - _rhs.z}
    }
}

impl ops::Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, _rhs: f64) -> Vec3D {
        Vec3D {x: _rhs * self.x, y: _rhs * self.y, z: _rhs * self.z}
    }
}

impl ops::Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, _rhs: f64) -> Vec3D {
        if _rhs != 0.0 {
            Vec3D {x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs}
        } else {
            panic!("impl ops::Div<f64> for Vec3D: division by zero.")
        }
    }
}

impl Vec3D {
    pub fn new() -> Vec3D {
        Vec3D {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn sqr_abs(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn abs(self) -> f64 {
        self.sqr_abs().sqrt()
    }
    pub fn dot(self, vector: &Vec3D) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }
    pub fn normalized(self) -> Vec3D {
        let vec_abs: f64 = self.clone().sqr_abs();

        if vec_abs != 0.0 {
            self.clone() / self.abs()
        } else {
            panic!("Impl Vec3D: fn normalized(): Division by zero.")
        }
    
    }
    pub fn cross(self, vector: &Vec3D) -> Vec3D{
        Vec3D {x: self.y * vector.z - self.z * vector.y,
               y: self.z * vector.x - self.x * vector.z,
               z: self.x * vector.y - self.y * vector.x}
    }
}

impl ops::Neg for Vec4D {
    type Output = Vec4D;

    fn neg(self) -> Vec4D {
        Vec4D {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

impl PartialEq for Vec4D {
    fn eq(&self, check: &Vec4D) -> bool {
        self.x == check.x && self.y == check.y && self.z == check.z && self.w == check.w
    }
}

impl ops::Add for Vec4D {
    type Output = Vec4D;

    fn add(self, _rhs: Vec4D) -> Vec4D {
        Vec4D {x: _rhs.x + self.x, y: _rhs.y + self.y, z: _rhs.z + self.z, w: _rhs.w + self.w}
    }
}

impl ops::Sub for Vec4D {
    type Output = Vec4D;

    fn sub(self, _rhs: Vec4D) -> Vec4D {
        Vec4D {x: self.x -_rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z, w: self.w - _rhs.w}
    }
}

impl ops::Mul<f64> for Vec4D {
    type Output = Vec4D;

    fn mul(self, _rhs: f64) -> Vec4D {
        Vec4D {x: _rhs * self.x, y: _rhs * self.y, z: _rhs * self.z, w: _rhs * self.w}
    }
}

impl ops::Div<f64> for Vec4D {
    type Output = Vec4D;

    fn div(self, _rhs: f64) -> Vec4D {
        if _rhs != 0.0 {
            Vec4D {x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs, w: self.w / _rhs}
        } else {
            panic!("impl ops::Div<f64> for Vec4D: division by zero.")
        }
    }
}

impl Vec4D {
    pub fn new() -> Vec4D {
        Vec4D {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }
    pub fn sqr_abs(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    pub fn abs(self) -> f64 {
        self.sqr_abs().sqrt()
    }
    pub fn dot(self, vector: &Vec4D) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z + self.w * vector.w
    }
    pub fn normalized(self) -> Vec4D {
        let vec_abs: f64 = self.clone().sqr_abs();

        if vec_abs != 0.0 {
            self.clone() / self.abs()
        } else {
            panic!("Impl Vec4D: fn normalized(): Division by zero.")
        }
    
    }
}
enum Vectors {
    Vec2D,
    Vec3D,
    Vec4D
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        let mut result = Matrix::zero();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.set[i][j] += self.set[i][k] * _rhs.set[k][j]
                }
            }
        }
        result
    }
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
}