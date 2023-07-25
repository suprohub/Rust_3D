use std::ops;
use crate::math::Vec3D::Vec3D;

#[derive(Clone, Copy, Debug)]
pub struct Vec4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
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
    pub fn newc(c: f64) -> Vec4D {
        Vec4D {x: c, y: c, z: c, w: c}
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
    pub fn convert_3d(self) -> Vec3D {
        Vec3D {x: self.x, y: self.y, z: self.z}
    }
    pub fn convert_3dv(vector: Vec4D) -> Vec3D {
        Vec3D {x: vector.x, y: vector.y, z: vector.z}
    }
}