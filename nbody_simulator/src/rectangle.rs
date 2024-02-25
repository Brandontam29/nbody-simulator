use crate::vector2::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub position: Vector2,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(position: Vector2, width: f64, height: f64) -> Rectangle {
        Rectangle {
            position,
            width,
            height,
        }
    }
}
