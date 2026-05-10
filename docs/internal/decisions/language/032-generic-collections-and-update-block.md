# Decision 032 - Generic Collections And Update Block

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / implementation
- Scope: generic `list<T>`, generic `map<Key, Value>`, collection mutability, map lookup, update block sugar

## Summary

Zenith Next supports generic collections through the monomorphization model defined in Decision 028.

The language-level collection model is:

- `list<T>` for ordered indexed values
- `map<Key, Value>` for key-value storage
- `const` collections are observably immutable
- `var` collections may be updated through explicit operations
- deep mutation through indexed access is not part of the MVP
- a future `update target as name ... end` block provides readable read-modify-write sugar

## Decision

Canonical generic list:

```zt
const players: list<Player> = [
    Player(name: "Julia", hp: 100),
    Player(name: "Talyah", hp: 90)
]
```

Canonical generic map:

```zt
const scores: map<text, int> = {
    "Julia": 10,
    "Talyah": 20
}
```

Normative rules for this cut:

- `list<T>` is generic over any monomorphizable `T`
- `map<Key, Value>` is generic over monomorphizable `Key` and `Value`
- `map<Key, Value>` requires `Key is Hashable<Key>` and `Key is Equatable<Key>`
- `list<T>` preserves insertion/order semantics
- `map<Key, Value>` does not guarantee iteration order in the MVP
- list and map indexing is zero-based where numeric indexes are used
- list slicing uses the previously accepted inclusive-end slice semantics
- `len(collection)` remains the compiler/core intrinsic for collection length in the current compiler cut
- `const` collections are observably immutable
- `var` collections allow explicit element/key update
- direct deep mutation through indexed access is not part of the MVP
- `map[key]` panics if `key` is absent
- a future `map.get(key)` API should return `optional<Value>`
- collection methods and method chaining belong to future stdlib/core design, not the current collection semantics cut

## List Operations

Supported language-level operations:

```zt
const first: Player = players[0]
const party: list<Player> = players[0..1]
const count: int = len(players)
```

Update requires `var`:

```zt
var players: list<Player> = [
    Player(name: "Julia", hp: 100)
]

players[0] = Player(name: "Julia", hp: 80)
```

This is invalid because the list is `const`:

```zt
const players: list<Player> = [
    Player(name: "Julia", hp: 100)
]

players[0] = Player(name: "Julia", hp: 80)
```

## Map Operations

Supported language-level operations:

```zt
const score: int = scores["Julia"]
const count: int = len(scores)
```

Update requires `var`:

```zt
var scores: map<text, int> = {}
scores["Julia"] = 10
scores["Julia"] = 20
```

Iteration does not guarantee order:

```zt
for key, value in scores
    show_score(key, value)
end
```

## Missing Map Keys

Index lookup is strict:

```zt
const score: int = scores["Missing"]
```

If `"Missing"` is absent, this produces a runtime panic.

The safe lookup direction belongs to stdlib/core collection APIs:

```zt
const score: optional<int> = scores.get("Julia")
```

With `?` propagation, future code may write:

```zt
const score: int = scores.get("Julia")?
```

inside a function returning `optional<...>`.

## Observable Immutability

`const` means the observable value cannot be changed through that binding.

This is invalid:

```zt
const numbers: list<int> = [1, 2, 3]
numbers[0] = 10
```

This is also invalid:

```zt
const players: list<Player> = [
    Player(name: "Julia", hp: 100)
]

players[0].hp = 80
```

Even though the list reference is unchanged, the observable contents of the list would change.

## Deep Mutation

Direct deep mutation through indexed access is not part of the MVP:

```zt
players[0].hp = 80
```

The explicit MVP form is read-modify-write:

```zt
var player: Player = players[0]
player.hp = 80
players[0] = player
```

This is more verbose, but it keeps ownership, validation and generated C behavior clear.

## Future Update Block

Zenith reserves a future update block as official sugar for read-modify-write.

List item update:

```zt
update players[0] as player
    player.hp = 80
end
```

Equivalent conceptual form:

```zt
var player: Player = players[0]
player.hp = 80
players[0] = player
```

Map value update:

```zt
update scores["Julia"] as score
    score = score + 10
end
```

Equivalent conceptual form:

```zt
var score: int = scores["Julia"]
score = score + 10
scores["Julia"] = score
```

Normative direction for `update`:

- `update` is sugar for read-modify-write
- `update` is not a general mutable-reference feature
- the target collection must be mutable
- the bound name inside the block is mutable
- the value is written back only if the block reaches `end` normally
- `return`, propagated `?` or contract panic do not write back
- `break` and `continue` inside `update` are prohibited in the MVP
- first target forms are `list[index]` and `map[key]`
- nested targets such as `team.players[0]` are out of scope for the first `update` cut

## Methods And Chaining

Methods on user structs already exist through `apply`.

Collection methods such as these are future stdlib/core API work:

```zt
players.get(0)
players.append(player)
scores.get("Julia")
scores.remove("Julia")
```

Method chaining is also future work:

```zt
const score: int = scores.get("Julia").or_error("missing")?
```

The current collection semantics should not block future methods, but it should not depend on them either.

The compiler core keeps these operations stable first:

```zt
len(players)
players[index]
players[start..end]
players[index] = value
scores[key]
scores[key] = value
```

## Ownership

A monomorphized collection instance owns or manages its elements according to the element type.

Managed types include, at minimum:

- `text`
- `list<T>`
- `map<Key, Value>`
- `optional<T>` when `T` is managed
- `result<Success, Error>` when either branch is managed
- user structs with managed fields
- user enums with managed payloads

Updating an element or map value must release/retain/copy/move according to the active ownership model.

The exact runtime representation may evolve, but the language requires the observable behavior to be safe.

## Non-Canonical Forms

Mutating a `const` collection is invalid:

```zt
const scores: map<text, int> = {}
scores["Julia"] = 10
```

Deep mutation through index access is not part of the MVP:

```zt
players[0].hp = 80
```

Assuming map iteration order is non-canonical:

```zt
for key, value in scores
    -- do not rely on a stable order in the MVP
end
```

Assuming `map[key]` returns optional is invalid:

```zt
const score: optional<int> = scores["Julia"]
```

Use the future `get` API for optional lookup.

## Diagnostics

Expected diagnostic directions:

```text
cannot update item of const list<Player>
```

```text
deep mutation through indexed access is not supported in this implementation cut
use read-modify-write or update ... as ... end
```

```text
map key type User must implement Hashable<User> and Equatable<User>
```

Runtime missing-key direction:

```text
map lookup failed: key not found
```

## Rationale

Observable immutability keeps `const` meaningful for readers.

Read-modify-write is verbose but explicit, and the future `update` block preserves that semantic clarity while reducing boilerplate.

Keeping `map[key]` strict mirrors list indexing: both are direct access operations that require the target to exist.

Providing future `map.get(key) -> optional<Value>` gives a safe path without making every direct lookup noisy.

Separating compiler-core syntax from future collection methods allows the backend and monomorphization model to stabilize before growing the stdlib API surface.

## Out of Scope

This decision does not yet define:

- concrete stdlib method names for all collection operations
- method chaining rules
- lambdas or higher-order collection operations
- lazy collections
- stable map iteration order
- nested `update` targets
- deep mutable references
- borrow checking or aliasing model
- custom collection implementations

