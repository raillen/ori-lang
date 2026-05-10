# Decision 061 - Stdlib Validate Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / validation
- Scope: pure predicate helpers, readability in `where`, numeric and length validation helpers

## Summary

`std.validate` is the small module of pure boolean predicates intended to read well in `where` clauses and ordinary boolean conditions.

It does not perform IO, transformation, parsing, sanitization or domain-specific heavy validation.

## Decision

Canonical import:

```zt
import std.validate as validate
```

Accepted first-wave API direction:

- `between`
- `positive`
- `non_negative`
- `negative`
- `non_zero`
- `one_of`
- `not_empty`
- `min_length`
- `max_length`
- `length_between`

Accepted executable expansion:

- float helpers use explicit names such as `between_float`, `positive_float` and `one_of_float`
- bool helpers use `is_true`, `is_false` and `one_of_bool`
- optional/result helpers cover the primitive/result shapes that the current C backend can execute safely
- list/map length and size helpers use concrete suffixes such as `*_list_text` and `*_map_text_int`
- fully generic public helpers remain deferred until public stdlib generics are stable through import and C emission

Examples:

```zt
age: int where validate.between(it, min: 0, max: 130)
price: float where validate.non_negative(it)
status: int where validate.one_of(it, [200, 201, 204])

name: text where validate.not_empty(it)
username: text where validate.length_between(it, min: 3, max: 20)
password: text where validate.min_length(it, 8)
slug: text where validate.max_length(it, 64)
```

Normative rules for this cut:

- `std.validate` functions are pure
- `std.validate` functions return `bool`
- the module exists primarily to improve readability in `where` and `if`
- `between` is inclusive by default
- `one_of` requires the compared type to support equality
- suffixes such as `_float`, `_list_text` and `_map_text_int` are intentional in this cut because overloads and public stdlib generics are not the stable teaching surface yet
- text-specific intrinsic predicates such as digit checks belong in `std.text`, not `std.validate`
- operators such as `>`, `<`, `==` remain valid and often preferable when they are already clearer than a helper call

## Length Helpers

Accepted text-length helpers:

- `not_empty(value: text) -> bool`
- `min_length(value: text, min: int) -> bool`
- `max_length(value: text, max: int) -> bool`
- `length_between(value: text, min: int, max: int) -> bool`

These helpers are accepted because they read naturally in contracts and are cheap to implement.

## Deferred Domain Validators

The module deliberately does not grow into a collection of high-cost domain validators in the MVP.

Deferred examples:

- `email`
- `url`
- `phone`
- `uuid`
- regex-backed validators

## Rationale

A tiny predicate module improves readability without replacing ordinary operators everywhere.

The chosen helpers cover the most common range and length contracts while keeping the module small.

Keeping digit checks in `std.text` and heavy domain rules out of `std.validate` preserves separation of concerns.

## Non-Canonical Forms

Using `std.validate` for transformations:

```zt
validate.clean_name(value)
```

Moving text predicates that belong in `std.text` into `std.validate`:

```zt
validate.is_digits(value)
```

Growing the MVP around expensive domain validators such as email or regex matching.

## Out of Scope

This decision does not yet define:

- regex validators
- domain-specific validators
- locale-aware validation
- parsing helpers
- sanitization/transformation helpers
