# Phase 1 Plan: Core Performance & Stabilization

## Goal
Optimize the simulation engine to support 5,000+ particles at 60 FPS by implementing a WASM-owned Structure of Arrays (SoA) memory model and shared buffer interop with JavaScript.

## Tasks

### Task 1: Stabilization - Fix Existing Tests
- [ ] Investigate and fix current failures in `nbody_simulator` unit tests.
- [ ] Re-enable commented-out tests in `particle.rs` and verify they pass with current architecture.
- [ ] **Verification**: Run `cargo test` and ensure all tests pass.

### Task 2: Refactor to SoA Architecture
- [ ] Redefine `Particle` struct or create a `SimulationState` that uses separate `Vec<f32>` for `x`, `y`, `vx`, `vy`, and `mass`.
- [ ] Update `QuadTree` to work with particle indices instead of full `Particle` objects.
- [ ] Implement index-based Barnes-Hut force calculation.
- [ ] Implement "Auto-fix" logic to remove particles with `NaN` or `Inf` values during the `step()` call.
- [ ] **Verification**: Add new unit tests for the SoA-based Quadtree insertion and force calculation.

### Task 3: Implement Shared Memory Interop
- [ ] Update `nbody_simulator/src/lib.rs` to expose raw pointers and lengths for all SoA buffers.
- [ ] Implement a `get_status()` function returning a simple health flag.
- [ ] Replace `serde_wasm_bindgen` calls in the hot simulation path with direct memory access.
- [ ] **Verification**: Verify pointer validity and data alignment in a small integration test.

### Task 4: Frontend Integration (JS)
- [ ] Update `web/src/script.js` to initialize the simulation and obtain WASM memory pointers.
- [ ] Replace `next_nbody_positions` calls with a new `step()` call that doesn't pass/return large JSON objects.
- [ ] Implement `Float32Array` views in JS to read particle data directly for rendering.
- [ ] **Verification**: Manually verify that the simulation runs smoothly in the browser with 5,000 particles.

### Task 5: Benchmarking & Cleanup
- [ ] (Optional) Implement a basic FPS/Compute-time display in the UI.
- [ ] Remove deprecated `serde` based functions from `lib.rs`.
- [ ] **Verification**: Confirm 60 FPS target for 5,000 particles.

## Success Criteria
- [ ] All existing and new unit tests pass.
- [ ] `serde_wasm_bindgen` is removed from the `step()` path.
- [ ] Simulation supports 5,000 particles at 60 FPS on modern hardware.
- [ ] "Broken" particles are automatically removed without crashing the simulation.
