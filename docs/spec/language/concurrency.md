# Zenith Concurrency Model

- Status: authoritative spec (updated for `0.4.2-beta.rc1`)
- Date: 2026-05-10
- Scope: isolate model, transfer boundaries, determinism rule, namespace state under concurrency
- Upstream: `docs/internal/decisions/language/087-concurrency-workers-and-transfer-boundaries.md`
- Cross-reference: `docs/spec/language/runtime-model.md` (sections `Concurrency Direction`, `Transferable Values`, `Implementation Phases`), `docs/internal/reports/R3.P1.A-namespace-shared-state-analysis.md`

## Purpose

Consolidate the official concurrency model of Zenith into a single authoritative document, so contributors do not need to cross-read `runtime-model.md`, `Decision 086`, `Decision 087`, and `R3.P1.A` to understand what is shipped today and what is deferred.

This document is normative. The implementation status section marks what is delivered vs deferred.

## Default Execution Model

1. Ordinary Zenith code executes on a single isolate (one logical owner of managed memory).
2. Managed values (`text`, `bytes`, `list<T>`, `map<K,V>`, structs, enums, `optional<T>`, `result<T,E>`) use non-atomic ARC.
3. There is no implicit cross-thread sharing of managed values.
4. The host (engine, runtime binding, FFI caller) may use threads internally, but crossing into Zenith managed memory must respect the boundary contract below.

Guarantees under this model:

- a function with identical input and identical reachable state produces identical output;
- a mutation inside a function cannot invisibly change state in unrelated code;
- an ordinary `public var` at namespace scope stays single-owner under concurrent execution (see `Namespace State` below).

## Boundary Contract

Crossing an isolate / worker / task boundary must use one of three modes:

1. **`copy`** - deep copy into the destination isolate.
   - This is the delivered boundary mode.
   - Surface: `std.concurrent.copy_*` helpers.
   - `Job<int>`, `Job<text>`, `Channel<int>`, and `Channel<text>` use this mode at runtime boundaries.
2. **`move`** - transfer of exclusive ownership when the compiler proves the source is no longer used.
   - Deferred to Phase 4.
3. **`shared wrapper`** - synchronized wrapper for narrow shared use cases.
   - Delivered only as the restricted `Shared<int>` facade in this cut.
   - `Atomic<int>` is the delivered atomic scalar facade.
   - General `Shared<T>` and non-`int` `Atomic<T>` remain deferred.

### Transferable shapes

A value is transferable if its shape is one of:

- a scalar (`int`, `float`, `bool`);
- `text`;
- `bytes`;
- `optional<T>` where `T` is transferable;
- `result<T, E>` where both `T` and `E` are transferable;
- `list<T>` where `T` is transferable;
- `map<K, V>` where both `K` and `V` are transferable;
- a struct or enum whose fields/payloads are transferable.

Not transferable by default:

- live platform handles;
- network connections;
- raw `extern`/FFI resources;
- engine scene objects with shared mutable identity;
- any managed value that was merely "reachable" but not explicitly transferred.

### Determinism rule

For any transferable value `v`, repeated copy calls must produce observationally equal results:

```zt
const a = std.concurrent.copy_int(v)
const b = std.concurrent.copy_int(v)
-- a == b must hold
```

For any `list<T>` or `map<K,V>`, mutation of the copy after a boundary crossing must not affect the source:

```zt
var copy = std.concurrent.copy_list_int(source)
copy[0] = 99
-- source[0] remains unchanged
```

These rules are enforced as behavior tests (see Tests section).

## Namespace State Under Concurrency (`public var`)

This section integrates the `R3.P1.A` analysis.

Normative rules:

- a `public var` is **owned** by the declaring namespace;
- the default model is **single-owner**: the variable is read/written only by code executing inside the declaring namespace;
- crossing a worker/task boundary does **not** share a `public var` automatically; the copy contract applies to any value that crosses;
- there is no implicit promotion from bare `public var` to a shared/atomic wrapper;
- cross-task shared mutation requires one of the explicit surfaces (`channels`, `Shared<T>`, `atomic<T>`).

These rules preserve the Decision 086 surface (read-public, write-owner) under concurrent execution.

## User-Facing API Direction

Delivered today:

- `std.concurrent.copy_int`
- `std.concurrent.copy_bool`
- `std.concurrent.copy_float`
- `std.concurrent.copy_text`
- `std.concurrent.copy_bytes`
- `std.concurrent.copy_list_int`
- `std.concurrent.copy_list_text`
- `std.concurrent.copy_map_text_text`
- typed handles: `std.jobs.Job<T>`, `std.channels.Channel<T>`, `std.shared.Shared<T>`, `std.atomic.Atomic<T>`
- generic facades currently monomorphized to backend-supported runtime ABIs:
  - `std.jobs.spawn(...) -> Job<int>` and `std.jobs.join(Job<int>) -> int`
  - `std.jobs.spawn(...) -> Job<text>` and `std.jobs.join(Job<text>) -> text`
  - `std.channels.create/send/receive/close` for `Channel<int>`
  - `std.channels.create/send/receive/close` for `Channel<text>`
  - `std.shared.create/get/set` for `Shared<int>`
  - `std.atomic.create/load/store/add` for `Atomic<int>`

Internal backend/runtime anchors (not public teaching surface):

- `spawn_int`, `spawn_text`, `join_int`, `join_text`, `create_int`, `create_text`, `send_int`, `send_text`, `receive_int`, `receive_text`, `close_int`, `close_text`, `get_int`, `set_int`, `load_int`, `store_int`, `add_int`

Canonical later direction:

```zt
-- future generalized payload expansion
const job = jobs.spawn(build_navmesh, snapshot)
const mesh = jobs.join(job)?

-- Phase 4
const channel = channels.create<Chunk>()
channels.send(channel, chunk)

-- Phase 5
var counter: Shared<int> = Shared.create(0)
```

Explicitly **not** part of any planned surface:

- raw thread handles;
- mutex/condvar-first programming;
- implicit cross-thread sharing of managed values;
- reintroduction of `global`.

## Implementation Phases

| Phase | Scope | Status |
|---|---|---|
| 1 | Boundary contract + `std.concurrent.copy_*` + transferability groundwork | delivered (this cut) |
| 2 | Checker understands `transferable` | delivered |
| 3 | `jobs.spawn/join` on copy-based transfer | `Job<int>` and `Job<text>` delivered |
| 4 | `channels` surface over explicit message passing | `Channel<int>` and `Channel<text>` delivered as single-slot non-blocking runtime handles |
| 5 | Explicit shared wrappers (`Shared<T>`, `atomic<T>`) | restricted `Shared<int>`/`Atomic<int>` facades delivered; wider payloads deferred |

Deferral is tracked in `docs/internal/archive/reports/legacy-main/R3-risk-matrix.md` as `R3-RISK-010`, `R3-RISK-011`, `R3-RISK-012`.

## Tests

Behavior fixtures covering the current cut:

- positive (copy helpers): `tests/behavior/std_concurrent_boundary_copy_basic`
- negative (non-transferable passed to `copy_text`): `tests/behavior/std_concurrent_boundary_copy_unsupported_error`
- determinism + boundary isolation + order: `tests/behavior/std_concurrent_boundary_copy_determinism`
- `Transferable` predicate positive/negative hardening: `tests/hardening/test_wave4_transferable_predicate.py`
- positive (`int` job spawn/join): `tests/behavior/std_jobs_int_basic`
- positive (`text` job spawn/join): `tests/behavior/std_jobs_text_basic`
- negative (closure rejected at job boundary): `tests/behavior/std_jobs_spawn_closure_error`
- Wave 4 concrete surface (`int` jobs/channels/shared/atomic): `tests/behavior/wave4_concurrency_surface`
- Wave 4 typed generic facade over `int` backend: `tests/behavior/wave4_concurrency_generic_surface`
- Wave 4 typed facade mismatch diagnostics: `tests/behavior/wave4_concurrency_generic_type_error`
- positive (`text` channel send/receive): `tests/behavior/std_channels_text_basic`
- runtime C text payload cleanup and copy boundary: `tests/runtime/c/test_concurrency_text.c`

Out of scope for this cut (waiting on wider runtime payload storage and runtime scheduling):

- race-condition fixtures;
- cancellation fixtures;
- runtime schedule nondeterminism fixtures.

## Non-Goals For `R3.M2`

- no shipping of arbitrary non-`int` runtime payload implementations for generalized handles in this milestone;
- no promotion of the existing copy helpers to a general-purpose concurrent API;
- no changes to the `public var` surface delivered in `R3.P1`.

## Residual Risk

- Generic transferable-copy remains deferred; current checking exposes the `Transferable` predicate and unified `concurrency.not_transferable` diagnostics, while copy helpers remain curated.
- Concurrency surfaces now expose typed handles and generic-looking calls over the concrete `int` and `text` job/channel runtime cut. Wider payload storage, cancellation, backpressure, and real scheduler policy remain deferred.
- The copy helpers cover a curated set of shapes; additional shapes (for example `list<struct>`) will need individual helpers or a generic transferable-copy surface before Phase 3 lands.
- Spec remains stable for delivered `int` and `text` job/channel surfaces. Each wider runtime payload phase requires its own spec revision in this document.
