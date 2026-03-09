# Phase 2 Research: Enhanced Visualization

## Standard Stack
- **HTML5 Canvas 2D**: Primary rendering engine.
- **WASM Memory**: Direct access for velocity and position data.
- **Look-Up Table (LUT)**: For fast velocity-to-color mapping.

## Architecture Patterns
- **Batched Rendering**: Grouping `fillRect` calls by color. Instead of changing `ctx.fillStyle` for every particle, the renderer will:
  1. Sort or bucket particles by their velocity-mapped color.
  2. Set `ctx.fillStyle` once for each bucket.
  3. Call `fillRect` for all particles in that bucket.
- **Velocity-to-Color Mapping**:
  - **WASM Side**: Compute velocity magnitude ($v = \sqrt{vx^2 + vy^2}$) in Rust during the `step()` call.
  - **JS Side**: Map magnitude to a 1D gradient. Using a pre-computed 256-entry array of `rgb()` strings is often faster than calculating strings per-particle.

## Don't Hand-Roll
- **Complex Color Math**: Use simple linear interpolation (lerp) between two or three key colors (e.g., Blue -> Cyan -> White).
- **Custom Sorting**: If grouping by color, use a fixed number of "bins" (e.g., 16 bins for 16 speed ranges) to avoid $O(N \log N)$ sorting overhead.

## Common Pitfalls
- **String Allocation**: Creating new `rgb()` strings in the rendering loop is a major performance killer. Pre-allocate these strings in an array.
- **Floating Point Math in JS**: Minimize calculations in the JS loop; WASM should ideally provide pre-scaled coordinates if the world-to-pixel ratio is constant.

## Code Examples

### JS: Grouped Rendering by Color Bin
```javascript
const bins = Array.from({ length: 16 }, () => []);
for (let i = 0; i < count; i++) {
    const binIdx = getVelocityBin(velocities[i]);
    bins[binIdx].push(i);
}

bins.forEach((indices, binIdx) => {
    ctx.fillStyle = precomputedColors[binIdx];
    for (const i of indices) {
        ctx.fillRect(posX[i], posY[i], size, size);
    }
});
```

## Confidence Levels
- **Canvas Batching**: High (Standard optimization for thousands of entities).
- **Color Bins**: High (Efficient compromise between individual colors and performance).
