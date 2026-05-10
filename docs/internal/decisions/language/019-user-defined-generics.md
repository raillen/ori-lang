# Decision 019 - User Defined Generics

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: generic declarations, generic type arguments, generic function calls, invariance

## Summary

Zenith Next extends the existing generic surface used by `list<T>`, `map<K, V>`, `optional<T>` and `result<Success, Error>` to user-defined `struct`, `trait`, `enum`, `func` and `apply` declarations.

Generic parameters are written explicitly with angle brackets after the declaration name.

Type arguments are explicit in type position.

Function call generic arguments may be omitted when they are inferable.

Generic constraints are defined separately by Decision 023.

## Decision

The canonical generic declaration forms are:

```zt
public struct Box<Item>
    value: Item
end
```

```zt
public trait Parser<Output>
    func parse(source: text) -> result<Output, text>
end
```

```zt
public enum Maybe<Item>
    Some(value: Item)
    None
end
```

```zt
public func identity<Item>(value: Item) -> Item
    return value
end
```

```zt
apply Box<Item>
    public func get() -> Item
        return self.value
    end
end
```

Normative rules for this cut:

- user-defined generic parameters are written in angle brackets after the declaration name
- user-defined generics are supported on `struct`, `trait`, `enum`, `func` and `apply`
- builtin generic types and user-defined generic types share the same surface form
- generic arguments in type position are explicit
- function call generic arguments may be omitted when the compiler can infer them from the call site
- explicit function call generic arguments are allowed when needed for clarity or disambiguation
- generic parameters are in scope only inside the declaration that introduces them
- generic parameter names are types and therefore use `PascalCase` in canonical style
- canonical generic parameter names are descriptive, such as `Item`, `Input`, `Output`, `Key`, `Value` and `Error`
- short names such as `T`, `U` and `E` are not the canonical style for Zenith Next
- generic parameters are invariant by default
- generic defaults are not part of the MVP
- partial specialization is not part of the MVP
- higher-kinded types are not part of the MVP

## Canonical Forms

### Generic Struct

```zt
public struct Pair<Left, Right>
    left: Left
    right: Right
end
```

### Generic Trait

```zt
public trait Parser<Output>
    func parse(source: text) -> result<Output, text>
end
```

### Generic Enum

```zt
public enum Maybe<Item>
    Some(value: Item)
    None
end
```

### Generic Function

```zt
public func identity<Item>(value: Item) -> Item
    return value
end
```

Multiple parameters:

```zt
public func pair<Left, Right>(left: Left, right: Right) -> Pair<Left, Right>
    return Pair<Left, Right>(
        left: left,
        right: right
    )
end
```

### Generic Apply

Inherent generic behavior:

```zt
apply Box<Item>
    public func get() -> Item
        return self.value
    end
end
```

Trait implementation with a concrete generic argument:

```zt
public struct IntParser
end

apply Parser<int> to IntParser
    func parse(source: text) -> result<int, text>
        ...
    end
end
```

## Type Position And Call Position

Type arguments are explicit in type position:

```zt
const boxed_count: Box<int> = Box<int>(value: 10)
const parser: Parser<int> = IntParser()
```

Function calls may omit generic arguments when inference is obvious:

```zt
const value: int = identity(10)
```

Explicit generic call arguments are still allowed:

```zt
const value: int = identity<int>(10)
```

## Rationale

Using one generic surface for both builtins and user-defined declarations keeps the language internally consistent and easier to learn.

Keeping type-position arguments explicit improves readability and avoids hidden type construction.

Allowing call-site inference for functions is a good compromise because it removes noise at ordinary call sites without weakening declaration clarity.

Descriptive generic parameter names fit the reading-first philosophy of Zenith better than single-letter mathematical names.

Separating basic generic declarations from generic constraints keeps the first generic cut smaller while still allowing constraints to be defined cleanly in a dedicated follow-up decision.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

single-letter generic parameters as the preferred style:

```zt
public struct Box<T>
    value: T
end
```

omitting generic arguments in type position:

```zt
const boxed_count: Box = Box(value: 10)
```

inline generic constraints inside the angle-bracket list:

```zt
public func render<Item where Item is Printable>(value: Item) -> text
    ...
end
```

generic defaults:

```zt
public struct Box<Item = int>
    value: Item
end
```

partial specialization:

```zt
apply Parser<int> to Box<Item>
    ...
end
```

This is non-canonical if the intention is partial specialization rather than an ordinary concrete trait implementation pattern supported by the semantic model later on.

## Out of Scope

This decision does not yet define:

- variance annotations
- generic defaults
- partial specialization semantics
- higher-kinded types
- monomorphization versus dictionary-based implementation strategy
- generic associated types or trait-level type members
- overload resolution interactions with generic functions
