# Zenith Wave 7.9: Callable and Closure ABI Closure

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: closure artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

Close Wave 7.9 for executable C-backend scope.
Define stable callable/closure ABI contract across checker, lowering, runtime, FFI, and jobs.

## ABI Shape

## A1: Runtime Closure Representation

Runtime closure carrier is `zt_closure`:
- `fn` pointer
- `ctx` pointer
- optional `drop_ctx` callback
- managed header for retain/release lifetime

Implementation anchors:
- `runtime/c/zenith_rt.h` (`typedef struct zt_closure`)
- `runtime/c/zenith_rt_outcome.c` (`zt_closure_create`, `zt_closure_create_with_drop`, closure free path).

## A2: Lowering Contract

HIR closure expressions lower to:
1. hoisted synthetic function;
2. `make_closure(name)` expression with capture list.

Implementation anchor:
- `compiler/zir/lowering/from_hir.c` (`ZT_HIR_CLOSURE_EXPR` lowering path).

## A3: Callable Type Surface

Callable type syntax remains:
- `func(T1, T2, ...) -> R`

Callable compatibility is structural and exact by checker rules.
Mismatches emit `callable.signature_mismatch`.

## Escape/Storage Rules (v1 closure)

- Callables allowed in local bindings and parameter/return positions already accepted by checker model.
- Forbidden escape positions remain enforced:
  - public vars (`callable.escape_public_var`)
  - struct fields (`callable.escape_struct_field`)
  - container element shapes (`callable.escape_container`)

These restrictions are part of stable v1 closure contract.

## FFI Callback Contract

Extern C callable boundaries accept only extern-safe callback shapes.
Captured anonymous closures are rejected at FFI boundary.

Diagnostic anchors:
- `callable.extern_c_signature`
- `callable.extern_c_closure_unsupported`

## Jobs Callback Contract

`std.jobs.spawn(...)` callable argument contract is stable in checker:
- top-level function references only;
- non-generic functions only;
- arity/signature must match spawn form;
- Transferable enforced on payload and return type.

This forms current jobs-callback ABI boundary.

## Validation Envelope

Minimum validation set:

- Positive:
  - `tests/behavior/callable_basic`
  - `tests/behavior/nested_function_basic`
- Negative callable diagnostics:
  - `callable_invalid_func_ref_error`
  - `callable_signature_mismatch_error`
  - `callable_escape_container_error`
  - `callable_escape_struct_field_error`
  - `callable_escape_public_var_error`
  - `extern_c_callback_closure_error`

- Jobs callback baseline:
  - `std_jobs_int_basic`

And compile sanity:
- `python build.py`

## Closure Result

Wave 7.9 closure contract is defined for C oracle subset.
Stored callable ABI, closure lowering/runtime representation, extern callback restrictions, and jobs callback boundaries are now explicit and auditable.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave 7.9 status.
- `post-v1-closure-matrix.md` - operational closure tracker entry 7.1.17.
- `post-v1-completeness-discussion.md` - closure rationale and risk tracking.
- `runtime-model.md` - runtime-level ownership/lifetime context.
