# Decision 078: Backend Scalability And Runtime Risk Policy

- Status: accepted
- Date: 2026-04-18
- Type: compiler / runtime / language semantics
- Scope: RC cycles, weak/handle policy, monomorphization, stack-vs-heap representation, runtime where and exhaustive match

## Summary

Zenith keeps the user-facing language simple, but the C backend and runtime must not hide unbounded scalability risks.

Reference counting, monomorphization, managed containers, `optional`, `result`, runtime contracts and payload enums are all architecture-critical. They must be tracked explicitly before the language is considered stable.

## Decision

The project adopts a backend scalability risk policy.

The following points are normative:

- RC without cycle collection is allowed in the MVP, but cycles are a known leak risk.
- Before rich callbacks, UI object graphs, game object graphs or general reference-like features become official, Zenith must choose one explicit cycle strategy: `weak<T>`, runtime handles/arenas, cycle collection, or a constrained ownership graph.
- `optional<T>` and `result<T,E>` should use stack/in-place representation when possible.
- Heap allocation for `optional`/`result` is not the desired default; it is acceptable only for managed payloads, escaping values, oversized payloads or bootstrap implementation cuts.
- `result<void,E>` success should not allocate in the final runtime model.
- Monomorphization remains the first generics implementation strategy for C, but it requires canonical type keys, instance caching and diagnostics/reporting for excessive instantiation.
- Payload enum `match` must support exhaustiveness checking when no `case default` is present.
- Value-level `where` remains a runtime contract and does not secretly turn constructors into `result`.
- Recoverable construction/validation must be explicit through user-authored or stdlib-authored `try_create`/validation APIs returning `result`.

## RC Cycles

RC is useful for a C backend because it is deterministic and does not require a full VM.

However, RC alone cannot reclaim cycles.

Known cycle-prone domains include:

- UI tree nodes with parent/child links
- game entity graphs
- observer/event subscriptions
- callbacks that capture managed values
- doubly-linked structures
- caches that point back to owners

MVP policy:

- RC is allowed without cycle collection
- rich closure/callback capture is not considered stable until a cycle policy exists
- general reference variables are still not MVP
- future `weak<T>` or handles must be designed before official UI/game object graph APIs

## Monomorphization

Monomorphization remains compatible with the C backend.

Required engineering controls:

- canonical type identity for generic instantiations
- one generated instance per canonical type argument set
- deduplication of equivalent instantiations
- diagnostics or build reports when instantiation count becomes suspiciously high
- tests covering nested generic types

Monomorphization is not rejected, but uncontrolled monomorphization is not acceptable for a stable compiler.

## Stack Vs Heap

The final runtime direction is stack-first where possible.

Examples:

- `optional<int>` should be a small value, not heap allocated
- `result<int,text>` should be in-place with managed retain/release for `text`
- `result<void,E>` success should be a tag-only success value
- managed payloads may store pointers internally while the wrapper remains stack/in-place when viable

Heap-first representation may exist during bootstrap, but it must be tracked as a performance debt.

## Runtime `where`

A struct with field `where` still constructs the struct type.

This must not happen silently:

```zt
const player: Player = Player(hp: -1)
```

must not secretly become:

```zt
const player: result<Player, Error> = Player(hp: -1)
```

Instead:

- direct construction checks the contract
- failed contract reports `runtime.contract`
- expected user-data validation uses explicit result-returning APIs

Canonical recoverable style:

```zt
func try_create_player(hp: int) -> result<Player, PlayerError>
    if hp < 0
        return error(PlayerError.InvalidHp(value: hp))
    end

    return success(Player(hp: hp))
end
```

## Exhaustive Match

For enum values, `match` without `case default` must be exhaustive when the compiler has the enum definition.

If a case is missing, this is a compile-time diagnostic.

Using `case default` remains allowed, but it opts out of future-proof exhaustiveness help.

## Rationale

These rules preserve the manifesto.

The language can stay simple only if the compiler/runtime absorbs complexity honestly. If RC leaks cycles, monomorphization explodes code size, `result` allocates everywhere or `where` changes constructor types secretly, Zenith would violate its predictability promise.

Explicit risks are easier to design, test and explain.

## Consequences

Roadmap and checklist items must treat these as acceptance criteria.

The runtime and backend are not stable until:

- RC cycle policy is documented and gated
- `optional`/`result` representation is stack-first where possible
- monomorphized instances are cached and observable in build diagnostics
- enum match supports exhaustiveness diagnostics
- `where` runtime contracts are implemented without changing constructor return types
