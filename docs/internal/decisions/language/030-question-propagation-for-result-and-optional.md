# Decision 030 - Question Propagation For Result And Optional

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics
- Scope: postfix `?`, early return propagation, `result`, `optional`, safe navigation reservation

## Summary

Zenith Next reserves postfix `?` exclusively for propagation with early return.

For `result<Success, Error>`, `?` extracts the success value or returns the error from the current function.

For `optional<Item>`, `?` extracts the present value or returns `none` from the current function.

The operator does not perform safe navigation and does not convert automatically between `optional` and `result`.

## Decision

Canonical `result` propagation:

```zt
func load_user(id: int) -> result<User, text>
    const raw: text = read_file("user.txt")?
    const user: User = parse_user(raw)?
    return success(user)
end
```

Canonical `optional` propagation:

```zt
func find_city(user: User) -> optional<text>
    const address: Address = user.address?
    return address.city
end
```

Normative rules for this cut:

- postfix `?` is propagation, not safe navigation
- `result<Success, Error>?` extracts `Success` when the value is `success(...)`
- `result<Success, Error>?` returns early with `error(...)` when the value is `error(...)`
- `optional<Item>?` extracts `Item` when a value is present
- `optional<Item>?` returns early with `none` when the value is `none`
- `result` propagation is valid only inside a function that returns compatible `result<OtherSuccess, Error>`
- `optional` propagation is valid only inside a function that returns compatible `optional<OtherItem>`
- there is no implicit conversion between `optional` and `result`
- safe navigation is not part of postfix `?`
- safe navigation, if added later, must use a stdlib API or a separate syntax
- the next executable cut limits `?` to statement-level forms
- nested expression-level `?` remains out of scope for now

## Statement-Level Forms

The accepted executable forms for the next implementation cut are:

```zt
const value: Item = expression?
```

```zt
var value: Item = expression?
```

```zt
return expression?
```

The `return expression?` form means:

- evaluate `expression`
- if it propagates failure/absence, return that from the current function
- otherwise return the extracted success/present value, wrapped or accepted according to the current function's declared return type

For example:

```zt
func read_name() -> result<text, text>
    return read_file("name.txt")?
end
```

is valid when `read_file("name.txt")` returns `result<text, text>`.

## Result Semantics

Given:

```zt
const value: Item = operation()?
```

where `operation()` returns `result<Item, Error>`, the behavior is equivalent to:

```zt
match operation()
case success(value):
    const value: Item = value
case error(err):
    return error(err)
end
```

The current function does not need to return the same success type, but it must return a compatible error channel:

```zt
func load_count() -> result<int, text>
    const raw: text = read_file("count.txt")?
    return parse_int(raw)
end
```

Here `read_file` may return `result<text, text>` and `load_count` returns `result<int, text>`.

## Optional Semantics

Given:

```zt
const value: Item = maybe_value?
```

where `maybe_value` has type `optional<Item>`, the behavior is equivalent to:

```zt
match maybe_value
case some(value):
    const value: Item = value
case none:
    return none
end
```

The current function does not need to return the same item type, but it must return `optional<...>`:

```zt
func find_city(user: User) -> optional<text>
    const address: Address = user.address?
    return address.city
end
```

## No Automatic Optional-Result Conversion

This is not valid:

```zt
func load_user(id: int) -> result<User, text>
    const user: User = find_user(id)?
    return success(user)
end
```

if `find_user(id)` returns `optional<User>`.

The user must convert explicitly through a future stdlib API:

```zt
func load_user(id: int) -> result<User, text>
    const user: User = find_user(id).or_error("user not found")?
    return success(user)
end
```

The exact `or_error` API belongs to the future stdlib design.

## Safe Navigation

Postfix `?` is not safe navigation.

This decision intentionally avoids syntax such as:

```zt
const city: optional<text> = user.address?.city
```

Even if `?.` could be tokenized separately, it creates visual ambiguity with propagation and weakens the reading-first rule.

Safe navigation may be addressed later through a stdlib API:

```zt
const city: optional<text> = user.address.map(get_city)
```

or through a future syntax that does not reuse plain `?`.

## Non-Canonical Forms

Nested expression-level `?` is not part of the next executable cut:

```zt
const total: int = parse_int(a)? + parse_int(b)?
```

Passing propagated expressions directly as call arguments is not part of the next executable cut:

```zt
save(parse_user(raw)?)
```

Mixing `optional` and `result` without explicit conversion is invalid:

```zt
const user: User = find_user(id)?
```

inside a function returning `result<User, text>`, when `find_user` returns `optional<User>`.

Safe navigation with `?` is not canonical:

```zt
const name: optional<text> = user?.name
```

## Diagnostics

When `?` is invalid, diagnostics should explain the channel mismatch.

Examples:

```text
cannot use ? on optional<User> inside function returning result<User, text>
convert optional to result explicitly before using ?
```

```text
cannot use ? on result<int, text> inside function returning int
? requires the current function to return result<..., text>
```

```text
nested ? expressions are not supported in this implementation cut
move the operation into a const or var declaration
```

## Rationale

Using `?` only for propagation keeps one clear meaning for a visually small operator.

Separating `optional` and `result` avoids hidden error-message invention and keeps failure semantics explicit.

Limiting the next implementation cut to statement-level forms keeps lowering predictable because `?` introduces control flow.

Leaving safe navigation to stdlib or a future syntax avoids overloading `?` with unrelated meanings.

## Out of Scope

This decision does not yet define:

- expression-level `?` lowering
- safe navigation syntax
- the final stdlib API for `optional.map`, `optional.and_then` or `or_error`
- automatic conversion between different error types
- error mapping syntax
- interaction with future async/concurrent effects
