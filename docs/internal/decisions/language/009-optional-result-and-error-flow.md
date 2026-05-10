# Decision 009 - Optional Result And Error Flow

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: absence, fallible results, propagation, error flow

## Summary

Zenith Next uses `optional<T>` for expected absence and `result<T, E>` for fallible operations with explicit error information.

`none` is the canonical absence value.

`optional<T>` does not require a wrapper like `some(...)` in return position when the surrounding type context already expects `optional<T>`.

`result<T, E>` uses explicit constructors `success(...)` and `error(...)`.

The postfix `?` operator is used for early propagation.

## Decision

The core forms are:

```zt
none

success(value)
success()
error(reason)
```

Normative rules for this cut:

- `optional<T>` represents presence or expected absence
- `result<T, E>` represents success or failure with an explicit error payload
- `null` is not part of the MVP
- exceptions are not part of the MVP error model
- `none` is the canonical absence value
- returning a plain value is allowed in a context that already expects `optional<T>`
- `some(...)` is not part of the canonical MVP syntax
- `result<T, E>` uses explicit `success(...)` and `error(...)` constructors
- `success()` is allowed when the success type is `void`
- postfix `?` performs early propagation
- on `optional<T>`, `expr?` propagates `none`
- on `result<T, E>`, `expr?` propagates `error(error_value)`
- there is no implicit conversion between `optional<T>` and `result<T, E>`
- optional chaining and null-coalescing syntax are not part of the MVP

## Rationale

`optional<T>` expresses ordinary absence without forcing a wrapper constructor on every successful return site.

Rejecting `some(...)` as the canonical syntax reduces visual ceremony and makes the common path easier to read.

`result<T, E>` is more immediately understandable than `outcome<T, E>` because it evokes an operation that either produced a result or failed.

Explicit `success(...)` and `error(...)` constructors are still valuable for `result<T, E>` because the two branches can otherwise become ambiguous, especially when success and error types are similar.

The postfix `?` operator gives concise propagation without introducing exceptions or hidden control flow.

## Canonical Examples

Optional return with direct value:

```zt
public func find_name(id: int) -> optional<text>
    if id == 0
        return none
    end
    return "Zenith"
end
```

Optional propagation:

```zt
public func normalize_name(id: int) -> optional<text>
    const name: text = find_name(id)?
    return text.trim(name)
end
```

Result with explicit success and error:

```zt
public func parse_port(source: text) -> result<int, text>
    if source == ""
        return error("missing port")
    end
    return success(8080)
end
```

Result propagation:

```zt
public func load_port(path: text) -> result<int, text>
    const content: text = file.read_text(path)?
    const port: int = parse_port(content)?
    return success(port)
end
```

Void success:

```zt
public func flush_output() -> result<void, text>
    return success()
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `some(...)` as the standard optional constructor:

```zt
return some("Zenith")
```

using `outcome<T, E>` as the core result type:

```zt
public func parse_port(source: text) -> outcome<int, text>
    ...
end
```

implicit truthiness on optional values:

```zt
if maybe_name
    ...
end
```

implicit conversion from optional to result:

```zt
const name: text = find_name(id)?
```

This is non-canonical when the surrounding function returns `result<text, text>` and expects the language to invent an error automatically.

## Out of Scope

This decision does not yet define:

- pattern matching for `optional` and `result`
- helper methods such as `is_some` or `is_error`
- recovery operators such as null-coalescing
- custom error hierarchies
- whether future sugar should exist for mapping or transforming `optional` and `result`
