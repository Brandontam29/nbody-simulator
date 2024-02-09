# nbody-simulator
## [View project](https://nbody-simulator.netlify.app/)

NBody simulator on a HTML canvas where all calculation are performed via WASM Rust.

### Time complexity

Right now, it is O(n^2). We calculate the gravity force of every particle from every other particle (bad).

This will also use the Barnes-Hut Algorithm to reduce the time complexity to O(n logn). In short, this algorithm separates each node into a unique quadtree which makes it very simple to search for particles that are at meaningful distances. This would be the case for gravity, a force that gets exponentially weaker in regards to distance and collision. [Detailed explanation](http://arborjs.org/docs/barnes-hut)




### Build, CI, CD

Everything is very lean simple. The only build process is `wasm-pack` for Rust to WebAssembly, and Tailwindcss. CI/CD are both in GitHub actions, and automatically deployed to Netlify. It only takes about 1 minute of build time.
