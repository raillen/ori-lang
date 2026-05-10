# Decision 095 - `group` Tuple Alias

- Status: accepted
- Date: 2026-05-01
- Scope: post-v1 syntax, tuple readability
- Upstream: `docs/spec/language/post-v1-surface-contract.md`, `docs/spec/language/post-v1-implementation-plan.md`

## Context

Post-v1 accepts `group` as an alternate spelling for `tuple`.

`tuple` remains valid. The new spelling is intended for readers who understand
"group of values" faster than "tuple", while keeping the same positional data
model.

## Decision

`group<T1, T2, ...>` is accepted anywhere a type name is parsed.

The checker resolves `group<...>` to the existing tuple type representation:

```zt
const item: group<text, int> = ("score", 10)
```

This is exactly the same runtime and semantic type as:

```zt
const item: tuple<text, int> = ("score", 10)
```

Diagnostics keep the spelling the user wrote when reporting arity or callable
container errors. Canonical type formatting may still print `tuple<...>` because
there is only one underlying type.

## Constraints

- `group` must have at least two type arguments, matching `tuple`.
- `group` is positional. It does not create named fields.
- `group` does not introduce a second runtime representation.
- `tuple` remains the canonical implementation name.

## Rationale

This is a small readability feature with low implementation risk.

Keeping one internal type avoids duplicate lowering, emitter, runtime, and ARC
paths. The feature adds an approachable spelling without changing the mental
model: a fixed positional group of values.

## Accessibility Notes

`group` is a familiar word. For some readers, it is easier to understand than
`tuple`.

The language should explain both spellings together:

- use `group` when teaching the concept;
- mention that `tuple` is the canonical technical name;
- avoid presenting them as two different features.

## Validation

Required coverage:

- behavior test where a function returns `group<text, int>`;
- behavior test where `list<group<text, int>>` uses generated tuple callbacks;
- existing `tuple<...>` behavior must keep passing.
