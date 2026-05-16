# Native ABI contract

Status: implementation contract for the current native route.

Date: 2026-05-14.

This document describes the ABI used between:

```text
Ori HIR -> native backend -> ori-runtime
```

It is intentionally small and concrete. If the backend and runtime disagree
with this file, the implementation is wrong.

## Direct values

These values are passed directly in native registers or stack slots:

| Ori type | Native ABI type |
| --- | --- |
| `bool` | `i8`, where `0` is false and non-zero is true |
| `int`, `int64`, `u64` | `i64` |
| `int32`, `u32` | `i32` |
| `int16`, `u16` | `i16` |
| `int8`, `u8` | `i8` |
| `float`, `float64` | `f64` |
| `float32` | `f32` |
| `tree.NodeId` | `i64` arena index scoped to one `tree.Tree<T>` |
| `void`, `never` | no return value |

## Runtime handles

Everything below is represented as a native pointer-sized handle:

| Ori type family | Handle target |
| --- | --- |
| `string` | null-terminated byte payload |
| `bytes` | null-terminated byte payload; embedded zero bytes are not supported by the current helper layer |
| `list<T>` | `OriList*` |
| `map<K,V>` | `OriMap*` |
| `set<T>` | `OriSet*` |
| `tree.Tree<T>` | `OriTree*` |
| `hash_table.HashTable<K,V>` | `OriMap*` |
| `graph.Graph<N>` | `OriGraph*` |
| `heap.Heap<T>` | `OriHeap*` |
| `option<T>` | tagged runtime aggregate pointer |
| `result<T,E>` | tagged runtime aggregate pointer |
| tuple values | runtime aggregate pointer |
| structs and enums | runtime aggregate pointer |
| closures and `func(...) -> ...` values | closure object pointer |
| `any<Trait>` | trait object pointer |
| `lazy<T>` | lazy cell pointer |
| `future<T>` | `OriFuture*` |
| `task.Job<T>` | `OriTaskJob*` |
| `channel.Channel<T>` | `OriChannel*` |
| `atomic.AtomicInt` | `OriAtomicInt*` |
| task/channel error marker types | opaque pointer handle |

The backend treats these as managed values for retain/release decisions unless a
more specific rule is documented in code.

## Internal async runtime helpers

These symbols are internal ABI helpers for native async lowering. They are not
public stdlib functions:

- `ori_future_poll`
- `ori_future_value_i64`
- `ori_future_value_f64`
- `ori_future_value_ptr`
- `ori_future_pending`
- `ori_future_on_ready`
- `ori_future_complete_i64`
- `ori_future_complete_f64`
- `ori_future_complete_ptr`
- `ori_future_complete_void`
- `ori_future_fail`
- `ori_future_cancel`
- `ori_executor_schedule`
- `ori_executor_run_one`
- `ori_executor_drain`

`ori_future_on_ready` stores a continuation while the future is pending. When
the future becomes ready, failed or cancelled, the continuation is pushed into
the executor FIFO queue. `ori_executor_schedule` consumes a closure object using
the same closure layout already used by `task.spawn`.

`ori_future_pending` returns a caller-owned pending `future<T>` handle. It is an
internal compiler/runtime ABI hook for generated async state machines: the public
async wrapper allocates the result future first, stores it in the frame, schedules
the first `step`, and returns the pending future immediately.

Pointer futures own a managed result while the future is alive. `ori_future_ready_ptr`
and `ori_future_complete_ptr` retain registered ARC payloads before storing them.
Non-managed pointers and static strings are ignored by ARC release, as usual.

The runtime still exports these legacy internal bridge helpers for compatibility
and runtime tests, but the native backend no longer imports them for
source-level `await` lowering:

- `ori_async_spawn_i64`
- `ori_async_spawn_f64`
- `ori_async_spawn_ptr`
- `ori_async_spawn_void`
- `ori_task_last_await_status`

Generated `await` must be driven by state-machine frames plus
`ori_future_poll`/`ori_future_on_ready`. `task.block_on` is not part of this
lowering path; it is only the explicit synchronous bridge exposed by `ori.task`
and used by process/test entry points.

## Runtime ownership rules

Current rules:

- Inputs passed into runtime functions are borrowed unless the function name or
  lowering explicitly retains them.
- A runtime function returning a pointer returns a value owned by the caller at
  the Ori semantic level.
- ARC-managed values are allocated through `ori_alloc` and released through
  `ori_arc_release`.
- `ori_arc_retain`, `ori_arc_release`, and edge helpers ignore null pointers.
- `ori_arc_release` also ignores pointers that are not registered ARC objects.
  This keeps static strings and legacy raw string buffers from crashing.
- Containers own their internal storage buffer. User-facing insertions of
  managed elements are paired with compiler-emitted ARC edge registration.
  Runtime-created snapshots and derived collections also register ARC edges for
  managed elements before returning them.
- `ori_map` and `ori_set` have specialized string-key helpers because generic
  equality/hash lowering is still ABI-sensitive.
- `ori_map_clear` and `ori_set_clear` remove dense entries and reset the
  key/element-kind marker, but keep allocated capacity for reuse.
- `ori_map_reserve` and `ori_set_reserve` guarantee a minimum dense capacity;
  `ori_map_capacity` and `ori_set_capacity` report that dense capacity.
- `ori_deque`, `ori_queue`, `ori_stack`, `ori_linked_list`, and
  `ori_doubly_linked_list` currently reuse the `OriList` handle layout. Empty
  read/removal operations return an `optional<T>` handle. Linked-list nodes are
  not public ABI objects in v1. The compiler still exposes these as distinct
  opaque stdlib types, not as public `list<T>` aliases.
- `ori_tree` uses an arena handle. Runtime calls pass `tree.NodeId` as `i64`.
  `tree.children` and traversal functions return `OriList*` snapshots of node
  ids. `tree.remove_subtree` unregisters ARC edges for removed node values and
  invalid node ids abort with `ori tree node id is invalid`.
- `ori_hash_table` intentionally reuses the `OriMap*` layout and hash engine.
  It differs at the language API level by returning `optional<V>` from
  `get/remove` and by exposing explicit `with_capacity`.
- `ori_map.keys`, `ori_map.values`, `ori_map.entries`, and the `hash_table`
  APIs that reuse those helpers return snapshots that keep managed keys,
  values, and entry tuples alive independently from the source map/table.
- `ori_graph` stores dense node and edge arrays. Node payloads are word-sized
  values, with specialized string entry points for content comparison. Traversal
  functions return `OriList*`; `nodes`, `neighbors`, BFS/DFS/topological
  snapshots, and `edges` retain managed node values while the returned snapshot
  is alive. `edges` returns tuple payload pointers with `tuple<N,N>` layout.
- Runtime list derivation helpers such as slice/copy/filter/take/skip/reverse,
  sort/unique, partition, flatten/flat_map, zip, group_by, random.shuffle,
  string.split/chars, os.args, and fs.list_dir either retain borrowed managed
  elements for the returned collection or transfer owned managed values into
  the returned collection before releasing local ownership.
- `ori_heap` stores a binary min-heap in a dense word-sized array. `int` heaps
  use built-in numeric ordering, `string` heaps use string content ordering,
  and user-defined `Comparable` heaps store a native compare function pointer
  selected by the compiler during `heap.new<T>()` lowering.

Known limitation:

- String and bytes payloads produced by the native runtime helper layer are
  null-terminated and ARC-registered through `ori_alloc`. Releasing them with
  `ori_arc_release` frees the payload.
- The current helper layer still treats embedded NUL bytes as unsupported for
  `bytes` values that pass through C-string shaped ABI helpers. This is a data
  model limitation, not an ownership leak.
- Static or borrowed string pointers may still be passed into runtime calls by
  generated code. `ori_arc_retain` and `ori_arc_release` intentionally ignore
  non-registered pointers.

## Destructors, finalizers and cleanup failures

Runtime destructors are implementation hooks attached to objects allocated by
`ori_alloc`. They must:

- free only storage owned by the runtime object;
- tolerate partially initialized objects;
- avoid calling user code;
- avoid panicking.

If a destructor, finalizer or cleanup hook panics while native cleanup is in
progress, the current contract treats that as a fatal runtime failure. The
runtime does not promise recovery after cleanup panic. Generated code should
therefore keep cleanup paths small and deterministic.

`using` cleanup calls user-visible dispose functions. Those calls are part of
language semantics and are handled separately from low-level ARC destructors.

## Caller responsibilities

The native backend must:

- retain managed arguments when a runtime call can store them beyond the call;
- release managed temporaries when they leave scope;
- update ARC edges when assigning managed fields;
- preserve managed values stored in closures, tuples, enums, options, results,
  lists, maps, sets and async state;
- not emit direct calls to undocumented runtime symbols.

The runtime must:

- keep `#[repr(C)]` on structs used directly by the ABI;
- keep layout tests for every handle whose fields are read by the backend;
- export every symbol listed by the stdlib manifest as native;
- keep internal helper symbols documented in the backend allowlist.

## Link metadata

`runtime-link.json` records:

- target triple;
- runtime artifact name;
- Ori compiler version;
- native ABI version;
- profile used to stage the runtime;
- native system libraries required by the static library.

The driver rejects mismatched target, compiler version or ABI version before
calling the linker.

## Export check

Use this command to compare:

- stdlib native manifest symbols;
- native backend runtime declarations;
- real symbols exported by the compiled `ori-runtime` static library.

```powershell
.\tools\check_native_runtime_exports.ps1
```

The script uses `llvm-nm` or `nm` and fails if a manifest symbol or backend
runtime import is missing from the compiled runtime artifact.
