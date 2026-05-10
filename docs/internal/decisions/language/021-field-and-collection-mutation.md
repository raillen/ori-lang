# Decision 021 - Field And Collection Mutation

- Status: accepted
- Date: 2026-04-16
- Type: language / semantics
- Scope: field assignment, list assignment, map assignment, mutable versus immutable bindings

## Summary

Zenith Next allows field and collection mutation through assignment statements, but only through mutable bindings or receiver-mutating methods.

A mutable binding declared with `var` may be changed through field assignment such as `player.name = "Talyah"`.

A `const` binding may not be changed through field or collection assignment.

Inside methods, assignment through `self` requires the method declaration to use `mut func`.

## Decision

The canonical mutation forms in this cut are:

```zt
var p1: Player = Player(
    name: "Julia",
    age: 18
)

p1.name = "Talyah"
```

```zt
var scores: list<int> = [10, 20, 30]
scores[1] = 25
```

```zt
var metadata: map<text, text> = {}
metadata["mode"] = "debug"
```

Normative rules for this cut:

- field assignment is a statement of the form `target.field = value`
- list element assignment is a statement of the form `target[index] = value`
- map assignment is a statement of the form `target[key] = value`
- field and collection assignment require the target binding to be mutable
- a binding declared with `var` is mutable
- a binding declared with `const` is not mutable
- assigning through a `const` binding is invalid even when the underlying type is nominal or collection-like
- assigning through `self` inside a method requires a `mut func` declaration
- a method without `mut` may not assign through `self.field` or `self[index]`
- field assignment requires the assigned value to be type-compatible with the field type
- list assignment requires the assigned value to be type-compatible with the element type
- map assignment requires the key and value to be type-compatible with the map key and value types
- list indexing for assignment follows the same 0-based indexing rules as read access
- map assignment sets or replaces the entry for the given key
- mutation syntax does not imply reconstruction of the target value

## Canonical Forms

### Field Mutation Through `var`

```zt
var p1: Player = Player(
    name: "Julia",
    age: 18
)

p1.name = "Talyah"
p1.age = p1.age + 1
```

### Field Read Without Mutation

```zt
const age_next_year: int = p1.age + 1
```

### List Mutation

```zt
var values: list<int> = [1, 2, 3]
values[0] = 10
```

### Map Mutation

```zt
var config: map<text, text> = {}
config["mode"] = "release"
```

### Receiver Mutation In `mut func` Methods

```zt
apply Player
    public mut func rename(new_name: text)
        self.name = new_name
    end
end
```

## Rationale

This keeps mutability centered on the already-established `const` versus `var` distinction instead of adding a second mutability system for fields.

Allowing assignment through mutable bindings matches ordinary user expectations and avoids forcing reconstruction for simple updates.

Keeping `mut func` as the receiver-mutation marker preserves explicitness and keeps mutating behavior visible in declarations.

Using the same assignment surface for fields, lists and maps keeps the language smaller and easier to learn.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

mutating through a `const` binding:

```zt
const p1: Player = Player(
    name: "Julia",
    age: 18
)

p1.name = "Talyah"
```

mutating `self` inside a non-`mut` method:

```zt
apply Player
    public func rename(new_name: text)
        self.name = new_name
    end
end
```

inventing a separate setter keyword:

```zt
set p1.name = "Talyah"
```

reconstructing only because mutation syntax exists:

```zt
var p1: Player = Player(name: "Julia", age: 18)
p1 = Player(name: "Talyah", age: p1.age)
```

This may still be a valid value-level reassignment, but it is not the canonical way to express a simple field update when direct mutation is intended.

## Out of Scope

This decision does not yet define:

- deep aliasing guarantees between multiple mutable references to the same value
- immutable-by-field declarations such as `readonly name: text`
- copy-on-write semantics
- mutation through chained expressions such as `players[0].name = "Talyah"` if later semantic restrictions are added
- transaction-style or persistent update APIs

