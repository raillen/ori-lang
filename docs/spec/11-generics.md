# Ori Language Specification — Chapter 11: Generics and Constraints

> Status: normative
> Audience: compiler implementers

---

## Overview

Generics allow functions, structs, enums, and traits to be parameterized over
types. The compiler produces a specialized concrete implementation for each
distinct type argument (monomorphization).

---

## Generic Functions

```ori
func identity<T>(value: T) -> T
    return value
end

func first<T>(items: list<T>) -> optional<T>
    if len(items) == 0
        return none
    end
    return some(items[0])
end
```

Type parameters are declared in `<T>` after the function name.
Multiple parameters: `<T, U>`, `<Key, Value>`.

---

## Generic Structs

```ori
struct Pair<A, B>
    first: A
    second: B
end

const p: Pair<int, string> = Pair(first: 1, second: "one")
```

---

## Generic Enums

```ori
enum Either<Left, Right>
    Left(value: Left)
    Right(value: Right)
end
```

---

## Generic Traits

```ori
trait Container<Item>
    mut func add(item: Item)
    func get(index: int) -> optional<Item>
    func length() -> int
end
```

---

## Type Constraints (`where`)

Type parameters may be constrained to require specific trait implementations:

```ori
func max<T>(a: T, b: T) -> T
    where T is Comparable
    if a.compare(b) == Order.Greater
        return a
    end
    return b
end
```

### Multiple Constraints

```ori
func sorted_keys<K, V>(m: map<K, V>) -> list<K>
    where (
        K is Hashable
        and K is Comparable
    )
    -- ...
end
```

### Inline `where` on Parameters

Value contracts on individual parameters use `where` inline:

```ori
func sqrt(value: float where value >= 0.0) -> float
```

This is a value contract (checked at runtime), not a type constraint.

### Negative Constraints

```ori
func raw_copy<T>(src: T, dst: T) where T is not Disposable
```

Prevents the function from being called with managed/resource types.

---

## Type Inference in Generic Calls

Ori infers type arguments at call sites when they can be determined from the
argument types:

```ori
-- Type argument T inferred as int from the argument 42:
const result: int = identity(42)

-- Type argument T inferred as string from the list contents:
const name: optional<string> = first(["Ada", "Bo"])
```

When inference is ambiguous or impossible, the type argument must be explicit:

```ori
const empty: optional<int> = first<int>([])
```

---

## Monomorphization

The compiler generates a concrete function/type for each unique combination
of type arguments used in the program.

```ori
identity(42)          -- generates identity_int
identity("hello")     -- generates identity_str
first([1, 2, 3])      -- generates first_list_int
```

This means:
- Generic code has zero runtime overhead compared to hand-written typed code.
- Large programs with many generic instantiations may have larger binaries.
- Circular generic instantiations are a compile error.

---

## Supported Generic Combinations

Not all combinations of types and generic functions are supported. The compiler
reports a clear error when a type argument fails to satisfy a `where` constraint:

```
error[generic.constraint_not_satisfied]: T does not satisfy constraint
  --> src/app/main.orl:12:5
   |
12 |    const keys: list<K> = sorted_keys(my_map)
   |                          ^^^^^^^^^^^^^^^^
   |
   = why: K = User, but User does not implement Comparable
   = action: add 'implement Comparable for User' with func compare(other: User) -> Order
```

---

## `Self` in Generic Contexts

`Self` inside a `trait` or `implement` block refers to the implementing type.
It may be used as a type argument:

```ori
trait Cloneable
    func clone() -> Self
end

implement Cloneable for Config
    func clone() -> Config
        return Config(
            timeout: self.timeout,
            retries: self.retries,
        )
    end
end
```

---

## Generic Type Aliases

```ori
alias StringMap<V> = map<string, V>
alias Callback<T>  = func(T) -> bool
```

---

## Limitations in v1

The following generic features are not supported in Ori v1:

- **Higher-kinded types** (type constructors as type parameters): `trait Functor<F<_>>` — not supported.
- **Associated types** in traits: `trait Iterator { type Item }` — not supported; use `Iterable<Item>` instead.
- **Const generics** (type parameters that are values): `struct Matrix<const N: int>` — not supported.
- **Variadic type parameters**: `tuple<T...>` — not supported; use `tuple<A, B, ...>` with fixed arity.

These may be added in future versions via explicit design decisions.
