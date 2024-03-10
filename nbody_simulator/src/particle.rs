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

            let v = softened_gravitational_force(&self, p, gravity, epsilon, scale);

            velocity = velocity + v;
        }

        return velocity;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector2::Vector2; // Make sure this path is correct based on your module structure

    // #[test]
    // fn test_particle_creation() {
    //     let mass = 1.0;
    //     let diameter = 1.0;
    //     let position = Vector2 { x: 0.0, y: 0.0 };
    //     let velocity = Vector2 { x: 1.0, y: 1.0 };
    //     let color = [255.0, 255.0, 255.0];

    //     let particle = Particle::new(mass, diameter, position, velocity, color);

    //     assert_eq!(particle.mass, mass);
    //     assert_eq!(particle.diameter, diameter);
    //     assert_eq!(particle.position, position);
    //     assert_eq!(particle.velocity, velocity);
    //     assert_eq!(particle.color, color);
    // }

    // // This test might have a degree of randomness, consider using fixed values for critical tests
    // #[test]
    // fn test_random_particle_creation() {
    //     let world_size = Vector2 { x: 100.0, y: 100.0 };
    //     let mass = 100.0;
    //     let mass_deviation = 0.60;
    //     let diameter = 1.0;

    //     let particle = Particle::new_rand(world_size, mass, mass_deviation, diameter);

    //     assert!(particle.mass >= mass * ((100.0 - mass_deviation) / 100.0));
    //     assert!(particle.mass <= mass * ((100.0 + mass_deviation) / 100.0));
    //     // Additional checks can be added for position and other properties
    // }

    // #[test]
    // fn test_next_position() {
    //     let particle = Particle {
    //         id: 0,
    //         mass: 1.0,
    //         diameter: 1.0,
    //         position: Vector2 { x: 0.0, y: 0.0 },
    //         velocity: Vector2 { x: 1.0, y: 1.0 },
    //         color: [255.0, 255.0, 255.0],
    //     };

    //     let next_position = particle.next_position();
    //     assert_eq!(next_position, Vector2 { x: 1.0, y: 1.0 });
    // }

    // #[test]
    // fn test_next_velocity() {
    //     let particle = Particle::new(
    //         1.0,
    //         1.0,
    //         Vector2 { x: 0.0, y: 0.0 },
    //         Vector2 { x: 0.0, y: 0.0 },
    //         [255.0, 255.0, 255.0],
    //     );

    //     let other_particle = Particle::new(
    //         1.0,
    //         1.0,
    //         Vector2 { x: 3.0, y: 4.0 },
    //         Vector2 { x: 0.0, y: 0.0 },
    //         [255.0, 255.0, 255.0],
    //     );
    //     let particles = vec![particle, other_particle];
    //     let gravity = 6.67430e-11;
    //     let epsilon = 0.01;
    //     let scale = 1.0;

    //     let next_velocity = particles[0].next_velocity(&particles, gravity, epsilon, scale);
    //     assert!(next_velocity.x < 5.0 && next_velocity.y < 5.0);

    //     // The assertion for next_velocity will depend on the expected outcome based on the gravitational force calculation.
    //     // Since the actual outcome will depend on the softened_gravitational_force implementation, you'll need to adjust
    //     // the expected values according to that function's behavior.
    // }

    #[test]
    fn real_scenario() {
        let mut particles: Vec<Particle> = (0..4)
            .map(|_| Particle::new_rand(Vector2 { x: 700.0, y: 700.0 }, 100.0, 0.0, 10.0))
            .collect();

        println!("{:?}", particles);

        let gravity = 6.6743e-11;
        let epsilon = 5.84e9;
        let scale = 1e16;

        for i in 0..particles.len() {
            particles[i].velocity = particles[i].next_velocity(&particles, gravity, epsilon, scale);
            particles[i].position = particles[i].next_position();
        }

        println!("{:?}", particles);
    }
}
