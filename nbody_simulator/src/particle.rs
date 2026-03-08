use std::fmt::{self, Debug};

use crate::utils::calculation_utils::softened_gravitational_force;
use crate::vector2::Vector2;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]

pub struct Particle {
    id: i32,
    pub mass: f64,
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

        let m = ((rng.gen::<f64>() - 0.5) * mass_deviation / 100.0 * mass * 2.0) + mass;

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
                continue;
            }

            let v = softened_gravitational_force(p, self, gravity, epsilon, scale);

            velocity = velocity + v;
        }

        return velocity;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let mass = 1.0;
        let diameter = 1.0;
        let position = Vector2 { x: 0.0, y: 0.0 };
        let velocity = Vector2 { x: 1.0, y: 1.0 };
        let color = [255.0, 255.0, 255.0];

        let particle = Particle::new(mass, diameter, position, velocity, color);

        assert_eq!(particle.mass, mass);
        assert_eq!(particle.diameter, diameter);
        assert_eq!(particle.position, position);
        assert_eq!(particle.velocity, velocity);
        assert_eq!(particle.color, color);
    }

    #[test]
    fn test_random_particle_creation() {
        let world_size = Vector2 { x: 100.0, y: 100.0 };
        let mass = 100.0;
        let mass_deviation = 0.60;
        let diameter = 1.0;

        let particle = Particle::new_rand(world_size, mass, mass_deviation, diameter);

        // mass_deviation 0.60 means +/- 0.6%? 
        // Logic was: ((rng.gen::<f64>() - 0.5) * mass_deviation / 100.0 * mass * 2.0) + mass
        // If gen is 1.0: 0.5 * 0.6 / 100 * 100 * 2 + 100 = 0.6 + 100 = 100.6
        // If gen is 0.0: -0.5 * 0.6 / 100 * 100 * 2 + 100 = -0.6 + 100 = 99.4
        assert!(particle.mass >= mass * (1.0 - mass_deviation / 100.0) - 1e-6);
        assert!(particle.mass <= mass * (1.0 + mass_deviation / 100.0) + 1e-6);
    }

    #[test]
    fn test_next_position() {
        let particle = Particle {
            id: 0,
            mass: 1.0,
            diameter: 1.0,
            position: Vector2 { x: 10.0, y: 10.0 },
            velocity: Vector2 { x: 1.0, y: 2.0 },
            color: [255.0, 255.0, 255.0],
        };

        let next_position = particle.next_position();
        assert_eq!(next_position.x, 11.0);
        assert_eq!(next_position.y, 12.0);
    }

    #[test]
    fn test_next_velocity() {
        let p1 = Particle::new(
            1.0e10,
            1.0,
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            [255.0, 255.0, 255.0],
        );

        let p2 = Particle::new(
            1.0,
            1.0,
            Vector2::new(10.0, 0.0),
            Vector2::new(0.0, 0.0),
            [255.0, 255.0, 255.0],
        );
        
        let particles = vec![p1, p2];
        let gravity = 1.0;
        let epsilon = 0.0;
        let scale = 1.0;

        let next_velocity = p2.next_velocity(&particles, gravity, epsilon, scale);
        
        // From previous test in calculation_utils, expected acceleration is (-1e8, 0)
        assert!((next_velocity.x - (-1.0e8)).abs() < 1e-6);
        assert_eq!(next_velocity.y, 0.0);
    }

    #[test]
    fn real_scenario() {
        let mut particles: Vec<Particle> = (0..4)
            .map(|_| Particle::new_rand(Vector2 { x: 700.0, y: 700.0 }, 100.0, 0.0, 10.0))
            .collect();

        let gravity = 6.6743e-11;
        let epsilon = 5.84e9;
        let scale = 1e16;

        for i in 0..particles.len() {
            particles[i].velocity = particles[i].next_velocity(&particles, gravity, epsilon, scale);
            particles[i].position = particles[i].next_position();
        }
    }
}
