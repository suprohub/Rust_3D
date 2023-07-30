use sfml::graphics::Color;
use std::ops;
use crate::math::Vec3D::Vec3D;
use crate::math::Vec4D::Vec4D;
use crate::math::Matrix::Matrix;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    color: Color,
    normal: Vec3D,
    p1: Vec4D,
    p2: Vec4D,
    p3: Vec4D,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {color: Color::rgb(255, 245, 194), normal: Vec3D::new(), p1: Vec4D::new(), p2: Vec4D::new(), p3: Vec4D::new()}
    }
    pub fn position(self) -> Vec3D {
        (self.p1 + self.p2 + self.p3).convert_3d() / 3.0
    }
    pub fn distance(self, vector: &Vec3D) -> f64 {
        let result: Vec3D = self.clone().p1.convert_3d() - vector.clone();
        self.normal.dot(&result)
    }
    pub fn calc_normalm(mut self) {
        let clone: Triangle = self.clone();
        let v1: Vec3D = (clone.p3 - clone.p1).convert_3d();
        let cross: Vec3D = (self.p2 - self.p1).convert_3d().cross(&v1);

        if cross.clone().sqr_abs() != 0.0 {
            self.normal = cross.normalized();
        } else {
            self.normal = Vec3D::newc(0.0);
        }
        
    }
    pub fn calc_normal(self) -> Vec3D {
        let clone: Triangle = self.clone();
        let v1: Vec3D = (clone.p3 - clone.p1).convert_3d();
        let cross: Vec3D = (self.p2 - self.p1).convert_3d().cross(&v1);

        if cross.clone().sqr_abs() != 0.0 {
            cross.normalized()
        } else {
            Vec3D::newc(0.0)
        }
        
    }
    pub fn isPointInside(self, point: &Vec3D) -> bool {
        let dot1: f64 = (*point - self.p1.convert_3d().cross(&(self.p2 - self.p1).convert_3d())).dot(&self.normal);
        let dot2: f64 = (*point - self.p2.convert_3d().cross(&(self.p3 - self.p2).convert_3d())).dot(&self.normal);
        let dot3: f64 = (*point - self.p3.convert_3d().cross(&(self.p1 - self.p3).convert_3d())).dot(&self.normal);

        (dot1 >= 0.0 && dot2 >= 0.0 && dot3 >= 0.0) || (dot1 <= 0.0 && dot2 <= 0.0 && dot3 <= 0.0)
    }
}

impl ops::Mul<Matrix> for Triangle {
    type Output = Triangle;

    fn mul(self, _rhs: Matrix) -> Triangle {
        Triangle { color: self.color, normal: Vec3D::newc(0.0), p1: _rhs.clone() * self.p1, p2: _rhs.clone() * self.p2, p3: _rhs * self.p3 }
    }
}