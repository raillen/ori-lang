# Decision 043 - Attributes And Annotations

- Status: accepted
- Date: 2026-04-17
- Type: language / syntax / tooling
- Scope: attributes, annotations, metadata, declaration modifiers, non-macro tooling hooks

## Summary

Zenith uses textual `attr` declarations for attributes.

Attributes are declarative metadata applied to the next declaration.

Attributes are not macros, do not generate arbitrary code and do not change the syntax of the language.

The `@` decorator style is not used.

## Decision

Canonical attribute form:

```zt
attr deprecated(message: "use load_user_by_id")
func load_user(id: int) -> User
    ...
end
```

Multiple attributes:

```zt
attr deprecated(message: "use load_user_by_id")
attr doc_hidden
func load_user(id: int) -> User
    ...
end
```

Attribute without arguments:

```zt
attr test
func parses_valid_user()
    ...
end
```

Normative rules for this cut:

- attributes use the `attr` keyword
- `@` attributes/decorators are not part of Zenith
- one `attr` per line is the canonical style
- `attr name` is used for attributes without arguments
- `attr name(arg: value)` is used for attributes with arguments
- attribute arguments use named arguments
- positional attribute arguments are not canonical in the MVP
- an attribute applies to the next declaration
- attributes are allowed only before declarations in the MVP
- attributes are not allowed on statements or expressions in the MVP
- attributes are declarative metadata
- attributes are not macros
- attributes cannot execute arbitrary compile-time code
- unknown attributes are errors unless a future tool namespace mechanism is explicitly defined

## Initial Attribute Direction

The initial/future attribute set direction is:

```zt
attr deprecated(message: "use new_name")
```

```zt
attr test
```

```zt
attr extern_name(name: "zt_log")
```

```zt
attr doc_hidden
```

These attributes may be implemented incrementally.

This decision reserves the syntax and semantic shape; it does not require all attributes to be executable immediately.

## Why `attr` Instead Of `@`

`@` is familiar but symbolic and carries expectations from decorator systems in other languages.

Zenith prefers a textual form:

```zt
attr deprecated(message: "use new_name")
```

because it is:

- explicit
- searchable
- easy to read aloud
- consistent with reading-first goals
- less likely to imply hidden decorator execution
- easier for dyslexic/TDAH-friendly scanning

Non-canonical:

```zt
@deprecated(message: "use new_name")
func old_name()
    ...
end
```

## Attribute Placement

Attributes apply to the next declaration.

Canonical:

```zt
attr test
func parses_valid_user()
    ...
end
```

Attributes should stay visually attached to the declaration they modify.

The formatter should avoid separating an attribute from its declaration with extra blank lines.

## Attribute Arguments

Attributes with arguments use named arguments:

```zt
attr deprecated(message: "use load_user_by_id")
```

```zt
attr extern_name(name: "zt_log")
```

Non-canonical:

```zt
attr deprecated("use load_user_by_id")
```

Named arguments preserve readability and make future extension safer.

## Unknown Attributes

Unknown attributes are errors in the compiler core.

Invalid:

```zt
attr random_magic
func run()
    ...
end
```

Expected diagnostic direction:

```text
error[attr.unknown]
Unknown attribute random_magic.

help
  Use a supported attribute or remove this line.
```

A future tool namespace mechanism may allow external tool metadata, but it must be explicitly designed.

## Attributes Are Not Macros

Attributes cannot:

- generate arbitrary code
- rewrite AST
- alter grammar
- hide control flow
- inject declarations
- create framework-specific language dialects

Rejected direction:

```zt
attr route(path: "/users")
func list_users()
    ...
end
```

This may exist as external framework metadata only if a future tool-attribute namespace is designed. It is not a core language macro.

## Non-Canonical Forms

Decorator syntax:

```zt
@test
func parses_user()
    ...
end
```

Grouped attributes:

```zt
attr deprecated(message: "use x"), doc_hidden
func old()
    ...
end
```

Statement attribute:

```zt
func main() -> int
    attr trace
    const value: int = compute()
    return value
end
```

Compile-time macro attribute:

```zt
attr generate_getters
struct User
    name: text
end
```

These are not part of the MVP direction.

## Rationale

`attr` preserves Zenith's textual, explicit design while still providing a standard place for compiler and tooling metadata.

One attribute per line improves scanning and avoids dense metadata lists.

Named arguments make attributes self-documenting.

Rejecting macro behavior prevents attributes from becoming a hidden language extension system.

## Out of Scope

This decision does not yet define:

- exact implementation of `attr test`
- exact implementation of `attr deprecated`
- exact implementation of `attr extern_name`
- exact implementation of `attr doc_hidden`
- tool-specific attribute namespaces
- attribute availability by target
- attribute validation phases
- attribute metadata format in HIR/ZIR
- doc rendering rules for attributes
