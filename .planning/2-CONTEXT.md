# Phase 2 Context: Enhanced Visualization

## Implementation Decisions

### Render Optimization: Main-Thread Batching
- **Approach**: The current per-particle `fillRect` loop will be replaced with a batched approach using a single `Path2D` or multiple `fill()` calls grouped by color.
- **Reasoning**: Minimizing the JS-to-Canvas API overhead is the priority for the 5,000+ particle target.
- **DPI**: 1:1 pixel mapping will be maintained for maximum performance; high-DPI (Retina) scaling is NOT required.
- **Layers**: A single canvas layer will be used for all particles; no separate background/trail layers.

### Visual Style: High-Speed Points
- **Geometry**: `fillRect` (square points) remains the baseline for performance, but we will explore batching these into larger `fill()` calls.
- **Trails**: EXPLICITLY DISABLED for this phase to focus on raw particle throughput and color performance.
- **Quality**: Fixed quality (no auto-scaling/skipping frames).

### Color Mapping: Velocity-based Gradients
- **Strategy**: Move color calculation to JavaScript or implement a high-performance LUT (Look-Up Table) in WASM to map particle velocity to a color gradient.
- **Gradient**: Transition from "cool" colors (e.g., Deep Blue/Purple) at low speeds to "hot" colors (e.g., Bright Cyan/White) at high speeds.

## Code Context
- **Primary File**: `web/src/script.js` (Rendering loop refactor).
- **Secondary File**: `nbody_simulator/src/lib.rs` (WASM interface for velocity access).
- **Optimization Target**: 5,000+ particles at 60 FPS on a single-threaded loop.
