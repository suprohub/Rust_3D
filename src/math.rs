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

pub struct Vec4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
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
        Vec2D {x: _rhs.x - self.x, y: _rhs.y - self.y}
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
    pub fn new(self) -> Vec2D {
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
        Vec3D {x: _rhs.x - self.x, y: _rhs.y - self.y, z: _rhs.z - self.z}
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
    pub fn new(self) -> Vec3D {
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
        Vec4D {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl PartialEq for Vec4D {
    fn eq(&self, check: &Vec4D) -> bool {
        self.x == check.x && self.y == check.y && self.z == check.z
    }
}

impl ops::Add for Vec4D {
    type Output = Vec4D;

    fn add(self, _rhs: Vec4D) -> Vec4D {
        Vec4D {x: _rhs.x + self.x, y: _rhs.y + self.y, z: _rhs.z + self.z}
    }
}

impl ops::Sub for Vec4D {
    type Output = Vec4D;

    fn sub(self, _rhs: Vec4D) -> Vec4D {
        Vec4D {x: _rhs.x - self.x, y: _rhs.y - self.y, z: _rhs.z - self.z}
    }
}

impl ops::Mul<f64> for Vec4D {
    type Output = Vec4D;

    fn mul(self, _rhs: f64) -> Vec4D {
        Vec4D {x: _rhs * self.x, y: _rhs * self.y, z: _rhs * self.z}
    }
}

impl ops::Div<f64> for Vec4D {
    type Output = Vec4D;

    fn div(self, _rhs: f64) -> Vec4D {
        if _rhs != 0.0 {
            Vec4D {x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs}
        } else {
            panic!("impl ops::Div<f64> for Vec4D: division by zero.")
        }
    }
}

impl Vec4D {
    pub fn new(self) -> Vec4D {
        Vec4D {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn sqr_abs(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn abs(self) -> f64 {
        self.sqr_abs().sqrt()
    }
    pub fn dot(self, vector: &Vec4D) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }
    pub fn normalized(self) -> Vec4D {
        let vec_abs: f64 = self.clone().sqr_abs();

        if vec_abs != 0.0 {
            self.clone() / self.abs()
        } else {
            panic!("Impl Vec4D: fn normalized(): Division by zero.")
        }
    
    }
    pub fn cross(self, vector: &Vec4D) -> Vec4D{
        Vec4D {x: self.y * vector.z - self.z * vector.y,
               y: self.z * vector.x - self.x * vector.z,
               z: self.x * vector.y - self.y * vector.x}
    }
}