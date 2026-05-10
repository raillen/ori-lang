# Zenith Formatting Rules

The formatter is **part of the language**, not optional polish.

## Command

```text
zt fmt              -- format project
zt fmt --check      -- check without writing
```

## Core Rules

- Indentation: **4 spaces** (no tabs)
- Formatting is **deterministic**
- Formatting is **idempotent**
- Broad style configuration is NOT MVP
- Comments preserved when possible
- Formatter must NOT change semantics
- Generated C is NOT formatted

## File Shape

Canonical order:
1. Leading comments (if any)
2. `namespace` declaration
3. Imports (sorted lexically by namespace path)
4. Declarations

## Blocks

All block forms align closing `end` with opening keyword:

```zt
if ready
    start()
else if waiting
    queue()
else
    stop()
end
```

## Function Signatures

Short signatures stay on one line:

```zt
func render_name(user: User) -> text
    return user.name
end
```

Long signatures use multiline parameters:

```zt
func create_user(
    name: text where validate.not_empty(it),
    age: int where validate.between(it, min: 0, max: 130),
    email: text where validate.email(it)
) -> result<User, UserError>
    ...
end
```

Multiline preferred when:
- Multiple parameters with `where`
- Line length exceeds hard target (100)
- Named defaults make signature visually dense

## Named Arguments

Long named calls use one argument per line:

```zt
const user: User = create_user(
    name: "Julia",
    age: 18,
    email: "julia@example.com"
)?
```

## Structs and Enums

- Struct fields: one field per line
- Inline enum form: only when short and visually clear
- Long/payload enums: block form

## Match

Each `case` is indented one level under `match`:

```zt
match result
    case success(value) ->
        return value
    case error(message) ->
        return 0
    case else:
        return 0
end
```

## Attributes

- One `attr` per line
- No blank line between attribute and its declaration

```zt
attr test
func loads_user()
    check(true)
end
```

## Comments

- Line comments: `--`
- Block comments: `--- ... ---`
- Long public documentation belongs in ZDoc

## Naming Conventions

| Element | Convention | Examples |
|---------|------------|----------|
| Types | PascalCase | `Player`, `LoadUserError`, `Widget` |
| Enum cases | PascalCase | `North`, `South`, `NotFound` |
| Functions | snake_case | `render_name`, `try_create_user`, `next_frame` |
| Variables | snake_case | `frame_count`, `user_name`, `hp` |
| Fields | snake_case | `name`, `hp`, `position` |
| Namespaces | lower snake_case segments | `app.users`, `std.io`, `app.runtime.state` |
| Generic parameters | Descriptive PascalCase | `Item`, `Key`, `Value`, `T` |
| Traits | PascalCase | `Equatable`, `Healable`, `TextRepresentable` |

## Target Width

- Line length target: **100 characters**
