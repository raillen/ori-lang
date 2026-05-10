# Decision 087 - Concurrency, Workers, and Transfer Boundaries

- Status: accepted
- Date: 2026-04-23
- Type: language / runtime / architecture
- Scope: user-facing concurrency model, transfer rules, shared wrappers, implementation phases

## Summary

Zenith keeps the default runtime path simple:

- one isolate at a time
- non-atomic ARC for ordinary managed values
- no implicit cross-thread sharing of ordinary `text`, `bytes`, `list`, `map`, structs or enums

Concurrency is still part of the language direction.

The difference is the shape:

- explicit workers/jobs/channels
- explicit transfer
- narrow shared wrappers only when truly needed

## What This Decision Means

This does **not** mean:

- "the whole program must be single-threaded"
- "games cannot use threads"
- "the host engine must stop doing parallel work"

This **does** mean:

- ordinary managed Zenith values stay on a single-isolate path by default
- the host or engine can use threads internally
- cross-thread work must happen through clear boundaries

## Default Rule

The default rule is:

- normal Zenith code is sequential and value-oriented
- parallel work is explicit
- shared mutable state is the exception, not the baseline

This keeps the language readable and keeps ownership bugs out of everyday code.

## Boundary Modes

Crossing a worker/isolate boundary must use one of these modes:

1. `copy`
   - deep copy into the destination isolate
   - this is the first and safest default
2. `move`
   - future optimization
   - only when the compiler can prove the source is no longer used
3. `shared wrapper`
   - explicit synchronized wrapper
   - reserved for narrow cases such as shared text/bytes snapshots

## Current Alpha Delivery

The first public slice is intentionally small.

Today the tree ships explicit typed copy helpers in `std.concurrent`:

- `copy_int`
- `copy_bool`
- `copy_float`
- `copy_text`
- `copy_bytes`
- `copy_list_int`
- `copy_list_text`
- `copy_map_text_text`

These helpers are the current public way to make a transfer boundary explicit in user code.
They are not a replacement for `jobs.spawn/join`.
They are the safe copy-based base layer that `jobs`, `workers` and `channels` can build on later.

## Transferable Data

The worker boundary should only accept transferable data.

Transferable by shape:

- `int`
- `float`
- `bool`
- `text`
- `bytes`
- `optional<T>` when `T` is transferable
- `result<T, E>` when both channels are transferable
- `list<T>` when `T` is transferable
- `map<K, V>` when `K` and `V` are transferable
- structs and enums whose fields/payloads are all transferable

Not transferable by default:

- network connections
- raw FFI handles
- engine scene objects with live shared identity
- host resources that require thread affinity
- ordinary managed values that were never explicitly transferred

## User-Facing Direction

Canonical direction:

```zt
const job = jobs.spawn(build_navmesh, snapshot)
const mesh = jobs.join(job)?
```

Possible later direction:

```zt
const channel = channels.create<Chunk>()
channels.send(channel, chunk)
```

Not the initial direction:

- raw thread handles
- mutex-first programming
- implicit shared mutation of ordinary collections

## Why This Fits Games

This model works well for:

- gameplay scripts on the main isolate
- asset loading in workers
- pathfinding/navmesh generation in workers
- build tools and content pipelines
- engine-side render/audio/input threads managed by the host

This is not yet the model for:

- free shared mutation of live game graphs across threads
- ECS/job-system style arbitrary shared writes from ordinary Zenith values

## Implementation Phases

### Phase 1

- make the runtime contract explicit
- make boundary-copy helpers explicit
- test transfer-by-copy behavior

Status in tree:

- delivered

### Phase 2

- teach the checker what `transferable` means
- reject worker payloads that carry non-transferable values

Status in tree:

- groundwork delivered in the checker
- public worker API still pending

### Phase 3

- add `jobs.spawn/join`
- use copy-based transfer first

### Phase 4

- optimize eligible payloads with move-based transfer
- add channels where the payload rule stays explicit

### Phase 5

- add narrow explicit shared wrappers only where profiling and product needs justify them

## Namespace State And Concurrency (R3.P1.A Integration)

This section integrates the `R3.P1.A` future analysis into the concurrency model.

Reference: `docs/internal/reports/R3.P1.A-namespace-shared-state-analysis.md`.

Normative rules under this decision:

- A `public var` declared at namespace scope is **owned by the declaring namespace** for execution purposes.
- The default execution contract is **single-owner**: the namespace variable is only read/written by code executing inside the declaring namespace.
- On a worker/task boundary crossing, a `public var` is **not automatically shared**. The copy-based transfer contract described above applies; shared identity is not preserved across the boundary unless an explicit wrapper is used.
- There is **no implicit promotion** from a bare `public var` to a shared/atomic wrapper.
- Cross-task shared mutation requires one of:
  - an explicit `Shared<T>` wrapper declared at the variable site (Phase 5); or
  - explicit message passing via a later `channels` surface (Phase 4).
- `atomic<T>` (when introduced) is opt-in per variable and applies only to primitive scalars; it is not a replacement for channel-based coordination.

This keeps the guarantee that `public` is visibility, not concurrent access, and that the existing `public var` surface (read-public, write-owner) remains valid under concurrent use once Phases 3-5 land.

## Consequences

- Zenith keeps its reading-first, explicit style
- the runtime keeps fast single-isolate defaults
- concurrency grows as a clear feature, not as implicit shared-state magic
- namespace `public var` stays single-owner by default under concurrent execution
