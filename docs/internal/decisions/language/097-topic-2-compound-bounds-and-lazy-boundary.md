# Decision 097 - Topic 2 Compound Bounds And Lazy Boundary

- Status: accepted
- Date: 2026-05-10
- Type: implementation boundary
- Scope: `0.4.2-beta.rc1`, advanced generics
- Upstream: `docs/internal/planning/0.4.2-beta.rc1-language-gap-implementation-plan.md`

## Context

Topic 2 closes the generic foundation for the RC.

Two areas needed a clear boundary before moving on:

- compound bounds;
- generic `lazy<T>`.

Both are useful language directions, but both can create a larger runtime
contract than this RC should promise.

## Decision

Repeated named-trait bounds are in scope.

Examples:

```zt
func accept<T>(value: T) -> int where T is Addable and T is Comparable
    return 0
end
```

```zt
trait OrderedBox<T> where T is Addable and T is Comparable
    func get() -> T
end
```

These are covered by:

- `generic_compound_bounds_basic`;
- `generic_trait_compound_bounds_basic`;
- `generic_compound_bounds_error`.

The following are deferred:

- shorthand compound spelling such as `where T: TraitA + TraitB`;
- public runtime capability traits such as `Cloneable`, `HashableKey`, or
  `OrderableKey`;
- user-defined capability composition for backend materialization.

Internal compiler capability conjunctions are allowed. They are implementation
gates, not public language traits.

For lazy values, the RC keeps the executable surface specialized:

- `lazy<int>`;
- `lazy<float>`;
- `lazy<bool>`;
- `lazy<text>`.

Other `lazy<T>` payloads are rejected during `zt check`.

Fully generic `lazy<T>` and lazy iterators remain post-RC work.

## Rationale

Repeated named-trait bounds are already understandable and covered by behavior
fixtures.

Adding public capability traits now would make backend/runtime details part of
the language surface too early.

Keeping `lazy<T>` specialized avoids promising clone/drop behavior for arbitrary
managed payloads before the runtime ownership model is broad enough.

## Validation

- `generic_compound_bounds_basic`
- `generic_trait_compound_bounds_basic`
- `generic_compound_bounds_error`
- `lazy_primitive_text_basic`
- `lazy_generic_deferred_error`
- `lazy_reuse_error`
