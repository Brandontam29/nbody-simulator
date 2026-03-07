# Conventions

## Coding Standards
- **Rust**: Follows standard `rustfmt` conventions and uses 2021 edition features. Logic is organized into modules.
- **JavaScript**: Follows modern ES module standards. ESLint is configured in `.eslintrc.cjs`.

## Naming
- **Rust**: `snake_case` for variables, functions, and modules; `PascalCase` for structs and enums.
- **JavaScript**: `camelCase` for variables and functions; `kebab-case` for some filenames or CSS classes.

## Documentation
- Minimal inline documentation; logic is intended to be self-documenting through clear module and function naming.

## Interop
- Explicit use of `wasm-bindgen` for public APIs intended for JS consumption.
