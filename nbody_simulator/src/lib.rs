mod particle;

extern crate wasm_bindgen;

use particle::{Particle, ParticleArgs, Vector2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_particle(input_args: JsValue) -> Result<JsValue, JsValue> {
    let args: ParticleArgs = serde_wasm_bindgen::from_value(input_args)?;

    let particle = Particle::new(
        args.diameter,
        args.mass,
        args.position,
        args.velocity,
        args.color,
    );

    return Ok(serde_wasm_bindgen::to_value(&particle)?);
}

#[wasm_bindgen]
pub fn generate_particles(
    number_js: JsValue,
    world_size_js: JsValue,
    mass_js: JsValue,
    mass_deviation_js: JsValue,
    diameter_js: JsValue,
) -> Result<JsValue, JsValue> {
    let number: usize = serde_wasm_bindgen::from_value(number_js)?;
    let world_size: Vector2 = serde_wasm_bindgen::from_value(world_size_js)?;
    let mass: f64 = serde_wasm_bindgen::from_value(mass_js)?;
    let mass_deviation: f64 = serde_wasm_bindgen::from_value(mass_deviation_js)?;
    let diameter: f64 = serde_wasm_bindgen::from_value(diameter_js)?;

    let particles: Vec<Particle> = (0..number)
        .map(|_| Particle::new_rand(world_size, mass, mass_deviation, diameter))
        .collect();

    return Ok(serde_wasm_bindgen::to_value(&particles)?);
}

#[wasm_bindgen]
pub fn next_nbody_positions(
    particles_js: JsValue,
    gravity_js: JsValue,
    epsilon_js: JsValue,
    scale_js: JsValue,
    // _collision_js: JsValue,
) -> Result<JsValue, JsValue> {
    let mut particles: Vec<Particle> = serde_wasm_bindgen::from_value(particles_js)?;
    let gravity: f64 = serde_wasm_bindgen::from_value(gravity_js)?;
    let epsilon: f64 = serde_wasm_bindgen::from_value(epsilon_js)?;
    let scale: f64 = serde_wasm_bindgen::from_value(scale_js)?;

    for i in 0..particles.len() {
        particles[i].velocity = particles[i].next_velocity(&particles, gravity, epsilon, scale);
        particles[i].position = particles[i].next_position();
    }

    return Ok(serde_wasm_bindgen::to_value(&particles)?);
}
