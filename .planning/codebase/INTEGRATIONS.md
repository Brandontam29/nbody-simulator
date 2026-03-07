# Integrations

## Rust & WebAssembly (WASM)
The core simulation logic is written in Rust and compiled to WASM. 
- **Build Workflow**: `npm run wasm` in the `web` directory triggers `wasm-pack build` and then copies the generated assets into `web/src/wasm/`.
- **Interface**: `wasm-bindgen` is used to expose Rust types and functions to JavaScript.
- **Data Exchange**: JSON-compatible structures are passed between JS and WASM using `serde-wasm-bindgen`.

## JavaScript & CSS
- **TailwindCSS**: Integrated via CLI to process `app.css` and output `tailwind.css`, watched during development via `npm run tw`.
