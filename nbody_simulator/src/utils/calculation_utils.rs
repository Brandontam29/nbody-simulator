use crate::particle::Particle;
use crate::vector2::Vector2;

pub fn softened_gravitational_force(
    p1: &Particle,
    p2: &Particle,
    gravity: f64,
    epsilon: f64,
    scale: f64,
) -> Vector2 {
    // println!("p1 {};p2 {}", p1.position, p2.position);

    if p1.position.x == p2.position.x && p1.position.y == p2.position.y {
        return Vector2 { x: 0.0, y: 0.0 };
    }

    let distance_vector = p1.position - p2.position;
    // println!("distance_vector {} ", distance_vector);

    let r = distance_vector.magnitude();
    // println!("r {} ", r);
    let force_magnitude = gravity * p1.mass * p2.mass / (r.powi(2) + epsilon.powi(2)).powf(1.5);
    // println!("force_magnitude {} ", force_magnitude);
    let force_direction = distance_vector.normalize();
    // println!("force_direction {} ", force_direction);
    let v = force_direction.scale(-force_magnitude);
    // println!("v {} ", v);
    // println!("scaled_v {} ", v.scale(scale / p1.mass));
    return v.scale(scale / p1.mass);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softened_gravitational_force() {
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
        assert_eq!(result_force.x, expected_force.x);
        assert_eq!(result_force.y, expected_force.y);
    }
}
