# Decision 027 - Core Prelude Traits And Operator Semantics

- Status: accepted
- Date: 2026-04-16
- Type: language / semantics
- Scope: implicit `core.*` prelude, core traits, builtin support, compound-type support, operator lowering

## Summary

Zenith Next defines a minimal semantic core through a small set of traits in the reserved `core.*` namespace.

These core traits are available implicitly through a language prelude and do not require explicit `import`.

User-defined types do not auto-derive these traits in the MVP and must implement them explicitly through `apply`.

## Decision

The semantic core traits for the MVP are:

```zt
public trait Equatable<Item>
    func equals(other: Item) -> bool
end

public trait Hashable<Item>
    func hash() -> u64
end

public trait TextRepresentable<Item>
    func to_text() -> text
end
```

Normative rules for this cut:

- `core.*` is a reserved logical namespace of the language core
- the user does not write `import core...` in the MVP
- core traits are available implicitly through the language prelude
- the core semantic traits in this cut are `Equatable<Item>`, `Hashable<Item>` and `TextRepresentable<Item>`
- `Self` is not part of the MVP type surface for traits or `apply`
- `self` remains the receiver value available inside method bodies
- user-defined `struct` and `enum` types do not auto-derive these traits in the MVP
- user-defined trait implementations use explicit `apply Trait<Type> to Type`
- builtins and selected builtin compound types may provide language-defined implementations

## Builtin Support

Builtin types implement these core traits as follows:

- `bool`, `int8`, `int16`, `int32`, `int64`, `u8`, `u16`, `u32`, `u64`, `float32`, `float64` and `text` implement `Equatable`
- `bool`, `int8`, `int16`, `int32`, `int64`, `u8`, `u16`, `u32`, `u64` and `text` implement `Hashable`
- `bool`, `int8`, `int16`, `int32`, `int64`, `u8`, `u16`, `u32`, `u64`, `float32`, `float64` and `text` implement `TextRepresentable`
- `float32` and `float64` do not implement `Hashable` in the MVP

## Compound Type Support

Builtin compound types implement these traits conditionally:

- `optional<T>` implements `Equatable`, `Hashable` and `TextRepresentable` when `T` implements the corresponding trait
- `list<T>` implements `Equatable` and `TextRepresentable` when `T` implements the corresponding trait
- `list<T>` does not implement `Hashable` in the MVP
- `result<Success, Error>` implements `Equatable`, `Hashable` and `TextRepresentable` when `Success` and `Error` implement the corresponding trait
- `map<Key, Value>` does not auto-implement these core traits in the MVP

## Operator And Core-Semantic Meaning

The canonical semantic lowering is:

- `a == b` requires `Equatable<Item>` and lowers to `a.equals(b)`
- `a != b` requires `Equatable<Item>` and lowers to `not a.equals(b)`
- `map<Key, Value>` requires `Key is Hashable<Key> and Key is Equatable<Key>`
- textual representation through the core trait remains explicit through `to_text(value)`
- there is no implicit coercion from arbitrary values to `text`

## Canonical Examples

Trait implementation for a user-defined type:

```zt
public struct Player
    name: text
    hp: int
end

apply Equatable<Player> to Player
    func equals(other: Player) -> bool
        return self.name == other.name and self.hp == other.hp
    end
end
```

Explicit textual representation:

```zt
apply TextRepresentable<Player> to Player
    func to_text() -> text
        return self.name
    end
end
```

Generic constraint against the core prelude:

```zt
public func render<Item>(value: Item) -> text
where Item is TextRepresentable<Item>
    return to_text(value)
end
```

Map key constraint:

```zt
public struct Cache<Key, Value>
where Key is Hashable<Key> and Key is Equatable<Key>
    items: map<Key, Value>
end
```

## Rationale

This keeps the language core small and semantically clear without introducing a magic root object model.

Making `core.*` implicit avoids boilerplate imports for traits that are part of the language's own semantic contract.

Keeping user-defined implementations explicit through `apply` matches the reading-first and non-magical direction already chosen for Zenith Next.

Avoiding automatic text coercion keeps textual intent visible and prevents hidden formatting behavior from spreading through ordinary expressions.

Excluding `float` from `Hashable` in the MVP avoids early semantic complexity around `NaN`, signed zero and stable hash contracts.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

explicitly importing the core prelude:

```zt
import core.equatable
import core.hashable
```

using a magical receiver type token such as `Self`:

```zt
public trait Equatable
    func equals(other: Self) -> bool
end
```

expecting user-defined structs to auto-derive core traits:

```zt
public struct Player
    name: text
    hp: int
end

// non-canonical assumption: Player automatically gains Equatable
```

relying on implicit text coercion:

```zt
const message: text = "hp: " + player.hp
```

## Out of Scope

This decision does not yet define:

- `Comparable<Item>` or ordering traits
- auto-derive annotations such as `derive`
- hashing for floating-point values
- text-formatting options beyond plain `to_text(value)`
- richer core semantic traits beyond the minimal MVP set

