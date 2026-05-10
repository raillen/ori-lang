# Decision 013 - MVP Semantic Validation

- Status: accepted
- Date: 2026-04-16
- Type: language / semantics
- Scope: compile-time validation, name resolution, typing, control flow, trait checks

## Summary

Zenith Next validates as much as possible at compile time in the MVP.

The compiler should reject ambiguous, implicit or contract-breaking code early, with precise diagnostics.

This decision defines the semantic checks that follow directly from the accepted surface decisions for namespaces, imports, functions, variables, types, traits, match and extern blocks.

Contracts, refinements and field invariants are explicitly outside this cut.

## Decision

The MVP semantic analyzer must enforce the following categories of rules.

### Name Resolution And Declarations

- every source file must declare exactly one top-level `namespace`
- imported namespaces must resolve uniquely
- referenced names and referenced types must resolve uniquely
- duplicate top-level names in the same namespace are invalid
- duplicate local names in the same scope are invalid
- name shadowing is invalid in the MVP
- `public` is valid only at namespace scope
- `public` may be applied only to declarations that are allowed to exist at namespace scope
- duplicate field names in a `struct` are invalid
- duplicate variant names in an `enum` are invalid
- duplicate method names in the same `trait` are invalid
- duplicate payload names inside one enum variant are invalid

### Mutability And Assignment

- `const` bindings cannot be reassigned after initialization
- `var` bindings may be reassigned
- assignment targets must be semantically writable
- assignment through `self` is allowed only inside methods declared with `mut func`
- methods without `mut` may not assign through `self`
- a trait contract and its implementation must agree on whether a method uses `mut func`

### Functions And Returns

- a function with `-> Type` must return a value compatible with that type on every required path
- a function without `-> Type` is `void`
- a `void` function may not use `return value`
- a value-returning function may not use bare `return`
- reaching the end of a `void` function is valid
- reaching the end of a value-returning function without returning a value is invalid

### Types And Expressions

- explicit type annotations must resolve to valid types
- conditions in `if`, `while` and similar constructs must have type `bool`
- truthiness is invalid
- `==` and `!=` require `Equatable<T>` for the compared type
- `text[index]` has type `text`
- `map[key]` has type `optional<V>`
- any concrete `map<K, V>` use requires `Hashable<K>` and `Equatable<K>`
- empty list and map literals require explicit type context
- there is no implicit conversion between `optional<T>` and `result<T, E>`
- the one accepted lifting rule is return-position lifting of a plain value to `optional<T>` when the surrounding return type is already `optional<T>`

### Optional Result And Propagation

- postfix `?` is valid only on expressions of type `optional<T>` or `result<T, E>`
- in a function returning `optional<U>`, using `expr?` on `optional<T>` is valid and propagates `none`
- in a function returning `result<U, E>`, using `expr?` on `result<T, E>` is valid and propagates `error(error_value)`
- using `?` in a function with an incompatible surrounding return family is invalid
- using `?` on `optional<T>` inside a `result<U, E>` function is invalid in the MVP
- using `?` on `result<T, E>` inside an `optional<U>` function is invalid in the MVP

### Traits Apply And Methods

- `apply Trait to Type` must implement the declared trait methods
- trait method implementations must match the trait contract in name, arity, parameter types, return type and mutability marker
- extra undeclared trait methods inside `apply Trait to Type` are invalid in the MVP
- only one implementation of a given trait for a given type is allowed in the MVP
- methods declared in `apply Type` and `apply Trait to Type` may use `self` in the body
- `self` is not a normal parameter and may not be redeclared as a local name

### Enums And Match

- enum construction must reference an existing variant
- payload construction must match the declared payload names for that variant
- payload binding in `match` must match the declared variant shape
- `case default` must be the final branch when present
- `match` must be exhaustive unless it ends with `case default`
- `match` has no fallthrough
- each `case` introduces a new local scope
- duplicate `case default` branches are invalid
- obviously unreachable duplicate literal or duplicate enum-variant cases should be rejected when the compiler can determine them directly

### Extern Blocks

- `extern c` and `extern host` are valid only at namespace scope
- only function declarations are valid inside `extern` blocks in the MVP
- extern declarations may not have bodies
- extern function signatures must still be semantically valid Zenith signatures
- host and C extern declarations remain distinct namespaces of meaning and are not interchangeable

### Diagnostics

- when possible, diagnostics should point both to the invalid use and to the original declaration that caused the conflict
- the compiler should prefer compile-time rejection over runtime fallback whenever the issue is statically decidable
- semantic errors should use language terms visible to the user, not ZIR-internal terminology

## Canonical Examples

Invalid shadowing:

```zt
const app_name: text = "Zenith"

func show_name() -> text
    const app_name: text = "Local"
    return app_name
end
```

This is invalid because shadowing is forbidden in the MVP.

Invalid receiver mutation without `mut`:

```zt
apply Player
    func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

This is invalid because only methods declared with `mut func` may assign through `self`.

Valid optional propagation:

```zt
public func normalize_name(id: int) -> optional<text>
    const name: text = find_name(id)?
    return text.trim(name)
end
```

Invalid mixed-family propagation:

```zt
public func load_name(id: int) -> result<text, text>
    const name: text = find_name(id)?
    return success(name)
end
```

This is invalid in the MVP because `find_name(id)?` propagates `optional`, not `result`.

Invalid non-bool condition:

```zt
if user_count
    ...
end
```

This is invalid because conditions must be `bool`.

## Out of Scope

This decision does not yet define:

- contracts or refinement types such as `where` clauses
- field invariants
- effect systems beyond the receiver-changing `mut func` convention
- aliasing rules for field mutation behind immutable bindings
- advanced unreachable-code analysis
- warnings policy versus errors policy
- future semantic rules required by `zenith.ztproj` configuration
