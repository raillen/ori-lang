# Decision 029 - Executable Enums With Payload

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / implementation
- Scope: enum declarations, enum constructors, enum pattern matching, backend C representation, payload ownership

## Summary

Zenith Next supports executable enums with payload.

An enum case may carry zero or more named payload fields. Enum values are constructed through qualified case constructors and inspected through `match`.

The backend C representation uses a tag plus a union so that simple enums and payload enums share the same implementation direction.

## Decision

The canonical block form for enums with payload is:

```zt
enum Event
    Click(x: int, y: int)
    KeyPress(key: text)
    Close
end
```

Simple short enums may use inline declaration:

```zt
enum Direction = North, South, East, West
```

Payload enums may also use inline declaration when short enough to stay readable:

```zt
enum LoadResult = Loading, Success(data: text), Failed(message: text)
```

Normative rules for this cut:

- enum declarations may use block form or inline form
- block form is the canonical style for payload enums and larger enums
- inline form is the canonical style for short simple enums
- enum cases without payload use `Case`
- enum cases with payload use `Case(field: Type, other: Type)`
- enum case names use `PascalCase`
- enum case fields use the same naming rules as ordinary fields
- enum constructors are qualified with the enum name
- payload constructors use named arguments at construction sites
- no-payload cases may be used as values without parentheses
- `match` over enum values uses qualified cases
- payload binding in `match` is positional
- `case default` is allowed
- exhaustive matching is preferred for known enum types
- enum values are represented in C as `tag + union`
- enum simple cases are represented as cases with empty payload
- ownership and release behavior must account for managed payload values

## Construction

Payload case construction:

```zt
const event: Event = Event.Click(x: 10, y: 20)
```

No-payload case construction:

```zt
const event: Event = Event.Close
```

For readability, no-payload cases do not require parentheses.

This is non-canonical:

```zt
const event: Event = Event.Close()
```

## Match

Enum inspection uses `match`:

```zt
match event
case Event.Click(x, y):
    return x + y
case Event.KeyPress(key):
    return len(key)
case Event.Close:
    return 0
end
```

Payload binding is positional in the case pattern.

The field names remain visible in the enum declaration and at construction sites. Repeating names in the pattern would add noise without improving the ordinary reading path.

A default case is allowed:

```zt
match direction
case Direction.North:
    return 1
case default:
    return 0
end
```

For known enum types, exhaustive matching should be preferred by diagnostics and style guidance.

## Inline Form

Inline enum declaration is intended for compact cases:

```zt
enum Direction = North, South, East, West
```

Inline payload declaration is valid but should be used sparingly:

```zt
enum LoadResult = Loading, Success(data: text), Failed(message: text)
```

The formatter may later expand inline payload enums into block form when the declaration becomes too long.

These two forms are semantically equivalent:

```zt
enum Direction = North, South, East, West
```

```zt
enum Direction
    North
    South
    East
    West
end
```

## Backend C Representation

Given:

```zt
enum Event
    Click(x: int, y: int)
    KeyPress(key: text)
    Close
end
```

the backend C representation is conceptually:

```c
typedef enum {
    APP_EVENT_CLICK,
    APP_EVENT_KEY_PRESS,
    APP_EVENT_CLOSE
} app_Event_Tag;

typedef struct {
    app_Event_Tag tag;
    union {
        struct {
            zt_int x;
            zt_int y;
        } click;

        struct {
            zt_text key;
        } key_press;
    } payload;
} app_Event;
```

A simple enum uses the same model without meaningful union payload:

```c
typedef enum {
    APP_DIRECTION_NORTH,
    APP_DIRECTION_SOUTH,
    APP_DIRECTION_EAST,
    APP_DIRECTION_WEST
} app_Direction_Tag;

typedef struct {
    app_Direction_Tag tag;
} app_Direction;
```

Using `struct { tag }` even for simple enums keeps the backend direction compatible with payload enums and avoids a future representation split.

## Ownership And Release

Enum payload fields follow the same ownership rules as ordinary values of their type.

Managed payload values include, at minimum:

- `text`
- `list<T>`
- `map<K, V>`
- `optional<T>` when `T` is managed
- `result<Success, Error>` when either payload is managed
- user structs with managed fields
- user enums with managed payloads

The compiler/runtime must be able to release the active payload branch based on the enum tag.

Copy, move, retain and release behavior must be explicit in the internal model before enums with managed payloads are considered fully backend-complete.

## Exhaustiveness

For `match` over a statically known enum type:

- all enum cases should be checked by the semantic layer
- a match with all cases is exhaustive
- a match with `case default` is exhaustive
- a non-exhaustive match without `case default` is invalid when the match must produce a value or fully determine control flow
- diagnostics should name missing enum cases

Example diagnostic direction:

```text
non-exhaustive match for Event
missing cases: Event.KeyPress, Event.Close
```

The exact stage that enforces exhaustiveness may evolve with HIR/ZIR lowering, but the language rule is semantic.

## Rationale

Enums with payload make state plus data explicit without requiring inheritance or object-oriented hierarchies.

They fit Zenith's composition model with structs, traits and apply.

They are useful for compiler internals, AST nodes, parser results, UI events, domain states and future standard library APIs.

Using qualified constructors keeps call sites readable and avoids case-name collisions between enums.

Named payload arguments at construction sites preserve readability, while positional bindings in `match` keep pattern matching concise.

Choosing `tag + union` for C keeps the generated code direct and avoids runtime type metadata.

## Non-Canonical Forms

Unqualified enum construction is not canonical:

```zt
const event: Event = Click(x: 10, y: 20)
```

Using no-payload cases with parentheses is not canonical:

```zt
const direction: Direction = Direction.North()
```

Named payload binding in `match` is not part of the MVP:

```zt
case Event.Click(x: x, y: y):
    return x + y
```

Object-style enum inheritance is not part of Zenith:

```zt
enum Event extends BaseEvent
    ...
end
```

## Out of Scope

This decision does not yet define:

- enum methods beyond ordinary `apply EnumType`
- auto-derived `Equatable`, `Hashable` or `TextRepresentable`
- explicit discriminant numeric values
- C ABI-stable enum layout guarantees
- recursive enums requiring heap indirection
- generic enum monomorphization details beyond Decision 028
- pattern guards in `match`
- named binding syntax in enum patterns
