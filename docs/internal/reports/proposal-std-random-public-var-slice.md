# Proposal: std.random State Slice (R3.P1)

Date: 2026-04-22
Owner: language/std
Status: implemented (2026-04-22)

## Goal

Use namespace mutable state in `std.random` with low risk and clear behavior.

This slice validates `public var` in a real stdlib module without spreading mutable state to pure modules.

## Why this module first

- `std.random` already has mutable behavior (`seed` affects future `next`).
- Decision 081 points to deterministic flows and future generator model.
- External write to namespace `public var` is blocked by language rules, reducing risk.

## Proposed API (minimal)

File: `stdlib/std/random.zt`

Keep existing API:

- `public func seed(seed: int) -> void`
- `public func next() -> int`
- `public func between(min: int, max: int) -> result<int, text>`

Add observable namespace state:

```zt
public var seeded: bool = false
public var last_seed: int = 0
public var draw_count: int = 0
```

Add stable read API:

```zt
public struct Stats
    seeded: bool
    last_seed: int
    draw_count: int
end

public func stats() -> random.Stats
```

Behavior contract:

- `seed(x)`:
  - calls host seed
  - sets `seeded = true`
  - sets `last_seed = x`
  - resets `draw_count = 0`
- `next()`:
  - calls host next
  - increments `draw_count`
- `between(min, max)`:
  - if `max < min`, returns error (no draw)
  - if `min == max`, returns `success(min)` (no draw)
  - otherwise uses one `next()` draw (increments once)

## Why this shape

- `public var` gives direct read visibility for diagnostics and test tooling.
- `stats()` gives a stable API so callers do not depend on three loose fields forever.
- Resetting `draw_count` on `seed` keeps deterministic tests simple.

## Test plan (behavior)

1. Extend deterministic baseline:
- file: `tests/behavior/std_random_basic/src/app/main.zt`
- keep existing deterministic checks.
- assert `seeded`, `last_seed`, `draw_count` after calls.

2. Add state-focused fixture:
- new: `tests/behavior/std_random_state_observability/`
- checks:
  - after `seed(7)`: `seeded == true`, `last_seed == 7`, `draw_count == 0`
  - after one `next()`: `draw_count == 1`
  - after one `between(10,20)`: `draw_count == 2`
  - `stats()` mirrors vars.

3. Add no-draw branches fixture:
- new: `tests/behavior/std_random_between_branches/`
- checks:
  - `between(5,5)` does not increment `draw_count`
  - `between(8,3)` returns error and does not increment `draw_count`

4. Add cross-namespace write negative fixture:
- new: `tests/behavior/std_random_cross_namespace_write_error/`
- attempt:
  - `random.draw_count = 0`
- expected: mutability/visibility write denial.

5. Matrix/docs update:
- update `tests/behavior/MATRIX.md`
- update `tests/behavior/README.md`

## Implementation status

Delivered in this repository on 2026-04-22:

- `stdlib/std/random.zt` now exposes `public var seeded`, `public var last_seed`, `public var draw_count`.
- Added `public struct Stats` and `public func stats() -> random.Stats`.
- Updated behavior fixture `tests/behavior/std_random_basic` with state assertions.
- Added new behavior fixtures:
  - `tests/behavior/std_random_state_observability`
  - `tests/behavior/std_random_between_branches`
  - `tests/behavior/std_random_cross_namespace_write_error`
- Added diagnostics fragment:
  - `tests/fixtures/diagnostics/std_random_cross_namespace_write_error.contains.txt`
- Updated behavior matrix/readme and suite mappings for the new invalid fixture.

## Compatibility and risk

Compatibility:

- no break in current call signatures (`seed/next/between`).
- existing users can ignore new fields/functions.

Risks:

- callers may over-couple to `public var` fields.
- state can make tests order-sensitive if not reseeded.

Mitigation:

- recommend `seed(...)` at test start.
- keep `stats()` as preferred read API for future migration.

## Forward path (post-slice)

Phase 2 should move deterministic-heavy workloads to instance generators:

- `random.Generator(seed: int)`
- `generator.next()`
- `generator.between(...)`

Namespace state can remain as module-level convenience telemetry, not as the only deterministic model.
