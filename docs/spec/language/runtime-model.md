# Zenith Runtime Model Spec

- Status: canonical closure spec
- Date: 2026-05-10
- Scope: C runtime, managed values, value semantics, cleanup, panic, contracts, checks and `std.mem` ownership intent APIs

## Purpose

The runtime must make Zenith's simple source model true.

The user sees value semantics, immutable `const` values, explicit mutation and typed recoverable errors. The implementation may use reference counting, copy-on-write and internal moves, but those choices must not leak shared mutable aliasing.

## Managed Values

Managed values include:

- `text`
- `bytes`
- `func(...)` closures
- `lazy<T>` values
- `list<T>`
- `map<K, V>`
- `optional<T>` when `T` is managed
- `result<T, E>` when either side is managed
- structs with managed fields
- enums with managed payloads

## Closure Runtime Values

Closure values are managed runtime values.

A closure value stores:

- a function pointer
- a context pointer
- an optional context drop hook

The context pointer stores captured values.

Rules:

- captures are immutable snapshots in closures v1
- managed captures are retained when the closure is created
- managed captures are released when the closure is released
- non-capturing functions still use the same `func(...)` value model
- generated Zenith functions receive an internal `zt_ctx` argument
- normal user code never writes or reads `zt_ctx`

## Lazy Runtime Values

`lazy<T>` values are managed runtime values.

The current executable backend ships `lazy<int>`, `lazy<float>`, `lazy<bool>`, and `lazy<text>` as one-shot values.
Other `lazy<T>` payloads are rejected during `zt check` in
`0.4.2-beta.rc1`; fully generic lazy storage remains post-RC work.

A lazy value stores:

- a closure thunk
- a consumed flag

Rules:

- creating the lazy value retains the thunk
- forcing the value calls the thunk once
- forcing releases the stored thunk after the call
- forcing the same value again raises `runtime.contract`
- ordinary expression evaluation remains eager

## Ownership Requirements

Runtime/compiler ownership rules must handle:

- local scope exit
- normal `return`
- early return through `?`
- branch exits
- loop exits through `break` and `continue`
- temporaries until end of statement
- function argument evaluation left-to-right
- construction failure
- contract panic where cleanup is viable

Current executable evidence:

- `using_basic` covers normal scope exit, early return, `?` propagation,
  loops, `break` and `continue`;
- `using_disposable_auto` covers automatic `Disposable.dispose()` cleanup;
- `using_panic_cleanup` covers cleanup before the current fatal panic path;
- `extern_c_callback_user_data_basic` covers a top-level C callback with
  explicit `user_data` and `using` cleanup inside the callback body.

## Generic Runtime Identity

Generic runtime helpers are internal implementation details, but their identity
must be stable.

Rules:

- each concrete generic shape has a canonical compiler/runtime type identity
- generated C helper names include a readable sanitized shape plus a stable hash
  suffix
- the hash is based on canonical identity, not only the simple source name
- two namespaces may define the same simple struct name without colliding in
  generated list, set, map, or element-ops helpers
- public Zenith code must not depend on the generated C helper spelling

Regression coverage:

- `tests/behavior/generic_helper_name_collision_safe`

## Ownership Intent Facade

`std.mem` is an advanced library surface for explicit ownership intent.
It is not ownership syntax and does not add lifetimes, `owned<T>`, `borrow<T>`,
`move`, or `ref` to Zenith source.

Compiler-known helpers:

- `mem.own(value) -> T`
- `mem.view(value) -> T`
- `mem.edit(value) -> T`

The 0.4.2-beta.rc1 executable subset accepts only shapes whose clone, retain,
move, destroy and editable-copy behavior is already supported by the C oracle:

- primitive scalars;
- `text`;
- safe tuples and structs made from scalar/text fields;
- primitive/text lists;
- `list<safe tuple>` and `list<safe struct>`;
- `set<int>` and `set<text>`;
- maps with `int` or `text` keys and scalar/text values.

Unsupported nested mutable managed shapes fail during checking with concrete
diagnostics. This is intentional: `mem.edit` must not return a retained alias
where the user expects an isolated editable value.

Regression coverage:

- `tests/behavior/std_mem_generic_facade_basic`
- `tests/behavior/std_mem_generic_facade_unsupported_type_error`
- `tests/behavior/std_mem_appendix_b_values`
- `tests/behavior/std_mem_appendix_b_*_deferred_error`

## Memory and Concurrency Core

The MVP is solidified on **Automatic Reference Counting (ARC)** without a tracing Garbage Collector. 

To maintain latency predictability and C-level speed, the default runtime uses **Non-Atomic ARC**. Concurrency relies on **Isolates** (message-passing/deep copy between boundaries). Atomic RC is restricted to designated explicit wrappers (e.g. `Shared<T>`).

This is a runtime contract, not a promise that the whole host process is single-threaded.

Practical reading:

- Zenith user code runs on a **single-isolate by default** path.
- The host/engine may use threads internally.
- Ordinary managed Zenith values are not shared across threads by default.
- Future user-facing concurrency should arrive as **workers/jobs/channels**, not as implicit shared mutable state.

This architecture creates a known limitation: RC cycles. 

Rules:

- ARC tracks all managed values.
- Non-Atomic ARC cannot cross thread isolates; doing so requires a safe deep-transfer structure.
- RC cycles are a leak risk, not undefined behavior
- rich callbacks, UI graphs, game object graphs and stored reference-like APIs are not stable until a cycle policy exists
- future cycle policy must choose an explicit mechanism such as `weak<T>`, handles/arenas, constrained ownership graphs or cycle collection
- the runtime must document which APIs can create cycles before those APIs become official

0.4.2-beta.rc1 does not introduce a new public cycle-forming API. For that
reason, weak references and broad automatic cycle collection remain deferred.
`std.orc.collect_cycles()` stays an advanced diagnostic/runtime hook for the
heap kinds supported today, not a promise that every future object graph is
automatically collected.

## Concurrency Direction

Zenith does **not** frame this as "the language is single-thread only".

The correct model is:

- default runtime path: single-isolate
- concurrency boundary: explicit isolate/worker/job boundary
- default transfer mode: deep copy
- future optimized transfer mode: move when exclusivity is provable
- explicit shared state: narrow wrappers only

This keeps ordinary code simple and predictable, while still leaving room for parallel hosts, game engines and worker-based APIs.

## Transferable Values

The long-term worker boundary should accept only transferable data.

Baseline transferable shapes:

- scalars (`int`, `float`, `bool`)
- `text`
- `bytes`
- `optional<T>` when `T` is transferable
- `result<T, E>` when both channels are transferable
- `list<T>` when `T` is transferable
- `map<K, V>` when `K` and `V` are transferable
- structs and enums whose fields/payloads are all transferable

Not transferable by default:

- live platform handles
- network connections
- raw `extern`/FFI resources
- engine scene objects with shared mutable identity
- ordinary managed values that are merely "reachable", but not explicitly transferred

## User-Facing API Direction

The intended public surface is small and explicit.

Current delivered 0.4.2-beta.rc1 slice:

- `std.concurrent.copy_int`
- `std.concurrent.copy_bool`
- `std.concurrent.copy_float`
- `std.concurrent.copy_text`
- `std.concurrent.copy_bytes`
- `std.concurrent.copy_list_int`
- `std.concurrent.copy_list_text`
- `std.concurrent.copy_map_text_text`
- `std.jobs.Job<int>` and `std.jobs.Job<text>`
- `std.channels.Channel<int>` and `std.channels.Channel<text>`
- `std.shared.Shared<int>`
- `std.atomic.Atomic<int>`

These helpers make the boundary explicit today. Jobs and channels remain
copy-based for the current supported payloads. Wider payloads must either lower
to typed runtime wrappers or fail with capability diagnostics.

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

Not part of the initial surface:

- raw thread handles
- mutex/condvar-first programming
- implicit cross-thread sharing of ordinary `text`, `list`, `map` and structs

## Implementation Phases

1. Document the runtime contract and make boundary-copy helpers explicit.
2. Teach the checker what "transferable" means.
3. Expose `jobs.spawn/join` on top of copy-based transfer.
4. Add move-based optimization where exclusivity is provable.
5. Add narrow explicit shared wrappers only where they are truly needed.

Current progress in the 0.4.2-beta.rc1 tree:

- Phase 1 delivered.
- Public boundary helpers are available in `std.concurrent`.
- `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>` and
  `Atomic<int>` are executable.
- Transferability and runtime capability checks reject unsupported wider
  payloads before lowering.

## Advanced Allocation Resources

Temporary regions and object pools are accepted as future library values, not
as language features.

Reserved names:

- `mem.Temp` for scoped temporary region/scratch allocation;
- `mem.Pool<T>` for reusable fixed-shape storage.

They are not exposed in 0.4.2-beta.rc1. The current `std.mem` facade already
covers the ownership-intent gap needed for this train, while `Temp` and `Pool`
still need real API pressure and dedicated cleanup fixtures before they should
be public.

When added, these resources must be usable through `using`, must clean up
deterministically on all supported exits, and must be restricted to payload
shapes whose clone/destroy/edit behavior is already proven.

## Stack Vs Heap Representation

`optional<T>` and `result<T,E>` should be stack/in-place values whenever practical.

Rules:

- scalar optionals/results should not allocate
- `result<void, E>` success should not allocate in the final runtime model
- managed payloads may store RC-managed pointers internally while the wrapper stays stack/in-place
- heap allocation is acceptable for escaping values, oversized payloads, managed payload internals or bootstrap implementation cuts
- heap-first wrappers are performance debt and must be tracked explicitly

## Value Semantics

Assignment and parameter passing create semantic value copies.

Implementation may use:

- direct copy for scalars
- retain/release for immutable managed values
- copy-on-write for mutable managed containers
- internal move when source is provably unused

Observable rule:

```zt
var b: list<int> = a
b[0] = 10
```

must not change `a`.

## Const Collections

`const` collections are observably immutable.

Invalid:

```zt
const items: list<Player> = [Player(name: "Julia", hp: 100)]
items[0] = Player(name: "Julia", hp: 80)
items[0].hp = 80
```

## Checks

Runtime checks include:

- list/text/bytes bounds
- map missing key for direct lookup
- division by zero where applicable
- contract `where`
- invalid UTF-8 conversion
- allocation failure
- platform failure

Expected platform failures should become `result<T, E>` in stdlib APIs. Broken invariants and direct invalid access produce panic.

## Panic (Safe Bounds and Contracts)

Zenith injects **Panic with Unwinding** (or controlled abort) rather than allowing Undefined Behavior when bounds check contracts are fundamentally broken at runtime.

Rules:

- Indexing out-of-bounds (e.g., `list[999]`) or arithmetic overflow checks inject an unconditional panic, avoiding segfaults and heap corruption.
- Panic is fatal language control flow, guaranteeing safe behavior defaults.
- Panic output must use the diagnostics model when source span and value context are available.
- Panic is not caught by result/optional flow.

## Runtime Contracts

Value-level `where` contracts run at runtime.

A struct constructor remains a constructor. It does not secretly become `result<T, E>` when fields contain `where`.

Expected recoverable validation should be written as an explicit result-returning API, such as `try_create_*`.

Canonical recoverable validation example:

```zt
struct User
    age: int where it >= 0
end

func try_create_user(age: int) -> result<User, text>
    if age < 0
        return error("age must be >= 0")
    end

    return success(User(age: age))
end
```

Required MVP sites:

- struct field construction
- struct field assignment
- function and method parameter boundary

Failed contracts report `runtime.contract`.

## Definition Of Done

Runtime hardening is complete only when:

- managed values have explicit ownership rules
- retain/release paths are tested
- COW/value semantics are behavior-tested
- all supported early exits release correctly
- runtime failures report structured diagnostics
- const collection mutation is rejected or impossible through supported operations
- RC cycle policy is documented before cycle-prone APIs become stable
- optional/result wrappers use stack/in-place representation where practical
- heap-first wrappers are tracked as performance debt
