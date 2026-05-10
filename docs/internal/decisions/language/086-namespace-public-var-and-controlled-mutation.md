# Decision 086 - Namespace Public Var And Controlled Mutation

- Status: accepted
- Date: 2026-04-22
- Type: language / visibility / mutability / packages
- Scope: namespace-scope `var`, `public` visibility, cross-namespace mutation rules

## Summary

Zenith should support `public var` at namespace scope to improve package ergonomics.

This must not introduce `global` semantics.

In this cut, `public var` is readable outside the namespace, but writable only inside its owner namespace.

## Decision

Normative rules for this cut:

- `public var` is valid only for namespace-scope declarations.
- `public var` exposes read access outside the namespace through qualified imports.
- assignment to a namespace `public var` is allowed only inside the same effective namespace.
- cross-namespace mutation must happen through explicit `public func` APIs.
- `public` means visibility, not global lifetime or global scope.
- `global` is still not part of Zenith canonical syntax.
- existing `public const` behavior is preserved with no syntax change.
- existing private namespace `var` behavior is preserved.

## Public Is Not Global

Canonical owner module:

```zt
namespace app.runtime.state

public var frame_count: int = 0

public func next_frame() -> int
    frame_count = frame_count + 1
    return frame_count
end
```

Canonical external read:

```zt
namespace app.main

import app.runtime.state as state

func main() -> int
    const current: int = state.frame_count
    return current
end
```

Non-canonical external write:

```zt
namespace app.main

import app.runtime.state as state

func force_reset()
    state.frame_count = 0
end
```

Expected direction: visibility/mutability diagnostic indicating that external mutation of namespace `public var` is not allowed in this cut.

## Compatibility Contract

- no breaking change for existing code that uses `public const`.
- no breaking change for existing local/function `var`.
- parser accepts top-level `var` only for namespace-scope declarations.
- no new keyword is introduced.

## Side Effects And Mitigation

Expected side effects when `public var` is used:

- state can change between calls, so function output may depend on call order;
- tests may become order-sensitive if state is not reset between scenarios;
- behavior can become less obvious if mutation is hidden inside helper functions.

Mitigation policy for this cut:

- prefer `public const` by default and justify each `public var`;
- expose mutation through explicit `public func` APIs instead of cross-namespace writes;
- document invariants and mutation points in ZDoc for each namespace variable;
- provide reset helpers for test isolation when namespace state is used in tests;
- keep diagnostic wording explicit that `public` is visibility, not `global` state.

## Struct/Trait Interaction And `mut`

`public var` does not replace `mut func`.

- `mut func` is still required when a method mutates `self`;
- mutating a namespace `public var` is a separate concern from mutating `self`;
- a method that mutates only namespace state and not `self` does not require `mut` in this cut;
- when both happen, `mut func` is still required because `self` mutation is present.

This keeps receiver mutation contracts stable while allowing controlled namespace state.

## Stdlib Impact

No broad refactor of stdlib is required only because `public var` exists.

Current direction:

- keep pure modules pure (for example path/text transform modules) without namespace mutable state;
- allow targeted namespace state only where process/runtime state is natural (for example counters, caches, feature toggles);
- require explicit docs for side effects in each stdlib module that exposes namespace mutable state.

## Rollout Plan (Implementation Priority 1)

1. Frontend and AST:
- accept top-level `var` declarations.
- add `is_public` and `is_module_level` metadata for `var_decl`.
2. Semantic model:
- include namespace `var` in top-level catalog resolution (`alias.member`).
- enforce read-public/write-owner rule.
3. Lowering and backend:
- stop const-style inlining for namespace `var`.
- allocate real module storage and deterministic initialization path.
4. Tooling:
- formatter support for `public var`.
- ZDoc/diagnostic wording distinguishing `public` from `global`.
5. Tests:
- positive: import + read of `public var`.
- negative: cross-namespace write denial.
- regression: existing `public const` module tests remain green.

## Implementation Status (2026-04-22)

Delivered in this repository:

- parser/AST/binder/checker/lowering/backend paths now support namespace `public var`;
- external read through import alias is supported (`alias.var_publica`);
- external write is rejected with `error[mutability.invalid_update]`;
- `public const` behavior remains compatible;
- stdlib pilot delivered in `stdlib/std/random.zt` (`seeded`, `last_seed`, `draw_count`, `stats()`).

Behavior evidence:

- positive: `tests/behavior/public_var_module`
- positive: `tests/behavior/public_var_module_state`
- negative: `tests/behavior/public_var_cross_namespace_write_error`
- stdlib positive: `tests/behavior/std_random_basic`
- stdlib positive: `tests/behavior/std_random_state_observability`
- stdlib positive: `tests/behavior/std_random_between_branches`
- stdlib negative: `tests/behavior/std_random_cross_namespace_write_error`

## Non-Canonical Forms

Using `global`:

```zt
global var frame_count: int = 0
```

Using unqualified external access:

```zt
frame_count = 0
```

Declaring `public var` in local function scope:

```zt
func f()
    public var x: int = 0
end
```

## Out of Scope

This decision does not yet define:

- concurrent mutation guarantees for namespace `public var`
- atomic/synchronized mutation primitives
- reexport behavior for namespace variables
- package-level capability controls for mutable public state
