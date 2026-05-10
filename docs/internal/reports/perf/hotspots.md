# Zenith Performance Hotspots Report (R2.M4)

## Summary
Performance analysis of Zenith 2.0 (M36/M37) identified critical bottlenecks in the backend compilation and potential regressions in the frontend.

## Identified Hotspots

### 1. Native Compilation (Critical)
- **Description**: `zt_compile_c_file` in `compiler/driver/pipeline.c` invokes `gcc` to compile the generated C code AND the runtime library (`runtime/c/zenith_rt.c`) for every build or run.
- **Impact**: Adds a baseline overhead of ~1s to any project, even for a single-line "hello world".
- **Recommended Action**: Pre-compile `zenith_rt.c` into a static library or an object file and link against it.

### 2. Frontend Latency Regression (Major)
- **Description**: Recent benchmarks show a 26.7% increase in latency for `macro_small_check` against the baseline.
- **Impact**: Slower development cycle and potentially non-linear scaling for large modules.
- **Recommended Action**: Profile `lexer` and `parser` with `perf` (Linux) or `VS Diagnostic Tools` (Windows) to identify the source of the regression.

### 3. Monomorphization Limit Enforcement (Minor)
- **Description**: `zt_enforce_monomorphization_limit` performs a full ZIR module scan after lowering to count generic instances.
- **Impact**: Performance degrades linearly with module size.
- **Recommended Action**: Move instance counting to the lowering phase or implement a more efficient ZIR traversal.

## Prioritized Plan
1.  **M5 (Compiler Optimization)**: Implement pre-compiled runtime linking to eliminate the ~1s overhead.
2.  **M5 (Compiler Optimization)**: Profile and optimize the parser to address the 26.7% regression.
3.  **M6 (Runtime Optimization)**: Review reference counting costs in `zenith_rt.c` for hot paths.
