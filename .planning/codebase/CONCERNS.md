# Concerns

## Technical Debt
- **JS Testing**: Lack of automated unit or integration tests for the frontend logic.
- **State Management**: Using multiple JS files for state without a centralized store or framework might lead to synchronization issues as complexity grows.
- **Commented Code**: `particle.rs` contains commented-out tests which should be addressed.

## Performance
- **N-body Scaling**: While Barnes-Hut is $O(N \log N)$, performance with very high $N$ in WASM/JS will still hit browser limitations.
- **Memory Management**: Frequent passing of data between JS and WASM might introduce overhead if not optimized (e.g., using shared memory or minimizing allocations).

## Maintainability
- The custom UI utilities (`removeElementsByIndices.js`, `register.js`) are bespoke and might be replaced by standard library methods or lightweight frameworks if they become bottlenecks.
