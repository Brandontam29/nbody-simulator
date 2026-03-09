# Phase 2 Plan: Enhanced Visualization

## Goal
Implement a high-performance velocity-based color mapping for the N-body simulation, utilizing a batched rendering approach in the JavaScript frontend to maintain 60 FPS for 5,000+ particles.

## Tasks

### Task 1: WASM Side - Velocity Calculation
- [ ] Add a `velocities_ptr()` method to `SimulationWrapper` in `lib.rs` (if not already existing/functional).
- [ ] Ensure `Simulation` struct in `simulation.rs` computes velocity magnitude ($v = \sqrt{vx^2 + vy^2}$) or provides raw $vx, vy$ to the frontend.
- [ ] **Verification**: Confirm `velocities_x_ptr()` and `velocities_y_ptr()` are correctly exposed via WASM.

### Task 2: JS Side - Velocity-to-Color Mapping
- [ ] Pre-compute a 32-entry array of `rgb()` color strings (e.g., transitioning from Deep Blue to Bright Cyan).
- [ ] Implement a `getVelocityBin(vx, vy)` helper to map a particle's speed to a bin index (0-31).
- [ ] Implement a "binning" pass that groups particle indices into buckets based on their velocity bin.
- [ ] **Verification**: Verify that the binning logic correctly assigns particles to different speed buckets.

### Task 3: Optimized Rendering Refactor
- [ ] Update `render()` in `script.js` to iterate through the color bins.
- [ ] Set `ctx.fillStyle` once per bin and draw all particles within that bin using `ctx.fillRect()`.
- [ ] Ensure the 1:1 pixel mapping and fixed quality constraints from `2-CONTEXT.md` are respected.
- [ ] **Verification**: Measure FPS and confirm it meets the 60 FPS target for 5,000 particles.

### Task 4: UI Enhancements & Presets
- [ ] (Optional) Add a toggle in the UI to enable/disable velocity coloring.
- [ ] Implement at least two distinct color presets (e.g., "Deep Space Blue" and "Solar Orange").
- [ ] **Verification**: Manually test the color presets and toggle in the UI.

## Success Criteria
- [ ] All 5,000 particles are rendered with colors mapped to their velocity.
- [ ] Simulation maintains a steady 60 FPS on a single-threaded loop.
- [ ] Canvas draw calls are minimized through color-based batching.
- [ ] No performance degradation compared to the uncolored Phase 1 baseline.
