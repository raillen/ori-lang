# Decision 020 - Construction And Initialization

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: struct construction, named fields, required fields, defaults, optional fields

## Summary

Zenith Next constructs nominal values through `TypeName(...)` with named fields.

The MVP does not use `new` and does not support positional struct construction.

Fields without defaults are required at construction time.

Defaults are allowed in struct field declarations, but they are conservative and do not depend on `self` or on other fields in this cut.

Optional fields are expressed with `optional<T>`, not with dedicated field sugar.

## Decision

The canonical construction form is:

```zt
const player: Player = Player(
    name: "Ayla",
    hp: 100
)
```

Normative rules for this cut:

- nominal value construction uses `TypeName(...)`
- struct construction uses named field arguments
- positional struct construction is not part of the MVP
- `new` is not part of the canonical construction syntax
- every field without a default value must be provided at construction time
- a field with a default value may be omitted at construction time
- omitted fields use their declared default value
- defaults are declared on the field itself with `field_name: Type = expression`
- default expressions may not reference `self` in this cut
- default expressions may not depend on sibling field values in this cut
- optional fields are modeled with `optional<T>`
- there is no dedicated optional-field marker syntax in this cut
- construction syntax is value creation, not mutation
- reading fields to derive other values does not require reconstructing the original struct value

## Canonical Forms

### Named Construction

```zt
public struct Player
    name: text
    hp: int
end

const player: Player = Player(
    name: "Ayla",
    hp: 100
)
```

### Required Fields

This is valid:

```zt
const player: Player = Player(
    name: "Ayla",
    hp: 100
)
```

This is not canonical and is semantically invalid because `hp` is missing:

```zt
const player: Player = Player(
    name: "Ayla"
)
```

### Defaults

```zt
public struct RetryPolicy
    max_attempts: int = 3
    delay_ms: int = 500
end

const standard_policy: RetryPolicy = RetryPolicy()

const fast_policy: RetryPolicy = RetryPolicy(
    delay_ms: 100
)
```

### Optional Fields

```zt
public struct User
    name: text
    nickname: optional<text> = none
end
```

### Deriving Other Values From Fields

```zt
var p_hp: int = player.hp + 40
```

This reads a field and creates another value. It does not reconstruct `player`.

## Rationale

Named construction is easier to read than positional construction, especially as structs grow over time.

Rejecting `new` keeps value creation lightweight and avoids importing class-shaped habits into a language that is not object-oriented.

Defaults are useful, but allowing them to reference `self` or other fields would make initialization order and semantic lowering more complex than necessary in the MVP.

Using `optional<T>` directly for optional fields keeps the type model uniform and avoids introducing field-specific sugar that would duplicate existing language concepts.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `new`:

```zt
const player: Player = new Player(
    name: "Ayla",
    hp: 100
)
```

positional construction:

```zt
const player: Player = Player("Ayla", 100)
```

field-specific optional sugar:

```zt
public struct User
    nickname?: text
end
```

defaults that depend on `self`:

```zt
public struct Example
    value: int = self.other
end
```

defaults that depend on sibling fields:

```zt
public struct Window
    width: int = 640
    area: int = width * 480
end
```

## Out of Scope

This decision does not yet define:

- copy/update sugar for existing values
- builder APIs or auto-generated constructor helpers
- field visibility modifiers beyond the existing `public` rules
- constructor validation hooks beyond ordinary contracts and defaults
