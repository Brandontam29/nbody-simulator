use crate::particle::Particle;
use crate::vector2::Vector2;

pub fn softened_gravitational_force(
    p1: &Particle,
    p2: &Particle,
    gravity: f32,
    epsilon: f32,
    scale: f32,
) -> Vector2 {
    if p1.position.x == p2.position.x && p1.position.y == p2.position.y {
        return Vector2 { x: 0.0, y: 0.0 };
    }

    let distance_vector = p1.position - p2.position; // Vector from p2 to p1

    let r_sq = distance_vector.x.powi(2) + distance_vector.y.powi(2);
    // Standard softened gravity: F = G*m1*m2 * r_vec / (r^2 + eps^2)^1.5
    let force_magnitude_scaled = gravity * p1.mass * p2.mass / (r_sq + epsilon.powi(2)).powf(1.5);
    
    // Force on p2 towards p1
    let force = distance_vector.scale(force_magnitude_scaled);
    
    // Acceleration a = F / m
    return force.scale(scale / p2.mass);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softened_gravitational_force() {
        let p1 = Particle::new(
            1.0e10,
            1.0,
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            [100.0, 100.0, 100.0],
        );
        let p2 = Particle::new(
            1.0,
            1.0,
            Vector2::new(10.0, 0.0),
            Vector2::new(0.0, 0.0),
            [100.0, 100.0, 100.0],
        );

        let gravity = 1.0;
        let epsilon = 0.0;
        let scale = 1.0;

        let result_acceleration = softened_gravitational_force(&p1, &p2, gravity, epsilon, scale);
        
        // F = G * m1 * m2 / r^2 = 1 * 1e10 * 1 / 10^2 = 1e8
        // a = F / m2 = 1e8 / 1 = 1e8
        // direction is p2 -> p1, so (-1, 0)
        let expected_acceleration = Vector2::new(-1.0e8, 0.0);

        assert!((result_acceleration.x - expected_acceleration.x).abs() < 1e-1);
        assert!((result_acceleration.y - expected_acceleration.y).abs() < 1e-1);
    }
}
