# Phase 1 Context: Core Performance & Stabilization

## Implementation Decisions

### Memory Strategy: WASM-Owned SoA (Structure of Arrays)
- **Choice**: The Rust simulation will manage its own memory heap and allocate contiguous buffers for particle properties (Positions, Velocities, and Static data).
- **Format**: `f32` (High Performance) precision will be used for all simulation variables to optimize for SIMD and reduce memory bandwidth.
- **JS Interop**: The WASM module will export pointers to these buffers. The JS frontend will create views (e.g., `new Float32Array(wasm.memory.buffer, pos_ptr, size)`) to read data directly without serialization.
- **Dynamism**: Buffer sizes are FIXED for the duration of a simulation run (no resizing during `step()`).

### Precision & Physics
- **Precision**: `f32` is the primary target to maximize particle count and throughput.
- **Force Calculation**: Will still use a softening epsilon to prevent infinite forces at zero distance.

### Stabilization & Testing
- **Focus**: Fixing all existing unit tests in `nbody_simulator` to ensure baseline correctness (`Functional Fixes`).
- **Tools**: standard `cargo test` framework.

### Diagnostics & Health
- **Monitoring**: A simple status flag will be returned from each simulation step.
- **Auto-Fix**: If a particle's position or velocity becomes `NaN` or `Inf`, the WASM module will surgically remove it from the active set to prevent simulation collapse.
- **State**: No binary/JSON state export for Phase 1; reproduction will rely on presets/seeds.

## Code Context
- **Rust Entry**: `nbody_simulator/src/lib.rs` (WASM interface)
- **Core Logic**: `nbody_simulator/src/quad_tree.rs`, `nbody_simulator/src/particle.rs`
- **Current Serialization**: `serde_wasm_bindgen` (to be deprecated in this phase for performance).
