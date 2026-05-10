# Zenith Stdlib Model Spec

- Status: canonical closure spec
- Date: 2026-04-22
- Scope: public standard library architecture and MVP module policy

## Purpose

The stdlib must extend Zenith without weakening the language philosophy.

Stdlib APIs should be explicit, typed, predictable and readable. Expected failure returns `result<T, E>`. Expected absence returns `optional<T>`. Panic is reserved for broken invariants and direct assertion-like operations.

## Module Set

The current MVP stdlib module set is:

- `std.io`
- `std.bytes`
- `std.fs`
- `std.fs.path`
- `std.json`
- `std.math`
- `std.text`
- `std.list`
- `std.map`
- `std.random`
- `std.validate`
- `std.time`
- `std.format`
- `std.lazy`
- `std.os`
- `std.os.process`
- `std.test`

`std.net` now ships a blocking TCP client foundation; TLS, UDP, server APIs, WebSocket, async IO, and stream integration remain future stdlib/network work.

## Import Policy

Stdlib modules are imported explicitly.

Canonical:

```zt
import std.io as io
import std.text as text
```

Core language traits and compiler intrinsics may be implicit only when they are part of the language semantic contract.

## Error Policy

Each side-effecting or fallible module owns its error type.

Examples:

- `io.Error`
- `fs.Error`
- `json.Error`
- `time.Error`
- `os.Error`
- `process.Error`

Rules:

- expected failure returns `result<T, Module.Error>`
- absence returns `optional<T>`
- timeout is an error variant, not panic and not absence
- invalid direct indexing may panic
- safe lookup APIs should return `optional<T>`

## Type Policy

Modules may introduce own public types when raw `text`, `int`, `list` or `map` would hide important semantics.

Required module-owned types include:

- `io.Input`
- `io.Output`
- `io.Error`
- `json.Value`
- `json.Object`
- `json.Array`
- `json.Kind`
- `json.Error`
- `time.Instant`
- `time.Duration`
- `time.Error`
- `fs.Metadata`
- `fs.Error`
- `format.BytesStyle`
- `os.Platform`
- `os.Arch`
- `os.Error`
- `process.Output`
- `process.Error`

## Naming Policy

Names should prefer clear verbs and nouns over abbreviated cleverness.

Rules:

- mutating methods are declared with `mut func`
- safe lookup functions should include names such as `get`, `find` or `try_` only when the distinction is needed
- whole-file byte APIs may be deferred until binary/runtime support is complete
- path operations live in `std.fs.path`, not `std.fs`

## Namespace Mutable State Policy (`public var`)

`public var` may exist in stdlib only when shared module state is a clear part of the contract.

Rules:

- prefer `public const` by default
- use `public var` only at namespace top-level
- allow external read through qualified import
- block external write outside the owner namespace
- `public` is visibility only; it is not `global`
- mutation should be exposed by explicit `public func` APIs
- tests must reseed/reset state to avoid order-dependent flakes

Current first-slice adoption:

- `std.random` exposes read-only observable module state to callers:
  - `seeded`
  - `last_seed`
  - `draw_count`
  - `stats()`

## Test Helper Policy

`std.test` is a small helper module for tests.

It complements `attr test` and core `check(...)`.
It must not replace test discovery.

Public helper categories:

- explicit outcomes: `fail(...)`, `skip(...)`
- bool checks: `is_true(...)`, `is_false(...)`
- simple comparisons: `equal_int(...)`, `equal_text(...)`, `not_equal_int(...)`, `not_equal_text(...)`

Comparison failures should include expected and received values when that helps the user fix the test.

## Numeric Math Policy

`std.math` exposes finite numeric constants as true constants:

- `pi`
- `e`
- `tau`

Non-finite values stay as functions in this cut:

- `infinity()`
- `nan()`

Reason: the current constant model has finite float literals, but no dedicated non-finite float literal. Creating non-finite values through runtime functions avoids division-by-zero tricks in the stdlib source and keeps the public API honest.

Float equality follows IEEE behavior, so `nan() == nan()` is false. Ordered comparison with `NaN` is a runtime math error with code `runtime.float_nan_compare`; callers should check `math.is_nan(value)` before ordering possibly non-finite values.

## Safe Collections API

Direct access remains strict:

```zt
const value: text = scores["Julia"]
```

If the key is missing, this is a runtime map-key panic.

Safe access must exist early:

```zt
const score_key: text = "Julia"
const maybe_score: optional<int> = scores.get(score_key)
const maybe_item: optional<int> = values.get(3)
const has_julia: bool = map.has_key(scores, score_key)
```

`std.list` and `std.map` also expose small compiler-known helpers:

- `list.is_empty(items)`
- `map.is_empty(values)`
- `map.has_key(values, key)`

The semantic rule is fixed: expected absence checks must not require panic.

Basic `std.list` value helpers are written as generic `list<T>` APIs. In the current C backend, they execute for primitive lists and `list<text>`:

- `first`, `last`, `rest`, `skip`
- `append`, `prepend`, `contains`, `reverse`, `concat`, `index_of`
- `set`, `remove_first`, `remove_last`, `remove_at`, `slice`

Managed struct, enum, and fully generic managed-list helper coverage remains future work.

`std.collections` is a current-subset module for advanced structures. It does
not promise arbitrary `grid2d<T>`, `pqueue<T>`, `circbuf<T>`, `btreemap<K,V>`,
or `btreeset<T>` storage in v1.

Current support matrix:

| Structure | v1 executable support |
| --- | --- |
| queue / stack | `list<int>`, `list<text>`, plus compiler-known `queue_values<T>` and `stack_values<T>` snapshots for list-backed values |
| grid2d / grid3d | `grid2d<int>`, `grid2d<text>`, `grid3d<int>`, `grid3d<text>` |
| pqueue | `pqueue<int>`, `pqueue<text>` |
| circbuf | `circbuf<int>`, `circbuf<text>` |
| btreemap | `btreemap<text,text>` |
| btreeset | `btreeset<text>` |
| HOF helpers | `map_int`, `filter_int`, `reduce_int` |

`std.collections` exposes iterable snapshots as lists for the supported
specialized structures:

- `queue_values<T>` and `stack_values<T>` are compiler-known because queues and stacks are list-backed.
- `grid2d_*_values` uses row-major order.
- `grid3d_*_values` uses layer, row, column order.
- `pqueue_*_values` uses pop order without mutating the heap.
- `circbuf_*_values` uses oldest-to-newest order.
- `btreemap_text_keys`, `btreemap_text_values`, and `btreeset_text_values` use sorted text order.

Fully generic advanced collection storage is explicit post-RC technical debt.
Priority queues and B-tree structures also require a backend ordering relation
or explicit comparator contract for their element or key type.

## Higher-Order Collection Helpers

R3.M7 introduced a narrow higher-order subset in `std.collections`; Wave 7.1 widens the executable C-backend surface.

Current stable helpers through `std.list`:

- `map(values: list<T>, mapper: func(T) -> T) -> list<T>` for executable primitive and `text` lists
- `filter(values: list<T>, predicate: func(T) -> bool) -> list<T>` for executable primitive and `text` lists
- `find(values: list<T>, predicate: func(T) -> bool) -> optional<T>` for executable primitive and `text` lists
- `any/all/count(values: list<T>, predicate: func(T) -> bool)` for executable primitive and `text` lists
- `sort_by(values: list<T>, key_selector: func(T) -> int) -> list<T>` for executable primitive and `text` lists
- `reduce(values: list<int>, initial: int, reducer: func(int, int) -> int) -> int`

Rules:

- helpers are same-type for executable primitive and `text` lists in the current C backend subset
- cross-type `map<T,U>` and generic `reduce<T>` remain deferred
- callbacks use normal `func(...)` closure values
- callbacks may capture immutable values
- hot-path users should benchmark before replacing explicit loops

## Explicit Lazy Helpers

R3.M8 introduces `std.lazy` as a narrow explicit lazy module.

Current helpers:

- `once_int(thunk: func() -> int) -> lazy<int>`
- `force_int(value: lazy<int>) -> int`
- `is_consumed_int(value: lazy<int>) -> bool`
- matching `once_*` / `force_*` / `is_consumed_*` helper families for `float`, `bool`, and `text`

Rules:

- users must import the module explicitly
- executable `lazy<int>`, `lazy<float>`, `lazy<bool>`, and `lazy<text>` values are one-shot in this cut
- there is no implicit lazy in collection helpers
- reusable lazy and lazy iterators remain future work

## Implementation Gate

A stdlib function is complete only when it has:

- public signature in docs
- runtime/backend implementation
- typed error behavior if fallible
- behavior tests
- diagnostic tests when misuse is likely
- examples using canonical import style

## Deferred

Deferred after Wave 7 executable C-backend closure:

- full public generic stream abstraction
- async IO via jobs and channels
- TLS
- websocket
- package registry integration
- optional dependencies and feature flags

