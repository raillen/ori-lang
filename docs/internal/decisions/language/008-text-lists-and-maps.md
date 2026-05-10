# Decision 008 - Text Lists And Maps

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: text literals, list literals, map literals, indexing, collection access

## Summary

Zenith Next uses double-quoted literals for `text`.

Lists use square-bracket literals and maps use brace literals.

Text indexing returns `text`, not a separate `char` type.

Map lookup through the indexing operator returns `optional<V>`.

Empty list and map literals are accepted only when type context is explicit.

Detailed rules for text escapes, multiline text, index base and slice syntax are defined by Decision 018.

## Decision

The canonical literal forms are:

- text literal: `"Zenith"`
- list literal: `[1, 2, 3]`
- map literal: `{"name": "Zenith"}`

Normative rules for this cut:

- `text` literals use double quotes
- single-quoted text literals are not part of the canonical syntax
- `list<T>` literals use square brackets
- `map<K, V>` literals use braces with `key: value` pairs
- indexing uses `value[index]`
- indexing `text` yields `text`
- the MVP does not define a separate `char` type
- indexing a `map<K, V>` yields `optional<V>`
- empty list and map literals require explicit type context
- richer operations on `text`, `list` and `map` belong to the standard library rather than dedicated collection-specific syntax

## Rationale

These literal forms are conventional and easy to recognize.

Returning `text` from text indexing keeps the type surface smaller and avoids introducing a separate character model before the language has decided its precise text semantics.

Returning `optional<V>` from map indexing makes key absence explicit in the type system and avoids hidden sentinel behavior.

Restricting empty literals to typed contexts keeps the parser and type checker simpler and avoids ambiguity around untyped `[]` and `{}`.

## Canonical Examples

Text literal:

```zt
const app_name: text = "Zenith"
```

List literal:

```zt
const ports: list<int> = [3000, 3001, 3002]
```

Map literal:

```zt
const metadata: map<text, text> = {
    "name": "Zenith",
    "version": "2"
}
```

Text indexing:

```zt
const first_letter: text = app_name[index]
```

Map lookup through indexing:

```zt
const version_name: optional<text> = metadata["version"]
```

Typed empty literals:

```zt
const empty_ports: list<int> = []
const empty_metadata: map<text, text> = {}
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

single-quoted text literal:

```zt
const app_name: text = 'Zenith'
```

assuming text indexing returns `char`:

```zt
const first_letter: char = app_name[index]
```

assuming map indexing returns the raw value directly:

```zt
const version_name: text = metadata["version"]
```

untyped empty list literal:

```zt
const ports = []
```

untyped empty map literal:

```zt
const metadata = {}
```

## Out of Scope

This decision does not yet define:

- duplicate-key policy in map literals
- mutation syntax for lists and maps
- deeper collection APIs beyond the literal and indexing surface
