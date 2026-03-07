# Testing

## Rust Testing
- **Unit Tests**: Inline modules marked with `#[cfg(test)]`.
- **Coverage**:
  - `quad_tree.rs`: Initialization, insertion, and compute logic.
  - `utils/`: Geometric calculations and quadrant mapping.
  - `particle.rs`: Physics calculations (some tests commented out).
- **Execution**: Run via `cargo test` in the `nbody_simulator` directory.

## JavaScript Testing
- **Current State**: No automated testing framework (like Jest or Vitest) is configured in `package.json`.
- **Manual Verification**: Changes are verified by running the application in the browser.
