# Tech Stack

## Core Technologies
- **Rust (2021 Edition)**: Primary language for simulation logic.
- **JavaScript (ES Modules)**: Web frontend and application state management.
- **WebAssembly (WASM)**: Target for Rust simulation code to run in the browser.

## Frameworks & Libraries
### Rust
- `wasm-bindgen`: JavaScript-Rust interoperability.
- `serde`, `serde-wasm-bindgen`: Serialization for data transfer between JS and WASM.
- `rand`, `getrandom`: Random number generation (with JS support).
- `approx`: Approximate floating point comparisons for tests.

### JavaScript
- **TailwindCSS**: CSS utility framework for styling.
- **ESLint**: Linting for JavaScript files.

## Build Tools
- `wasm-pack`: Rust to WASM build and packaging tool.
- `npm`: Package management and script execution for the web project.
- `tailwindcss`: CLI tool for processing CSS.
