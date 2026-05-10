# Decision 010 - Structs Traits Apply Enums And Match

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: data types, composition, trait contracts, receiver methods, enums, match

## Summary

Zenith Next models composition through `struct`, `trait` and `apply`, not through class-based inheritance.

`struct` defines named data.

`trait` defines behavior contracts.

`apply Type` adds inherent behavior to a type, and `apply Trait to Type` implements a trait for a type.

Methods use an implicit receiver in the signature and a reserved `self` inside the body.

Methods that change the receiver are declared with `mut func`.

`match` is part of the MVP as a statement form with `case`, comma-separated multiple patterns and `case default`.

## Decision

The canonical forms are:

```zt
public struct Player
    nome: text
    hp: int
end

public trait Healable
    mut func heal(amount: int)
end

apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end

apply Player
    public func is_alive() -> bool
        return self.hp > 0
    end
end
```

And for enums and match:

```zt
public enum ReadResult
    Success(content: text)
    NotFound
    InvalidEncoding(message: text)
end

public func describe(result: ReadResult) -> text
    match result
        case ReadResult.Success(content)
            return content

        case ReadResult.NotFound, ReadResult.InvalidEncoding(message)
            return "could not read file"

        case default
            return "unknown state"
    end
end
```

Normative rules for this cut:

- `struct` defines data with named fields
- `struct` bodies contain fields, not method implementations
- `trait` defines method contracts
- `apply Type` defines inherent methods for a type
- `apply Trait to Type` implements a trait for a type
- class inheritance is not part of the MVP
- receiver methods do not list `self` in the signature
- `self` is a reserved name inside trait and apply method bodies
- a method declared with `mut func` may change the receiver
- a method without `mut` may not assign through `self`
- a trait method that changes the receiver must also use `mut func`
- enum variants may have no payload or a named payload
- enum variant construction uses `EnumName.VariantName(...)`
- `match` is a statement in the MVP, not an expression
- `match` uses `case` branches and closes with `end`
- a `case` may list multiple patterns separated by commas
- `case default` is the canonical fallback branch
- `case default` must be the final branch
- `match` has no fallthrough
- `match` must be exhaustive unless it ends with `case default`
- each `case` introduces its own local scope
- enum payload binding in patterns uses the declared payload names in the MVP
- payload renaming inside patterns is not part of this cut

## Rationale

This keeps data, contracts and implementation clearly separated.

Using `apply` instead of method bodies inside `struct` avoids drifting toward a class-shaped model while still allowing method-call syntax.

Keeping `self` implicit in the signature reduces noise, because the receiver is already implied by the surrounding `apply` or `trait`.

Using `mut func` makes mutation part of the visible contract without adding extra clauses such as `changes self`.

Comma-separated multiple `case` patterns are shorter and clearer than using boolean syntax such as `or` inside pattern position.

Using `case default` instead of a symbolic wildcard keeps fallback matching more legible.

## Canonical Examples

Struct with named fields:

```zt
public struct User
    name: text
    age: int
end
```

Trait declaration:

```zt
public trait Healable
    mut func heal(amount: int)
end
```

Trait implementation with receiver mutation:

```zt
apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Inherent behavior without mutation:

```zt
apply Player
    public func is_alive() -> bool
        return self.hp > 0
    end
end
```

Enum construction:

```zt
return ReadResult.Success(content: file_text)
```

Literal-style multiple match cases:

```zt
match status_code
    case 10, 20, 30
        return "accepted"
    case default
        return "other"
end
```

Enum match with payload binding:

```zt
match result
    case ReadResult.Success(content)
        return content
    case ReadResult.NotFound
        return "not found"
    case ReadResult.InvalidEncoding(message)
        return message
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

methods inside the `struct` body:

```zt
struct Player
    nome: text
    hp: int

    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

listing `self` as an explicit parameter:

```zt
trait Healable
    mut func heal(self, amount: int)
end
```

using a receiver-effect clause instead of `mut func`:

```zt
func heal(amount: int) changes self
    ...
end
```

using boolean `or` in `case` patterns:

```zt
match status_code
    case 10 or 20 or 30
        return "accepted"
end
```

using a symbolic fallback wildcard as the canonical default:

```zt
match result
    case _
        return "unknown"
end
```

## Out of Scope

This decision does not yet define:

- field invariants such as `hp: int where it >= 0`
- dispatch strategy details for traits
- pattern renaming such as `case Variant(field: local_name)`
- match expressions that yield a value directly
- mutation policy for fields behind immutable bindings
