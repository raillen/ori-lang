# Decision 069 - Stdlib Test Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / testing
- Scope: complementary test helpers, explicit fail and skip operations, relationship with `attr test` and `check`

## Summary

`std.test` is a small helper module for tests.

It complements the language-level testing model instead of replacing it.

`attr test` remains the way tests are declared, and `check(...)` remains the primary assertion form.

## Decision

Canonical import:

```zt
import std.test as test
```

Accepted first-wave API direction:

- `test.fail(message: text) -> void`
- `test.skip(reason: text = "") -> void`

Examples:

```zt
import std.test as test

attr test
func invalid_user_returns_error()
    const result: result<User, text> = create_user(name: "", age: 18)

    match result
    case success(user):
        test.fail("expected error")
    case error(message):
        check(message == "name required")
    end
end
```

```zt
import std.test as test

attr test
func websocket_suite_placeholder()
    test.skip("network stack not implemented yet")
end
```

Normative rules for this cut:

- `std.test` is complementary and does not declare tests by itself
- tests are still declared with `attr test`
- `check(...)` remains the canonical assertion form
- `test.fail(...)` immediately marks the current test as failed
- `test.skip(...)` immediately marks the current test as skipped
- `skip` is a distinct test outcome, not a passing assertion
- the default `reason` for `skip` may be empty in this cut
- `std.test` is intentionally small in the MVP

## Helper Semantics

### Fail

`test.fail(...)` exists for cases where a test must fail explicitly outside a simple boolean check.

This is especially useful in control-flow branches such as `match` on success/error results.

### Skip

`test.skip(...)` exists for cases where a test is intentionally not run or not yet applicable.

Skip should be surfaced by the runner as skipped, not passed and not failed.

## Boundary

`std.test` does not own:

- test discovery
- test file conventions
- runner filtering
- parallel execution
- fixtures
- snapshots
- mocking
- benchmarking

Those concerns belong to tooling or future expansions, not to the initial helper module.

## Rationale

A tiny helper module is enough to cover explicit fail/skip cases without duplicating the role of `attr test` and `check(...)`.

Keeping `std.test` small preserves clarity and prevents the testing model from splitting across two competing systems.

Adding `skip(...)` now is useful because it expresses a real test outcome that cannot be represented cleanly as a normal boolean check.

## Non-Canonical Forms

Using `std.test` to declare tests without `attr test`.

Replacing `check(...)` with assertion-heavy APIs such as:

```zt
test.equal(a, b)
test.true(condition)
expect(condition)
```

Turning `std.test` into a fixture, snapshot and mocking framework in the MVP.

## Out of Scope

This decision does not yet define:

- fixture APIs
- setup/teardown APIs
- snapshot testing
- mocking helpers
- benchmark helpers
- temp-directory helpers
- panic assertions
- runner reporting format for skipped tests
