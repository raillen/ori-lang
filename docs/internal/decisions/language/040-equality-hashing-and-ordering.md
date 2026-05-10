# Decision 040 - Equality, Hashing And Ordering

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics
- Scope: `Equatable`, `Hashable`, `Comparable`, `Order`, operators, float NaN behavior, text ordering

## Summary

Zenith defines equality, hashing and ordering through explicit core traits.

`Equatable<Item>` powers `==` and `!=`.

`Hashable<Item>` powers hash-based keys and must be consistent with equality.

`Comparable<Item>` powers ordered comparison operators through an explicit `Order` enum.

Floats implement ordered comparison, but attempting to order `NaN` produces runtime panic.

## Decision

Core ordering enum:

```zt
enum Order = Less, Equal, Greater
```

Core comparison trait:

```zt
public trait Comparable<Item>
    func compare(other: Item) -> Order
end
```

Normative rules for this cut:

- `==` requires `Equatable<Item>`
- `!=` requires `Equatable<Item>`
- `a != b` is equivalent to `not a.equals(b)`
- `hash()` returns `u64`
- `Hashable<Item>` must be consistent with `Equatable<Item>`
- if `a == b`, then `a.hash() == b.hash()`
- `map<Key, Value>` requires `Key is Hashable<Key>` and `Key is Equatable<Key>`
- `<`, `<=`, `>` and `>=` require `Comparable<Item>`
- `Comparable<Item>.compare` returns `Order`, not `int`
- user structs and enums do not auto-derive these traits in the MVP
- user implementations are explicit through `apply`
- `text` ordering is stable byte-wise UTF-8 lexicographic ordering in the MVP
- locale-aware collation is outside the MVP
- float equality follows IEEE behavior
- float ordered comparison with `NaN` produces runtime panic
- float does not implement `Hashable` in the MVP

## Equality

Equality operators lower through `Equatable<Item>`.

Example:

```zt
apply Equatable<Player> to Player
    func equals(other: Player) -> bool
        return self.name == other.name and self.hp == other.hp
    end
end
```

Usage:

```zt
if player_a == player_b
    ...
end
```

`!=` is defined as logical negation of equality:

```zt
if player_a != player_b
    ...
end
```

## Hashing

`Hashable<Item>` is used for hash-based structures.

```zt
public trait Hashable<Item>
    func hash() -> u64
end
```

Hash consistency rule:

```zt
if a == b
    // then a.hash() must equal b.hash()
end
```

A type used as a `map` key must implement both traits:

```zt
map<Key, Value>
where Key is Hashable<Key> and Key is Equatable<Key>
```

`Hashable` without `Equatable` is not sufficient for map keys.

## Ordering

`Comparable<Item>` uses `Order`:

```zt
enum Order = Less, Equal, Greater

public trait Comparable<Item>
    func compare(other: Item) -> Order
end
```

Operators lower conceptually as:

- `a < b` means `a.compare(b) == Order.Less`
- `a <= b` means `a.compare(b) == Order.Less or a.compare(b) == Order.Equal`
- `a > b` means `a.compare(b) == Order.Greater`
- `a >= b` means `a.compare(b) == Order.Greater or a.compare(b) == Order.Equal`

The compiler may evaluate `compare` only once when lowering compound checks.

## Builtin Trait Support

Builtin integer types implement:

- `Equatable`
- `Hashable`
- `Comparable`
- `TextRepresentable`

Builtin unsigned integer types implement:

- `Equatable`
- `Hashable`
- `Comparable`
- `TextRepresentable`

`text` implements:

- `Equatable`
- `Hashable`
- `Comparable`
- `TextRepresentable`

Floating-point types implement:

- `Equatable`
- `Comparable`
- `TextRepresentable`

Floating-point types do not implement:

- `Hashable` in the MVP

## Float Semantics

Float equality follows IEEE behavior.

Consequences include:

```zt
nan() == nan()
```

is false.

```zt
nan() != nan()
```

is true.

Ordered comparison involving `NaN` is a runtime panic:

```zt
const value: float = nan()
const ok: bool = value < 10.0
```

Runtime diagnostic direction:

```text
error[runtime.float_nan_compare]
Cannot order NaN.

help
  Check is_nan(value) before comparing.
```

Rationale:

- ordinary float comparisons remain practical
- generic ordering through `Comparable<float>` exists
- `NaN` does not silently produce surprising ordering results
- panic makes invalid ordered comparison visible

## Text Ordering

`text` ordering is byte-wise UTF-8 lexicographic ordering in the MVP.

This is stable, deterministic and target-independent.

It is not locale-aware human collation.

Locale-aware sorting belongs to future stdlib APIs.

Example future direction:

```zt
text.compare_locale(a, b, locale: "pt-BR")
```

## Compound Types

`list<T>` implements `Equatable` when `T is Equatable<T>`.

`list<T>` does not implement `Hashable` in the MVP.

`list<T>` does not implement `Comparable` in the MVP.

`optional<T>` implements conditionally:

- `Equatable` when `T is Equatable<T>`
- `Hashable` when `T is Hashable<T>`
- `TextRepresentable` when `T is TextRepresentable<T>`

`result<Success, Error>` implements conditionally:

- `Equatable` when both branches are equatable
- `Hashable` when both branches are hashable
- `TextRepresentable` when both branches are text-representable

`map<Key, Value>` does not automatically implement:

- `Equatable`
- `Hashable`
- `Comparable`

in the MVP.

## User Types

User structs and enums do not auto-derive core traits in the MVP.

Explicit implementation:

```zt
apply Comparable<Player> to Player
    func compare(other: Player) -> Order
        if self.hp < other.hp
            return Order.Less
        end

        if self.hp > other.hp
            return Order.Greater
        end

        return Order.Equal
    end
end
```

Future auto-derive may be discussed later:

```zt
derive Equatable, Hashable
struct Player
    ...
end
```

This is out of scope for the MVP.

## Non-Canonical Forms

Using `int` return from `compare` is not canonical:

```zt
trait Comparable<Item>
    func compare(other: Item) -> int
end
```

Relying on auto-derived equality is not canonical:

```zt
struct Player
    name: text
    hp: int
end

// non-canonical assumption: Player automatically supports ==
```

Using float as a map key is not valid in the MVP because float is not `Hashable`:

```zt
const scores: map<float, text> = {}
```

Assuming locale-aware text order is not canonical:

```zt
// non-canonical assumption: text order follows user's locale
```

## Diagnostics

Expected diagnostic directions:

```text
error[trait.missing_impl]
Type Player does not implement Equatable<Player>.

where
  src/app/main.zt:10:12

help
  Implement Equatable<Player> with apply Equatable<Player> to Player.
```

```text
error[trait.missing_impl]
Map key type User must implement Hashable<User> and Equatable<User>.
```

```text
error[runtime.float_nan_compare]
Cannot order NaN.

help
  Check is_nan(value) before comparing.
```

## Rationale

Explicit traits keep operator behavior visible and extensible without classical inheritance.

Using `Order` is more readable than returning magic negative/zero/positive integers.

Requiring both hashing and equality for map keys matches the real semantic requirement of hash maps.

Avoiding auto-derive in the MVP keeps behavior explicit and prevents hidden method generation.

Treating float `NaN` ordering as panic avoids silent surprising comparisons while keeping ordinary float ordering available.

Byte-wise UTF-8 text ordering provides deterministic behavior and leaves human collation to explicit future APIs.

## Out of Scope

This decision does not yet define:

- auto-derive syntax
- locale-aware collation
- custom comparator arguments for sort
- sort stdlib APIs
- hashing of floating-point values
- stable hash algorithm guarantees across compiler versions
- `Comparable` for compound collections
- total ordering wrappers for float
