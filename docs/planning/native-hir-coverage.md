# Native HIR coverage

Status: implementation contract.

Audience: compiler maintainers.

The native backend must make every HIR shape explicit. A valid HIR node should
either have a native codegen path or fail before Cranelift with a clear
`backend.native_unsupported` message.

The regression tests in `compiler/crates/ori-codegen/src/native_backend.rs`
compare these matrices with `HirExprKind` and `HirStmt`. If a new HIR variant is
added, the test fails until the native backend coverage is updated.

## Expression matrix

| HIR expression | Native backend behavior |
|---|---|
| `BoolLit` | Emits direct bool literal. |
| `IntLit` | Emits direct integer literal with the typed Cranelift width. |
| `FloatLit` | Emits `f32` or `f64` literal. |
| `StrLit` | Emits a managed runtime string constant. |
| `InterpolatedStr` | Emits string parts and calls native string helpers. |
| `BytesLit` | Emits managed runtime bytes. |
| `Unit` | Emits the direct zero-sized void/unit representation. |
| `Var` | Loads local, global, or constant value. |
| `Binary` | Emits scalar operators or helper-backed operations. |
| `Unary` | Emits numeric negation or logical not. |
| `Field` | Loads from the native struct layout. |
| `Index` | Emits list, string, or bytes indexing. Unsupported typed input gets a clear native backend error. |
| `TupleIndex` | Loads from the native tuple layout. |
| `Call` | Emits function, runtime helper, stdlib, or closure calls. |
| `MethodCall` | Emits slice helpers, dynamic `any<Trait>` method dispatch, or resolved function call. |
| `StructLit` | Allocates struct storage and stores fields by native layout. |
| `EnumVariant` | Allocates enum storage, writes tag, and stores payload fields by native layout. |
| `ListLit` | Builds `ori.list` through native runtime calls. |
| `ListSpreadLit` | Builds `ori.list` and expands spread elements through native runtime calls. |
| `TupleLit` | Allocates tuple storage and stores elements by native layout. |
| `Some_` | Builds managed optional payload. |
| `None_` | Builds managed optional without payload. |
| `Ok_` | Builds managed result ok payload. |
| `Err_` | Builds managed result err payload. |
| `Propagate` | Emits `?` propagation for optional/result. |
| `Await` | Lowered only through the native async state-machine path. The supported subset covers direct params, simple pre-await locals, `await value`, `const x: T = await value`, `return await value`, final void expressions, tail `if`/`while`/`for`/`match` without nested await/return, and narrow `const x = (await value)?` for same-typed result propagation. It uses `ori_future_poll`, `ori_future_value_*`, `ori_future_on_ready`, generated frame slots, per-`await` liveness, and ARC frame edges for managed params/locals/bindings. Async shapes outside this subset fail before Cranelift with `backend.native_unsupported`; the native `await` lowering must not call `ori_task_block_on*`. |
| `IfExpr` | Emits a Cranelift `select` over already typed values. |
| `Range` | Allocates managed range storage. |
| `MapLit` | Builds `ori.map` through native runtime calls. |
| `SetLit` | Builds `ori.set` through native runtime calls. |
| `StructUpdate` | Copies base storage and overwrites updated fields by native layout. |
| `Closure` | Builds a closure object with function pointer and captured environment. |
| `IsCheck` | Emits static type comparison or `any<Trait>` vtable type check. |

## Statement matrix

| HIR statement | Native backend behavior |
|---|---|
| `Let` | Creates local binding and registers ARC cleanup when managed. |
| `Assign` | Emits variable, field, and list-index assignment with ARC edge updates. |
| `Return` | Retains managed return value, emits cleanup, and returns direct/future value. |
| `Break` | Emits loop exit jump after scope cleanup. |
| `Continue` | Emits loop continue jump after scope cleanup. |
| `Expr` | Emits expression and discards the result. |
| `If` | Emits conditional branches and optional else-if/else blocks. |
| `While` | Emits conditional loop. |
| `For` | Emits range, list, set, map, string, and bytes iteration. Unsupported typed input gets `backend.native_unsupported`. |
| `Loop` | Emits unconditional loop with break/continue targets. |
| `Repeat` | Emits counted loop and traps negative counts. |
| `Match` | Emits native pattern checks and bindings. |
| `IfSome` | Emits optional tag check and payload binding. |
| `WhileSome` | Emits optional loop with payload binding. |
| `Using` | Registers lexical resource cleanup. |
| `Check` | Emits runtime trap when condition is false. |

## Maintenance rule

Do not add wildcard arms to native HIR traversal or emission code. Wildcards make
new HIR variants look supported when they are not.

If a valid language feature cannot be emitted yet, reject it before Cranelift
with `backend.native_unsupported` and add a focused test.
