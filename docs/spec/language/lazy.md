# Explicit Lazy Values

- Status: canonical R3.M8 slice
- Date: 2026-04-24
- Scope: explicit `lazy<T>` values and one-shot consumption rules

## Purpose

`lazy<T>` exists for work that should be delayed on purpose.

It must be visible in the type.
It must be created by an explicit helper.
It must be consumed by an explicit helper.

Zenith does not have implicit lazy evaluation in ordinary expressions.

## Current Surface

R3.M8 delivers the first concrete slice:

```zt
import std.lazy as lazy

const later: lazy<int> = lazy.once_int(func() => compute_value())
const value: int = lazy.force_int(later)
```

Available helpers:

- `once_int(thunk: func() -> int) -> lazy<int>`
- `force_int(value: lazy<int>) -> int`
- `is_consumed_int(value: lazy<int>) -> bool`
- `once_float(thunk: func() -> float) -> lazy<float>`
- `force_float(value: lazy<float>) -> float`
- `is_consumed_float(value: lazy<float>) -> bool`
- `once_bool(thunk: func() -> bool) -> lazy<bool>`
- `force_bool(value: lazy<bool>) -> bool`
- `is_consumed_bool(value: lazy<bool>) -> bool`
- `once_text(thunk: func() -> text) -> lazy<text>`
- `force_text(value: lazy<text>) -> text`
- `is_consumed_text(value: lazy<text>) -> bool`

This remains intentionally monomorphic.

Generic `lazy<T>` and lazy iterators are the language direction, but only the executable C-backend `int`, `float`, `bool`, and `text` helpers are stable in this cut.

In `0.4.2-beta.rc1`, `zt check` rejects other lazy payloads before C emission.
This keeps unsupported `lazy<Payload>` shapes from escaping as backend errors.

## Evaluation Order

Creating a lazy value does not run the thunk.

The thunk runs only when the matching `force_*` helper is called.

Example:

```zt
public var hits: int = 0

func tick() -> int
    hits = hits + 1
    return 41
end

const later: lazy<int> = lazy.once_int(func() => tick())
-- hits is still 0 here

const value: int = lazy.force_int(later)
-- hits is 1 here
```

## Consumption Rule

R3.M8 lazy values are one-shot.

That means:

- first `force_int(...)` runs the thunk and returns the value
- second `force_int(...)` on the same lazy value is a runtime contract error
- `is_consumed_int(...)` can be used for defensive checks

Runtime diagnostic:

```text
error[runtime.contract]
lazy value already consumed
```

## What Lazy Is Not

`lazy<T>` is not a global variable.

`lazy<T>` is not hidden evaluation.

`lazy<T>` is not automatic memoization for any expression.

These forms are not part of R3.M8:

- automatic lazy function arguments
- implicit lazy list/map/filter chains
- hidden lazy conversion from `T` to `lazy<T>`
- reusable/cached lazy values
- lazy iterators as public executable API

## Design Rules

Rules for implementation and libraries:

- the type must say `lazy<T>`
- construction must use an explicit API
- forcing must use an explicit API
- ordinary expressions stay eager
- runtime must reject invalid reuse
- docs must show the one-shot rule near examples

## Rationale

Lazy evaluation can make code faster or cleaner.

It can also hide when work happens.

Zenith chooses a narrow first slice so package authors can use delayed work without making evaluation order surprising for readers.
