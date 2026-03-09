use std::fmt;

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Vector2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Vector2 {
        return Vector2 { x, y };
    }

    pub fn distance(&self, other: &Vector2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Vector2::new(0.0, 0.0);
        }
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn scale(&self, factor: f32) -> Vector2 {
        Vector2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, pos: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
}
impl std::ops::Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, pos: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f32 = 1e-4;
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}
