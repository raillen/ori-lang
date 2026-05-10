# Decision 034 - Value Semantics And Ownership

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / implementation
- Scope: value semantics, ownership, managed values, references, parameter passing, mutation visibility

## Summary

Zenith uses value semantics by default.

Assignment, parameter passing and returns behave as value operations from the user's point of view.

The implementation may use reference counting, copy-on-write or internal moves to optimize managed values, but those optimizations must not expose shared mutable aliasing in ordinary code.

Explicit references are not part of the MVP surface.

A future reference feature may be considered, but it should start as a restricted parameter-passing mechanism rather than general reference variables.

## Decision

Canonical value behavior:

```zt
const original: list<int> = [1, 2, 3]
var changed: list<int> = original

changed[0] = 10

// original remains [1, 2, 3]
// changed is [10, 2, 3]
```

Struct values behave as values:

```zt
const p1: Player = Player(name: "Julia", hp: 100)
var p2: Player = p1

p2.hp = 80

// p1.hp remains 100
// p2.hp is 80
```

Normative rules for this cut:

- Zenith has value semantics by default
- assignment creates a semantic value copy
- parameter passing creates a semantic value copy
- return produces/transfers a value to the caller
- `int`, `bool`, numeric types and similar scalar values copy by value
- `text` is an immutable value
- `struct` values have value semantics
- `enum` values have value semantics
- `optional<T>` has value semantics
- `result<Success, Error>` has value semantics
- `list<T>` has observable value semantics
- `map<Key, Value>` has observable value semantics
- `const` prevents observable mutation through that binding
- `var` permits mutation of the local value
- mutating methods declared with `mut func` are the primary user-visible mutation mechanism
- no `ref`, `borrow`, `&` or `&mut` surface syntax exists in the MVP
- no explicit `move` surface syntax exists in the MVP
- implementation optimizations must preserve observable value semantics

## Managed Values

Some values may require runtime ownership management.

Managed values include, at minimum:

- `text`
- `list<T>`
- `map<Key, Value>`
- `optional<T>` when `T` is managed
- `result<Success, Error>` when either branch is managed
- user structs with managed fields
- user enums with managed payloads

The compiler/runtime must release managed values when they leave scope.

The compiler/runtime may retain, clone, move internally or use copy-on-write when safe.

## Text

`text` is immutable from the language point of view.

This allows implementation sharing:

```zt
const a: text = "hello"
const b: text = a
```

Internally, `a` and `b` may share the same buffer.

Because `text` cannot be mutated, sharing is not observable as shared mutable state.

## Lists And Maps

Collections have observable value semantics.

This must not produce shared mutation:

```zt
const a: list<int> = [1, 2, 3]
var b: list<int> = a

b[0] = 10

// a[0] remains 1
```

The implementation may satisfy this through:

- deep copy on assignment
- reference counting plus copy-on-write
- internal move optimization when the source is provably unused
- other optimizations that preserve the same observable behavior

The language does not expose which strategy was used.

## Function Parameters

Ordinary function parameters receive values.

```zt
func damage(player: Player, amount: int) -> Player
    var next: Player = player
    next.hp = next.hp - amount
    return next
end
```

Calling the function does not mutate the original argument:

```zt
const p1: Player = Player(name: "Julia", hp: 100)
const p2: Player = damage(p1, 10)

// p1 remains unchanged
```

If mutation is intended, it must be visible through a mutating method or a future explicit reference feature.

## Mutating Methods

Mutating methods are declared with `mut func`.

```zt
apply Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Call site:

```zt
var player: Player = Player(name: "Julia", hp: 100)
player.heal(10)
```

Rules:

- `mut func` marks mutation in declarations
- mutating methods require a mutable receiver
- mutating methods cannot be called on `const` receivers
- ordinary function parameters are not secretly mutated

## No Surface Ref In MVP

The MVP does not include:

```zt
var b: ref list<int> = ref a
```

or:

```zt
func fill(items: ref mut list<int>)
    ...
end
```

The reason is that references are not just syntax. They require a coherent aliasing and lifetime model.

Open questions avoided by excluding `ref` from the MVP:

- immutable references versus mutable references
- whether references can be stored in variables
- whether references can be returned
- whether references can be fields in structs
- whether references can appear in lists or maps
- whether multiple mutable references can exist
- how to prevent dangling references
- how references interact with `return`, `?`, methods and future lambdas
- how references interact with managed value release

Introducing `ref` without this model would make mutation visible but still unsafe.

## Future Reference Direction

Explicit references remain a possible future feature.

If introduced, they should start restricted to function/method parameters.

Possible future direction:

```zt
mut func normalize(items: ref mut list<int>)
    ...
end

normalize(ref mut numbers)
```

Initial restrictions should likely include:

- references can be passed as parameters
- references cannot be stored in local variables
- references cannot be struct fields
- references cannot be returned
- references cannot be elements of lists or maps
- references cannot outlive the call
- mutable reference use must be visible at the call site
- aliasing rules must be defined before implementation

This keeps the door open for performance and controlled mutation without making the MVP a borrow-checking language.

## No Explicit Move In MVP

The MVP does not expose:

```zt
const b: text = move a
```

Move semantics introduce a new mental state: a value that existed but can no longer be used.

That model may be useful later, but it adds cognitive load and should not be part of the first stable surface.

The compiler may still perform internal moves as an optimization when they do not affect observable behavior.

## Relationship With Update Block

The future `update` block from Decision 032 is not a reference feature.

```zt
update players[0] as player
    player.hp = 80
end
```

It is sugar for read-modify-write:

```zt
var player: Player = players[0]
player.hp = 80
players[0] = player
```

It does not expose a general mutable reference to the list element.

## Rationale

Value semantics are easier to reason about, easier to teach and more aligned with Zenith's reading-first philosophy.

Everything-by-reference models create invisible aliasing. A reader cannot know whether a change through one name mutates another name without tracking identity.

Rust-style ownership and borrowing can be safe and performant, but it brings significant cognitive complexity. Zenith should not adopt that complexity in the MVP.

Reference counting and copy-on-write give the implementation room to be efficient while preserving a simpler user model.

Keeping explicit references out of the MVP prevents the language from committing to an incomplete lifetime/aliasing system too early.

## Non-Canonical Forms

Everything-by-reference behavior is not Zenith's model:

```zt
var b: list<int> = a
b[0] = 10

// non-canonical assumption: a also changed
```

General reference variables are not part of the MVP:

```zt
var b: ref list<int> = ref a
```

Returning references is not part of the MVP:

```zt
func first(items: list<int>) -> ref int
    ...
end
```

Explicit moves are not part of the MVP:

```zt
const b: list<int> = move a
```

## Diagnostics

Expected diagnostic directions:

```text
cannot call mutating method heal on const Player
```

```text
reference types are not supported in this implementation cut
```

```text
move expressions are not supported in this implementation cut
```

```text
cannot mutate item through const list<Player>
```

## Out of Scope

This decision does not yet define:

- concrete reference syntax
- borrow checking
- lifetimes
- aliasing rules for future references
- move-only types
- destructor customization
- weak references
- cycle collection for reference-counted structures
- pinning or stable addresses
- FFI reference ownership details
