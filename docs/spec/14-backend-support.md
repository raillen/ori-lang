# Backend support matrix

Status: current as of 2026-06-27.

This page separates three things:

- Language promise: the feature is part of Ori.
- Native backend: Cranelift plus packaged Rust runtime.
- C/debug backend: C source emission used for debug and compatibility checks.

Legend:

- yes: supported and covered by tests.
- partial: supported only for a documented subset.
- no: intentionally rejected today.
- internal: only an internal defensive error path.

## Summary

| Feature | Language promise | Native backend | C/debug backend | Notes |
| --- | --- | --- | --- | --- |
| Basic expressions and statements | yes | yes | partial | Native is the main execution path. C/debug is not full parity. |
| Functions and imports | yes | yes | partial | Native tests cover local imports, transitive imports and entry namespace. |
| Structs, enums and tuples | yes | yes | partial | Native ABI has layout tests. |
| Traits and `any<Trait>` | yes | yes | partial | Native tests cover dynamic dispatch. |
| Generics and monomorphization | yes | yes | partial | Native tests cover generic functions and imported generic traits. |
| Lists, maps, sets, deques, queues, stacks | yes | yes | partial | Native runtime owns ARC edges. |
| Structural equality | yes | yes | partial | Native and C/debug cover primitives, `bytes`, `optional`, `result`, tuples, lists, generic structs, `set<T>`, and `map<K,V>` when keys/elements support equality. |
| Hash tables, trees, graphs, heaps | yes | yes | partial | Native tests cover stdlib operations. |
| JSON (`json.parse` / `json.Value`) | yes | yes | partial | C backend emits `ori_json_parse` FFI stubs without dedicated C lowering; execution requires native runtime. |
| `bytes` with internal NUL | yes | yes | partial | `string` still rejects internal NUL at conversion boundary. |
| Unicode `string.len`, `slice`, `index_of` | yes | yes | partial | Indices are Unicode scalar indices, not byte offsets. |
| Async functions and `await` | yes | partial | no | Native supports the subset below. C/debug rejects async. |
| `using` resource cleanup | yes | yes | partial | Sync and async `using` supported; async dispose on normal return, `?`, cancel, fail, and `break`. |
| `lazy.once` / `lazy.force` | yes | yes | partial | Native uses inline Cranelift codegen; C backend has dedicated lowering. |
| LSP diagnostics positions | yes | yes | n/a | LSP uses UTF-16 columns and handles CRLF. |

## Native async subset

Supported today (covered by `concurrency_async.rs`):

- `await` inside `if`, `else`, `match`, `while`, `for`, and other control-flow bodies (branching state machine).
- `await future` as a top-level expression statement.
- `const x: T = await future`.
- `return await future`.
- `const x: T = (await future)?`.
- `await` inside top-level return expressions, call arguments, and operators.
- `await` inside top-level statement conditions, such as `if await flag()`.
- `using` inside `async func` with `dispose()` on scope exit, cancellation, failure, propagate (`?`), and `break`.
- Multiple awaits in the same async function with preserved ARC locals across suspensions.

Still blocked or limited:

- Async functions whose `await` shapes cannot be lowered to the simple or general state machine (nested awaits not lifted).
- `for` over iterable types without native iterator ABI (`int`, raw ranges, unsupported opaques).
- Indexed assignment on unsupported managed bases.

Current failure mode:

- Native codegen emits `backend.native_unsupported` with a direct message when a shape is outside the supported subset.

## `backend.native_unsupported` inventory

| Message / code path | Classification | Tests |
| --- | --- | --- |
| Async function contains an `await` shape not covered by the state machine | partial async | Negative: shapes that fail `collect_general_async_plan` |
| Indexed assignment base unsupported | backend gap | No positive test yet |
| `` `for` iterable type `{ty}` `` | backend gap | `compile_runs_*` per supported opaque iterator |
| `` `for` element type `{ty}` `` | backend gap | Same |
| `` map runtime call `{name}` `` | internal defense | Stdlib paths should resolve before emission |
| `` hash_table runtime call `{name}` `` | internal defense | Same |
| `` graph runtime call `{name}` `` | internal defense | Same |
| `` set runtime call `{name}` `` | internal defense | Same |
| `` tree runtime call `{name}` `` | internal defense | Same |
| `` heap runtime call `{name}` `` | internal defense | Same |

## C/debug backend scope

Intentionally **not** supported on the C route:

- `async func`, `await`, `task.*`, `channel.*`, `atomic.*`
- `json.parse` / structured `json.Value` (C emits FFI stubs only; no dedicated C lowering)

C/debug **does** support (see `multifile_imports.rs` `build_c_backend_*`):

- Structural equality (structs, lists, maps, sets)
- `lazy.once` / `lazy.force` lowering
- Stdlib surfaces: math, format, os, time, random, mem, iter (partial), test asserts

## C/debug backend stdlib matrix (`c_backend` flag)

The `stdlib!` macro in `compiler/crates/ori-types/src/stdlib.rs` tags each
runtime function with a `c_backend` flag. When the flag is set, the C/debug
backend ships a matching implementation in its inline runtime header
(`ORI_RUNTIME_H`, enforced by the `c_backend_inline_runtime_exports_manifest_symbols`
test). Functions without the flag are native-runtime-only: the C backend may
still emit them as `extern` calls or lower them via dedicated code paths
(structural equality, string concat, lazy), but they do not have a C runtime
body and require the native Rust runtime to actually execute.

Legend:

- **yes**: every function in the module carries the `c_backend` flag.
- **partial**: a subset of the module carries the flag (see Notes).
- **no**: no function carries the flag; C backend emits extern stubs or dedicated lowerings only.
- **inline**: handled by inline C codegen, not runtime FFI (no flag needed).

| Module | `c_backend` flag | C execution | Notes |
| --- | --- | --- | --- |
| `io.print`, `io.println` | yes | yes | Flagged. |
| `io.eprint`, `io.eprintln`, `io.read_line` | no | extern only | Native runtime provides the body. |
| `math.*` | yes | yes | All 16 functions flagged. |
| `time.now`, `time.sleep`, `time.duration_ms` | yes | yes | All flagged. |
| `format.*` | yes | yes | number, percent, hex, binary, date, datetime, bytes_size. |
| `os.*` | yes | yes | args, env, exit, pid, platform, arch. |
| `random.*` | yes | yes | int, float, bool, choice, shuffle. |
| `iter.*` | yes | yes | map, filter, any, all, count_where, take, skip, reverse, reduce, find, sort, sort_by, unique, flat_map, zip, partition, group_by, flatten. |
| `test.assert`, `test.assert_eq`, `test.assert_ne`, `test.fail` | yes | yes | Flagged. |
| `test.live_allocations`, `test.collect_cycles`, `test.assert_no_leaks` | no | extern only | Leak checks require native ARC runtime. |
| `string` (global), `int`, `float` builtins | yes | yes | Conversion builtins flagged. |
| `len` (global builtin) | no | extern only | Native runtime. |
| `string.*` | no | dedicated lowering + extern | C backend lowers `concat` inline; remaining ops are extern stubs. |
| `bytes.*` | no | extern only | Native runtime. |
| `convert.*` | no | extern only | float_to_string, bool_to_string, string_to_int, string_to_float. |
| `list.*`, `deque.*`, `queue.*`, `stack.*` | no | extern only | Native runtime owns ARC edges. |
| `linked_list.*`, `doubly_linked_list.*` | no | extern only | Native runtime. |
| `tree.*` | no | extern only | Native runtime. |
| `map.*`, `set.*`, `hash_table.*` | no | dedicated lowering + extern | C backend lowers structural equality and iterator ABI; ops are extern. |
| `graph.*`, `heap.*` | no | extern only | Native runtime. |
| `json.parse`, `json.stringify`, `json.stringify_pretty` | no | extern stub | C emits FFI stub without dedicated lowering; execution requires native runtime. |
| `fs.*`, `files.*` | no | extern only | Native runtime. |
| `task.*`, `channel.*`, `atomic.*` | no | rejected | C backend rejects async/concurrency symbols entirely. |
| `lazy.once`, `lazy.force` | inline | yes | Inline C codegen; no runtime FFI flag. |
| `panic` | no | extern only | Native runtime. |

### Rules for the `c_backend` flag

- Adding a new stdlib function with a C runtime body: use the `c_backend`
  variant of `stdlib!` and add the matching body to `ORI_RUNTIME_H`. The
  `c_backend_inline_runtime_exports_manifest_symbols` test enforces consistency.
- Adding a native-only function: omit the flag. Document the native-only
  constraint in this matrix.
- Changing a row from `no` to `yes` requires a positive `build_c_backend_*`
  test in `multifile_imports.rs`.

## C/debug async parity (v2 backlog — deferred)

Full async/concurrency parity in the C/debug backend is **not planned for v1**.
The native Cranelift backend is the reference implementation for `async func`,
`await`, `task.*`, `channel.*`, and `atomic.*`.

Current C/debug behaviour (unchanged until v2):

- `async func` / `await` in user code: rejected at C codegen with an actionable
  message (`backend.c_unsupported` via `ori build`).
- Stdlib async/concurrency symbols: rejected at C codegen (same route).
- Sync subset (`ori build` on non-async programs): supported per the matrix above.

Rationale: async on native uses a dedicated state machine, ARC frame edges, and
runtime executor hooks that would duplicate a large fraction of `ori-runtime`
in `ORI_RUNTIME_H`. The C route remains a **debug/transpile** path for sync
programs, not a second production backend.

Future options (v2, pick one):

1. **Selective parity** — inline executor stubs for a minimal async subset.
2. **Explicit deprecation** — document C backend as sync-only permanently.
3. **Shared IR** — generate async state machines in a backend-agnostic layer
   (large refactor).

Until a v2 decision lands, do not mark C async as partial/yes in the matrix.

## Rules for future work

- Add a positive native test before changing a row from partial to yes.
- Keep a negative test when a shape is intentionally blocked.
- Update this matrix in the same commit as the implementation change.
- Do not call async "complete" while any promised `await` shape still reaches `backend.native_unsupported`.
