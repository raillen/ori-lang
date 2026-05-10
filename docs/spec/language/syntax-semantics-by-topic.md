# Zenith Syntax And Semantics By Topic

> Audience: maintainer, contributor, language designer, documentation author
> Status: topical consolidation
> Surface: final language syntax and semantics
> Source of truth: companion reference; `final-language-contract.md` prevails on status/current-gap distinctions
> Last updated: 2026-05-04

This document organizes Zenith syntax and semantics by topic.

It is intentionally practical: each topic states the canonical syntax, the meaning of that syntax, the rejected forms, and the current implementation notes when the final contract is ahead of the executable subset.

When this file conflicts with another document, precedence is:

1. `final-language-contract.md`
2. latest topic-specific closure artifact
3. `post-v1-remaining-language-work.md`
4. this topical document
5. older language/reference/history documents

---

## 1. Design Principles

### Syntax

Zenith source uses explicit words and visible block endings:

```zt
namespace app.main

import std.io as io

func main()
    io.print("hello")
end
```

### Semantics

Zenith is a reading-first systems language:

- explicit intent is preferred over clever compression;
- imports are explicit and qualified;
- mutation is explicit through `var`, mutable receivers, and mutable resources;
- absence is modeled with `optional<T>`, not `null`;
- recoverable failure is modeled with `result<T, E>`, not exceptions;
- fatal invariant failure uses `panic(message)`;
- composition uses `struct`, `enum`, `trait`, and `apply`, not class inheritance;
- managed values behave as values even when the runtime uses reference counting internally.

### Rejected Forms

The final surface rejects:

- `null`;
- `try` / `catch` / `throw`;
- `async` / `await` syntax;
- implicit return from named functions;
- language-level `move`, `borrow`, `ref`, lifetimes, and pointer ownership syntax;
- macros as part of the core language;
- wildcard imports and selective imports;
- broad function/method/operator overloading.

---

## 2. Source File Shape

### Syntax

Every `.zt` source file starts with one namespace declaration:

```zt
namespace app.users

import std.io as io
import std.text as text

public func main()
    io.print("users")
end
```

### Semantics

- `namespace` defines the module identity of the file.
- The namespace must match the file path suffix in formal projects.
- Multiple files may share the same namespace.
- Files in the same namespace share private namespace-level symbols.
- `import` imports namespaces, not individual symbols.
- Imported members remain qualified through the namespace name or alias.
- The source file is the unit of parsing; the namespace is the unit of language visibility.

### Rejected Forms

```zt
from std.io import print
import std.io.*
import *
```

Rejected because they hide symbol origin and increase reading cost.

---

## 3. Comments And Documentation

### Syntax

Line comment:

```zt
-- one line
```

Block comment:

```zt
---
multiple lines
---
```

### Semantics

- Comments are ignored by the compiler.
- Block comments do not nest in v1.
- Source comments are local implementation notes.
- Public API documentation belongs in ZDoc (`.zdoc`), not in source-code doc comments.

### Rejected Forms

```zt
// comment
/* comment */
# comment
/// doc comment
doc "text"
```

---

## 4. Names, Visibility, Imports, And Reexports

### Syntax

Private declaration by default:

```zt
func helper() -> int
    return 1
end
```

Public declaration:

```zt
public func load_user(id: int) -> result<User, LoadError>
    ...
end
```

Namespace import:

```zt
import std.fs as fs
```

Explicit facade reexport:

```zt
export fs.Path
export local.User
```

### Semantics

- Top-level declarations are private to their effective namespace by default.
- `public` exposes a top-level symbol outside its namespace.
- `public` is visibility only; it does not mean global lifetime.
- `public var` can be read through qualified imports but can only be written inside its owner namespace.
- Imports are namespace imports only.
- Reexports are explicit API surface and semver-visible.
- Shadowing and duplicate names in the same scope are errors.

### Rejected Forms

- `pub` as a canonical spelling;
- `private`, `protected`, `internal`, or `package` visibility markers;
- `public import`;
- wildcard import/export;
- selective `from ... import ...`.

---

## 5. Values, Bindings, And Mutability

### Syntax

Immutable binding:

```zt
const name: text = "Ada"
```

Mutable binding:

```zt
var score: int = 0
score = score + 1
```

Compound assignment:

```zt
score += 10
score -= 1
score *= 2
score /= 2
score %= 3
```

### Semantics

- `const` cannot be reassigned.
- `var` can be reassigned.
- Field and index mutation require a mutable receiver path.
- Assignment is a statement, not an expression.
- Compound assignment is also a statement.
- Compound assignment uses the same type, overflow, division, and modulo rules as the corresponding binary operation plus assignment.
- The assignment target of compound assignment is evaluated once.
- Local variables require explicit type annotations in the final surface.

### Rejected Forms

```zt
let x = 1
const x = 1
score++
score--
if score = 1
    ...
end
```

`++` and `--` are rejected for readability and side-effect ambiguity.

---

## 6. Primitive Types

### Syntax

Canonical primitive types:

```zt
bool
int
int8
int16
int32
int64
u8
u16
u32
u64
float
float32
float64
text
bytes
void
```

### Semantics

- `bool` is only `true` or `false`.
- `int` is the default signed integer.
- `float` is the default floating-point type.
- Width-specific numeric types are explicit opt-in types.
- `u8`, `u16`, `u32`, and `u64` are unsigned fixed-width integer types.
- `text` is valid UTF-8 text.
- `bytes` is binary data and is distinct from `text`.
- `void` means no value and is valid mainly in function returns and `result<void, E>`.

### Rejected Forms

- standalone `uint`;
- `char` as a separate primitive type;
- implicit conversion between `text` and `bytes`;
- implicit conversion between numeric types.

---

## 7. Numeric Semantics

### Syntax

Explicit conversions:

```zt
const small: int32 = int32(value)
const count: int = int(value)
const ratio: float = float(value)
```

Arithmetic:

```zt
const total: int = a + b
const diff: int = a - b
const product: int = a * b
const quotient: int = a / b
const rest: int = a % b
```

### Semantics

- Numeric conversions are explicit.
- Numeric literals may fit contextually when representable.
- Mixed numeric expressions require explicit conversion to a compatible type.
- Integer overflow and underflow are checked.
- Division by zero and modulo by zero are runtime numeric errors.
- Unsigned integer arithmetic does not silently wrap by default.
- Conditions do not use numeric truthiness.

### Rejected Forms

```zt
if 1
    ...
end

const n: int = 1.5
```

---

## 8. Text And Interpolation

### Syntax

Text literal:

```zt
const title: text = "Zenith"
```

Multiline text:

```zt
const page: text = """
hello
zenith
"""
```

Interpolation:

```zt
const message: text = f"hello {user.name}"
```

### Semantics

- Text is UTF-8.
- Ordinary text literals do not interpolate.
- `f"..."` enables interpolation.
- Each interpolated expression must be representable as text through `TextRepresentable`.
- `{{` writes a literal `{` inside an interpolated string.
- `len(value)` on `text` returns text length according to the language/runtime contract.

### Rejected Forms

```zt
fmt"hello {name}"
f"{}"
f"{unterminated"
f"{value:format}"
```

`fmt"..."` is removed from the final language.

---

## 9. Bytes

### Syntax

```zt
const magic: bytes = hex bytes "DE AD BE EF"
const first: u8 = magic[0]
const header: bytes = magic[..2]
```

### Semantics

- `bytes` stores binary data.
- Hex byte literals use `hex bytes "..."`.
- Indexing bytes yields `u8`.
- Slicing bytes yields `bytes`.
- `text` and `bytes` never convert implicitly.

---

## 10. Collections And Literals

### Syntax

List:

```zt
const values: list<int> = [1, 2, 3]
```

Map:

```zt
const ages: map<text, int> = {"Ada": 36, "Bo": 41}
```

Set:

```zt
const tags: set<text> = {"admin", "editor"}
```

Indexing and mutation:

```zt
const first: int = values[0]
var mutable_values: list<int> = [1, 2]
mutable_values[0] = 9
```

Slice:

```zt
const middle: list<int> = values[1..3]
const tail: list<int> = values[1..]
const head: list<int> = values[..2]
```

### Semantics

- `list<T>`, `map<K, V>`, and `set<T>` are final generic collections.
- List literals use `[]`.
- Map literals use `{ key: value }`.
- Set literals use `{ value, ... }`.
- Empty `{}` requires an expected type.
- If expected type is `map<K, V>`, brace entries with `:` are map entries.
- If expected type is a struct, brace entries with field names are struct fields.
- List/text/bytes indexing is zero-based.
- Out-of-bounds list/text/bytes indexing is a fatal runtime error/panic.
- Missing direct map lookup is a fatal runtime error/panic.
- Recoverable lookup uses helpers returning `optional<T>`, such as `map.get` or `list.get` where available.
- Map keys require hash/equality support.
- Set elements require hash/equality support.
- The current C backend materializes `int`, `text`, safe structural set keys,
  and safe structural map keys with `bool`/integral/`text` fields.
- Map and set iteration order is not guaranteed in v1.
- Mutation requires a mutable binding or mutable receiver.

### Rejected Forms

- relying on map/set iteration order;
- using missing-key direct lookup for expected absence;
- mutating through `const` collections;
- using `{}` without expected type.

---

## 11. Tuples

### Syntax

Tuple type:

```zt
tuple<int, text>
```

Tuple value:

```zt
const pair: tuple<int, text> = (1, "one")
```

Destructuring:

```zt
const (id, label) = pair
```

### Semantics

- `tuple<T1, T2, ...>` is the only canonical tuple type spelling.
- Tuples are positional product values.
- Tuple destructuring binds by position.
- Tuple destructuring arity must match tuple arity.
- Tuple bindings created by `const (...)` are immutable.
- Use a `struct` when fields need stable names.

### Rejected Forms

```zt
group<int, text>
```

`group` has been removed entirely.

---

## 12. Structs

### Syntax

Declaration:

```zt
struct User
    name: text
    age: int where it >= 0
end
```

Explicit construction:

```zt
const user: User = User(name: "Ada", age: 36)
```

Expected-type struct literal:

```zt
const user: User = {name: "Ada", age: 36}
```

Mutation:

```zt
var user: User = User(name: "Ada", age: 36)
user.age = 37
```

### Semantics

- Structs are nominal product types.
- Fields are explicit and typed.
- Construction is named-field based.
- Field defaults are allowed where supported by the current implementation.
- Field contracts use `where` and are checked as runtime contracts.
- Structs have value semantics.
- Field mutation requires a mutable receiver.
- There is no inheritance or per-field visibility in v1.
- Behavior is attached through `apply` blocks and traits.

### Rejected Forms

```zt
new User("Ada", 36)
User("Ada", 36)
{name, age}
```

Positional struct construction and bare field-name shorthand are not final syntax.

---

## 13. Enums

### Syntax

Declaration:

```zt
enum LoadState
    Loading
    Ready(data: text)
    Failed(message: text)
end
```

Construction:

```zt
const state: LoadState = LoadState.Ready(data: "ok")
```

### Semantics

- Enums are closed tagged unions.
- Variants are qualified as `Type.Variant` in expressions.
- Variants may carry typed payloads.
- Named payload construction is canonical.
- Enums are the preferred way to model typed error categories in `result<T, E>`.
- Exhaustive `match` over enum values is preferred over catch-all fallback when the enum is known.

---

## 14. Pattern Matching

### Syntax

Basic match:

```zt
match state
case LoadState.Loading:
    io.print("loading")
case LoadState.Ready(data):
    io.print(data)
case LoadState.Failed(message):
    io.print(message)
end
```

Guard:

```zt
match score
case value if value >= 90:
    io.print("excellent")
case value if value >= 60:
    io.print("ok")
case else:
    io.print("retry")
end
```

Optional matching:

```zt
match maybe_user
case some(user):
    io.print(user.name)
case none:
    io.print("missing")
end
```

Tuple matching:

```zt
match (status, code)
case (200, 0):
    io.print("ok")
case else:
    io.print("other")
end
```

### Semantics

- `match value ... end` evaluates cases in order.
- `case pattern:` matches a pattern.
- `case pattern if condition:` adds a boolean guard.
- Guard bindings are scoped to the guard and body of that case.
- `case else:` is the only fallback spelling.
- Every `match` must be exhaustive through full coverage or `case else:`.
- Guarded cases do not count as exhaustive coverage.
- Supported final patterns include literals, bindings, enum variants, tuples, optional/result shapes, and simple struct patterns.
- Unreachable cases should produce diagnostics.

### Rejected Forms

- Historical `case default:` is removed; use `case else:`.
- Historical `case pattern given condition:` is removed; use `case pattern if condition:`.
- OR patterns, range patterns, and rest/spread patterns are not part of the final surface.

---

## 15. Optional And Result

### Syntax

Optional:

```zt
func find_name(id: int) -> optional<text>
    if id < 0
        return none
    end

    return some("Ada")
end
```

Result:

```zt
enum LoadError
    NotFound(id: int)
    Invalid(message: text)
end

func load_user(id: int) -> result<User, LoadError>
    if id < 0
        return error(LoadError.Invalid(message: "negative id"))
    end

    return success(User(name: "Ada", age: 36))
end
```

Propagation:

```zt
func start(id: int) -> result<void, LoadError>
    const user: User = load_user(id)?
    io.print(user.name)
    return success()
end
```

### Semantics

- `optional<T>` represents expected absence.
- `none` is absence.
- `some(value)` is presence.
- `result<T, E>` represents recoverable failure.
- `success(value)` is success.
- `success()` is success for `result<void, E>`.
- `error(value)` is recoverable failure.
- `E` should be a typed enum or struct when text is too vague.
- Postfix `?` propagates absence or error only from a function returning a compatible `optional` or `result`.
- `?` is not safe navigation.
- There is no implicit conversion between `optional<T>` and `result<T, E>`.
- There is no automatic error conversion.

### Rejected Forms

```zt
try load_user(id)
throw LoadError.NotFound(id: id)
user?.name
result<User>
load_user(id)!
```

---

## 16. Fatal Errors, Contracts, And Panic

### Syntax

```zt
panic("unreachable state")
```

Value contract:

```zt
struct Account
    balance: int where it >= 0
end
```

### Semantics

- `panic(message)` signals fatal invariant failure, impossible state, runtime boundary failure, or contract violation.
- Panic is not ordinary business control flow.
- Recoverable failure should use `result<T, E>`.
- Expected absence should use `optional<T>`.
- `where` on fields and parameters expresses runtime contracts.
- A failed runtime contract is fatal.
- Testing checks belong in `std.test`; global assertion helpers are not the final teaching surface.

---

## 17. Functions

### Syntax

Function returning a value:

```zt
func add(a: int, b: int) -> int
    return a + b
end
```

Function returning no value:

```zt
func log(message: text)
    io.print(message)
end
```

Explicit void:

```zt
func close() -> void
    return
end
```

Default parameter:

```zt
func connect(host: text, port: int = 80) -> result<void, NetError>
    ...
end
```

Named argument:

```zt
connect("localhost", port: 8080)?
```

### Semantics

- `func` declares a function.
- Function blocks close with `end`.
- Return types use `->`.
- Omitting `-> Type` means no value is returned.
- `return expr` must match the declared return type.
- `return` returns from a void function.
- Void functions may omit the final `return`.
- Parameters use `name: Type`.
- Required parameters precede defaulted parameters.
- Default arguments are evaluated at call time.
- Public parameter names are API surface.
- After a named argument is used in a call, remaining provided arguments must be named.

### Rejected Forms

- implicit return from named functions;
- variadic parameters;
- overloading multiple functions by the same name and different parameter types.

---

## 18. Callable Types And Closures

### Syntax

Callable type:

```zt
func(int) -> int
func(int, text) -> result<void, Error>
```

Function value:

```zt
func double(value: int) -> int
    return value * 2
end

const op: func(int) -> int = double
```

Block closure:

```zt
const add_one: func(int) -> int = func(value: int) -> int
    return value + 1
end
```

Single-expression closure:

```zt
const add_two: func(int) -> int = func(value: int) value + 2
```

Persistent closure-local state:

```zt
func counter() -> func() -> int
    return func() -> int
        capture count: int = 0
        count = count + 1
        return count
    end
end
```

### Semantics

- Callable types use `func(ParamTypes...) -> ReturnType`.
- Callable type parameters have types only, not names or defaults.
- Callable matching is structural.
- Function values can be stored in locals, passed as arguments, and returned where allowed.
- Closures capture local immutable values by default.
- Captured ordinary outer variables cannot be mutated through the closure.
- `capture name: Type = init` creates closure-local persistent mutable state.
- Each closure instance owns its own captured state.
- Local named functions behave like local closures.
- Stored closures are managed runtime values.
- FFI callback positions accept only supported non-capturing callable shapes.

### Rejected Forms

- symbolic lambdas such as `x => x + 1`;
- mutating an outer local through implicit capture;
- captured closures crossing `extern c` callback boundaries;
- callable types as public mutable namespace state, struct fields, or collection element types where the callable escape rules reject them.

---

## 19. Generics And Constraints

### Syntax

Generic function:

```zt
func first<Item>(items: list<Item>) -> optional<Item>
    if len(items) == 0
        return none
    end

    return some(items[0])
end
```

Generic struct:

```zt
struct Box<Item>
    value: Item
end
```

Trait constraint:

```zt
func render<Item>(value: Item) -> text
where Item is TextRepresentable
    return value.to_text()
end
```

### Semantics

- Generic functions, structs, enums, and traits are final surface features.
- Generic code is implemented through monomorphization.
- Constraints use `where TypeParam is Trait` or `where TypeParam is Trait<TypeParam>` style clauses.
- Generic argument inference is accepted from argument positions.
- Return-context-only inference is rejected.
- Full local type inference remains rejected.
- Generic constraints are checked statically.
- Monomorphization limits are project/build configuration, not source syntax magic.

### Rejected Forms

```zt
func render<Item: TextRepresentable>(value: Item) -> text
    ...
end

func render<Item>(value: Item) -> text
given Item is TextRepresentable
    ...
end
```

`given` is removed from the final constraints surface.

---

## 20. Traits, Apply, Methods, And Operator Traits

### Syntax

Trait:

```zt
trait Drawable
    func draw() -> text
end
```

Inherent methods:

```zt
apply User
    func display_name() -> text
        return self.name
    end
end
```

Trait implementation:

```zt
apply Drawable to User
    func draw() -> text
        return self.name
    end
end
```

Mutating method:

```zt
trait Damageable
    mut func damage(amount: int)
end

apply Damageable to Player
    mut func damage(amount: int)
        self.hp -= amount
    end
end
```

### Semantics

- `trait` defines shared behavior.
- Methods in traits may be `func` or `mut func`.
- `apply Type` defines inherent methods.
- `apply Trait to Type` implements a trait for a type.
- `self` is implicit in method signatures and available in method bodies.
- `mut func` requires a mutable receiver.
- Trait default methods are allowed.
- Overlapping trait implementations are rejected.
- Trait inheritance, associated types, specialization, and blanket overlap are not final v1 features.

### Operator Traits

Restricted operator behavior is final only for:

- `Addable` for `+`;
- `Subtractable` for `-`;
- `Comparable` for `<`, `<=`, `>`, `>=`.

No other user-defined operator overloading is final.

### Rejected Forms

- classical class inheritance;
- arbitrary method overloading;
- arbitrary custom operators;
- operator overloading for multiplication, division, modulo, or bitwise operators without a new explicit decision.

---

## 21. Dynamic Dispatch With `any<Trait>`

### Syntax

```zt
const shape: any<Drawable> = Circle(radius: 10)
const shapes: list<any<Drawable>> = [shape]
```

### Semantics

- `any<Trait>` is the canonical dynamic dispatch type.
- A concrete value can be boxed into `any<Trait>` when the expected type is known and the concrete type implements the trait.
- Method calls on `any<Trait>` are limited to the trait methods.
- `any<Trait>` is for heterogeneous values.
- Generic/static dispatch is preferred for homogeneous fast paths.
- Traits used with `any<Trait>` must be object-safe.
- `any<Trait>` is not reflection, dynamic field access, or a universal object type.
- `any<Trait>` does not cross `extern c` boundaries.

### Rejected Forms

- `any Drawable` is removed; use `any<Drawable>`.
- Historical `dyn<Drawable>` is removed; use `any<Drawable>`.
- `any<any<Drawable>>` is rejected.
- `any<Readable and Drawable>` is rejected.
- Downcast APIs such as `value.as<Player>()` are rejected.

---

## 22. Expressions And Operators

### Syntax

Common expression forms:

```zt
value
function(arg)
value.field
value[index]
(value + other)
-value
not ready
a and b
a or b
value |> transform
value |> transform(extra)
if ready then "yes" else "no"
```

### Semantics

- Function arguments are evaluated before the call.
- Field access reads a field or method target.
- Index access checks bounds/key existence at runtime.
- Parentheses group expressions.
- Boolean operators require `bool` operands.
- Conditions require `bool`.
- `and` and `or` are logical operators, not bitwise operators.
- `not` is boolean negation.
- Pipe `value |> f` is call sugar for `f(value)`.
- Pipe `value |> f(extra)` is call sugar for `f(value, extra)`.
- Inline `if cond then a else b` is an expression and requires `else`.
- Both branches of an if-expression must have compatible types.

### Operator Precedence

From strongest to weakest:

1. field access, call, indexing;
2. unary operators;
3. multiplicative operators;
4. additive operators;
5. comparison operators;
6. `and`;
7. `or`;
8. `|>`.

### Rejected Forms

```zt
cond ? a : b
!ready
a && b
a || b
```

---

## 23. Bitwise Operators

### Syntax

```zt
const flags: u32 = a | b
const masked: u32 = flags & mask
const toggled: u32 = flags ^ bit
const inverted: u32 = ~flags
const shifted: u32 = flags << 2
const reduced: u32 = flags >> 1
```

### Semantics

- Bitwise operators are advanced/low-level integer operators.
- They are distinct from boolean logical operators.
- They are intended primarily for explicit-width integer types.
- Shift counts must not be negative or out of width.
- Bitwise operators are not general overloadable user operators.

---

## 24. Control Flow

### Syntax

If statement:

```zt
if ready
    start()
else if waiting
    queue()
else
    stop()
end
```

While loop:

```zt
while running
    tick()
end
```

Infinite loop:

```zt
while true
    tick()
end
```

For loop:

```zt
for item in items
    io.print(item)
end
```

For with second binding:

```zt
for item, index in items
    io.print(item)
end

for key, value in scores
    io.print(key)
end
```

Repeat loop:

```zt
repeat 5 times
    tick()
end
```

Loop control:

```zt
break
continue
```

Range:

```zt
for value in range(0, 10)
    io.print(value)
end

for value in range(10, 0, -2)
    io.print(value)
end
```

### Semantics

- `if` statement conditions must be `bool`.
- `else if` is the canonical chained form.
- Loops are statements, not expressions.
- `while` repeats while the condition is true.
- `for item in collection` iterates over `list<T>`, `map<K, V>`, `set<T>`, and `text` where supported.
- For list/set/text, the second binding is an `int` index.
- For map, the first binding is key and the second binding is value.
- `repeat N times` evaluates `N` once before the loop.
- Repeat count `0` runs zero iterations.
- Negative repeat count is a runtime error.
- `break` exits the nearest enclosing loop.
- `continue` skips to the next iteration of the nearest enclosing loop.
- `break` and `continue` are valid only inside loops.
- `break` and `continue` trigger `using` cleanup before the jump.
- `range(start, end)` uses default step `1`.
- `range(start, end, step)` supports explicit positive or negative step.

### Rejected Forms

```zt
unless condition
    ...
end

elif condition
    ...
end

loop
    ...
end

for i = 0; i < 10; i += 1
    ...
end

repeat
    ...
until done
```

---

## 25. Resource Cleanup With `using`

### Syntax

Immutable resource:

```zt
using file = open_file(path)?
read_file(file)?
```

Mutable resource:

```zt
using var buffer = open_buffer()?
buffer.write("data")?
```

Block form:

```zt
using conn = connect(url)?
    send(conn, request)?
end
```

Explicit cleanup:

```zt
using handle = open_handle() then close_handle(handle)
use(handle)
```

Automatic `Disposable` cleanup:

```zt
using resource = acquire_resource()
use(resource)
```

### Semantics

- `using name = expr` binds a resource for deterministic cleanup.
- `using var name = expr` binds a mutable resource.
- Explicit cleanup uses `then cleanup_expr`.
- Block-form `using` scopes the resource to the block.
- Flat-form `using` scopes the resource to the current block.
- If the resource type implements `Disposable`, block/flat cleanup may call `dispose()` automatically.
- If `dispose()` is mutating, the resource must be bound with `using var`.
- Cleanups run in reverse creation order.
- Cleanups run on normal scope exit, `return`, `?` propagation, `break`, and `continue`.
- Panic cleanup is performed where viable by the runtime/backend strategy.
- Resource cleanup is separate from memory ownership.

---

## 26. Memory, Ownership, And Value Semantics

### Syntax

There is no ownership syntax in ordinary Zenith:

```zt
const a: list<int> = [1, 2, 3]
var b: list<int> = a
b[0] = 9
```

### Semantics

- Zenith exposes value semantics to users.
- Assignment and parameter passing behave as semantic value copies.
- Mutating `b` must not mutate `a` in the example above.
- The implementation may use ARC/ORC, retain/release, copy-on-write, deep copy, or internal last-use moves.
- These implementation strategies are not source-level language features.
- Managed values include `text`, `bytes`, collections, closures, lazy values, optionals/results containing managed payloads, and structs/enums with managed fields.
- RC cycles are leak risks, not undefined behavior.
- Ordinary safe Zenith has no explicit pointer/reference ownership model.

### Rejected Forms

```zt
move value
borrow value
&value
&mut value
retain(value)
release(value)
```

Advanced memory APIs, when present, live in explicit library modules and do not define the core language syntax.

---

## 27. Concurrency

### Syntax

Typed jobs:

```zt
import std.jobs as jobs

const job: jobs.Job<int> = jobs.spawn(compute_score)
const score: int = jobs.join(job)
```

Typed channels:

```zt
import std.channels as channels

const channel: channels.Channel<int> = channels.create<int>()
channels.send(channel, 42)
const value: optional<int> = channels.receive(channel)
channels.close(channel)
```

### Semantics

- Final user-facing concurrency uses typed handles such as `Job<T>` and `Channel<T>`.
- Values crossing concurrency boundaries must satisfy `Transferable`.
- The default transfer model is deep copy.
- Ordinary managed values are not implicitly shared across threads.
- Jobs and channels are explicit library/runtime surfaces, not `async`/`await` syntax.
- `std.shared` and `std.atomic` are advanced/low-level APIs, not ordinary core teaching surface.
- Current C backend execution may use an `int`-backed runtime ABI for typed facades.
- `_int` APIs are backend/runtime anchors and must not be taught as the final public surface.

### Rejected Forms

- hidden scheduler semantics;
- implicit cross-thread sharing;
- raw thread handles as ordinary language syntax;
- `async fn`, `await`, or colored-function syntax.

---

## 28. FFI And `extern c`

### Syntax

Basic C extern:

```zt
extern c
    func puts(message: text) -> int
end
```

Symbol rename:

```zt
extern c
    attr name("zt_ffi_apply_i64")
    func apply_i64(value: int, callback: func(int) -> int) -> int
end
```

ABI annotation:

```zt
extern c
    attr abi("stdcall")
    func add_stdcall(a: int, b: int) -> int
end
```

### Semantics

- `extern c` declares native C interop.
- FFI is explicit and boundary-safe.
- `attr name("symbol")` maps a Zenith function name to a native symbol.
- `attr abi("cdecl")` and `attr abi("stdcall")` select supported calling conventions.
- Callable callback parameters are allowed only for supported non-capturing, boundary-safe shapes.
- Capturing closures cannot be passed as raw C callbacks.
- Managed values crossing FFI must follow supported ABI shapes and runtime shielding rules.
- Native resources should be wrapped in safe Zenith types implementing `Disposable` and used with `using`.

### Rejected Forms

- raw pointer syntax in safe Zenith;
- arbitrary `void*` in the public language surface;
- C varargs as ordinary safe calls;
- extern mutable globals as normal safe variables;
- unannotated user structs crossing C layout boundaries.

---

## 29. Attributes

### Syntax

Test:

```zt
attr test
func parses_user()
    test.is_true(true)
end
```

Skip:

```zt
attr test
attr skip("waiting for fixture")
func slow_case()
end
```

Deprecation:

```zt
attr deprecated("use new_name")
func old_name()
end
```

FFI:

```zt
extern c
    attr name("native_symbol")
    attr abi("cdecl")
    func call_native(value: int) -> int
end
```

### Semantics

- Attributes apply to the following declaration.
- One `attr` appears per line.
- Attributes are metadata, not macros.
- Final v1 closed set: `test`, `skip`, `deprecated`, `todo`, `name`, `abi`.
- `test`, `skip`, `deprecated`, and `todo` apply only to `func`.
- `name` and `abi` apply only to `extern` functions.
- `attr skip` requires `attr test`.
- `attr deprecated("message")` and `attr todo("message")` require string arguments.
- Unknown attributes are errors.

### Rejected Forms

- custom user attributes;
- attributes on structs/enums/traits in v1;
- macro-like attributes that transform code.

---

## 30. Tests

### Syntax

```zt
namespace app.tests

import std.test as test

attr test
func adds_numbers()
    test.equal_int(4, 2 + 2)
end
```

### Semantics

- Tests are ordinary functions marked with `attr test`.
- Test functions take no parameters.
- Test functions have no type parameters.
- Test functions return no value.
- Test helper assertions live in `std.test`.
- `zt test` discovers and runs `attr test` functions.
- `attr skip` marks a test as skipped.

---

## 31. Prelude And Builtins

### Syntax

Always-available core concepts include:

```zt
bool
int
text
optional<T>
result<T, E>
none
some(value)
success(value)
error(value)
len(value)
panic("message")
```

### Semantics

The final implicit prelude is intentionally small. It contains:

- primitive/core types;
- numeric types;
- `optional<T>` and `result<T, E>`;
- `none`, `some`, `success`, and `error`;
- core traits such as `Equatable`, `Hashable`, `Comparable`, `TextRepresentable`, `Disposable`, and `Transferable`;
- `Order`;
- `len(...)`;
- `panic(...)`.

Ordinary modules are imported explicitly:

```zt
import std.io as io
import std.fs as fs
import std.test as test
```

### Rejected Forms

- broad auto-import of the whole standard library;
- package custom preludes;
- relying on global testing assertions instead of `std.test` in final teaching material.

---

## 32. Standard Library Boundary

### Syntax

Canonical import style:

```zt
import std.fs as fs
import std.fs.path as path
import std.time as time
```

### Semantics

- Stdlib modules are explicit and namespace-qualified.
- Foundation modules belong in stdlib.
- Higher-level frameworks and domain-specific libraries belong in packages.
- Expected absence returns `optional<T>`.
- Expected failure returns `result<T, Module.Error>`.
- Panic is reserved for broken invariants and direct invalid operations.
- `std.shared`, `std.atomic`, `std.debug`, manual allocation, and manual memory management are advanced/low-level surfaces.
- Full module, function, helper, constant, and observable state details live in `stdlib-reference-by-topic.md`.

Canonical ordinary modules include:

- `std.text`;
- `std.bytes`;
- `std.math`;
- `std.validate`;
- `std.format`;
- `std.time`;
- `std.io`;
- `std.fs`;
- `std.fs.path`;
- `std.json`;
- `std.os`;
- `std.os.process`;
- `std.test`;
- `std.jobs`;
- `std.channels`.

---

## 33. Projects And Packages

### Syntax

Formal projects use `zenith.ztproj`.

Application project shape:

```toml
[project]
name = "demo"
kind = "app"

[source]
root = "src"

[app]
entry = "app.main.main"

[build]
target = "native"
```

Library project shape:

```toml
[project]
name = "mylib"
kind = "lib"

[source]
root = "src"

[lib]
root_namespace = "mylib"
```

### Semantics

- `zenith.ztproj` is the official project manifest file.
- The syntax is TOML-like but the official file extension is not `.toml`.
- Project kinds are `app` and `lib`.
- A published package is a `lib`, not a separate `project.kind`.
- Apps require `[app].entry`.
- Libs require `[lib].root_namespace`.
- Unknown sections and keys are errors.
- Dependencies use `[dependencies]` and `[dev_dependencies]`.
- Dependency resolution writes `zenith.lock`.
- `zpm` is the package manager for install/add/publish workflows.

---

## 34. Tooling

### Syntax

Common commands:

```text
zt check
zt build
zt run
zt test
zt fmt
zt fmt --check
zt doc
zpm install --locked
```

### Semantics

- `zt` is the main compiler/tooling CLI.
- `check` validates syntax and semantics.
- `build` produces a native artifact for the configured target.
- `run` builds/runs an application or source file.
- `test` discovers and runs `attr test` functions.
- `fmt` applies canonical formatting.
- `fmt --check` verifies formatting without changing files.
- `doc` works with public symbols and ZDoc.
- `emit-c` and verifier commands are advanced/internal surfaces.
- Tooling is LSP-first and external; compiler plugins are not part of the final core model.

---

## 35. Formatting

### Syntax

Canonical formatting uses:

```zt
func add(a: int, b: int) -> int
    return a + b
end
```

### Semantics

- Indentation is 4 spaces.
- Tabs are rejected.
- Target line width is 100 columns.
- `end` aligns with the opening construct.
- One blank line separates top-level declarations.
- Long signatures, calls, and literals use multiline form with one item per line.
- `case` aligns with `match`.
- One `attr` per line.
- No vertical alignment.
- No per-project style configuration in v1.

Naming conventions:

| Element | Convention | Example |
|---|---|---|
| Types | `PascalCase` | `User` |
| Enum cases | `PascalCase` | `NotFound` |
| Functions | `snake_case` | `load_user` |
| Variables, parameters, fields | `snake_case` | `user_id` |
| Namespaces | `snake_case` | `app.users` |
| Generic parameters | descriptive `PascalCase` | `Item`, `Key`, `Value` |

Naming conventions are guidance in v1, not formatter-enforced semantic rules.

---

## 36. Diagnostics

### Syntax

There is no source syntax for diagnostics, but diagnostic output should follow stable structured forms.

Terminal-style detailed diagnostics use:

```text
error[code]: message
 --> path/to/file.zt:line:col
  |
  | source snippet
  | ^ label
  = why: explanation
  = action: concrete fix
  = next: next step
  = help: optional deeper help
```

### Semantics

A good diagnostic carries:

- severity;
- stable code;
- compiler stage;
- primary message;
- file/span;
- related spans when useful;
- why;
- action;
- next;
- help;
- metadata for tools.

IDE problem lists should remain compact and reveal details progressively.

---

## 37. Explicit Non-Goals And Removed Syntax

The final surface rejects or removes the following forms:

| Removed/rejected form | Canonical direction |
|---|---|
| `group<T>` | `tuple<T>` |
| `fmt"..."` | `f"..."` |
| `case default:` | `case else:` |
| `case pattern given condition:` | `case pattern if condition:` |
| `dyn<Trait>` | `any<Trait>` |
| `any Trait` | `any<Trait>` |
| `<T: Trait>` constraints | `where T is Trait` style constraints |
| `try/catch/throw` | `result<T, E>`, `error(...)`, `panic(...)` |
| `?.` | explicit `match` or helper functions |
| `??` | explicit `match` or optional/result helpers |
| `async/await` | jobs/channels library APIs |
| `owned<T>` / `borrow<T>` / lifetimes | managed value semantics and library-level advanced APIs |
| C-style `for` | `for item in collection` or `range(...)` |
| `unless` | `if not condition` |
| `elif` | `else if` |
| `loop` | `while true` |
| wildcard imports | explicit namespace imports |
| selective imports | namespace imports plus explicit qualification |
| `++` / `--` | `+= 1` / `-= 1` |
| `cond ? a : b` | `if cond then a else b` |
| macros | explicit functions, types, traits, packages, tooling |
| `char` | `text` and text helpers |
| standalone `uint` | `u8` / `u16` / `u32` / `u64` |
| variadic Zenith params | explicit `list<T>` argument |

---

## 38. Current Executable Subset Notes

Some final decisions are broader than the current executable C backend/runtime subset.

Important current-subset notes:

- The C backend remains the executable oracle.
- Generic monomorphization exists and is expanding under explicit limits.
- List higher-order helpers are implemented for important same-type primitive/text subsets, including `list.reduce<T,T>`.
- `Job<T>` and `Channel<T>` are the final public direction, while current runtime storage may still use `int` ABI anchors.
- `Atomic<int>` is the stable current atomic payload; broader payloads are future implementation.
- `std.shared` and `std.atomic` are advanced APIs.
- FFI callbacks are narrow and non-capturing.
- Captured closures across `extern c` remain rejected.
- Runtime ownership uses ARC/ORC implementation strategies behind value semantics.
- Public docs should teach final spelling and mark executable gaps explicitly instead of teaching backend-only names.

---

## 39. Compact Example

```zt
namespace app.main

import std.io as io
import std.test as test

enum LoadError
    NotFound(id: int)
    Invalid(message: text)
end

struct User
    name: text
    age: int where it >= 0
end

trait Displayable
    func display() -> text
end

apply Displayable to User
    func display() -> text
        return f"{self.name} ({self.age})"
    end
end

func load_user(id: int) -> result<User, LoadError>
    if id < 0
        return error(LoadError.Invalid(message: "negative id"))
    end

    if id == 0
        return error(LoadError.NotFound(id: id))
    end

    return success(User(name: "Ada", age: 36))
end

func main() -> result<void, LoadError>
    const user: User = load_user(1)?
    io.print(user.display())
    return success()
end

attr test
func load_user_rejects_negative_id()
    const result: result<User, LoadError> = load_user(-1)
    test.is_true(result.is_error())
end
```

This example demonstrates:

- namespace and imports;
- enum error payloads;
- struct field contracts;
- trait and apply;
- `result<T, E>`;
- `?` propagation;
- text interpolation;
- `attr test` and `std.test`.
