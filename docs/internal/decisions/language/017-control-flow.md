# Decision 017 - Control Flow

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: branching, loops, loop control, fixed repetition

## Summary

Zenith Next uses explicit block-based control flow with `if`, `else if`, `else`, `while`, `for ... in ...`, key-value `for key, value in ...` and `repeat ... times`.

All control-flow forms close with `end`.

Conditions must have type `bool`.

The MVP includes `break` and `continue`.

`unless` is not part of the Zenith Next MVP.

## Decision

The canonical control-flow forms are:

```zt
if player.hp > 0
    return true
else if player.hp == 0
    return false
else
    return false
end
```

```zt
while index < total
    index = index + 1
end
```

```zt
for player in players
    display(player)
end
```

```zt
for key, value in scores
    show_score(key, value)
end
```

```zt
repeat 5 times
    print("Ola, Zenith!")
end
```

Normative rules for this cut:

- `if` introduces a conditional branch
- `else if` is the canonical chained conditional form
- `else` is the canonical fallback branch
- `if` blocks close with `end`
- `while` introduces a condition-controlled loop
- `while` blocks close with `end`
- `for item in collection` introduces iteration over a collection-like value
- `for key, value in map_like` introduces key-value iteration
- `for` blocks close with `end`
- `repeat ... times` introduces fixed-count repetition
- `repeat` blocks close with `end`
- all conditions used by `if` and `while` must have type `bool`
- the repeat count expression must have type `int`
- the repeat count expression is evaluated once before loop execution starts
- a repeat count of `0` executes zero iterations
- a negative repeat count is invalid
- `break` exits the nearest enclosing loop
- `continue` skips to the next iteration of the nearest enclosing loop
- `break` and `continue` are valid only inside loops
- `for` in this MVP is collection-oriented, not C-style
- `unless` is not part of this MVP cut

## Canonical Forms

### `if`

```zt
if not player.is_alive()
    return
end
```

```zt
if value > 0
    return "positive"
else if value < 0
    return "negative"
else
    return "zero"
end
```

### `while`

```zt
var index: int = 0

while index < total
    process(index)
    index = index + 1
end
```

### `for`

Single binding:

```zt
for player in players
    display(player)
end
```

Key-value iteration:

```zt
for name, score in scores
    show_score(name, score)
end
```

### `repeat ... times`

```zt
repeat 3 times
    print("tick")
end
```

The repeat expression may be any expression of type `int`:

```zt
repeat retries + 1 times
    attempt()
end
```

## Loop Control

`break` and `continue` are part of the MVP.

Canonical example:

```zt
while true
    if should_stop
        break
    end

    if should_skip
        continue
    end

    process()
end
```

## Rationale

Using one explicit block closer, `end`, keeps control-flow forms consistent with the rest of the language.

Rejecting `unless` in the MVP avoids a second branching form whose meaning becomes less clear once `else` and nested negation are involved.

Keeping `for` collection-oriented avoids introducing a second mini-language for loop control when `while` already covers open-ended iteration.

Including `repeat ... times` improves readability for fixed-count repetition and avoids forcing authors to write a manual counter loop for simple repeated actions.

Making conditions explicitly `bool` preserves the language rule against truthiness and keeps flow analysis more predictable.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `unless` as a second branching keyword:

```zt
unless player.is_alive()
    return
end
```

using `elif` instead of `else if`:

```zt
if ready
    start()
elif waiting
    queue()
end
```

using `then` in conditional syntax:

```zt
if ready then
    start()
end
```

using a C-style `for` loop:

```zt
for i = 0; i < total; i = i + 1
    process(i)
end
```

using `repeat` without `times`:

```zt
repeat 5
    print("tick")
end
```

## Out of Scope

This decision does not yet define:

- range syntax such as `0..10`
- labeled loops
- `switch`
- `repeat until`
- loop expressions that yield values
- destructuring beyond the simple `for key, value in map` form
- async or parallel control-flow constructs
- exact iteration order guarantees for all collection types
