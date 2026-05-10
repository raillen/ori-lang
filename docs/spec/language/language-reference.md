# Zenith Language Reference

- Status: canonical unified spec
- Date: 2026-05-02
- Scope: `.zt` source syntax, type system, closures, dynamic dispatch, callables
- Supersedes: `surface-syntax.md`, `closures.md`, `dyn-dispatch.md`, `callables.md`
- Upstream: Decision 094 (syntax coherence refinements)
- Tier 7 note: `zenith-language-spec.md` is the primary current spec. This file
  remains the implementation-facing reference.
- Post-v1 closure note: syntax decisions are frozen in `post-v1-syntax-freeze.md`

---

## Philosophy

Zenith prioritizes reading comfort, predictability and explicitness.

The language is designed for low neural friction, visual stability and accessibility for TDAH/dyslexia-friendly workflows.

Rules:

- prefer clear words over dense symbolic syntax when that improves reading
- prefer one canonical form over many equivalent forms
- keep behavior explicit at the source level
- avoid hidden imports, hidden coercions and magical defaults
- keep generated C and runtime internals outside the user-facing language model

---

## Source File Shape

Every `.zt` file declares exactly one namespace.

Canonical shape:

```zt
namespace app.main

import std.io as io

func main()
    print("Hello, Zenith!")
end
```

Rules:

- `namespace` is the first real declaration
- only blank lines and comments may appear before `namespace`
- imports appear after `namespace`
- imports import namespaces, not individual symbols
- imported symbols remain qualified through the namespace or alias
- `import path.to.namespace as alias` is allowed
- `from ... import ...`, `import *` and relative imports are not syntax

---

## Comments

Line comments use `--`.

Block comments use `--- ... ---`.

```zt
-- This is a line comment

---
This is a block comment.
Multiple lines are allowed.
---
```

Comments are local implementation notes. Long public documentation belongs in ZDoc.

---

## Visibility And Scope

Top-level declarations are namespace-scoped.

Declarations are private by default.

`public` exposes a namespace-scope declaration outside its namespace.

Rules:

- `pub` is not canonical
- `global` is not used
- shadowing is rejected
- files with the same effective namespace share private namespace symbols
- duplicate names in the same effective namespace/scope are errors
- `public var` is valid only at namespace top level
- read access to `public var` is allowed through qualified imports
- write access to `public var` is allowed only inside the owner namespace
- external write to imported `public var` is rejected with `mutability.invalid_update`
- `public` means visibility, not global scope or global lifetime

---

## Functions

Functions use `func` and close with `end`.

Return types use `->`.

Omitting `-> Type` means the function returns no value.

Canonical examples:

```zt
func render_name(user: User) -> text
    return user.name
end

func log(message: text)
    io.print(message)
end

func main()
    print("Hello!")
end
```

Rules:

- ordinary no-return functions may omit the return clause
- explicit `-> void` is accepted for API clarity and public signatures
- `func main()` may omit return type; implicit exit code 0
- `return` is explicit when returning a value
- function parameters use `name: Type`
- local variable and parameter types are explicit (type inference is deferred)

---

## Parameters, Named Arguments And Defaults

Parameters are declared with explicit types.

Default values use `=`.

Named call arguments use `name: expression`.

```zt
func open_file(
    path: text,
    mode: text = "read"
) -> result<File, text>
    ...
end

const file: File = open_file("data.txt", mode: "write")?
```

Rules:

- required parameters come before defaulted parameters
- defaulted parameters form a trailing suffix
- public parameter names are API surface
- after a named argument appears, remaining provided arguments are named
- default expressions are evaluated at call time
- default expressions do not reference sibling parameters or `self`

---

## Variables And Mutability

Bindings use `const` or `var`.

```zt
const name: text = "Julia"
var hp: int = 100
hp = hp - 10
```

Rules:

- `const` bindings cannot be reassigned
- `var` bindings may be reassigned
- assignment is a statement, not an expression
- `let` is not canonical
- full local type inference is rejected (`const x = 42` is invalid)

Namespace-level mutable state:

```zt
namespace app.runtime.state

public var frame_count: int = 0

public func next_frame() -> int
    frame_count = frame_count + 1
    return frame_count
end
```

---

## Types

Builtin scalar types:

```
bool    int     int8    int16   int32   int64
u8      u16     u32     u64
float   float32 float64
text    bytes   void
```

Generic surface types:

```
list<T>
map<Key, Value>
set<T>
optional<T>
result<Success, Error>
lazy<T>
```

Type aliases:

```zt
public type io_result = result<void, core.Error>
type player_list = list<Player>
```

Rules:

- `text` is canonical; `string` is not a separate user type
- `int` means fixed `int64` on all platforms
- `float` means fixed `float64` on all platforms
- plain `uint` is not in surface syntax
- width-specific numeric types are opt-in
- `bytes` is a fundamental binary type in the surface language
- `void` is restricted to no-value type positions such as function returns and `result<void, E>`
- `void` is not valid for locals, fields, parameters, lists, maps or optionals
- `lazy<T>` is explicit delayed evaluation
- `set<T>` is hash-based with O(1) lookup; elements require `Hashable` and `Equatable`

---

## Numeric Model

Numeric conversions are explicit.

```zt
const count: int = int(value)
const precise: int64 = int64(value)
const ratio: float = float(value)
const label: text = to_text(value)
```

Rules:

- no implicit conversion between numeric types
- contextual literal fitting is allowed
- mixed numeric operations require explicit conversion
- integer overflow is checked by default
- division by zero is a runtime numeric error
- float to int conversion truncates toward zero

---

## Text

Text is UTF-8. Text literals use double quotes. Multiline text uses triple double quotes.

```zt
const title: text = "Zenith"
const page: text = """
Hello
Zenith
"""
```

Concatenation uses `+` only for `text + text`. No implicit conversions.

Interpolation uses `f"..."`:

```zt
const message: text = f"Player {player.name} has {player.hp} HP"
```

Rules:

- ordinary text literals do not interpolate
- interpolation requires `f"..."`
- expressions inside `{...}` in f-strings are ordinary expressions
- each interpolated expression must implement `TextRepresentable<T>`
- `len(text)` counts code points

---

## Bytes

`bytes` is immutable binary data.

```zt
const magic: bytes = hex bytes "DE AD BE EF"
```

Rules:

- byte literals use the `hex bytes` prefix
- indexing `bytes` yields `u8`
- slicing `bytes` yields `bytes`
- no implicit conversion between `text` and `bytes`

---

## Collections

Canonical literals:

```zt
const ports: list<int> = [3000, 3001]
const scores: map<text, int> = {"Julia": 10}
const tags: set<text> = set {"admin", "editor"}
```

Rules:

- `const` collections are observably immutable
- `var` collections allow explicit element/key updates
- list mutations: `list.append(item)`, `list.prepend(item)`
- list helpers: `list.first()`, `list.last()`, `list.rest()`, `list.skip()`
- list, text and bytes indexing are zero-based
- `list[index]` panics on out of bounds; use `list.get(index) -> optional<Item>` for safe lookup
- `map[key]` panics on missing key; use `map.get(key) -> optional<Value>` for safe lookup
- map keys require `Hashable<Key>` and `Equatable<Key>`
- set elements require `Hashable<T>` and `Equatable<T>`

---

## Optional, Result And Absence

Zenith has no `null`.

Absence uses `optional<T>` and `none`.

Recoverable failure uses `result<T, E>`.

Optional match:

```zt
match maybe_user
case some(user):
    return user.name
case none:
    return "anonymous"
end
```

Propagation uses `?`:

```zt
const user: User = load_user(id)?
```

Rules:

- `case some(name)` binds an optional present value
- `case none:` matches absence
- `?` is reserved for propagation/early return in functions that return `optional<...>` or `result<...>`
- `?` is not safe navigation
- `optional<T>.is_some()` returns `bool`
- `optional<T>.is_none()` returns `bool`
- `optional<T>.or(default_value)` returns `T`
- `result<T,E>.is_success()` returns `bool`
- `result<T,E>.is_error()` returns `bool`
- `.or_return(value)` helper for early return pattern
- `.or_wrap(context)` helper for error chaining
- panic is not caught by result/optional flow

---

## Expressions And Operators

Expression forms:

- function call: `name(args)`
- field access: `value.field`
- indexing: `value[index]`
- grouping: `(expression)`
- unary: `-value`, `not value`
- arithmetic: `*`, `/`, `%`, `+`, `-`
- comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- logical: `and`, `or`
- pipe: `value |> function` and `value |> function(extra)`
- if-else expression: `if cond then a else b`

Rules:

- conditions must be `bool`
- no truthiness
- no ternary operator (`cond ? a : b` is rejected)
- no `++` or `--`
- no `&&`, `||` or symbolic `!`
- `value |> f` is call sugar for `f(value)`
- `value |> f(extra)` is call sugar for `f(value, extra)`
- the right side of `|>` must be a callable target or a call expression
- user-defined operator behavior is limited to core traits:
  - `Comparable<T>` maps `<`, `<=`, `>`, `>=`
  - `Addable<T>` maps `+`
  - `Subtractable<T>` maps `-`
- no arbitrary custom operators
- no full operator overloading outside those trait contracts

Precedence from strongest to weakest:

1. field access, call, indexing
2. unary
3. multiplicative
4. additive
5. comparison
6. `and`
7. `or`
8. `|>`

### If-Else As Expression

Inline form (requires `else`):

```zt
const label: text = if active then "on" else "off"
```

Multiline form:

```zt
const result: int = if score > 100
    compute_bonus()
else
    default_value()
end
```

Rules:

- `then` is required for inline form
- `else` is always required in expression form (no dangling if)
- `then` is a contextual keyword (not reserved)

---

## Control Flow

Supported forms:

```zt
if ready
    start()
else if waiting
    queue()
else
    stop()
end

while index < total
    index = index + 1
end

for player in players
    display(player)
end

for key, value in scores
    show_score(key, value)
end

repeat 5 times
    io.print("tick")
end
```

Rules:

- all control blocks close with `end`
- `else if` is canonical; `elif` is not
- `unless` is not syntax
- `for` is collection-oriented, not C-style
- `for in range(start, end, step?)` covers numeric iteration with step
- `break` and `continue` apply to the nearest loop
- `repeat` count is evaluated once before the loop
- negative repeat count is invalid

---

## Resource Cleanup With `using`

`using` binds a value and gives it a predictable cleanup point.

Block form:

```zt
using handle = open_handle()
    use(handle)
end
```

Flat form:

```zt
using handle = open_handle()
use(handle)
return 0
```

Custom cleanup form:

```zt
using handle = open_handle() then close_handle(handle)
use(handle)
return 0
```

Rules:

- `using name = expr` declares `name` as immutable
- in block form, `name` is visible only inside the indented block
- in flat form, `name` is visible until the current block ends
- `using name = expr then cleanup(name)` runs the cleanup at scope exit
- cleanups run in LIFO order
- cleanups run before early `return`, before `?` propagation, and before `break`/`continue`

---

## Structs

Structs are value-oriented composition units.

```zt
struct Player
    name: text
    hp: int where it >= 0
end

var p1: Player = Player(name: "Julia", hp: 100)
p1.hp = 80
```

### Struct Construction

Struct construction is explicit and uses `Type(field: value)` form:

```zt
const pos: Point = Point(x: 10, y: 20)

func origin() -> Point
    return Point(x: 0, y: 0)
end

draw_rect(Point(x: 0, y: 0, width: 100, height: 50))

const points: list<Point> = [Point(x: 0, y: 0), Point(x: 1, y: 1)]
```

Rules:

- construction uses explicit `Type(field: value)` form
- bare `{ field: value }` struct type omission is not accepted syntax
- if the expected type is `map<K, V>`, brace form stays a map and key side is an expression
- `new` is not canonical
- fields are explicitly typed
- field defaults are allowed
- assigning fields requires a writable receiver

---

## Traits, Apply And Methods

Traits define behavior contracts.

`apply` attaches behavior to a type.

```zt
trait Healable
    mut func heal(amount: int)
end

apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Trait default implementations are allowed:

```zt
trait Describable
    func describe() -> text
        return "unknown"
    end
end
```

Rules:

- Zenith does not use classical OOP inheritance
- composition uses structs, traits and apply
- mutating methods are explicitly marked with `mut func`
- `mut` means the receiver may be changed through `self`
- receiver methods do not list `self` in the signature
- `self` is a reserved receiver value inside method bodies
- method bodies may use explicit `self.field` or shorthand `@field` for receiver access

---

## Enums And Match

Enums may have payloads. Block and inline forms are valid.

```zt
enum LoadUserError
    NotFound(id: int)
    InvalidName(message: text)
end

enum Direction = North, South, East, West
```

Construction uses qualified cases:

```zt
const error: LoadUserError = LoadUserError.NotFound(id: 10)
```

### Enum Dot Shorthand

When the expected type is known, the type name can be omitted:

```zt
const dir: Direction = .North
const color: Color = .Rgb(255, 0, 0)
```

Rules:

- shorthand requires an expected enum type
- it works in typed declarations, typed parameters, returns, and typed collection elements
- payload variants still use call syntax: `.Variant(value)` or `.Variant(name: value)`

### Match

`match` supports multiple literal cases:

```zt
match status
case 10, 20, 30:
    handle_known()
case 40 if can_handle_late_status:
    handle_late()
case else:
    handle_other()
end
```

Multi-value match uses tuple syntax:

```zt
match (status, code)
case (200, 0):
    handle_ok()
case (404, 1):
    handle_missing()
case else:
    handle_other()
end
```

Destructuring match:

```zt
match load_result
case some(user):
    print(user.name)
case none:
    print("not found")
end

match error
case NotFound(id):
    print(f"ID {id} not found")
case InvalidName(msg):
    print(msg)
end
```

Rules:

- match case delimiter is `:`
- `case else:` is the default/catch-all case
- `case pattern if condition:` adds a bool guard after the pattern match
- `match (a, b)` compares tuple case patterns element by element
- `case some(name):` binds optional present values
- `case variant(binding):` destructures enum payloads

---

### Const Destructuring

Tuple values can be destructured into immutable local bindings:

```zt
const (score, label) = make_pair()
```

Rules:

- the initializer must be a tuple
- the number of names must match the tuple arity
- each binding is immutable and receives the matching tuple element type

---

## Generics And Constraints

Generic declarations are explicit. The implementation strategy is **monomorphization** (specialized C code per type substitution).

### Generic Constraints

The canonical constraint form uses `where TypeParam is Trait`:

```zt
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable
    ...
end

struct Box<T>
where T is Greetable
    pub item: T
end
```

### Where Clause Constraints

Constraints use `where TypeParam is Trait`:

```zt
func merge<K, V>(a: map<K, V>, b: map<K, V>) -> map<K, V>
where K is Hashable and Equatable
    ...
end
```

Rules:

- `where TypeParam is Trait` is the canonical constraint form
- `where` may appear in generic declarations and in value-level contracts
- generic calls infer type arguments only from argument positions
- all generic parameters must be inferred from arguments; return context does not count
- if inference cannot determine every generic parameter, the call must provide explicit type arguments when that syntax is available

Core traits are implicit and do not require `import core...`:

- `Equatable<T>`
- `Hashable<T>`
- `Comparable<T>`
- `Addable<T>`
- `Subtractable<T>`
- `TextRepresentable<T>`

Operator trait method names:

- `Comparable<T>.less(other: T) -> bool` maps `<`
- `Comparable<T>.less_or_equal(other: T) -> bool` maps `<=`
- `Comparable<T>.greater(other: T) -> bool` maps `>`
- `Comparable<T>.greater_or_equal(other: T) -> bool` maps `>=`
- `Addable<T>.add(other: T) -> T` maps `+`
- `Subtractable<T>.subtract(other: T) -> T` maps `-`

---

## Dynamic Dispatch With `any`

To support heterogeneous collections where monomorphization is impossible, Zenith provides **dynamic dispatch via `any` (fat pointers)**.

```zt
const shape: any Shape = Circle(radius: 10)
var shapes: list<any Shape> = [circle, rect]
```

### Syntax

```zt
any TraitName
any<TraitName>
any<TraitName<TypeArg>>
```

`any` is a type constructor that produces a dynamic dispatch type from a trait name.

### Fat Pointer Representation

An `any<Trait>` value is a fat pointer containing:

1. A pointer to the boxed concrete value (RC-managed)
2. A pointer to the vtable for that `(concrete_type, trait)` pair

### Boxing

When a value of type `S` is used where `any<T>` is expected:

1. The compiler checks that `apply T to S` exists
2. The value is deep-copied into a new box
3. The box is tagged with the vtable for `(S, T)`
4. The fat pointer (box + vtable) is the `any<T>` value

Boxing takes ownership: the original value is moved into the box.

### Method Dispatch

```zt
const drawable: any Drawable = Circle(radius: 10)
drawable.draw()  -- indirect call through vtable
```

### Rules And Limits

Allowed:

- Traits with only non-mutating methods (`func`, not `mut func`)
- Traits with up to 8 methods
- Traits with copyable parameter and return types
- Non-generic traits
- `list<any<Trait>>` and other generic collections with any-typed elements

Disallowed (with diagnostic):

| Case | Diagnostic |
|------|------------|
| `mut func` in trait used with `any` | `any.mut_method` |
| Generic trait with `any` | `any.generic_trait` |
| Trait with >8 methods | `any.too_many_methods` |
| Uncopyable param/return type | `any.uncopyable` |
| No `apply` found for concrete type | `any.no_apply` |
| `any<T>` in `extern c` signature | `any.ffi_unsafe` |

Ownership rules:

- Boxing moves the value into the box (original is consumed)
- The box is RC-managed; cloning clones the inner value
- `any<T>` values cannot cross isolate boundaries
- `any<T>` values cannot be passed to `extern c` functions
- `any<T>` values cannot be stored in `public var`

ORC move analysis:

- ARC remains the observable memory model.
- The backend may optimize a final use of a managed local as a move instead of
  retain/copy.
- The backend may also move a managed struct field and clear that source field.
  Field liveness is field-aware: reading `box.other` does not by itself block
  moving `box.value`, but reading `box.value` or the whole `box` still blocks it.
- These moves are only valid when the source has no later relevant use on any
  generated function path reachable after the move point.
- Direct calls may transfer ownership into a compiler-detected sink parameter.
  If the caller still uses the argument later, the caller passes a retained
  owner. If the argument is a final use, the caller passes it directly and
  clears the source after the call. Assignment, standalone effect-call, and
  return-call cleanup contexts may move final-use arguments. If the same source
  is passed to more than one sink parameter, only one argument may consume the
  original owner; other sink arguments must receive retained owners.
- After the move, the generated cleanup path treats the source as empty.
- Source-level behavior does not change: the optimization is invisible unless
  you inspect generated C.

### Example

```zt
trait Drawable
    func draw() -> text
end

struct Circle
    radius: int
end

apply Drawable to Circle
    func draw() -> text
        return f"Circle(r={self.radius})"
    end
end

struct Rectangle
    width: int
    height: int
end

apply Drawable to Rectangle
    func draw() -> text
        return f"Rectangle({self.width}x{self.height})"
    end
end

func render_all(shapes: list<any Drawable>) -> text
    var result: text = ""
    for shape in shapes
        result = result + shape.draw() + "\n"
    end
    return result
end

func main()
    var shapes: list<any Drawable> = []
    shapes.append(Circle(radius: 10))
    shapes.append(Rectangle(width: 5, height: 3))
    print(render_all(shapes))
end
```

---

## Value-Level Where Contracts

`where` is a runtime value contract for fields and parameters.

```zt
struct User
    name: text where len(it) > 0
end

func set_age(age: int where age >= 0)
    ...
end
```

Rules:

- field `where` is allowed
- parameter `where` is allowed
- local variable `where` is not syntax
- return type `where` is not syntax
- failed checks produce contract panic
- `it` and the parameter name may refer to the checked value

---

## Panic And Recoverable Errors

`panic(message: text)` is a core fatal error mechanism. Panic is not ordinary recoverable control flow.

Recoverable errors use `result<T, E>`.

`todo(message: text)` marks a known incomplete path.

`unreachable(message: text)` marks a path that should be impossible.

`check(condition: bool)` validates an internal condition. `check(condition: bool, message: text)` uses the message when the condition is false.

---

## Attributes

Attributes use `attr`.

```zt
attr test
func loads_user()
    check(true)
end
```

Rules:

- one `attr` per line
- attributes apply to the next declaration
- attributes are metadata, not macros

---

## Tests In Source

Tests are ordinary functions marked with `attr test`.

```zt
attr test
func creates_user()
    check(true)
end
```

Rules:

- tests take no parameters and return no value
- `check(...)` is the canonical assertion helper
- `std.test.throws(func() -> void)` asserts that the body raises a runtime error
- `zt test` discovers and runs tests
- `zt test` reports each test name and duration
- failed or skipped tests include a short source trace

```zt
import std.test as test

attr test
func rejects_empty_name()
    test.throws(func() -> void
        panic("empty name")
    end)
end
```

---

## Callable Types

The callable type syntax:

```
func(ParamType, ...) -> ReturnType
```

All `func` types represent fat pointers (function pointer + context pointer).

```zt
func add(a: int, b: int) -> int
    return a + b
end

const op: func(int, int) -> int = add
const r: int = op(40, 2)
```

Rules:

- no parameter names in a callable type
- no default values in a callable type
- no type parameters (non-generic in v1)
- bare identifiers resolving to non-generic top-level `func` can appear in value position

### Escape Rules

Allowed positions:

- `const` / `var` at function scope
- function parameter
- function return
- argument to a call
- argument to an `extern c` callable parameter

Rejected positions:

- `public var` at namespace scope (`callable.escape_public_var`)
- struct field (`callable.escape_struct_field`)
- collection element types (`callable.escape_container`)

### FFI Integration

An `extern c` function may declare callable parameters with boundary-safe shapes:

```zt
extern c
    func qsort(base: bytes, count: int, size: int, cmp: func(int, int) -> int)
end
```

Passing a capturing closure to `extern c` is forbidden (`callable.extern_c_closure_unsupported`).

---

## Closures

Closures are anonymous `func` expressions that capture variables from the enclosing scope.

### Multi-statement closures

```zt
const factor: int = 5

const add_factor: func(int) -> int = func(value: int) -> int
    return value + factor
end
```

### Single-expression closures

```zt
const numbers: list<int> = [1, 2, 3, 4]

const doubled: list<int> = numbers.map(func(x: int) x * 2)
const evens: list<int> = numbers.filter(func(x: int) x % 2 == 0)
const sum: int = numbers.fold(0, func(acc: int, x: int) acc + x)
```

### Closure return type inference

When the return type is omitted, the checker first uses the expected callable
type when one is available. If there is no expected callable return type, it
infers from `return` statements in the closure body.

```zt
const process_item: func(Item) -> ProcessedItem = func(item: Item)
    return transform(item)
end
```

Rules:

- all returned values must fit the inferred return type
- `return` with no value infers `void`
- a closure body with no `return` also infers `void`

### Immutable Capture

By default, captured values are copied into the closure context as immutable snapshots:

- captured values are immutable inside the closure
- assigning to a captured outer variable is an error (`closure.mut_capture_unsupported`)
- closures may read `public var` through normal namespace access

### Persistent Mutable Capture With `capture`

The `capture` keyword declares a heap-allocated mutable binding local to the closure:

```zt
func counter() -> func() -> int
    return func() -> int
        capture count: int = 0
        count = count + 1
        return count
    end
end

const next: func() -> int = counter()
print(next())  -- 1
print(next())  -- 2
print(next())  -- 3
```

Rules:

- `capture name: Type = init` declares a persistent binding
- the binding is heap-allocated with ARC
- the binding is always mutable (no `mut` needed)
- scope is local to the closure instance
- each closure instance gets its own independent capture state
- `capture` does not mutate variables from the outer scope

### Local Named Functions

A `func` declaration inside a block acts as a local closure:

```zt
func process_items(items: list<int>, threshold: int) -> list<int>
    func is_valid(val: int) -> bool
        return val >= threshold
    end
    return list.filter(items, is_valid)
end
```

### ARC Integration

Closures that capture ARC-managed values (`text`, `list`, `map`, structs) retain captured references. Disposal hooks release them when the closure is no longer referenced.

---

## Extern And FFI

```zt
extern c
    func zt_sqlite3_memory_exec(sql: text) -> int
end
```

Rules:

- `extern c` blocks declare C-linkage functions
- boundary-safe types: `int`, `int32`, `int64`, `float`, `float64`, `bool`, `text`, `bytes`, `void`
- structs, enums, optional, result, `any<Trait>` are not boundary-safe
- function types as parameters are allowed for callbacks (non-capturing only)
- C libraries that require raw pointers should use a small C shim

---

## Explicit Lazy Values

```zt
import std.lazy as lazy

const later: lazy<int> = lazy.once_int(func() -> int
    return compute_value()
end)
const value: int = lazy.force_int(later)
```

The executable `0.4.2-beta.rc1` subset supports `lazy<int>`, `lazy<float>`,
`lazy<bool>`, and `lazy<text>`. Fully generic `lazy<T>` and lazy iterators are
post-RC work and are rejected during `zt check` when unsupported payloads are
used.

---

## UI And Abstract Trees

Zenith does not use syntactic-sugar macros or DSLs for UI. Building UI trees uses explicit structural composition:

```zt
var interface: Column = Column(
    alignment: .Center,
    children: [
        Image(source: "logo.png"),
        Text("Bem-vindo!", size: 24),
    ]
)
```

Rules:

- interfaces are standard parameterized structs
- children are arrays (`list<any Widget>`) passed to constructor arguments
- the compiler performs no specialized parsing for UI

---

## Builtins

Available without import:

| Builtin | Signature | Description |
|---------|-----------|-------------|
| `print` | `(value: TextRepresentable) -> void` | stdout + newline |
| `read` | `() -> text` | stdin line |
| `len` | `(value) -> int` | polymorphic length (list, map, set, text, bytes) |
| `debug` | `(value) -> void` | stderr with `[debug]` prefix |
| `type_name` | `(value) -> text` | static type name; compatibility alias, prefer `std.debug.type_name(value)` in new code |
| `range` | `(start: int, end: int) -> list<int>` | inclusive range |
| `range` | `(start: int, end: int, step: int) -> list<int>` | inclusive range with step |
| `panic` | `(message: text) -> void` | fatal error |
| `todo` | `(message: text) -> void` | incomplete path |
| `unreachable` | `(message: text) -> void` | impossible path |
| `check` | `(condition: bool) -> void` | assertion |
| `check` | `(condition: bool, message: text) -> void` | assertion with message |
| `to_text` | `(value: TextRepresentable) -> text` | explicit text conversion |

---

## Keywords

### Reserved Keywords (50)

```
namespace  import  as  func  end  const  var  return
if  else  while  for  in  repeat  times  break  continue
struct  trait  apply  to  enum  match  case
public  attr  where  is  and  or  not
true  false  none  some  success  error
optional  result  list  map  set
extern  any  type  void  mut  using  self  capture
```

### Contextual Keywords (1)

```
then
```

Contextual keywords are only recognized in specific syntactic positions and can be used as identifiers elsewhere.

---

## Explicitly Not In The Language

- `null`
- `char` type
- `uint` standalone
- tuples
- `?.` safe navigation
- `??` null coalescing
- implicit return
- rest operator (`...`)
- `try/catch`
- `async/await`
- `owned<T>` / `borrow<T>` / lifetimes
- macros
- overloads
- ternary `cond ? a : b`
- postfix guard `return x if cond`
- `unpack` destructuring on `const`
- full local type inference (`const x = 42`)
- struct type omission shorthand (`{ fields }`)
