# Ori Language Specification — Chapter 05: Expressions

> Status: normative
> Audience: compiler implementers

---

## Overview

Expressions produce values. Every expression has a type determined at compile time.

Ori distinguishes expressions (produce values) from statements (produce effects).
Most control flow constructs in Ori are statements. The exceptions are documented
in this chapter.

---

## Literals

All literal forms are expressions. Their types are:

| Literal | Type |
|---|---|
| `true`, `false` | `bool` |
| `42` | `int` |
| `42i8`, `42u8`, etc. | explicit integer type |
| `3.14` | `float` |
| `3.14f32` | `float32` |
| `"hello"` | `string` |
| `f"hello {name}"` | `string` |
| `"""..."""` | `string` |
| `b"data"` | `bytes` |
| `0..9` | `range<int>` |

---

## Arithmetic Expressions

```ori
a + b       -- addition (requires Addable)
a - b       -- subtraction (requires Subtractable)
a * b       -- multiplication
a / b       -- division
a % b       -- modulo
-a          -- negation
```

Integer division truncates toward zero. Division by zero is a runtime panic.
Float division by zero produces `Infinity` or `NaN` per IEEE 754.

Operator `+` and `-` require the operands to implement `Addable` and
`Subtractable` respectively when applied to user-defined types.
On primitives, these operators work directly.

---

## Comparison Expressions

```ori
a == b      -- equality
a != b      -- inequality
a < b       -- less than (requires Comparable)
a <= b      -- less than or equal
a > b       -- greater than
a >= b      -- greater than or equal
```

All comparison expressions produce `bool`.

**Comparison chaining is a compile error:**

```ori
-- Error: comparison chaining not allowed
a < b < c

-- Correct:
a < b and b < c
```

---

## Boolean Expressions

```ori
a and b     -- logical and (short-circuit)
a or b      -- logical or (short-circuit)
not a       -- logical not
```

`and` evaluates `b` only if `a` is `true`.
`or` evaluates `b` only if `a` is `false`.

---

## Field Access

```ori
user.name
config.timeout
point.x
```

Field access on a struct returns the field's type.
Field access on an enum variant's payload uses the variant's field names.

---

## Index and Slice

```ori
items[0]          -- index: returns element type T
items[2..5]       -- slice: returns list<T>, elements at 2, 3, 4, 5
items[2..]        -- slice from index 2 to end
items[..5]        -- slice from start to index 5
items[..]         -- full copy
text[0..3]        -- string slice: characters at 0, 1, 2, 3
```

Index bounds are checked at runtime. Out-of-bounds is a runtime panic.
Slice bounds are validated: `start` and `end` must be within `0..len-1`.

---

## Function Calls

```ori
add(1, 2)
io.print("hello")
user.display()
```

**Named arguments:**

```ori
connect(host: "localhost", port: 8080)
format.date(millis, style: "iso")
```

Once a named argument is used in a call, all subsequent arguments must also
be named.

**Spread into variadic:**

```ori
const parts: list<string> = ["a", "b", "c"]
concat(..parts)
```

---

## Error Propagation (`?`)

The `?` operator propagates errors and absence upward.

On `result<T, E>`:

```ori
const value: T = fallible_operation()?
-- If error(e): returns error(e) from the enclosing function
-- If success(v): unwraps to v
```

On `optional<T>`:

```ori
const value: T = maybe_value?
-- If none: returns none from the enclosing function
-- If some(v): unwraps to v
```

Rules:
- The enclosing function's return type must be compatible with the propagated type.
- `?` on `result<T, E>` requires the enclosing function to return `result<_, E>`.
- `?` on `optional<T>` requires the enclosing function to return `optional<_>`.
- `?` is a postfix operator with the highest postfix precedence (evaluated before `.`).

---

## Pipe Operator (`|>`)

The pipe operator passes the left-hand value as the first argument to the
right-hand function call.

```ori
const result: list<string> =
    users
    |> iter.filter(do(u: User) => u.active)
    |> iter.map(do(u: User) => u.name)
```

`a |> f(b)` is equivalent to `f(a, b)`.
`a |> f` is equivalent to `f(a)`.

---

## If Expression (Inline)

```ori
const label: string = if active then "on" else "off"
```

Rules:
- Both branches must produce the same type.
- `else` is mandatory in expression position.
- The condition must be `bool`.

---

## Closure Expressions (`do`)

See Chapter 07 — Functions and Closures for full specification.

```ori
do(x: int) => x * 2                     -- inline: produces func(int) -> int
do(x: int) -> int ... end               -- block closure
```

---

## Struct Literal

**Full form** — always valid, type is explicit:

```ori
const p: Point = Point(x: 0, y: 0)
const u: User  = User(name: "Ada", age: 36)
```

**Anonymous form** — `.{field: value}` when the type is known from context:

```ori
-- From type annotation
const p: Point = .{x: 0, y: 0}

-- From function return type
func origin() -> Point
    return .{x: 0, y: 0}
end

-- From parameter type
func draw(point: Point)
draw(.{x: 10, y: 20})

-- Nested
const l: Line = .{start: .{x: 0, y: 0}, end: .{x: 5, y: 5}}

-- With Default: all fields use their default values
const p: Point = .{}
```

The `.{` prefix is unambiguous — it cannot be confused with a map literal
(`{"key": value}`) or a block.

Rules:
- All fields must be provided unless the type implements `Default`.
- Field names are required (positional construction is not allowed).
- If the expected type cannot be inferred, `.{...}` is a compile error:
  `error[type.anon_struct_type_unknown]`.
- If a field name does not exist on the target struct:
  `error[type.anon_struct_field_mismatch]`.

---

## Struct Update Expression

Creates a new struct value with selected fields overridden:

```ori
const updated: Config = original with
    verbose: true
    timeout: 60
end
```

All fields not mentioned keep their original values.
The result is a new value; `original` is not mutated.

---

## Enum Variant Expression

**Full form:**

```ori
Direction.North
Shape.Circle(radius: 10.0)
Shape.Rectangle(width: 5.0, height: 3.0)
```

**Shorthand** (when the enum type is known from context):

```ori
const d: Direction = .North
const s: Shape = .Circle(radius: 5.0)
```

**Struct variant shorthand** (enum variant with named fields):

```ori
const s: Shape = .Rectangle{width: 5.0, height: 3.0}
```

The `.Variant{field: value}` form is the struct-variant equivalent of `.{field: value}`.
It follows the same type-inference rules.

---

## Collection Literals

```ori
-- list
const names: list<string> = ["Ana", "Bo", "Cara"]

-- map
const ages: map<string, int> = {"Ana": 31, "Bo": 25}

-- set
const ids: set<int> = set{1, 2, 3}

-- tuple
const pair: tuple<int, string> = tuple(1, "one")
```

Empty collections must have their type annotated:

```ori
const empty: list<int> = []
const empty_map: map<string, int> = {}
```

---

## Range Expression

```ori
0..9        -- range<int>: 0 to 9 inclusive
5..3        -- range<int>: 5, 4, 3 (descending)
0.0..1.0    -- range<float>
```

The type of a range literal is `range<T>` where `T` is the type of the endpoints.
Endpoints must have the same type.

---

## Type Checking Expression (`is`)

Tests whether a dynamic value has a specific type:

```ori
if shape is Circle
    -- shape is narrowed to Circle in this block
end
```

Valid only when the left operand is `any<Trait>` or an enum type.

---

## Expression Evaluation Order

Expressions are evaluated left to right, depth first.
Short-circuit operators (`and`, `or`) skip the right operand when the result
is determined.

Side effects in sub-expressions occur in the order they appear in source.
