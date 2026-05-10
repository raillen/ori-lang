# Decision 003 - Functions And Blocks

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: function declarations, block delimiters, frontend

## Summary

Zenith Next uses `func` for function declarations.

Functions that return a value use `-> Type`.

Functions without an explicit return type are `void`.

Blocks are explicit and terminate with `end`.

Indentation exists for readability, not as part of the grammar.

## Decision

The canonical forms are:

```zt
func render_name(user: user_types.User) -> text
    return text.trim(user.name)
end

func log_message(message: text)
    text.write_line(message)
end
```

Normative rules for this cut:

- function declarations start with `func`
- parameters use the form `name: Type`
- functions with a value result use `-> Type`
- functions without `-> Type` are `void`
- the function body starts on the following line
- blocks close with `end`
- indentation is stylistic and improves reading, but does not define scope
- braces are not part of the canonical block syntax
- one-line function declarations are not part of the canonical style
- `-> void` is not part of the canonical style

## Rationale

`func` is short without being overly compressed.

`fn` was rejected as the canonical keyword because it is too abbreviated for a language that prioritizes reading comfort and scanability.

`->` was accepted for value-returning functions because it is visually distinct and widely understood, while still remaining compact.

Allowing omission of the return clause for `void` functions reduces visual noise in effect-oriented code without making the parser or type checker substantially harder.

Explicit block closure with `end` makes structure easier to follow and keeps the syntax family consistent with future constructs such as `if`, `while` and `match`.

## Canonical Examples

Basic function:

```zt
func render_name(user: user_types.User) -> text
    return text.trim(user.name)
end
```

Function with multiple parameters:

```zt
func format_full_name(first_name: text, last_name: text) -> text
    return text.concat(first_name, last_name)
end
```

Function with a local variable and explicit block ending:

```zt
func normalized_name(user: user_types.User) -> text
    const trimmed_name: text = text.trim(user.name)
    return text.lower(trimmed_name)
end
```

Void function with implicit return type:

```zt
func log_message(message: text)
    text.write_line(message)
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `fn` as the declaration keyword:

```zt
fn render_name(user: user_types.User) -> text
    return text.trim(user.name)
end
```

using `returns` instead of `->`:

```zt
func render_name(user: user_types.User) returns text
    return text.trim(user.name)
end
```

using braces as block delimiters:

```zt
func render_name(user: user_types.User) -> text {
    return text.trim(user.name)
}
```

omitting the closing `end`:

```zt
func render_name(user: user_types.User) -> text
    return text.trim(user.name)
```

writing `-> void` explicitly:

```zt
func log_message(message: text) -> void
    text.write_line(message)
end
```

## Out of Scope

This decision does not yet define:

- anonymous functions
- method syntax
- overload policy
- default parameter values
- multiline signature formatting rules
