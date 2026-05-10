# Decision 026 - Named Parameters And Default Values

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: named call arguments, parameter defaults, call ordering rules, public API parameter names

## Summary

Zenith Next supports named call arguments and default parameter values in the MVP.

Named arguments improve readability, especially for booleans and longer parameter lists, but they do not introduce arbitrary argument reordering.

Default values are part of the function declaration surface and are matched with the same reading-first rules as the rest of the language.

## Decision

The canonical default-parameter declaration form is:

```zt
public func open_file(
    path: text,
    create_if_missing: bool = false,
    mode: text = "read"
) -> result<void, text>
    ...
end
```

The canonical named-argument call forms are:

```zt
open_file("data.txt")
open_file("data.txt", create_if_missing: true)
open_file("data.txt", mode: "write")
open_file(path: "data.txt", create_if_missing: true, mode: "write")
```

Normative rules for this cut:

- named call arguments use the surface form `parameter_name: expression`
- default parameter values use the declaration form `parameter_name: Type = expression`
- parameters without defaults must appear before parameters with defaults in the same signature
- defaulted parameters therefore form a trailing suffix of the parameter list in canonical MVP code
- arguments still follow declaration order in the MVP
- named arguments do not create arbitrary argument reordering
- once a named argument appears in a call, all remaining provided arguments must also be named
- a named argument may target any declared parameter that has not already been filled by an earlier argument
- omitted arguments are valid only when the corresponding parameters have defaults
- duplicate filling of the same parameter in one call is invalid
- unknown named argument names are invalid
- public parameter names are part of the callable API surface in the MVP because named calls may rely on them
- default expressions are evaluated as ordinary call-time expressions in declaration context
- default expressions may not reference `self` in this cut
- default expressions may not reference sibling parameters in this cut
- a default expression must be type-compatible with its parameter type
- named arguments are available for ordinary functions and methods alike

## Canonical Examples

Boolean clarity:

```zt
public func render_button(label: text, emphasized: bool = false)
    ...
end

render_button("Save", emphasized: true)
```

Skipping one defaulted parameter while keeping order:

```zt
public func open_file(
    path: text,
    create_if_missing: bool = false,
    mode: text = "read"
) -> result<void, text>
    ...
end

open_file("data.txt", mode: "write")
```

Fully named call in declaration order:

```zt
open_file(path: "data.txt", create_if_missing: true, mode: "write")
```

## Rationale

Named arguments fit the reading-first goal of Zenith, especially when calls contain booleans, multiple values of the same type or defaults that would otherwise be invisible at the call site.

Keeping declaration order even for named arguments avoids turning labels into a second ordering system. The reader can still scan the call from left to right and compare it directly with the function signature.

Restricting defaults to a trailing suffix keeps the rules predictable and keeps call-site omission straightforward.

Treating public parameter names as part of the callable API surface is the honest consequence of supporting named calls. Anything else would make refactors look safer than they really are.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

placing a required parameter after a defaulted one:

```zt
public func open_file(path: text = "data.txt", mode: text)
    ...
end
```

using positional arguments after a named one:

```zt
open_file(path: "data.txt", true)
```

reordering named arguments away from declaration order:

```zt
open_file(mode: "write", path: "data.txt")
```

using a default expression that references another parameter:

```zt
public func slice_text(source: text, start: int = 0, end: int = start)
    ...
end
```

## Out of Scope

This decision does not yet define:

- variadic parameters
- keyword-only parameter markers
- positional-only parameter markers
- overload resolution by parameter labels
- destructured parameters
- default values on trait declarations with different implementation-time override rules
