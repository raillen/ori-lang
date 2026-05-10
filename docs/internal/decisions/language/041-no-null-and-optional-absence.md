# Decision 041 - No Null And Optional Absence

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics
- Scope: absence, `optional<T>`, `none`, optional matching, initialization, C interop nullable values

## Summary

Zenith has no `null`.

Expected absence is represented only through `optional<T>`.

The absence value is `none`.

Presence is written as the plain contained value in explicit `optional<T>` contexts, avoiding mandatory `some(value)` ceremony in ordinary code.

Matching an optional uses `case value name` and `case none`.

## Decision

Canonical optional return:

```zt
func find_user(id: int) -> optional<User>
    if id < 0
        return none
    end

    return User(name: "Julia")
end
```

Canonical optional match:

```zt
match find_user(id)
case value user:
    show_user(user)
case none:
    show_not_found()
end
```

Canonical optional field:

```zt
struct User
    name: text
    email: optional<text>
end

const user: User = User(
    name: "Julia",
    email: none
)
```

Normative rules for this cut:

- Zenith has no `null`
- expected absence is represented with `optional<T>`
- `none` represents absence
- `none` is valid only when the expected type is `optional<T>`
- mandatory `some(value)` construction is not part of the MVP surface
- a plain `T` value may be used as present value in explicit `optional<T>` contexts
- optional match uses `case value name` for presence
- optional match uses `case none` for absence
- `optional<T>` is not truthy
- `if optional_value` is invalid
- variables must be initialized
- there is no default null initialization
- struct fields without defaults must be initialized
- nullable C interop must be represented explicitly as `optional<T>`

## Contextual Presence

Presence can be written directly when the expected type is explicitly `optional<T>`.

Return from an optional function:

```zt
func find_name(id: int) -> optional<text>
    if id < 0
        return none
    end

    return "Julia"
end
```

Explicit local binding:

```zt
const name: optional<text> = "Julia"
```

Struct construction:

```zt
const user: User = User(
    name: "Julia",
    email: "julia@example.com"
)
```

when `email` has type `optional<text>`.

This contextual wrapping is intentionally limited. The type context must be explicit and local.

## No Broad Automatic Wrapping

Passing `T` where a function expects `optional<T>` is not part of the MVP.

Given:

```zt
func save_user(user: optional<User>)
    ...
end
```

This is not canonical in the MVP:

```zt
save_user(User(name: "Julia"))
```

Prefer making the optional value visible:

```zt
const user: optional<User> = User(name: "Julia")
save_user(user)
```

This keeps call-site intent easier to read and avoids broad implicit conversion.

## Matching Optional

Optional matching uses `value` and `none`.

```zt
match maybe_user
case value user:
    show_user(user)
case none:
    show_not_found()
end
```

`case value user` binds the present value to `user`.

This avoids requiring users to write `some(user)` while keeping the branch visually explicit.

## No Truthiness

`optional<T>` is not a boolean.

Invalid:

```zt
if find_user(id)
    ...
end
```

Valid:

```zt
match find_user(id)
case value user:
    show_user(user)
case none:
    show_not_found()
end
```

Or, inside a function returning `optional<...>`:

```zt
const user: User = find_user(id)?
```

## Initialization

Initialization is mandatory.

Invalid:

```zt
var user: User
```

Valid:

```zt
var user: User = User(name: "Julia")
```

Valid for optional state:

```zt
var user: optional<User> = none
```

Struct fields without defaults must be initialized:

```zt
struct User
    name: text
    email: optional<text>
end

const user: User = User(
    name: "Julia",
    email: none
)
```

No field receives implicit `null`.

## C Interop

C APIs that can return null must be modeled as `optional<T>`.

Future examples:

```zt
extern c func getenv(name: text) -> optional<text>
```

```zt
extern c func fopen(path: text, mode: text) -> optional<FileHandle>
```

A null pointer maps to `none`.

A non-null pointer maps to a present optional value.

The exact ownership rules at the C boundary require separate interop decisions.

## Relationship With Result

`optional<T>` means expected absence.

`result<T, E>` means expected recoverable failure.

There is no implicit conversion between them.

Example future explicit conversion:

```zt
const user: User = find_user(id).or_error("user not found")?
```

## Non-Canonical Forms

`null` is not a Zenith value:

```zt
const user: User = null
```

Mandatory `some(value)` is not canonical MVP style:

```zt
return some(user)
```

Treating optional as bool is invalid:

```zt
if maybe_user
    ...
end
```

Uninitialized variables are invalid:

```zt
var count: int
```

Broad automatic wrapping at arbitrary call sites is not part of the MVP:

```zt
save_user(User(name: "Julia"))
```

when `save_user` expects `optional<User>`.

## Diagnostics

Expected diagnostic directions:

```text
error[name.unresolved]
Zenith has no null value.

help
  Use optional<T> with none for expected absence.
```

```text
error[type.mismatch]
Expected User, but found none.

help
  Change the type to optional<User> or provide a User value.
```

```text
error[type.not_bool]
optional<User> cannot be used as a condition.

help
  Use match, ?, or compare explicitly with none when supported.
```

```text
error[init.required]
Variable user must be initialized.

help
  Provide an initial value or use optional<User> = none.
```

## Rationale

Removing `null` eliminates an entire class of hidden absence bugs.

Using `optional<T>` makes absence visible in signatures and fields.

Avoiding mandatory `some(value)` reduces ceremony in common optional-return code.

Limiting contextual presence to explicit optional contexts preserves readability without allowing broad implicit conversion.

`case value user` keeps optional matching textual and clear without introducing `some` as user-facing ceremony.

Mandatory initialization prevents hidden invalid states and aligns with Zenith's explicitness philosophy.

## Out of Scope

This decision does not yet define:

- optional helper stdlib APIs
- optional equality syntax with `none`
- optional chaining/safe navigation
- broad optional wrapping in function arguments
- exact C nullable ownership transfer rules
- optional memory layout
- nullable raw pointers for unsafe interop
