# Decision 042 - Overload, Lambdas And Macros

- Status: accepted
- Date: 2026-04-17
- Type: language / syntax / semantics
- Scope: function overloads, lambdas, anonymous functions, macros, metaprogramming

## Summary

Zenith does not include function overloads or macros in the MVP.

Overload is not a central direction for the language.

Lambdas are accepted as a possible future feature, but they are outside the MVP and their syntax is intentionally not finalized yet.

User-defined macros are rejected as part of the language direction.

## Decision

Normative rules for this cut:

- function overload is not part of the MVP
- overload by type is not part of the MVP
- overload by arity is not part of the MVP
- overload by named labels is not part of the MVP
- duplicate names in the same effective namespace/scope remain errors
- polymorphism should use generics, traits and explicit names
- lambdas are not part of the MVP
- lambda syntax is not decided in this cut
- future lambdas must address capture, ownership, lifetime and diagnostics before implementation
- user-defined macros are not part of Zenith
- user-defined macros are not a short-term or central language direction
- attributes/annotations are a separate topic and are not considered macros by default
- external code generation and tooling may exist outside the language

## No Overload

This is not allowed in the MVP:

```zt
func print(value: int)
    ...
end

func print(value: text)
    ...
end
```

A name in the same effective namespace/scope must resolve to one declaration.

Preferred explicit forms:

```zt
func print_int(value: int)
    ...
end

func print_text(value: text)
    ...
end
```

Or generic/trait-based forms when appropriate:

```zt
func print_value<Item>(value: Item)
where Item is TextRepresentable<Item>
    ...
end
```

## Why No Overload

Overload introduces invisible resolution.

The reader must know:

- all functions with the same name
- argument types
- default parameters
- named arguments
- generic inference rules
- trait constraints
- conversion rules

before understanding which function is called.

That conflicts with Zenith's reading-first philosophy.

Avoiding overload also improves diagnostics and keeps name resolution simpler.

## Lambdas

Lambdas are useful for future APIs such as:

- collection mapping
- filtering
- sorting
- callbacks
- event handlers
- optional helper APIs

Example future need:

```zt
users.map(...)
```

However, lambda syntax and semantics are not part of the MVP.

Open design questions include:

- syntax
- single-expression lambdas
- multiline lambdas
- parameter type annotations
- capture by value or future reference
- ownership of captured managed values
- interaction with `return`
- interaction with `?`
- interaction with `attempt/rescue`
- diagnostics and formatter rules

In the MVP, use named functions:

```zt
func user_name(user: User) -> text
    return user.name
end
```

A future lambda decision must be made before collection higher-order APIs depend on lambdas.

## No User Macros

User-defined macros are not part of Zenith.

Rejected direction:

```zt
macro make_getter(...)
    ...
end
```

Rationale:

- macros can create language dialects
- macros can hide control flow
- macros can make diagnostics harder
- macros can make tooling harder
- macros increase cognitive load
- macros conflict with the accessibility goals of Zenith

Zenith should prefer:

- explicit language features
- generics
- traits
- `apply`
- formatter support
- ZDoc
- external code generation when needed
- build tooling outside the language

## Attributes Are Separate

This decision does not reject simple attributes or annotations.

Future examples may include:

```zt
@deprecated("use new_name")
func old_name()
    ...
end
```

or:

```zt
@test
func parses_valid_user()
    ...
end
```

Attributes must be discussed separately.

They should be declarative metadata, not arbitrary compile-time code execution.

## Non-Canonical Forms

Overload by type:

```zt
func parse(value: text) -> int
    ...
end

func parse(value: list<text>) -> int
    ...
end
```

Overload by arity:

```zt
func log(message: text)
    ...
end

func log(message: text, level: text)
    ...
end
```

Overload by named labels:

```zt
func open(path: text)
    ...
end

func open(url: text)
    ...
end
```

User macro:

```zt
macro route(path)
    ...
end
```

These are not part of Zenith's MVP direction.

## Diagnostics

Expected diagnostic directions:

```text
error[name.duplicate]
Function print is already defined in this namespace.

help
  Use a distinct name or a generic function with trait constraints.
```

```text
error[feature.unsupported]
Lambdas are not supported in this implementation cut.

help
  Use a named function instead.
```

```text
error[feature.unsupported]
Macros are not part of Zenith.

help
  Use explicit code, generics, traits, apply, or external code generation.
```

## Rationale

No overload keeps name resolution visible and predictable.

Deferring lambdas avoids committing to capture and ownership rules before the value/reference model is fully implemented.

Rejecting macros protects the language from hidden syntax, tooling fragmentation and accessibility regressions.

Zenith can still be expressive through generics, traits, enums, `apply`, explicit functions and future carefully designed syntax sugar.

## Out of Scope

This decision does not yet define:

- lambda syntax
- function type syntax
- closure representation
- capture semantics
- callback ABI
- attributes/annotations
- external code generation conventions
- test annotations
