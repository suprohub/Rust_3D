use crate::triangle::Triangle;
use sfml::graphics::Color;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mesh {
    tris: Vec<Triangle>,
    color: Color,
    visible: bool,
    
}