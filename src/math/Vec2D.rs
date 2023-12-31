use std::ops;


#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
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