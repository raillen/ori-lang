# C.14 - Primitive Numeric Lists Benchmark

Date: 2026-04-28

## Scope

Roadmap item:
- C.14 - Benchmarks: before/after for numeric lists.

Implemented benchmark:
- `tests/perf/primitive_numeric_lists`
- Runner id: `micro_primitive_numeric_lists`
- Scenario: `micro_runtime`

The benchmark exercises:
- `list<int>` read, write, `len`, and slice
- `list<float>` read, write, `len`, and slice
- `list<int32>` read, write, `len`, and slice
- `list<u32>` read, write, `len`, and slice

## Current Result

Command:

```powershell
python tests/perf/run_perf.py micro_primitive_numeric_lists --update-baseline
```

Result:
- status: `pass`
- platform: `windows-AMD64`
- median latency: `900.437 ms`
- p95 latency: `919.105 ms`
- peak working set: `161382400`
- binary size: `381597`

Baseline saved:
- `tests/perf/baselines/windows-AMD64/micro_primitive_numeric_lists.json`

## Full Quick Gate

Command:

```powershell
python tests/perf/run_perf.py --suite quick --release-gate
```

Result:
- overall status: `fail`
- `micro_primitive_numeric_lists`: `pass`
- failing existing benchmarks: `micro_stdlib_core`, `macro_small_build_warm`, `macro_medium_check`

This means the new C.14 benchmark is healthy, but the broader perf quick gate still has unrelated baseline failures.

## Generated C Evidence

The generated benchmark C uses specialized primitive list helpers:
- `zt_list_i64_from_array`, `zt_list_i64_get`, `zt_list_i64_set_owned`, `zt_list_i64_slice`
- `zt_list_f64_from_array`, `zt_list_f64_get`, `zt_list_f64_set_owned`, `zt_list_f64_slice`
- `zt_list_i32_from_array`, `zt_list_i32_get`, `zt_list_i32_set_owned`, `zt_list_i32_slice`
- `zt_list_u32_from_array`, `zt_list_u32_get`, `zt_list_u32_set_owned`, `zt_list_u32_slice`

No `zt_retain` call appears in the generated benchmark hot paths for primitive list elements.

## Before/After Note

There was no dedicated numeric-list benchmark before this item. The previous checked-in perf runner also did not contain `micro_primitive_numeric_lists`.

Because the current worktree already contains the primitive-list specialization work, a true historical before run cannot be reconstructed safely from this dirty workspace without creating a separate clean checkout. This item now provides the durable "after" baseline and the runner hook needed for real before/after comparisons on future changes.

