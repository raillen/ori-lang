# Stdlib Public Var Analysis (2026-04-22)

Status note (2026-04-22):

- this report started as a pre-implementation audit;
- the recommended first slice is now implemented in `stdlib/std/random.zt`;
- other module recommendations remain valid (no broad rollout of `public var`).

## Scope

Audit target:

- `stdlib/std/*.zt` and submodules
- related design decisions in `docs/internal/decisions/language/*`

Question:

- which stdlib modules really need namespace `public var`
- which modules could benefit, but do not need it now

## Snapshot (pre-implementation audit)

Current code snapshot:

- audited modules: 16
- modules with `public var`: 0
- modules with top-level `var`: 0
- model in use today: mostly stateless wrappers + `public func` + some `public const`

## Current status after R3.P1 implementation

- modules with `public var`: 1 (`std.random`)
- implemented public state in `std.random`:
  - `seeded`
  - `last_seed`
  - `draw_count`
  - `stats()`
- behavior coverage added:
  - `std_random_basic`
  - `std_random_state_observability`
  - `std_random_between_branches`
  - `std_random_cross_namespace_write_error`

## Decision Matrix

Legend:

- Need now: should enter near-term backlog
- Benefit: expected gain if adopted carefully

| Module | Need now | Benefit | Recommendation |
|---|---|---|---|
| `std.random` | Yes (high) | High | Add controlled namespace state for deterministic flow observability (short-term bridge), while planning `random.Generator(seed)` as canonical long-term model. |
| `std.io` | No (now) | Medium | Keep current `public const` defaults for now. Revisit only if you need runtime output redirection policy in tests/tooling. |
| `std.test` | No (now) | Medium | Keep module minimal for MVP. Consider counters/telemetry only if runner/reporting contract expands. |
| `std.os` | No | Low/Medium | Prefer direct host query calls. If caching is needed, use private namespace `var`, not `public var`. |
| `std.net` | No | Low (for `public var`) | Do not add global timeout state. Keep timeout policy explicit per connection. |
| `std.time` | No | Low (for `public var`) | Do not add configurable global state for `now()`. Keep explicit API only. |
| `std.fs` | No | Low | Keep API side-effecting only through function calls; no namespace mutable policy knobs now. |
| `std.fs.path` | No | Very low | Keep pure module contract. No namespace mutable state. |
| `std.text` | No | Very low | Keep pure module contract. No namespace mutable state. |
| `std.format` | No | Very low | Keep pure module contract. No namespace mutable state. |
| `std.math` | No | Very low | Keep pure and deterministic contract. No namespace mutable state. |
| `std.validate` | No | Very low | Keep pure predicates only. |
| `std.bytes` | No | Very low | Keep pure conversion/helpers only. |
| `std.collections` | No | Very low | Keep state inside collection values, not namespace state. |
| `std.json` | No | Low | Keep parse/stringify/read/write explicit; avoid namespace mutable defaults now. |
| `std.os.process` | No | Low | Keep process execution explicit per call; no namespace mutable policy knobs now. |

## Why only `std.random` is high-priority

`std.random` already depends on mutable runtime behavior (`seed` and `next`).

Near-term, namespace state can improve debug/repro visibility (for example, last seed and draw count) without opening cross-namespace writes.

But the direction in architecture docs already points to a stronger model:

- deterministic generator API (`random.Generator(seed: ...)`) should be the primary long-term shape.

So recommendation:

1. short-term: minimal controlled namespace state in `std.random`
2. medium-term: move deterministic workflows to instance-based generator API

## Guardrails (do not break existing contracts)

- `std.net`: no hidden global timeout
- `std.time`: `now()` meaning must not be globally configurable
- pure modules stay pure (`std.text`, `std.format`, `std.math`, `std.fs.path`)
- `std.test` remains intentionally small in MVP unless runner scope expands

## Suggested first implementation slice

If you want to start now, do this only in `std.random`:

- add internal namespace mutable state for observability (not as external write point)
- expose explicit API (`seed`, `next`, `between`, plus optional read-only stats helpers)
- add behavior tests for deterministic repeatability and reset/isolation

Do not introduce `public var` in other stdlib modules in this same slice.

## References used

- `stdlib/std/random.zt`
- `stdlib/std/io.zt`
- `stdlib/std/net.zt`
- `stdlib/std/time.zt`
- `stdlib/std/test.zt`
- `docs/internal/decisions/language/081-stdlib-collections-and-random.md`
- `docs/internal/decisions/language/074-network-blocking-and-timeout-policy.md`
- `docs/internal/decisions/language/063-stdlib-time.md`
- `docs/internal/decisions/language/060-stdlib-format.md`
- `docs/internal/decisions/language/059-stdlib-text.md`
- `docs/internal/decisions/language/062-stdlib-math.md`
- `docs/internal/decisions/language/065-stdlib-fs-path.md`
- `docs/internal/decisions/language/058-stdlib-io.md`
- `docs/internal/decisions/language/069-stdlib-test.md`
