# Phase 1 Research: Core Performance & Stabilization

## Standard Stack
- **Rust (WASM)**: For core simulation logic.
- **wasm-bindgen**: Essential for JS-WASM interoperability and memory sharing.
- **js-sys**: Used to create JavaScript views of WASM memory in Rust if needed, though direct pointer exposure is often simpler for raw buffers.
- **SIMD (wasm32)**: Enabled via `-C target-feature=+simd128` to accelerate force calculations.

## Architecture Patterns
- **SoA (Structure of Arrays)**: Instead of a `Vec<Particle>`, use separate `Vec<f32>` for `x`, `y`, `vx`, `vy`, and `mass`. This aligns data for SIMD and improves cache locality during quadtree traversal and integration.
- **Shared Memory Buffer**:
    1. Rust allocates fixed-size `Vec<f32>` buffers.
    2. Rust exposes public functions that return the raw pointer (`*const f32`) and length.
    3. JS accesses `wasm.memory.buffer` and creates a `Float32Array(wasm.memory.buffer, pointer, length)`.
- **Index-based Quadtree**: Store particle indices in the quadtree nodes instead of cloning particle data. This makes it trivial to map the quadtree back to the SoA buffers.

## Don't Hand-Roll
- **Vector Math**: While current implementation has `vector2.rs`, consider if raw `f32` operations in SoA loop are more efficient than object-oriented vector math.
- **Serde for Hot Path**: Do NOT use `serde_wasm_bindgen` for the per-frame particle data update. It is too slow for thousands of particles.

## Common Pitfalls
- **Memory Growth**: If WASM memory grows, existing `Float32Array` views in JS become **invalid**. Always re-instantiate views if the buffer length changes or at the start of each frame if growth is possible.
- **Precision Drift**: Using `f32` instead of `f64` can lead to faster drift. However, for a visualization-focused simulation, the $2	imes$ memory throughput and SIMD potential usually outweigh the precision loss.
- **Recursive Quadtree Depth**: Large numbers of particles in a small area can cause deep recursion. Implement a `MAX_DEPTH` to prevent stack overflows.

## Code Examples

### Rust: Exposing Buffer Pointer
```rust
#[wasm_bindgen]
pub struct Simulation {
    pos_x: Vec<f32>,
    // ...
}

#[wasm_bindgen]
impl Simulation {
    pub fn pos_x_ptr(&self) -> *const f32 {
        self.pos_x.as_ptr()
    }
}
```

### JS: Viewing WASM Memory
```javascript
const pointer = wasm_instance.pos_x_ptr();
const count = 5000;
const pos_x_view = new Float32Array(wasm_instance.memory.buffer, pointer, count);
```

## Confidence Levels
- **Memory Sharing**: High (Industry standard for WASM performance).
- **SoA Performance**: High (Standard in data-oriented design and physics engines).
- **Quadtree Optimization**: Medium (Implementation details matter; need to ensure index management is clean).
