# Decision 012 - Syntax Sugar Policy

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: sugar policy, MVP boundaries, readability rules

## Summary

Zenith Next adopts a conservative syntax sugar policy for the MVP.

Sugar enters the language only when it improves reading, not merely typing speed.

The canonical form of the language must remain small, stable and easy to teach.

Any sugar that introduces hidden control flow, ambiguous name resolution or a second equally official style stays out of the MVP.

## Decision

Sugar is accepted into the MVP only when it satisfies all of the following:

- it improves readability, not only brevity
- it desugars locally and predictably
- it does not change name resolution rules
- it does not hide control flow
- it does not require new inference rules
- it does not create a second equally canonical style for the same construct

The following are part of the canonical surface and are not treated as deferred sugar:

- omitting `-> void` on functions that do not return a value
- marking receiver-changing methods with `mut func`
- returning a plain value in a context that already expects `optional<T>`

The following stay out of the MVP:

- pipe operators
- additional UFCS sugar beyond the method-call forms already implied by `apply`
- omitting `self.`
- omitting `return`
- ternary expressions
- destructuring sugar
- optional chaining such as `?.`
- null-coalescing such as `??`
- selective imports
- alias magic or auto-import behavior
- convenience operators added only for terseness
- alternate short forms of `match`

## Rationale

The MVP needs one dominant, teachable form for each major construct.

Adding sugar too early creates several half-official styles, which increases documentation cost, parser complexity, formatter burden and cognitive load during reading.

Because Zenith prioritizes reading comfort and predictability, the burden of proof is on the sugar. A shorter form is not sufficient on its own.

This policy also protects future language evolution: once the core forms are stable in practice, sugar can be evaluated with real examples instead of guesses.

## Canonical Guidance

When designing or evaluating future sugar, the default answer is "not yet" unless the proposal clearly improves reading.

The expected workflow is:

1. define the explicit canonical form first
2. validate that form in documentation, examples and implementation
3. add sugar only if it remains transparent after desugaring

## Canonical Examples

Canonical explicit return:

```zt
public func normalize_name(id: int) -> optional<text>
    const name: text = find_name(id)?
    return text.trim(name)
end
```

Canonical receiver mutation marker:

```zt
apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Canonical explicit branching:

```zt
match status_code
    case 10, 20, 30
        return "accepted"
    case default
        return "other"
end
```

## Non-Canonical Forms

These forms are not accepted as part of the MVP:

pipe operator:

```zt
const display_name: text = user
    |> get_name()
    |> text.trim()
```

implicit `self.` access:

```zt
mut func heal(amount: int)
    hp = hp + amount
end
```

implicit return:

```zt
func is_alive() -> bool
    self.hp > 0
end
```

optional chaining:

```zt
const city_name: text = user?.address?.city ?? "unknown"
```

## Out of Scope

This decision does not yet define:

- which future sugar proposals will eventually be accepted
- formatter normalization rules for future sugar
- whether some sugar should be allowed only in local scopes
- a formal process for promoting a sugar proposal from experimental to canonical
