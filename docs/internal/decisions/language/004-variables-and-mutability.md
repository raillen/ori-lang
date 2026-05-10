# Decision 004 - Variables And Mutability

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: variable declarations, mutability, type annotations

## Summary

Zenith Next uses `const` for immutable bindings and `var` for mutable bindings.

Type annotations are explicit in all variable declarations.

Local type inference is not part of the MVP.

Binding immutability is distinct from deep value immutability.

## Decision

The canonical forms are:

```zt
const display_name: text = text.trim(user.name)
var retry_count: int = 0

retry_count = retry_count + 1
```

Normative rules for this cut:

- variable declarations use `const name: Type = expression` or `var name: Type = expression`
- `const` creates an immutable binding and cannot be reassigned after initialization
- `var` creates a mutable binding and may be reassigned
- type annotations are required on local and namespace-scope variable declarations
- local type inference is not part of the MVP syntax
- the broader MVP policy is explicit typing in declarations, including function parameters and all value-returning function signatures
- `const` controls rebinding of the name, not deep immutability of the referenced value

## Rationale

`const` and `var` are visually distinct and immediately signal whether a binding may change.

That improves scanability and reduces surprise during reading.

Rejecting local inference in the MVP keeps the frontend simpler, reduces ambiguity around empty literals and option-like values, and makes diagnostics easier to explain.

Separating binding immutability from deep value immutability also keeps the semantic model smaller and clearer for the first implementation.

## Canonical Examples

Local immutable binding:

```zt
const trimmed_name: text = text.trim(user.name)
```

Local mutable binding:

```zt
var retry_count: int = 0
retry_count = retry_count + 1
```

Function using both forms:

```zt
func normalized_name(user: user_types.User) -> text
    const trimmed_name: text = text.trim(user.name)
    var result_name: text = text.lower(trimmed_name)
    return result_name
end
```

Binding immutability versus value semantics:

```zt
const names: list<text> = user_types.load_names()
```

In this example, `names` cannot be rebound to another list. Whether the list contents themselves may change is defined by `list`, not by `const`.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `let` and `let mut`:

```zt
let display_name: text = text.trim(user.name)
let mut retry_count: int = 0
```

using `mutable` instead of `var`:

```zt
mutable retry_count: int = 0
```

omitting the type annotation:

```zt
const retry_count = 0
```

## Out of Scope

This decision does not yet define:

- deep immutability for collections and structs
- declarations without an initializer
- destructuring bindings
- compile-time constant evaluation rules
