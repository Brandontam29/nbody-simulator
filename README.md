# nbody-simulator
## [View project](https://nbody-simulator.netlify.app/)
NBody simulator on a HTML canvas where all calculation are performed via WASM Rust.

### Time complexity
Naive algorithm is O(n^2). For every particle, we calculate the force that it has on every other one.

Barnes-Hut Algorithm is O(n logn). In short, this algorithm stores nodes into quadrants and pre-calculates the center of mass of clusters of particles. This enables calculating the force from a cluster as oppose to individual particles. It can even be tweaked to skip clusters that are too far. [Detailed explanation](http://arborjs.org/docs/barnes-hut).


### Build, CI, CD
CI/CD are both in GitHub actions, and automatically deployed to Netlify. The only build process is `wasm-pack` for Rust to WebAssembly, and Tailwindcss. This lean pipeline is only 1 minute of build time.
