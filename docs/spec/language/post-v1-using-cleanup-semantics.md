# Zenith Wave 7.10: Resource Cleanup (`using`) Semantics

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: closure artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

Close Wave 7.10 for current language/runtime scope.
Define deterministic `using` cleanup behavior across control-flow boundaries.

## Surface Forms

Supported forms:

```zenith
using x = init_expr
    -- scoped body
end
```

```zenith
using x = init_expr then cleanup_expr
```

## Core Semantics

## U1: Lifetime

- Binding created at `using` entry.
- Binding visible in using body (block form) or enclosing scope until scope exit (flat form).
- Cleanup expression associated with binding executes when scope exits.

## U2: Cleanup Order

- Cleanup order is LIFO for nested `using` bindings.
- Latest active cleanup runs first.

## U3: Normal Scope Exit

On normal fallthrough out of scope, all cleanups added in that scope run in LIFO order.

## U4: Early Return

Before `return` terminator, active cleanups run in LIFO order.
Return value remains preserved.

## U5: Error Propagation (`?`, `.or_return` family)

When propagation path exits function, active cleanups run before propagated return/error terminator.

## U6: Panic Path

Before panic terminator emission, active cleanups run in LIFO order.

## U7: Loop Flow (`break` / `continue`)

`break` and `continue` execute active cleanups before jump terminator.
Behavior validated with `using` loop-control fixtures.

## Lowering Contract (HIR -> ZIR)

Current lowering model uses explicit cleanup stack in function context:
- push cleanup on entering `using` with cleanup expression;
- pop/emit on scope exit;
- emit active cleanups for return/error/panic/loop jumps.

Implementation anchor:
- `compiler/zir/lowering/from_hir.c` cleanup stack + `ZT_HIR_USING_STMT` lowering path.

## Concurrency and FFI Boundary Note

This closure item defines in-function deterministic cleanup behavior.
Cross-boundary ownership/ABI coordination with jobs/channels/FFI remains linked to Wave 7.11 and 7.18 audits.

## Validation Envelope

Minimum validation set:

- `tests/behavior/using_basic`
  - scope exit cleanup
  - flat using lifetime
  - LIFO cleanup
  - early return cleanup
  - `?` propagation cleanup
  - loop iteration cleanup
  - break/continue cleanup

And compile sanity:
- `python build.py`

## Closure Result

Wave 7.10 semantics are now explicitly defined and implemented for current scope:
`using` cleanup is deterministic, LIFO, and active across return/propagation/panic/loop-control exits.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave 7.10 status.
- `post-v1-closure-matrix.md` - operational closure tracker entry 7.1.18.
- `post-v1-completeness-discussion.md` - closure rationale and risks.
- `runtime-model.md` - ownership/cleanup runtime context.
