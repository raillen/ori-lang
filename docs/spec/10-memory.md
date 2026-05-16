# Ori Language Specification — Chapter 10: Memory and Cleanup

> Status: normative
> Audience: compiler implementers

---

## Overview

Ori manages memory automatically. There are no explicit `malloc`/`free` calls.
Memory safety is guaranteed without requiring the programmer to track ownership
or lifetimes in the source language.

The memory model has two layers:
1. **Value semantics** — the default behavior for all types.
2. **Automatic Reference Counting (ARC)** — for managed types.

---

## Value Semantics

All types in Ori have value semantics by default.

```ori
const a: Point = Point(x: 1, y: 2)
const b: Point = a          -- b is a copy of a
```

Assigning a struct copies all its fields. There is no aliasing between `a`
and `b`. Mutating `b` (if `var`) does not affect `a`.

This applies to:
- All primitive types
- Structs
- Enums
- Tuples

---

## Managed Types

`string`, `bytes`, and all collection types (`list<T>`, `map<K, V>`, `set<T>`)
are **managed types**. They are heap-allocated and reference-counted.

Assigning a managed type copies the **reference**, not the heap data:

```ori
const a: list<int> = [1, 2, 3]
const b: list<int> = a          -- b holds the same reference as a
```

The heap data is freed when the last reference to it goes out of scope.

In practice, Ori's value semantics and immutability mean that managed types
behave as if they were copied — mutations produce new values, not in-place
changes — so the distinction is invisible in most code.

---

## Automatic Reference Counting (ARC)

The compiler inserts retain/release calls at compile time based on lexical scope.

Rules:
- A reference is retained when it is stored in a binding or passed to a function.
- A reference is released when the binding goes out of scope.
- In the Rust runtime used by the native backend, retain/release use atomic
  reference counts.
- The native backend links against the Rust `ori-runtime` static library. That
  runtime is the source of truth for managed values, ARC behavior, and runtime
  symbols used by `ori compile` and `ori test`.

**Cycle detection:** the native runtime tracks compiler-registered strong edges
between managed heap objects. `ori_arc_collect_cycles()` reclaims unreachable
cycles from that registered graph. The return value is the number of heap
objects reclaimed in the pass.

### Backend Status

- The native backend inserts ARC retain/release calls for managed values.
- The Rust `ori-runtime` crate provides the runtime symbols consumed by the
  native backend.
- The standalone C backend remains a debug/transpile backend with partial
  feature parity. Its inline ARC runtime exists only for generated C output and
  does not define core language semantics.

---

## Async and ARC

Managed values that remain live across an `await` must stay retained until the
async continuation can use them again.

Current native status:

- `await` is accepted only inside `async func`.
- The runtime has pollable `future<T>` values, continuation registration, a FIFO
  executor queue, failed/cancelled internal states, and non-blocking timers.
- The current backend creates a `future<T>` immediately when an `async func` is
  called, allocates a native async frame, and schedules the generated `step`
  function on the native executor.
- Supported source-level `await` shapes suspend through `ori_future_poll` and
  `ori_future_on_ready`; they do not call `task.block_on`.
- Managed params, pre-await locals and await bindings stored in the frame have
  ARC edges. The state machine calculates liveness after each `await`, releases
  dead managed frame values after resumption, and still runs terminal cleanup.
- Failed/cancelled future states observed by the state machine are propagated by
  the generated async wrapper.
- `using` inside `async func` is rejected with `async.using_unsupported` until
  the compiler can guarantee deterministic cleanup across every suspension,
  return, failed future, and cancelled future path.

Async shapes outside the current state-machine subset are rejected before
Cranelift with `backend.native_unsupported` instead of falling back to a sync
bridge.

---

## `using` — Deterministic Cleanup

For resources that need explicit cleanup (file handles, network connections,
database connections), use `using`:

```ori
func read_file(path: string) -> result<string, string>
    using file: ori.fs.File = ori.fs.open_read(path)?
    const content: string = ori.fs.read_all(file)?
    return success(content)
end
```

When `file` goes out of scope, `file.dispose()` is called automatically.

### Cleanup Guarantee

`using` cleanup runs on **every** exit path from the scope:

- Normal `end` of block
- `return` statement
- `?` propagation (error path)
- `break` or `continue` in a loop
- Panic

### Cleanup Order (LIFO)

Multiple `using` bindings are disposed in **reverse declaration order**:

```ori
using a: ResourceA = open_a()?
using b: ResourceB = open_b()?
using c: ResourceC = open_c()?
-- When scope exits: c.dispose(), then b.dispose(), then a.dispose()
```

### `Disposable` Trait

A type participates in `using` by implementing `Disposable`:

```ori
trait Disposable
    mut func dispose()
end
```

Attempting to use a type in `using` that does not implement `Disposable`
is a compile error.

### Interaction with `?`

```ori
using conn: Connection = get_connection()?
-- If get_connection() returns error: conn is never bound, nothing to dispose.

const data: bytes = conn.fetch(url)?
-- If fetch() returns error: conn.dispose() IS called before error propagates.
```

---

## Stack vs Heap Allocation

The compiler decides allocation strategy. The programmer does not control this.

General rules (subject to optimization):
- Primitive types and small structs: stack-allocated.
- Managed types (`string`, `bytes`, collections): heap-allocated.
- `any<Trait>` values: heap-allocated (boxed).
- Closures that capture values: heap-allocated if they escape the current scope.

---

## No Manual Memory Management

Ori does not expose:
- Pointer arithmetic
- `malloc` / `free` / `realloc`
- Raw pointers (except through `extern c` FFI, where they must be handled carefully)
- Stack allocation directives

`ori.unsafe` provides escape hatches for low-level operations, but its use
is restricted to explicit `unsafe` annotated contexts and is not part of
the ordinary language surface.

---

## FFI Memory

When crossing the `extern c` boundary:
- Ori-managed values may not be passed as raw pointers without explicit conversion.
- C-allocated memory returned to Ori must be wrapped in a type implementing
  `Disposable` that frees it via the appropriate C function.
- The programmer is responsible for memory safety at FFI boundaries.

See the FFI documentation for detailed ABI shapes.

---

## `ori.mem`

`ori.mem` provides explicit memory inspection utilities:

```ori
import ori.mem as mem

mem.size_of(value)         -- size in bytes of value's static type
mem.align_of(value)        -- alignment in bytes of value's static type
```

These are compile-time constants. The argument is used as a type witness and
is not needed in ordinary code.

Current status: `ori.mem` is importable. The current parser does not support
type-argument call syntax such as `size_of<T>()`, so code should use the
expression-based form above.
