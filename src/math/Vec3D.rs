use std::ops;
use crate::math::Vec4D::Vec4D;

#[derive(Clone, Copy, Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
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
    pub fn newc(c: f64) -> Vec3D {
        Vec3D {x: c, y: c, z: c}
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
    pub fn make_point_4d(self) -> Vec4D {
        Vec4D {x: self.x, y: self.y, z: self.z, w: 1.0}
    }
}