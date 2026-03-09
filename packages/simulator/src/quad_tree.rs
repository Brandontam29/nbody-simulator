use crate::rectangle::Rectangle;
use crate::utils::quadrant_utils::{find_quadrant, quadrant_to_rectangle};
use crate::vector2::Vector2;

#[derive(Debug)]
pub enum QuadNode {
    Internal(Box<QuadTree>),
    Leaf(Vec<usize>), // Indices into the SoA arrays
    Empty,
}

const MAX_DEPTH: u32 = 20;

#[derive(Debug)]
pub struct QuadTree {
    pub boundary: Rectangle,
    pub total_mass: f32,
    pub center_of_mass: Vector2,
    pub children: [QuadNode; 4],
    pub depth: u32,
}

impl QuadTree {
    pub fn new(boundary: Rectangle) -> QuadTree {
        QuadTree::with_depth(boundary, 0)
    }

    fn with_depth(boundary: Rectangle, depth: u32) -> QuadTree {
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
            depth,
        }
    }

    pub fn insert(
        &mut self,
        index: usize,
        pos_x: &[f32],
        pos_y: &[f32],
        masses: &[f32],
    ) {
        let p_pos = Vector2::new(pos_x[index], pos_y[index]);
        let p_mass = masses[index];
        
        self.update_mass_and_center(p_pos, p_mass);

        let quadrant = match find_quadrant(&self.boundary, &p_pos) {
            Some(v) => v,
            None => return,
        };

        let quad_node = &mut self.children[usize::from(&quadrant)];

        match quad_node {
            QuadNode::Empty => {
                *quad_node = QuadNode::Leaf(vec![index]);
            }
            QuadNode::Leaf(indices) => {
                if self.depth >= MAX_DEPTH {
                    indices.push(index);
                    return;
                }

                let old_indices = indices.clone();
                let boundary = quadrant_to_rectangle(&self.boundary, &quadrant);

                let mut qtree = QuadTree::with_depth(boundary, self.depth + 1);
                for old_idx in old_indices {
                    qtree.insert(old_idx, pos_x, pos_y, masses);
                }
                qtree.insert(index, pos_x, pos_y, masses);

                *quad_node = QuadNode::Internal(Box::new(qtree));
            }
            QuadNode::Internal(quad_tree) => {
                quad_tree.insert(index, pos_x, pos_y, masses);
            }
        }
    }

    fn update_mass_and_center(&mut self, p_pos: Vector2, p_mass: f32) {
        let total_mass_new = self.total_mass + p_mass;
        if total_mass_new > 0.0 {
            let scaled_center =
                self.center_of_mass.scale(self.total_mass) + p_pos.scale(p_mass);
            self.center_of_mass = scaled_center.scale(1.0 / total_mass_new);
        }
        self.total_mass = total_mass_new;
    }

    pub fn compute_force(
        &self,
        p_pos: Vector2,
        gravity: f32,
        epsilon: f32,
        scale: f32,
        pos_x: &[f32],
        pos_y: &[f32],
        masses: &[f32],
    ) -> Vector2 {
        if self.total_mass == 0.0 {
            return Vector2::new(0.0, 0.0);
        }

        let s = self.boundary.width;
        let d = self.center_of_mass.distance(&p_pos);

        // Barnes-Hut threshold
        let theta = 0.5;

        // If the node is far enough, use its center of mass as a single particle
        if d > 0.0 && s / d < theta {
            // We need a dummy particle for softened_gravitational_force
            // but softened_gravitational_force takes Particle refs.
            // Let's refactor softened_gravitational_force to take components.
            return self.force_from_mass(p_pos, self.center_of_mass, self.total_mass, gravity, epsilon, scale);
        }

        let mut acceleration = Vector2::new(0.0, 0.0);
        for quad_node in &self.children {
            match quad_node {
                QuadNode::Empty => {}
                QuadNode::Leaf(indices) => {
                    for &idx in indices {
                        let other_pos = Vector2::new(pos_x[idx], pos_y[idx]);
                        let other_mass = masses[idx];
                        acceleration = acceleration + self.force_from_mass(p_pos, other_pos, other_mass, gravity, epsilon, scale);
                    }
                }
                QuadNode::Internal(quad_tree) => {
                    acceleration = acceleration + quad_tree.compute_force(p_pos, gravity, epsilon, scale, pos_x, pos_y, masses);
                }
            }
        }
        acceleration
    }
    
    fn force_from_mass(&self, p_pos: Vector2, other_pos: Vector2, other_mass: f32, gravity: f32, epsilon: f32, scale: f32) -> Vector2 {
        if p_pos.x == other_pos.x && p_pos.y == other_pos.y {
            return Vector2::new(0.0, 0.0);
        }

        let distance_vector = other_pos - p_pos; // Vector from self (p2) to other (p1)
        let r_sq = distance_vector.x.powi(2) + distance_vector.y.powi(2);
        let force_magnitude_scaled = gravity * other_mass / (r_sq + epsilon.powi(2)).powf(1.5);
        
        // Acceleration = (G * m1 * r_vec) / (r^2 + eps^2)^1.5 * scale
        distance_vector.scale(force_magnitude_scaled * scale)
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
    }

    #[test]
    fn test_particle_insertion_and_mass_update() {
        let mut quadtree = QuadTree::new(Rectangle::new(Vector2::new(0.0, 0.0), 100.0, 100.0));
        
        let pos_x = vec![10.0, 90.0, 10.0, 90.0];
        let pos_y = vec![10.0, 10.0, 90.0, 90.0];
        let masses = vec![1.0, 1.0, 1.0, 1.0];

        for i in 0..4 {
            quadtree.insert(i, &pos_x, &pos_y, &masses);
        }

        assert_eq!(quadtree.total_mass, 4.0);
        assert_eq!(quadtree.center_of_mass, Vector2::new(50.0, 50.0));
    }

    #[test]
    fn test_identical_positions_stack_overflow() {
        let mut quadtree = QuadTree::new(Rectangle::new(Vector2::new(0.0, 0.0), 100.0, 100.0));
        
        // Two particles at the exact same position
        let pos_x = vec![50.0, 50.0];
        let pos_y = vec![50.0, 50.0];
        let masses = vec![1.0, 1.0];

        quadtree.insert(0, &pos_x, &pos_y, &masses);
        quadtree.insert(1, &pos_x, &pos_y, &masses);

        assert_eq!(quadtree.total_mass, 2.0);
        assert_eq!(quadtree.center_of_mass, Vector2::new(50.0, 50.0));
    }
}
