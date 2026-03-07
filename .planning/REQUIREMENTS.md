# Requirements

## Functional Requirements
- **FR1: Particle Simulation**: Support for at least 5,000 particles at 60 FPS using Barnes-Hut.
- **FR2: Gravity Model**: Implementation of softened gravitational force to prevent infinities at zero distance.
- **FR3: Interactive UI**: Controls for simulation speed, gravity strength, and particle resets.
- **FR4: Visualization**: Smooth rendering of particles with trailing effects or velocity-based coloring.
- **FR5: Presets**: Ability to load initial conditions (e.g., galaxy collision, stable orbit).

## Non-Functional Requirements
- **NFR1: Performance**: Minimize JS-WASM bridge overhead by using shared memory buffers if possible.
- **NFR2: Portability**: Must run in all modern browsers supporting WASM.
- **NFR3: Scalability**: Architecture should allow for future transition to WebGPU if needed.

## Constraints
- Must stay within the existing `nbody_simulator` (Rust) and `web` (JS) structure.
- Use Vanilla Canvas for rendering to keep dependencies minimal.
