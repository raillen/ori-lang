# Decision 082: Syntax Accessibility and Ergonomics (ADHD and Dyslexia)

- Status: accepted
- Date: 2026-04-18
- Type: language / accessibility / syntax
- Scope: readability, visual stability, mutation signaling, where readability

## Context

Zenith is a reading-first language. Accessibility for ADHD and dyslexia is a first-class requirement, not optional polish.

This decision consolidates syntax ergonomics to reduce visual crowding, lower cognitive friction, and keep code scanning predictable.

## Decision

### 1. Logical operators stay textual

Canonical logical operators are:

- `not`
- `and`
- `or`

Non-canonical symbolic forms such as `&&`, `||`, and symbolic unary negation are not part of the MVP surface.

### 2. Mutating methods use explicit prefix

Mutating methods are declared with the explicit prefix `mut func`.

Canonical:

```zt
mut func heal(amount: int)
    self.hp = self.hp + amount
end
```

This keeps mutation intention visible at declaration time and avoids punctuation-based markers attached to identifiers.

### 3. Block closure uses `end`

Zenith keeps textual block closure (`end`) and rejects brace-heavy closure as canonical source style.

This preserves vertical reading anchors and helps line-by-line scanning.

### 4. Value contracts allow `it`

For value-level contracts, both forms are accepted:

- `age: int where age > 0`
- `age: int where it > 0`

`it` is retained because it can reduce local repetition noise in short predicates.

## Consequences

- `surface-syntax.md` is the canonical source of truth for these rules.
- `formatter-model.md` and `tooling-model.md` must keep these forms stable in examples and formatter output.
- Legacy documents that still use older mutation markers are historical only and are superseded by the canonical spec.
