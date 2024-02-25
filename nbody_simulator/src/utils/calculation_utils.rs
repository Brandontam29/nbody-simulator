use crate::particle::Particle;
use crate::vector2::Vector2;

pub fn softened_gravitational_force(
    p1: &Particle,
    p2: &Particle,
    gravity: f64,
    epsilon: f64,
    scale: f64,
) -> Vector2 {
    if p1.position.x == p2.position.x && p1.position.y == p2.position.y {
        return Vector2 { x: 0.0, y: 0.0 };
    }

    let distance_vector = p1.position - p2.position;

    let r = distance_vector.magnitude();
    let force_magnitude = gravity * p1.mass * p2.mass / (r.powi(2) + epsilon.powi(2)).powf(1.5);
    let force_direction = distance_vector.normalize();

    // The force vector is in the direction of the distance vector, scaled by the force magnitude
    let v = force_direction.scale(-force_magnitude);

    return v.scale(scale / p1.mass);
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq; // Using `approx` crate for floating point comparisons

    #[test]
    fn test_softened_gravitational_force() {
        let p1 = Particle::new(
            1.0,
            1.0,
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            [1.0, 1.0, 1.0],
        );
        let p2 = Particle::new(
            1.0,
            1.0,
            Vector2::new(3.0, 4.0),
            Vector2::new(0.0, 0.0),
            [1.0, 1.0, 1.0],
        );

        let gravity = 6.67430e-11;
        let epsilon = 0.01;
        let scale = 1.0;

        let result_force = softened_gravitational_force(&p1, &p2, gravity, epsilon, scale);
        // Expected force calculation based on the `softened_gravitational_force` logic
        let expected_distance = Vector2::new(-3.0, -4.0).magnitude(); // Distance vector magnitude
        let expected_force_magnitude =
            gravity / (expected_distance.powi(2) + epsilon.powi(2)).powf(1.5);
        let expected_force = Vector2::new(-3.0, -4.0)
            .normalize()
            .scale(-expected_force_magnitude)
            .scale(scale / p1.mass);

        // Using assert_relative_eq from the `approx` crate for floating point comparison
        assert_relative_eq!(result_force.x, expected_force.x, epsilon = 1e-9);
        assert_relative_eq!(result_force.y, expected_force.y, epsilon = 1e-9);
    }
}
