# Decision 014 - Contracts Refinement And Field Invariants

- Status: accepted
- Date: 2026-04-16
- Type: language / semantics
- Scope: where clauses, declaration contracts, field invariants, runtime contract checks

## Summary

Zenith Next uses `where` as the single MVP surface for contracts and refinement.

Field invariants are expressed with the same `where` mechanism used for other declaration-level contracts.

The compiler should prove contract satisfaction statically when possible and insert runtime contract checks when it cannot.

`validate` is not part of the Zenith Next MVP.

If reusable validation helpers are added later, they should live in ordinary library modules and be called from `where`, not reintroduced as a second contract keyword.

## Decision

The canonical contract form is a single `where` clause attached to a typed declaration surface.

Canonical examples:

```zt
public struct Player
    nome: text
    hp: int where it >= 0
end
```

```zt
public func heal_amount(amount: int where it > 0) -> int where it > 0
    return amount
end
```

Normative rules for this cut:

- `where` is the canonical MVP mechanism for contracts and refinement
- `validate` is not part of the Zenith Next MVP
- reusable validation helpers belong in library modules used from `where`, not in a separate `validate` keyword
- field invariants use `field_name: Type where predicate`
- function parameters may use `name: Type where predicate`
- value-returning function results may use `-> Type where predicate`
- `it` is the bound value being constrained by the `where` predicate
- a `where` predicate must have type `bool`
- multiple independent checks should be combined with `and` inside one predicate rather than repeated `where` clauses
- contracts do not implicitly change the underlying runtime representation of the constrained type
- contracts do not create a distinct nominal type in this MVP cut
- generic type constraints use the same keyword in a separate declaration position and are defined separately from value-level contracts

## Enforcement Model

The compiler should enforce contracts in two phases:

1. prove them statically when the program already makes the truth obvious
2. insert runtime contract checks at the relevant boundaries when static proof is unavailable

The relevant runtime boundaries in this cut are:

- struct construction for field invariants
- assignment to a field that carries an invariant
- function entry for parameter contracts
- function return for return contracts

A contract violation in the MVP is a runtime failure, not an automatic `result` or `optional` conversion.

Contract failures are part of the runtime error model and should carry source location when available.

## Rationale

Using one keyword, `where`, keeps the language surface smaller and easier to teach.

Reusing the same mechanism for field invariants, parameter checks and return checks gives the language one mental model instead of several near-duplicates.

Not introducing a separate `validate` form in the MVP avoids duplicated syntax and avoids forcing users to learn two overlapping ways to say "this value must satisfy a rule".

Keeping contracts as checks over ordinary values, rather than as distinct nominal types, keeps the first implementation smaller and easier to lower into the existing runtime and backend work.

If reusable predicates become common, they can be provided by a library namespace such as `validation`, while `where` remains the single surface form for contracts.

## Canonical Examples

Field invariant:

```zt
public struct Player
    nome: text
    hp: int where it >= 0
end
```

Parameter contract:

```zt
public func take_damage(current_hp: int where it >= 0, amount: int where it > 0) -> int
    return current_hp - amount
end
```

Return contract:

```zt
public func clamp_to_zero(value: int) -> int where it >= 0
    if value < 0
        return 0
    end
    return value
end
```

Combined predicate:

```zt
public struct UserName
    value: text where text.len(it) > 0 and not text.contains(it, " ")
end
```

Reusable helper via library function:

```zt
public struct EmailAddress
    value: text where validation.email(it)
end
```

## Semantic Requirements

The semantic analyzer must reject the following:

- a `where` predicate whose type is not `bool`
- a contract that references names outside the declaration context in an invalid way
- contract syntax attached to declarations not supported by this cut
- multiple repeated `where` clauses on the same declaration surface in canonical MVP code

The semantic analyzer should also attempt constant or obviously-provable satisfaction when values are known directly.

## Runtime Requirements

When runtime checks are emitted, they must:

- run at the relevant boundary, not at arbitrary later use sites
- preserve the source-level meaning of the original type
- fail with a structured runtime contract violation
- not silently coerce, clamp or rewrite the offending value

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

using `validate` in the Next MVP:

```zt
hp: int validate min_value(0)
```

treating `validate` as a contract keyword instead of a library namespace:

```zt
name: text validate non_empty
```

repeating multiple `where` clauses instead of combining them:

```zt
hp: int where it >= 0 where it <= 999
```

expecting a contract to auto-convert into `result`:

```zt
public func heal_amount(amount: int where it > 0) -> result<int, text>
    ...
end
```

This is non-canonical if the intention is that a failed contract should automatically become `error(...)`.

treating a contract as if it created a new nominal type:

```zt
const hp: int where it >= 0 = 10
```

and later expecting `int where it >= 0` to be a separately named runtime type.

## Out of Scope

This decision does not yet define:

- user-defined named refinement aliases
- purity annotations for helper functions used inside contracts
- the final name and shape of any future standard validation helper module
- whether local variable declarations may carry `where` in the MVP
- collection-wide invariants beyond ordinary field contracts
- recovery-oriented contract APIs instead of fail-fast runtime behavior
