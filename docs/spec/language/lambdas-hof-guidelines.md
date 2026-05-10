# Zenith Lambdas and HOF Guidelines

- Status: R3.M7 guidance
- Date: 2026-04-24
- Scope: expression lambdas and first higher-order collection helpers

## Purpose

Lambdas should make simple data flow easier to read.

They should not hide complex behavior.

## Lambda Shape

Use the expression form for short transformations:

```zt
const doubled: list<int> = collections.map_int(values, func(value: int) => value * 2)
```

Rules:

- keep the body to one clear expression
- keep parameters typed
- prefer meaningful parameter names
- use `func ... end` when the body needs `if`, `while`, `match`, `?`, or local variables
- avoid nesting lambdas inside lambdas in user-facing examples

## Capture Rules

Lambdas use closure v1 capture rules.

- captures are immutable snapshots
- mutable capture is not available
- `public var` is still namespace-owned state, not a global capture mechanism

## HOF Subset

R3.M7 ships a narrow, explicit stdlib subset:

- `std.collections.map_int(values, mapper)`
- `std.collections.filter_int(values, predicate)`
- `std.collections.reduce_int(values, initial, reducer)`

These functions are intentionally concrete.

Generic HOFs should wait until the generic callable and monomorphization story is wider.

## Performance Guidance

Use HOFs for clarity at normal scale.

For tight hot loops:

- benchmark before replacing a hand-written loop
- prefer explicit loops when allocation or callback overhead matters
- keep HOF chains short in performance-critical code

## Accessibility Guidance

For docs and examples:

- show the input list
- show the lambda
- show the expected output
- avoid clever one-letter names except `x` in tiny examples
- keep examples under 5 lines when teaching the feature
