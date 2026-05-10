# Zenith Final Conformance Matrix (R2.M11)

- Status: conformant for RC 2.0 gate
- Date: 2026-04-22
- Scope: compiler + runtime + tooling + perf gates for release candidate

## Inputs Used

- `docs/spec/language/implementation-status.md`
- `docs/spec/language/surface-implementation-status.md`
- `tests/behavior/MATRIX.md`
- `reports/suites/smoke__latest.json`
- `reports/suites/pr_gate__latest.json`
- `reports/suites/nightly__latest.json`
- `reports/suites/stress__latest.json`
- `reports/perf/summary-nightly.json`

## Gate Snapshot

| Gate | Status | Evidence |
| --- | --- | --- |
| `smoke` | `Conformant` | `9/9` pass (`reports/suites/smoke__latest.json`) |
| `pr_gate` | `Conformant` | `112/112` pass (`reports/suites/pr_gate__latest.json`) |
| `nightly` | `Conformant` | `114/114` pass (`reports/suites/nightly__latest.json`) |
| `stress` | `Conformant` | `21/21` pass (`reports/suites/stress__latest.json`) |
| Perf nightly | `Conformant` | `status=pass`, `23` benchmarks (`reports/perf/summary-nightly.json`) |

## Layer Matrix

Canonical labels: `Spec`, `Parsed`, `Semantic`, `Lowered`, `Emitted`, `Runtime`, `Executable`, `Conformant`, `Risk`, `Deferred`, `Rejected`.

| Area | Parser | Semantic | Lowered | Emitted | Runtime | Overall |
| --- | --- | --- | --- | --- | --- | --- |
| Project model (`.ztproj`, namespace/imports) | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Core syntax/control-flow/functions | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Structs/traits/apply/methods | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Namespace `public var` (read public, write owner namespace) | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Collections + optional/result + `?` | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Enum match + exhaustiveness | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |
| Formatter (`zt fmt --check`) | `Spec` | `-` | `-` | `-` | `Executable` | `Conformant` |
| ZDoc (`zt doc check`) | `Spec` | `Semantic` | `-` | `-` | `Executable` | `Conformant` |
| Stdlib MVP (`std.*`) | `Parsed` | `Semantic` | `Lowered` | `Emitted` | `Executable` | `Conformant` |

## Risk Matrix

| Risk Item | Label | Current State |
| --- | --- | --- |
| RC cycles strategy (ownership graph) | `Risk` | tracked, not blocking RC gate |
| Long-horizon monomorphization scale | `Risk` | gate active (`monomorphization_limit_error`) |
| Runtime diagnostic code taxonomy drift | `Risk` | tracked as P2 docs/runtime alignment |

## RC Statement

Release candidate 2.0 gate is green on all mandatory suites and nightly perf.

No open P0 blocker was found in this cut.

## Feature Delta (2026-04-22)

- `public var` is implemented at namespace scope with controlled mutation:
  - external read allowed through alias;
  - external write rejected with `mutability.invalid_update`.
- stdlib pilot is live in `std.random` with observable state and behavior coverage.
