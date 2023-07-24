use sfml::graphics::Color;

struct Triangle {
    color: Color::rgb,
    p1: Vec4D,
    p2: Vec4D,
    p3: Vec4D,
}

impl Triangle {
    fn new() -> Triangle {
        Triangle {color: Color(255, 245, 194), p1: Vec4D::new(), p2: Vec4D::new(), p3: Vec4D::new()}
    }
}