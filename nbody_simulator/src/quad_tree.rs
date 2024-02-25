use crate::rectangle::Rectangle;
use crate::utils::calculation_utils::softened_gravitational_force;
use crate::utils::quadrant_utils::{find_quadrant, quadrant_to_rectangle};
use crate::{particle::Particle, vector2::Vector2};
enum QuadNode {
    Internal(Box<QuadTree>),
    Leaf(Particle),
    Empty,
}

pub struct QuadTree {
    boundary: Rectangle,
    total_mass: f64,
    center_of_mass: Vector2,
    children: [QuadNode; 4],
}
impl QuadTree {
    pub fn new(boundary: Rectangle) -> QuadTree {
        QuadTree {
            boundary,
            total_mass: 0.0,
            center_of_mass: Vector2 { x: 0.0, y: 0.0 },
            children: [
                QuadNode::Empty,
                QuadNode::Empty,
                QuadNode::Empty,
                QuadNode::Empty,
            ],
        }
    }

    pub fn insert(&mut self, particle: Particle) {
        self.update_mass_and_center(&particle);

        let quadrant = match find_quadrant(&self.boundary, &particle.position) {
            Some(v) => v,
            None => return,
        };

        let quad_node = &mut self.children[usize::from(&quadrant)];

        match quad_node {
            QuadNode::Empty => {
                let leaf = QuadNode::Leaf(particle);

                self.children[usize::from(&quadrant)] = leaf;
            }
            QuadNode::Leaf(p) => {
                let boundary = quadrant_to_rectangle(&self.boundary, &quadrant);

                let mut qtree = QuadTree::new(boundary);

                qtree.insert(p.to_owned());
                qtree.insert(particle);

                self.children[usize::from(&quadrant)] = QuadNode::Internal(Box::new(qtree));
            }
            QuadNode::Internal(quad_tree) => quad_tree.insert(particle),
        }
    }

    fn update_mass_and_center(&mut self, particle: &Particle) {
        let scaled_center =
            self.center_of_mass.scale(self.total_mass) + particle.position.scale(particle.mass);

        self.total_mass += particle.mass;

        self.center_of_mass = scaled_center.scale(1.0 / self.total_mass);
    }

    pub fn compute_force(
        &self,
        particle: &Particle,
        gravity: f64,
        epsilon: f64,
        scale: f64,
        vector: &mut Vector2,
    ) {
        // println!("{}", vector);
        for i in 0..4 {
            let quad_node = &self.children[i];

            match quad_node {
                QuadNode::Empty => {}

                QuadNode::Leaf(p) => {
                    let v = softened_gravitational_force(&p, &particle, gravity, scale, epsilon);

                    println!("{} {} {}", particle.position, p.position, v);

                    vector.x += v.x;
                    vector.y += v.y;
                }

                QuadNode::Internal(quad_tree) => {
                    let s = (self.boundary.width + self.boundary.width) / 2.0;
                    let d = self.center_of_mass.distance(&particle.position);
                    let sd: f64 = s / d;

                    let threshold = 0.2;

                    if sd > threshold {
                        quad_tree.compute_force(particle, gravity, epsilon, scale, vector);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_real_scenario() {
        let mut particles: Vec<Particle> = (0..4)
            .map(|_| Particle::new_rand(Vector2 { x: 700.0, y: 700.0 }, 100.0, 0.0, 10.0))
            .collect();

        let r = Rectangle::new(Vector2 { x: 0.0, y: 0.0 }, 700.0, 700.0);

        let mut q = QuadTree::new(r);

        for i in 0..particles.len() {
            q.insert(particles[i])
        }

        for i in 0..particles.len() {
            let mut velocity = Vector2 { x: 0.0, y: 0.0 };

            q.compute_force(&particles[i], 0.005, 5.0, 10.0, &mut velocity);
            // println!("{}: {}", i, velocity);

            particles[i].velocity = particles[i].velocity + velocity;
            particles[i].position = particles[i].next_position();
        }

        for i in 0..particles.len() {
            // println!("{}", particles[i]);
        }
        // println!("2nd round");

        let mut q = QuadTree::new(r);

        for i in 0..particles.len() {
            q.insert(particles[i])
        }

        for i in 0..particles.len() {
            let mut velocity = Vector2 { x: 0.0, y: 0.0 };

            q.compute_force(&particles[i], 0.005, 5.0, 10.0, &mut velocity);
            // println!("{}: {}", i, velocity);
            particles[i].velocity = particles[i].velocity + velocity;
            particles[i].position = particles[i].next_position();
        }

        for i in 0..particles.len() {
            // println!("{}", particles[i]);
        }
    }
}
