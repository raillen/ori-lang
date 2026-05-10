# Decision 023 - Generic Constraints

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: generic type constraints, trait bounds, constraint placement, compile-time enforcement

## Summary

Zenith Next uses `where` for generic constraints as well as value-level contracts.

In generic declarations, `where` introduces compile-time type constraints rather than value predicates.

Generic constraints are written after the declaration header and before the declaration body.

Generic constraints are compile-time requirements and do not emit runtime contract checks by themselves.

## Decision

The canonical generic constraint forms are:

```zt
public func render<Item>(value: Item) -> text
where Item is TextRepresentable<Item>
    return to_text(value)
end
```

```zt
public struct Cache<Key, Value>
where Key is Hashable<Key> and Key is Equatable<Key>
    items: map<Key, Value>
end
```

```zt
public func read_with<Output, Reader>(reader: Reader, source: text) -> result<Output, text>
where Reader is Parser<Output>
    return reader.parse(source)
end
```

Normative rules for this cut:

- generic constraints use `where`
- generic constraints appear after the declaration header and before the body
- generic constraints may be used on generic `struct`, `trait`, `enum`, `func` and `apply` declarations
- generic constraints are compile-time type requirements, not runtime value predicates
- the canonical relation form is `TypeParameter is TraitType`
- parameterized traits may appear on the right-hand side, for example `Reader is Parser<Output>`
- multiple constraints are combined with `and`
- the same type parameter may appear in multiple constraint clauses joined by `and`
- only generic parameters introduced by the declaration may appear on the constrained side of a generic constraint in this cut
- generic constraints are checked by semantic analysis
- satisfied generic constraints do not imply runtime checks by themselves
- the `where` used for generic constraints and the `where` used for value contracts are distinguished by syntactic position and semantic context
- inline generic constraint syntax inside the angle-bracket parameter list is not part of the canonical form

## Canonical Forms

### Generic Function Constraint

```zt
public func render<Item>(value: Item) -> text
where Item is TextRepresentable<Item>
    return to_text(value)
end
```

### Multiple Constraints

```zt
public struct Cache<Key, Value>
where Key is Hashable<Key> and Key is Equatable<Key>
    items: map<Key, Value>
end
```

### Parameterized Trait Constraint

```zt
public trait Parser<Output>
    func parse(source: text) -> result<Output, text>
end

public func read_with<Output, Reader>(reader: Reader, source: text) -> result<Output, text>
where Reader is Parser<Output>
    return reader.parse(source)
end
```

### Constraint On Generic Apply

```zt
apply Box<Item>
where Item is TextRepresentable<Item>
    public func render() -> text
        return to_text(self.value)
    end
end
```

## Relationship To Value-Level `where`

Zenith Next deliberately keeps the same keyword for two declaration-level restriction forms:

- value-level contracts, such as `hp: int where it >= 0`
- generic type constraints, such as `where Item is TextRepresentable<Item>`

They are not the same semantic construct.

Value-level `where` attaches to a typed value surface and evaluates a boolean predicate over a bound value, usually through `it`.

Generic-constraint `where` attaches to a generic declaration header and expresses compile-time trait or capability requirements over type parameters.

The compiler distinguishes them by syntactic position and semantic context.

## Rationale

Using `where` for both forms keeps the language surface smaller and avoids introducing a second keyword for declaration-level restrictions.

Placing generic constraints after the header keeps the declaration name and generic parameter list easy to scan before the bounds appear.

The `TypeParameter is TraitType` form reads directly and stays consistent with the existing trait-oriented surface of the language.

Keeping generic constraints compile-time only avoids conflating them with runtime contracts and keeps lowering cleaner.

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

inline constraints inside the generic parameter list:

```zt
public func render<Item where Item is TextRepresentable<Item>>(value: Item) -> text
    ...
end
```

using a second keyword such as `requires`:

```zt
public func render<Item>(value: Item) -> text
requires Item is TextRepresentable<Item>
    ...
end
```

using `conforms to` as the canonical relation phrase:

```zt
public func render<Item>(value: Item) -> text
where Item conforms to TextRepresentable<Item>
    ...
end
```

using symbolic separators instead of `and`:

```zt
public struct Cache<Key, Value>
where Key is Hashable<Key> && Key is Equatable<Key>
    items: map<Key, Value>
end
```

## Out of Scope

This decision does not yet define:

- associated types
- equality constraints between types
- negative constraints
- specialization based on constraints
- constraint inference
- advanced trait-system features beyond ordinary `TypeParameter is Trait` bounds

