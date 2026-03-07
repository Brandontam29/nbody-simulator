# Codebase Structure

## Root
- `nbody_simulator/`: Rust source code for the simulation.
- `web/`: Frontend source code, assets, and build configuration.
- `script/`: A separate Rust tool/script, possibly for offline testing or benchmarking.

## nbody_simulator/src/
- `lib.rs`: WASM entry point.
- `particle.rs`: Particle struct and physics.
- `quad_tree.rs`: Quadtree implementation for Barnes-Hut.
- `quadrant.rs`, `rectangle.rs`: Geometry primitives.
- `vector2.rs`: 2D vector math.
- `utils/`: Calculation and quadrant helper functions.

## web/src/
- `index.html`: Main entry point.
- `script.js`: Main frontend logic and WASM interaction.
- `app-state.js`, `form-state.js`: Modular state management.
- `stats.js`: UI updates for simulation statistics.
- `profile1.json`: Example configuration or preset.
- `register.js`: Likely handles component registration or initialization.
- `removeElementsByIndices.js`: Utility for array/element manipulation.
