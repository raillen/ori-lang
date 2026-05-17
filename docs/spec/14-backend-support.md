# Backend support matrix

Status: current as of 2026-05-17.

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
| Hash tables, trees, graphs, heaps | yes | yes | partial | Native tests cover stdlib operations. |
| `bytes` with internal NUL | yes | yes | partial | `string` still rejects internal NUL at conversion boundary. |
| Unicode `string.len`, `slice`, `index_of` | yes | yes | partial | Indices are Unicode scalar indices, not byte offsets. |
| Async functions and `await` | yes | partial | no | Native supports the subset below. C/debug rejects async. |
| `using` resource cleanup | yes | yes | partial | Async `using` is still rejected until cleanup across suspension is designed. |
| LSP diagnostics positions | yes | yes | n/a | LSP uses UTF-16 columns and handles CRLF. |

## Native async subset

Supported today:

- `await future` as a top-level expression statement.
- `const x: T = await future`.
- `return await future`.
- `const x: T = (await future)?`.
- `await` inside top-level return expressions.
- `await` inside top-level call arguments.
- `await` inside top-level operators.
- `await` inside top-level statement conditions, such as `if await flag()`.
- Multiple awaits in the same simple async function, as long as the state machine can order them before the tail block.

Still blocked:

- `await` inside the body of an `if`, `else`, `match`, loop, `if some`, or other nested statement body.
- `await` inside loops where each iteration would need its own suspension state.
- `using` inside async functions.

Current failure mode:

- Native codegen emits `backend.native_unsupported` with a direct message when a shape is outside the state machine subset.

## `backend.native_unsupported` inventory

| Code path | Classification | Current action |
| --- | --- | --- |
| Async outside state-machine subset | partial async support | Keep clear diagnostic until nested states land. |
| Raw `emit_await` outside async lowering | internal defense | Keep error; direct `await` must be lowered before expression codegen. |
| Indexed assignment base unsupported | backend gap | Add fixture when general indexed assignment lowering is expanded. |
| `for` iterable type without native ABI | backend gap | Add fixture per iterable when support is promised. |
| Unknown map/hash_table/graph/set/tree/heap runtime call | internal defense | Keep error; valid stdlib paths should resolve before this point. |

## Rules for future work

- Add a positive native test before changing a row from partial to yes.
- Keep a negative test when a shape is intentionally blocked.
- Update this matrix in the same commit as the implementation change.
- Do not call async "complete" while any promised `await` shape still reaches `backend.native_unsupported`.
