# Decision 081: Stdlib Scope and Architecture (Collections and Random)

- Status: accepted
- Date: 2026-04-18
- Type: stdlib / architecture
- Scope: std.collections and std.random direction for MVP and post-MVP

## Context

After the core architecture closure, Zenith needed a strict boundary between language primitives and rich library modules.

The goal is to keep syntax small and explicit while moving advanced data structures and random generation behavior into stdlib modules with predictable C-backend implementation.

## Decision

### 1. Text I/O and interpolation remain explicit

- `io.print(text)` accepts only `text`.
- Explicit conversion is done with `to_text(value)`.
- Canonical interpolation uses `fmt "..."`.
- Advanced presentation formatting belongs to `std.format`.

### 2. Collections live in `std.collections`

First-wave collection scope includes:

1. `Queue<T>`, `Stack<T>`, `CircularBuffer<T>`
2. `PriorityQueue<T>`
3. `BTreeMap<K, V>`, `BTreeSet<T>`
4. `Grid2D<T>`, `Grid3D<T>` implemented as flat arrays

Rules:

- prefer explicit mutation names such as `append` and `prepend`
- safe absence should use `optional` (for example `map.get(key) -> optional<V>`)
- avoid hidden behavior in language syntax for collection-specific features

### 3. Random generation lives in `std.random`

`std.random` should provide both ergonomic and deterministic flows:

- ergonomic API examples: `random.float()`, `random.int(min: 0, max: 100)`, `random.choice(items)`
- deterministic generator API example: `random.Generator(seed: 12345)`

The runtime must avoid weak platform RNG defaults and keep deterministic behavior explicit for reproducible workloads.

## Consequences

- Stdlib growth does not force new syntax primitives.
- M19 implementation can stay focused on backend/runtime quality.
- Advanced domains (game engine specific structures, heavy 3D systems) remain better suited to package-level libraries when outside core stdlib scope.