# Decision 031 - Runtime Where Contracts

- Status: accepted
- Date: 2026-04-17
- Type: language / semantics / runtime
- Scope: value-level `where`, runtime contract checks, struct fields, function parameters, validation style

## Summary

Zenith Next uses value-level `where` as a runtime contract mechanism for fields and function parameters.

A failed value-level `where` contract is a contract panic, not a recoverable `result`.

Recoverable validation remains explicit user code through `result`.

The MVP does not support `where` on local variables or function return types.

## Decision

Canonical field contract:

```zt
struct Player
    name: text
    hp: int where it >= 0
end
```

Canonical parameter contract:

```zt
func damage(
    player: Player,
    amount: int where it > 0
) -> Player
    var next: Player = player
    next.hp = next.hp - amount
    return next
end
```

Normative rules for this cut:

- value-level `where` is allowed on struct fields
- value-level `where` is allowed on function and method parameters
- value-level `where` is not allowed on local `const` or `var` declarations in the MVP
- value-level `where` is not allowed on function return types in the MVP
- generic `where` constraints remain separate compile-time semantic constraints
- failed value-level `where` checks produce a runtime contract panic
- `where` does not automatically convert construction, assignment or calls into `result`
- recoverable validation must be modeled explicitly with `result`
- field contracts are checked during struct construction
- field contracts are checked during field update
- field contracts are not rechecked on ordinary field read
- parameter contracts are checked on function/method entry
- contracts are active in the MVP, including release builds
- future formatter behavior should prefer multiline function signatures when parameter `where` clauses would harm readability

## Field Contracts

Field contracts preserve invariants of valid struct values.

Example:

```zt
struct User
    name: text where len(it) > 0
    age: int where it >= 0
end
```

Construction checks all field contracts:

```zt
const user: User = User(name: "Julia", age: 18)
```

Field update checks the target field contract:

```zt
var user: User = User(name: "Julia", age: 18)
user.age = 19
```

This fails with a contract panic:

```zt
user.age = -1
```

Ordinary reads do not revalidate:

```zt
const current_age: int = user.age
```

## Parameter Contracts

Parameter contracts validate values at function or method entry.

Simple predicate:

```zt
func heal(amount: int where it > 0)
    ...
end
```

Multiline signature is the canonical style when contracts would make the line long:

```zt
func create_user(
    name: text where len(name) > 0,
    age: int where it >= 0,
    email: text where validate.email(it)
) -> User
    ...
end
```

## `it` And Parameter Names

Inside a value-level `where` clause:

- `it` refers to the value currently being validated
- a parameter contract may also refer to the parameter by name
- a parameter contract may refer to earlier parameters
- a parameter contract may not refer to later parameters

Both forms are valid:

```zt
func set_age(age: int where it >= 0)
    ...
end
```

```zt
func set_age(age: int where age >= 0)
    ...
end
```

Referencing an earlier parameter is valid:

```zt
func slice_text(
    source: text,
    start: int where it >= 0,
    end: int where end >= start
) -> text
    ...
end
```

Referencing a later parameter is invalid:

```zt
func slice_text(
    start: int where start <= end,
    end: int
) -> text
    ...
end
```

because `end` has not been introduced when `start` is validated.

## Contract Panic

A failed contract means the program violated an invariant.

Example runtime direction:

```text
contract failed: Player.hp requires it >= 0
```

Generated C may lower checks to runtime calls such as:

```c
zt_contract_require_bool(condition, message, file, line);
```

The exact runtime API may evolve, but the language rule is that contract failure is not normal recoverable control flow.

## Recoverable Validation

External or user-provided data should use explicit `result` when failure is expected.

```zt
func create_player(name: text, hp: int) -> result<Player, text>
    if hp < 0
        return error("hp must be >= 0")
    end

    return success(Player(name: name, hp: hp))
end
```

This keeps the distinction clear:

- `where` means invariant contract
- `result` means expected recoverable failure
- `optional` means expected absence

## Future `validate.*` Predicates

A future stdlib module may provide readable validation predicates for `where`.

Examples:

```zt
struct User
    name: text where validate.not_empty(it)
    age: int where validate.between(it, min: 0, max: 130)
end
```

```zt
func create_user(
    email: text where validate.email(it),
    password: text where validate.min_length(it, 8)
) -> User
    ...
end
```

Expected direction for `validate.*`:

- functions return `bool`
- functions are deterministic
- functions do not perform I/O
- functions do not mutate state
- functions improve readability but do not change `where` semantics
- failed predicates still produce contract panic
- recoverable validation still requires explicit `result`

The exact `validate` API belongs to future stdlib design.

## Non-Canonical Forms

Local variable contracts are not part of the MVP:

```zt
const age: int where it >= 0 = input_age
```

Return type contracts are not part of the MVP:

```zt
func percent(value: int) -> int where it >= 0 and it <= 100
    ...
end
```

Automatic `result` conversion is not canonical:

```zt
const player: Player = Player(name: name, hp: hp)?
```

unless the constructor is explicitly a function that returns `result<Player, Error>`.

## Diagnostics

Invalid `where` usage should produce direct diagnostics.

Examples:

```text
where on local variables is not supported in this implementation cut
```

```text
where on function return types is not supported in this implementation cut
```

```text
parameter contract cannot reference later parameter end
```

Runtime contract failures should include the contract subject and predicate when possible.

## Rationale

Keeping `where` on fields and parameters gives Zenith strong invariant expression without making ordinary signatures too visually heavy.

Excluding return contracts avoids clutter in the most important part of a function signature.

Excluding local variable contracts keeps the first implementation cut smaller and pushes durable invariants toward named types and structs.

Using contract panic instead of automatic `result` preserves a clean distinction between invariant violation and expected failure.

Allowing both `it` and the parameter name improves readability: simple contracts stay compact, while longer contracts can name the value directly.

Allowing references to earlier parameters supports common range and relationship contracts while preserving left-to-right validation.

## Out of Scope

This decision does not yet define:

- formatter line-length thresholds
- the concrete `validate.*` stdlib API
- configurable debug/release contract levels
- contract recovery hooks
- custom contract error messages
- return type contracts
- local variable contracts
- dependent types or compile-time proof of `where` predicates
