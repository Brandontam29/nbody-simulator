use crate::vector2::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(position: Vector2, width: f32, height: f32) -> Rectangle {
        Rectangle {
            position,
            width,
            height,
        }
    }
}
