# Decision 048 - Tests In The Language

- Status: accepted
- Date: 2026-04-17
- Type: language / tooling / testing
- Scope: test declarations, `attr test`, test runner, assertions, helper boundary and test visibility

## Summary

Zenith uses `attr test` on functions for tests.

Tests are ordinary functions marked with metadata, not a separate `test "name"` declaration.

The primary test assertion form is `check(...)`.

A complementary `std.test` module may provide helpers such as `fail(...)` and `skip(...)`, but it does not replace `attr test` or `check(...)`.

A future test runner command discovers and runs functions marked with `attr test`.

## Decision

Canonical test:

```zt
attr test
func loads_user()
    const user: User = User(name: "Julia")
    check(user.name == "Julia")
end
```

Normative rules for this cut:

- tests are functions marked with `attr test`
- there is no separate `test "name"` declaration in the MVP
- test functions take no parameters
- test functions return `void` / no explicit return value
- test functions are discovered by tooling through `attr test`
- build normal does not include/run tests by default
- a future `zt test` command compiles and runs tests
- `check(condition)` is the canonical assertion form
- `std.test` is complementary and does not replace `attr test` or `check(...)`
- `std.test.fail(message: text)` may exist as an explicit failure helper
- `std.test.skip(reason: text = "")` may exist as a skip helper
- `expect(...)` is not the canonical assertion name
- tests may access namespace-private symbols when declared in the same namespace
- panic expectation helpers are future work
- test file/folder conventions are deferred to CLI/project design

## Why `attr test`

Using `attr test` keeps test support aligned with the attribute model.

It avoids adding a separate top-level declaration form:

```zt
test "loads user"
    ...
end
```

Tests remain ordinary functions, so they are easier for the compiler, tooling and users to understand.

Moving test declaration entirely into `std.test` would make discovery more indirect and would weaken the static tooling model.

## Check

`check(...)` is the primary assertion helper.

```zt
attr test
func calculates_total()
    const total: int = calculate_total(price: 10, quantity: 3)
    check(total == 30)
end
```

If the condition is false, the test fails.

Failure diagnostics should follow the diagnostic philosophy from Decisions 038 and 039.

## Complementary Helpers

A small helper module may complement `check(...)` without replacing it.

The canonical location for such helpers is `std.test`.

### Fail

`test.fail(message: text)` may exist as an explicit failure helper.

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

### Skip

`test.skip(reason: text = "")` may exist as an explicit skip helper.

```zt
import std.test as test

attr test
func future_parser_case()
    test.skip("parser recovery not implemented yet")
end
```

Skip is a test-runner outcome, not a passing assertion.

## Test Visibility

Tests in the same namespace can access namespace-private helpers.

```zt
namespace app.users

func normalize_name(name: text) -> text
    ...
end

attr test
func normalize_name_trims_spaces()
    check(normalize_name(" Julia ") == "Julia")
end
```

This keeps tests useful without forcing helpers to become public API.

## Test Discovery

The future test runner discovers:

```zt
attr test
func name()
    ...
end
```

Potential future command:

```text
zt test
```

The exact CLI command and project configuration are deferred.

## Test Files

This decision does not require a file convention.

Future possibilities:

- `tests/`
- `*_test.zt`
- test target entries in `zenith.ztproj`

The CLI/project model should decide this later.

## Panic Tests

Testing panic is useful, but not part of this decision's MVP surface.

Possible future direction:

```zt
attr test
func invalid_hp_panics()
    check_panic(create_invalid_player)
end
```

Because lambdas are not in the MVP, panic-test ergonomics require separate design.

## Non-Canonical Forms

Separate test declaration is not canonical in the MVP:

```zt
test "loads user"
    ...
end
```

`expect` is not the canonical assertion name:

```zt
expect(total == 30)
```

Test functions with parameters are invalid:

```zt
attr test
func loads_user(id: int)
    ...
end
```

Using `std.test` alone to declare tests is not canonical.

Tests should not run as part of ordinary `build` unless explicitly configured in future tooling.

## Diagnostics

Expected diagnostic directions:

```text
error[test.invalid_signature]
Test function loads_user must not have parameters.
```

```text
error[test.invalid_return]
Test function loads_user must not return a value.
```

```text
error[test.check_failed]
Check failed.

where
  tests/users.zt:8:5

code
  8 | check(total == 30)
    |       ^^^^^^^^^^^
```

## Rationale

`attr test` is explicit, simple and consistent with the attribute system.

Using `check` avoids the heavier expectation vocabulary and reads as a direct validation.

Keeping tests as functions avoids adding special test syntax before the core language and CLI are stable.

Allowing `std.test` as a small helper layer preserves room for explicit fail/skip operations without making library calls responsible for test discovery.

Allowing same-namespace private access supports practical testing without weakening public API boundaries.

## Out of Scope

This decision does not yet define:

- concrete `zt test` CLI
- test target configuration in `zenith.ztproj`
- test file naming conventions
- parallel test execution
- fixtures
- setup/teardown
- panic assertions
- snapshot/golden testing
- test filtering
- test reporting format
- coverage
