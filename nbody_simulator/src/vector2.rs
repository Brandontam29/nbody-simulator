use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 { x, y };
    }

    pub fn distance(&self, other: &Vector2) -> f64 {
        let a = (self.x + other.x).powf(2.0) + (self.y + other.y).powf(2.0);
        let distance = a.sqrt();
        return distance;
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let mag = self.magnitude();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn scale(&self, factor: f64) -> Vector2 {
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
        const EPSILON: f64 = 1e-6;
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}
