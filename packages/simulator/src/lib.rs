mod particle;
mod quad_tree;
mod quadrant;
mod rectangle;
mod utils;
mod vector2;
mod simulation;

extern crate wasm_bindgen;

use particle::Particle;
use vector2::Vector2;
use wasm_bindgen::prelude::*;
use simulation::Simulation;

#[wasm_bindgen]
pub struct SimulationWrapper {
    inner: Simulation,
}

#[wasm_bindgen]
impl SimulationWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(particles: Vec<Particle>) -> SimulationWrapper {
        SimulationWrapper {
            inner: Simulation::new(particles),
        }
    }

    pub fn step(&mut self, world_width: f32, world_height: f32, gravity: f32, epsilon: f32, time_step: f32) {
        let world_size = Vector2::new(world_width, world_height);
        self.inner.step(world_size, gravity, epsilon, time_step);
    }

    pub fn positions_x_ptr(&self) -> *const f32 {
        self.inner.positions_x.as_ptr()
    }

    pub fn positions_y_ptr(&self) -> *const f32 {
        self.inner.positions_y.as_ptr()
    }

    pub fn velocities_x_ptr(&self) -> *const f32 {
        self.inner.velocities_x.as_ptr()
    }

    pub fn velocities_y_ptr(&self) -> *const f32 {
        self.inner.velocities_y.as_ptr()
    }

    pub fn masses_ptr(&self) -> *const f32 {
        self.inner.masses.as_ptr()
    }

    pub fn diameters_ptr(&self) -> *const f32 {
        self.inner.diameters.as_ptr()
    }

    pub fn colors_ptr(&self) -> *const f32 {
        self.inner.colors.as_ptr()
    }

    pub fn count(&self) -> usize {
        self.inner.count
    }
}

#[wasm_bindgen]
pub fn generate_particles(
    number: usize,
    world_width: f32,
    world_height: f32,
    mass: f32,
    mass_deviation: f32,
    diameter: f32,
) -> SimulationWrapper {
    let world_size = Vector2::new(world_width, world_height);

    let particles: Vec<Particle> = (0..number)
        .map(|_| Particle::new_rand(world_size, mass, mass_deviation, diameter))
        .collect();
    
    SimulationWrapper {
        inner: Simulation::new(particles),
    }
}
