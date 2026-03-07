# Architecture

## System Overview
The project is a Barnes-Hut N-body simulation split into a high-performance Rust backend and a web-based JavaScript frontend.

## Components
### Simulation Engine (Rust)
- **Barnes-Hut Algorithm**: Implemented using a Quadtree structure (`quad_tree.rs`) for efficient calculation of gravitational forces ($O(N \log N)$ vs $O(N^2)$).
- **Core Entities**: Particles with position, velocity, and mass.
- **Math Utilities**: Custom `Vector2`, `Rectangle`, and `Quadrant` implementations for 2D spatial reasoning.

### Web Frontend (JavaScript)
- **State Management**: Distributed across `app-state.js` and `form-state.js`.
- **UI/UX**: HTML5 Canvas (likely, though `script.js` needs deeper look) for rendering, with controls styled via Tailwind.
- **Bridge**: Loads the WASM module and invokes simulation steps per frame.
