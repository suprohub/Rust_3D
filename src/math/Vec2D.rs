use std::ops;

struct Vec2D {
    x: f64,
    y: f64,
}

impl Vec2D {
    fn new(self) -> Vec2D {
        Vec2D{x: 0.0, y: 0.0}
    }
    fn sqr_abs(self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    fn abs(self) -> f64 {
        self.sqr_abs().sqrt()
    }
    fn dot(self, vector: &Vec2D) -> f64 {
        self.x * vector.x + self.y * vector.y
    }
    fn normalized(self) -> Vec2D {

    }
}