use std::fmt::{self, Debug};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 { x, y };
    }

    pub fn distance(&self, other: Vector2) -> f64 {
        let a = (self.x + other.x).powf(2.0) + (self.y + other.y).powf(2.0);
        let distance = a.sqrt();
        return distance;
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalize(&self) -> Vector2 {
        let mag = self.magnitude();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn scale(&self, factor: f64) -> Vector2 {
        Vector2 {
            x: self.x * factor,
            y: self.y * factor,
        }
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]

pub struct Particle {
    id: i32,
    mass: f64,
    diameter: f64,
    pub position: Vector2,
    pub velocity: Vector2,
    color: [f32; 3],
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Particle {{ Mass: {:.2}, Position: {}, Velocity: {} }}",
            self.mass, self.position, self.velocity
        )
    }
}

impl Particle {
    pub fn new(
        mass: f64,
        diameter: f64,
        position: Vector2,
        velocity: Vector2,
        color: [f32; 3],
    ) -> Particle {
        let mut rng = rand::thread_rng();
        let id: i32 = rng.gen();

        return Particle {
            id,
            mass,
            diameter,
            position,
            velocity,
            color,
        };
    }
    pub fn new_rand(
        world_size: Vector2,
        mass: f64,
        mass_deviation: f64,
        diameter: f64,
    ) -> Particle {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();

        let m = ((rng.gen::<f64>() - 0.5) * mass_deviation / 100.0 * mass * 2.0).floor() + mass;

        let d = diameter * m / mass; //diameter based on mass

        let position = Vector2 {
            x: rng.gen::<f64>() * world_size.x,
            y: rng.gen::<f64>() * world_size.y,
        };

        let velocity = Vector2 { x: 0.0, y: 0.0 };

        let color = [
            rng.gen::<f32>() * 255.0,
            rng.gen::<f32>() * 255.0,
            rng.gen::<f32>() * 255.0,
        ];

        return Particle {
            id,
            mass: m,
            diameter: d,
            position,
            velocity,
            color,
        };
    }

    pub fn next_position(&self) -> Vector2 {
        return self.position + self.velocity;
    }

    pub fn next_velocity(
        &self,
        particles: &Vec<Particle>,
        gravity: f64,
        epsilon: f64,
        scale: f64,
    ) -> Vector2 {
        let mut velocity = self.velocity;

        for p in particles {
            if self.id == p.id {
                break;
            }

            let mut v = calculate_softened_gravitational_force(&self, p, gravity, epsilon);

            v = v.scale(scale / self.mass);

            velocity = velocity + v;
        }

        return velocity;
    }
}

pub fn calculate_softened_gravitational_force(
    p1: &Particle,
    p2: &Particle,
    gravity: f64,
    epsilon: f64,
) -> Vector2 {
    let distance_vector = p1.position - p2.position;
    let r = distance_vector.magnitude();
    let force_magnitude = gravity * p1.mass * p2.mass / (r.powi(2) + epsilon.powi(2)).powf(1.5);
    let force_direction = distance_vector.normalize();

    // The force vector is in the direction of the distance vector, scaled by the force magnitude
    let result = force_direction.scale(-force_magnitude);

    return result;
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_real_scenario() {
//         let mut particles: Vec<Particle> = (0..2)
//             .map(|_| Particle::new_rand(Vector2 { x: 500.0, y: 500.0 }))
//             .collect();

//         for i in 0..particles.len() {
//             particles[i].velocity = particles[i].next_velocity(&particles);
//             particles[i].position = particles[i].next_position();
//         }
//     }
// }
