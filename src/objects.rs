use sfml::graphics::Color;

struct Triangle {
    color: Color::rgb,
    p1: Vec4D,
    p2: Vec4D,
    p3: Vec4D,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {color: Color(255, 245, 194), p1: Vec4D::new(), p2: Vec4D::new(), p3: Vec4D::new()}
    }
    pub fn position(self) -> Vec3D {
        (self.p1 + self.p2 + self.p3).convert_3d() / 3
    }
    pub fn norm(self) -> Vec3D {
        // Implement
    }
    pub fn distance(self, vector: &Vec3D) -> f64 {
        self.norm().dot(p1.convert_3d() - vector)
    }
    pub fn calculate_normal(self) -> Vec3D {
        // неуспел сделать
    }
}