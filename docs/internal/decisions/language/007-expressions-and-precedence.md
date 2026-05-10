# Decision 007 - Expressions And Precedence

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: expressions, operators, precedence, conditions

## Summary

Zenith Next uses a conservative expression model centered on readability and predictability.

Logical operators use words instead of symbolic forms.

Assignment is a statement, not an expression.

The MVP does not include a dedicated exponentiation operator.

## Decision

The canonical expression forms in this cut are:

- function call: `func_name(arg1, arg2)`
- field access: `value.field`
- indexing: `value[index]`
- grouping: `(expression)`
- unary: `-value`, `not value`
- arithmetic: `*`, `/`, `%`, `+`, `-`
- comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- logical: `and`, `or`

Normative rules for this cut:

- calls, field access and indexing are expression forms
- assignment is not an expression
- conditional expressions such as ternary syntax are not part of the MVP
- `++` and `--` are not part of the MVP
- logical operators are `and`, `or` and `not`
- `&&`, `||` and `!` are not part of the MVP
- conditions must have type `bool`
- truthiness is not part of the MVP
- chained comparisons such as `0 < x < 10` are not part of the MVP
- the MVP does not define a dedicated exponentiation operator
- exponentiation is expressed with a standard-library function such as `math.pow(base, exponent)`

## Precedence

From strongest binding to weakest binding:

1. field access, call, indexing
2. unary operators
3. multiplicative operators
4. additive operators
5. comparison operators
6. logical `and`
7. logical `or`

## Rationale

This operator set is intentionally small.

Word-based boolean operators are easier to scan than symbolic boolean operators in a language that prioritizes reading comfort.

Keeping assignment out of expression position reduces compact but harder-to-read code patterns.

Truthiness was rejected because it hides intent and increases implicit coercion in conditionals.

`^` was rejected as an exponentiation operator because many readers associate it with xor, not power. Exponentiation is also much less frequent than the core arithmetic operators, so keeping it as a standard-library function is clearer for the MVP.

## Canonical Examples

Arithmetic and precedence:

```zt
const total: int = price + tax * quantity
```

Boolean logic:

```zt
const is_valid: bool = age >= 18 and age <= 120
```

Indexing and call expressions:

```zt
const first_letter: text = name[0]
const display_name: text = text.trim(user.name)
```

Explicit exponentiation through the standard library:

```zt
const squared_radius: float = math.pow(radius, 2)
```

Explicit boolean condition:

```zt
if user_count > 0 and is_ready
    ...
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

symbolic boolean operators:

```zt
const is_valid: bool = age >= 18 && age <= 120
```

truthiness in conditions:

```zt
if user_count and is_ready
    ...
end
```

assignment in expression position:

```zt
if value = next_value
    ...
end
```

ternary expression:

```zt
const result: int = is_ready ? 1 : 0
```

using `^` for exponentiation:

```zt
const squared_radius: float = radius ^ 2
```

## Out of Scope

This decision does not yet define:

- bitwise operators
- xor syntax, if any
- null-coalescing or similar sugar
- range syntax
- pattern expressions
- detailed literal forms outside the expression grammar itself
