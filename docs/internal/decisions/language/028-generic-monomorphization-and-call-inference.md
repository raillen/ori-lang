# Decision 028 - Generic Monomorphization And Call Inference

- Status: accepted
- Date: 2026-04-17
- Type: language / implementation
- Scope: generic function calls, type inference at call sites, backend C monomorphization, executable generic subset

## Summary

Zenith Next implements generics for the C backend through monomorphization.

A generic declaration remains written once in Zenith source, but the compiler emits one concrete backend representation for each type-argument combination that is actually used by the project.

Function call generic arguments may be omitted only when the compiler can infer them without ambiguity from the call arguments and/or the expected result type.

This decision completes the implementation strategy left out of Decision 019.

## Decision

The canonical generic function declaration form remains:

```zt
public func first<Item>(items: list<Item>) -> Item
    return items[0]
end
```

Explicit generic calls are valid:

```zt
const numbers: list<int> = [10, 20, 30]
const first_number: int = first<int>(numbers)
```

Generic calls may omit type arguments when inference is unambiguous:

```zt
const numbers: list<int> = [10, 20, 30]
const first_number: int = first(numbers)
```

Normative rules for this cut:

- generic declarations stay explicit at declaration sites
- generic type arguments stay explicit in type positions
- function call generic arguments may be explicit
- function call generic arguments may be omitted only when every generic parameter can be inferred
- inference may use argument types
- inference may use the expected result type from an explicitly typed context
- inference must not guess when more than one substitution is possible
- ambiguous generic calls are invalid and must produce a diagnostic that asks for explicit type arguments
- monomorphization happens per project, not globally
- the compiler emits only concrete generic instances that are reachable from the project entrypoint and checked declarations
- each concrete type-argument combination maps to one backend instance
- backend C names for monomorphized instances must be deterministic
- recursive generic expansion that would produce an unbounded set of instances is invalid in the MVP
- dictionary passing, runtime generic metadata and dynamic generic dispatch are not part of the MVP backend strategy

## Inference Sources

Inference from ordinary arguments:

```zt
public func identity<Item>(value: Item) -> Item
    return value
end

const value: int = identity(10)
```

Here `Item` is inferred as `int` from the argument.

Inference from compound arguments:

```zt
public func first<Item>(items: list<Item>) -> Item
    return items[0]
end

const names: list<text> = ["Ana", "Bia"]
const name: text = first(names)
```

Here `Item` is inferred as `text` from `list<text>`.

Inference from expected result type:

```zt
public func empty<Item>() -> list<Item>
    return []
end

const numbers: list<int> = empty()
```

Here `Item` is inferred as `int` from the explicitly declared variable type.

## Ambiguous Calls

This is invalid when there is no expected type or argument context:

```zt
print(empty())
```

unless the parameter type of `print` is known and gives enough information to infer the item type.

The fix is to make the type visible:

```zt
const numbers: list<int> = empty()
```

or:

```zt
const numbers: list<int> = empty<int>()
```

This is invalid:

```zt
const value: list<int> = choose_empty()
```

if `choose_empty` has multiple generic overload-like candidates or constraints that do not determine a single concrete substitution.

This is also invalid:

```zt
const numbers: list<int> = empty
```

because Zenith does not treat an uncalled function value as an inferred constructor in this cut.

## Backend C Model

Given:

```zt
public func first<Item>(items: list<Item>) -> Item
    return items[0]
end
```

and this use:

```zt
const numbers: list<int> = [10, 20, 30]
const value: int = first(numbers)
```

the compiler may emit a concrete C function equivalent to:

```c
zt_int app_main_first__int(zt_list_int items) {
    return zt_list_int_get(items, 0);
}
```

If the same project also uses:

```zt
const names: list<text> = ["Ana", "Bia"]
const name: text = first(names)
```

the compiler emits a separate concrete instance equivalent to:

```c
zt_text app_main_first__text(zt_list_text items) {
    return zt_list_text_get(items, 0);
}
```

The exact C symbol spelling is an implementation detail, but it must be deterministic enough for debugging, golden tests and stable builds.

## Runtime Consequences

Generic runtime-backed types must stop relying on fixed special cases as the final architecture.

The current C backend may continue to support early concrete combinations such as:

- `list<int>`
- `list<text>`
- `map<text, text>`
- `optional<T>`
- `result<Success, Error>`

but the direction is that these become ordinary monomorphized concrete instances rather than ad hoc backend exceptions.

## Rationale

Monomorphization fits the C target because C has no native generic runtime model.

Generating concrete instances keeps generated code straightforward, avoids runtime type metadata in the MVP and makes optimization easier for the C compiler.

Allowing safe call-site inference reduces visual noise without weakening the reading-first style, because declaration sites and type positions stay explicit.

Rejecting ambiguous inference is important for Zenith's readability goal: a reader should not have to reconstruct hidden type guesses to understand a call.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

omitting type arguments in type position:

```zt
const values: list = [1, 2, 3]
```

requiring all generic calls to be explicit even when obvious:

```zt
const value: int = identity<int>(10)
```

This is allowed, but not required as the canonical everyday style when inference is clear.

depending on hidden runtime generic dispatch:

```zt
const value: any = identity(10)
```

This is invalid because `any` is not part of Zenith Next.

## Diagnostics

When inference fails, the compiler should prefer actionable diagnostics:

- name the generic function
- name the generic parameter that could not be inferred
- point at the call site
- suggest explicit type arguments when possible

Example diagnostic direction:

```text
cannot infer generic parameter Item for call to empty
try: empty<int>()
```

## Out of Scope

This decision does not yet define:

- overload resolution between multiple generic functions with the same name
- partial specialization
- generic defaults
- higher-kinded types
- dynamic generic dispatch
- runtime generic reflection
- dictionary-passing implementation
- cross-package monomorphization and binary caching strategy for ZPM
