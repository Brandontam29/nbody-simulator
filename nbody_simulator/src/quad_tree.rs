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

                    // println!("{} {} {}", particle.position, p.position, v);

                    vector.x += v.x;
                    vector.y += v.y;
                }

                QuadNode::Internal(quad_tree) => {
                    // let s = (self.boundary.width + self.boundary.width) / 2.0;
                    // let d = self.center_of_mass.distance(&particle.position);
                    // let sd: f64 = s / d;

                    // let threshold = 0.000000000000001;

                    // if sd > threshold {
                    quad_tree.compute_force(particle, gravity, epsilon, scale, vector);
                    // }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quadtree_initialization() {
        let boundary = Rectangle::new(Vector2::new(0.0, 0.0), 100.0, 100.0);
        let quadtree = QuadTree::new(boundary);

        assert_eq!(quadtree.total_mass, 0.0);
        assert_eq!(quadtree.center_of_mass, Vector2 { x: 0.0, y: 0.0 });
        assert!(matches!(quadtree.children[0], QuadNode::Empty));
        // Check other children similarly
    }

    #[test]
    fn test_particle_insertion_and_mass_update() {
        let mut quadtree = QuadTree::new(Rectangle::new(Vector2::new(0.0, 0.0), 100.0, 100.0));
        let p1 = Particle::new(
            1.0,
            1.0,
            Vector2 { x: 10.0, y: 10.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );
        let p2 = Particle::new(
            1.0,
            1.0,
            Vector2 { x: 90.0, y: 10.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );
        let p3 = Particle::new(
            1.0,
            1.0,
            Vector2 { x: 10.0, y: 90.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );
        let p4 = Particle::new(
            1.0,
            1.0,
            Vector2 { x: 90.0, y: 90.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );

        quadtree.insert(p1);
        quadtree.insert(p2);
        quadtree.insert(p3);
        quadtree.insert(p4);

        assert!(matches!(quadtree.children[0], QuadNode::Leaf(_))); // Assuming NW quadrant
        assert!(matches!(quadtree.children[1], QuadNode::Leaf(_))); // Assuming NW quadrant
        assert!(matches!(quadtree.children[2], QuadNode::Leaf(_))); // Assuming NW quadrant
        assert!(matches!(quadtree.children[3], QuadNode::Leaf(_))); // Assuming NW quadrant
        assert_eq!(quadtree.total_mass, 4.0);
        assert_eq!(quadtree.center_of_mass, Vector2::new(50.0, 50.0));
    }

    #[test]
    fn test_compute() {
        let p1 = Particle::new(
            2.8121667702779295e+30,
            1967738920.8774006,
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            [100.0, 100.0, 100.0],
        );
        let p2 = Particle::new(
            3.112157648312442e+30,
            2177649560.9061766,
            Vector2::new(3.0, 4.0),
            Vector2::new(0.0, 0.0),
            [100.0, 100.0, 100.0],
        );
        let p3 = Particle::new(
            1.0,
            1.0,
            Vector2 { x: 10.0, y: 90.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );
        let p4 = Particle::new(
            3.112157648312442e+30,
            1.0,
            Vector2 { x: 90.0, y: 90.0 },
            Vector2 { x: 0.0, y: 0.0 },
            [255.0, 255.0, 255.0],
        );

        let mut particles = vec![p1, p2, p3, p4];

        let r = Rectangle::new(Vector2 { x: 0.0, y: 0.0 }, 100.0, 100.0);

        let mut quandtree: QuadTree = QuadTree::new(r);

        for i in 0..particles.len() {
            quandtree.insert(particles[i]);
        }

        for i in 0..particles.len() {
            println!("{}", particles[i].velocity);
        }

        for i in 0..particles.len() {
            let mut velocity = Vector2 { x: 0.0, y: 0.0 };

            quandtree.compute_force(&particles[i], 10000.0, 10.0, 100.0, &mut velocity);
            // println!("{}: {}", i, velocity);

            particles[i].velocity = particles[i].velocity + velocity;
            particles[i].position = particles[i].next_position();
        }

        for i in 0..particles.len() {
            println!("{}", particles[i].velocity);
        }
        println!("2nd round");

        let mut quandtree = QuadTree::new(r);

        for i in 0..particles.len() {
            quandtree.insert(particles[i])
        }

        for i in 0..particles.len() {
            let mut velocity = Vector2 { x: 0.0, y: 0.0 };

            quandtree.compute_force(&particles[i], 0.005, 5.0, 10.0, &mut velocity);
            // println!("{}: {}", i, velocity);
            particles[i].velocity = particles[i].velocity + velocity;
            particles[i].position = particles[i].next_position();
        }

        for i in 0..particles.len() {
            println!("{}", particles[i].velocity);
        }
    }
}
