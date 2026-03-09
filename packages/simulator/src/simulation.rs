use crate::particle::Particle;
use crate::quad_tree::QuadTree;
use crate::rectangle::Rectangle;
use crate::vector2::Vector2;

#[derive(Debug)]
pub struct Simulation {
    pub positions_x: Vec<f32>,
    pub positions_y: Vec<f32>,
    pub velocities_x: Vec<f32>,
    pub velocities_y: Vec<f32>,
    pub masses: Vec<f32>,
    pub diameters: Vec<f32>,
    pub colors: Vec<f32>, // Flat [r, g, b, r, g, b, ...]
    pub count: usize,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>) -> Simulation {
        let count = particles.len();
        let mut positions_x = Vec::with_capacity(count);
        let mut positions_y = Vec::with_capacity(count);
        let mut velocities_x = Vec::with_capacity(count);
        let mut velocities_y = Vec::with_capacity(count);
        let mut masses = Vec::with_capacity(count);
        let mut diameters = Vec::with_capacity(count);
        let mut colors = Vec::with_capacity(count * 3);

        for p in particles {
            positions_x.push(p.position.x);
            positions_y.push(p.position.y);
            velocities_x.push(p.velocity.x);
            velocities_y.push(p.velocity.y);
            masses.push(p.mass);
            diameters.push(p.diameter);
            colors.push(p.color_r);
            colors.push(p.color_g);
            colors.push(p.color_b);
        }

        Simulation {
            positions_x,
            positions_y,
            velocities_x,
            velocities_y,
            masses,
            diameters,
            colors,
            count,
        }
    }

    pub fn step(&mut self, world_size: Vector2, gravity: f32, epsilon: f32, scale: f32) {
        let boundary = Rectangle::new(Vector2::new(0.0, 0.0), world_size.x, world_size.y);
        let mut q = QuadTree::new(boundary);
        
        for i in 0..self.count {
            q.insert(i, &self.positions_x, &self.positions_y, &self.masses);
        }

        for i in 0..self.count {
            let p_pos = Vector2::new(self.positions_x[i], self.positions_y[i]);
            let acceleration = q.compute_force(
                p_pos,
                gravity,
                epsilon,
                scale,
                &self.positions_x,
                &self.positions_y,
                &self.masses,
            );
            
            self.velocities_x[i] += acceleration.x;
            self.velocities_y[i] += acceleration.y;
            self.positions_x[i] += self.velocities_x[i];
            self.positions_y[i] += self.velocities_y[i];
            
            // Auto-fix: Reset particles with NaN/Inf
            if self.positions_x[i].is_nan() || self.positions_x[i].is_infinite() {
                 self.positions_x[i] = 0.0;
                 self.positions_y[i] = 0.0;
                 self.velocities_x[i] = 0.0;
                 self.velocities_y[i] = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector2::Vector2;

    #[test]
    fn test_simulation_step() {
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

        let mut sim = Simulation::new(vec![p1, p2]);
        let world_size = Vector2::new(100.0, 100.0);
        let gravity = 1.0;
        let epsilon = 0.0;
        let scale = 1.0;

        sim.step(world_size, gravity, epsilon, scale);

        // Particle 2 should have moved towards Particle 1 (negative x direction)
        assert!(sim.positions_x[1] < 10.0);
        assert_eq!(sim.positions_y[1], 0.0);
    }
}
