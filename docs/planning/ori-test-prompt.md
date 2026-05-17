# Ori Language — Deep Implementation Test Prompt

> Use this prompt with any LLM or paste it directly into your dev workflow.
> It covers every chapter of the Ori spec: lexical, types, expressions,
> statements, functions, traits, errors, memory, and generics.

---

## Context

You are a compiler engineer and language tester for **Ori**, a statically typed,
reading-first language that compiles to native code. The compiler lives in a
Rust workspace with the crates: `ori-lexer`, `ori-parser`, `ori-ast`, `ori-hir`,
`ori-types`, `ori-diagnostics`, `ori-codegen`, `ori-runtime`, `ori-lsp`, and
`ori-driver`.

Your job is to produce exhaustive test cases — both **valid programs** that must
compile and run correctly, and **invalid programs** that must produce the exact
diagnostic errors listed in the spec.

For every test case state clearly:
- What is being tested
- Whether the program is valid or invalid
- The expected outcome (output, value, or exact error code from `13-error-catalog`)
- Any edge case or corner case being exercised

---

## Part 1 — Lexical Structure

### 1.1 Comments
Write Ori source snippets that test:
- A line comment `--` in the middle of an expression
- A block comment `--| ... |--` spanning 5 lines
- A documentation comment immediately before a `public func`, with `@param` and `@returns` tags
- A mismatched `@param` tag naming a parameter that does not exist → expect `doc.param_name_mismatch`
- An unclosed block comment `--|` with no closing `|--` → expect `lex.unclosed_block_comment`

### 1.2 Integer Literals
Write valid expressions using:
- Decimal with underscore separator: `1_000_000`
- Hexadecimal: `0xFF`
- Binary: `0b1010_1010`
- Octal: `0o755`
- Explicit width suffixes: `42i8`, `42u64`, `0u8`

Write invalid cases:
- `42i128` — unsupported width → expect a lexer or type error

### 1.3 String Literals
Write valid strings using:
- All escape sequences: `\\`, `\"`, `\n`, `\r`, `\t`, `\0`, `\u{1F600}`
- A triple-quote string with 3 lines and mixed indentation; verify the indentation baseline stripping rule
- An interpolated string `f"..."` with a field access: `f"user: {user.name}"`
- A byte string: `b"\xFF\x00"`
- A multi-line interpolated string: `f"""..."""`

Write invalid cases:
- `f"{a + "b"}"` — string literal inside interpolation → expect a parse error
- A `b"..."` string containing `\u{0041}` — unicode escape in byte string → expect a lexer error

### 1.4 Range Literals
Write and verify:
- `0..9` — 10 elements, ascending
- `9..0` — 10 elements, descending
- `5..5` — 1 element
- `0.0..1.0` — invalid today; expect a type error because ranges use `int` endpoints only

### 1.5 Reserved Words and Identifiers
- Attempt to declare `var times: int = 1` — valid (times is contextual)
- Attempt to declare `var loop: int = 1` — invalid (`loop` is reserved) → expect a lexer/parser error
- Use `times` as an identifier in an unrelated expression

---

## Part 2 — Type System

### 2.1 Primitive Types
Write a function that declares bindings for every primitive:
`bool`, `int`, `int8`, `int16`, `int32`, `int64`,
`u8`, `u16`, `u32`, `u64`,
`float`, `float32`, `float64`,
`string`, `bytes`, `void`.

Verify each default type: `42` is `int`, `3.14` is `float`.

### 2.2 Struct — Field Contracts
```
struct BoundedAge
    value: int if it >= 0 and it <= 150
end
```
- Construct with `value: 25` → valid
- Construct with `value: -1` → runtime panic `contract.field_violation`
- Construct with `value: 200` → runtime panic `contract.field_violation`
- Mutate a `var` binding to a valid value → valid
- Mutate a `var` binding to an invalid value → panic

### 2.3 Enum — Named Variants Only
```
enum Shape
    Circle(radius: float)
    Rectangle(width: float, height: float)
    Dot
end
```
- Construct all three variants using full form and shorthand `.Circle(radius: 1.0)`
- Try to define an enum variant with a positional (unnamed) payload → expect a parse error

### 2.4 Tuple
- Declare `const pair: tuple<int, string> = tuple(1, "one")`
- Access `pair.0` and `pair.1`
- Access `pair.2` on a 2-element tuple → expect a compile error

### 2.5 Optional
- `const x: optional<int> = some(5)` → unwrap with `if some`
- `const y: optional<int> = none` → match on it
- Use `?` to propagate `none` through a chain of two functions

### 2.6 Result
- Return `success(42)` and `error("fail")` from the same function
- Use `success()` with no args in a `result<void, string>` function
- Use `success()` with no args in a `result<int, string>` function → expect a type error

### 2.7 Equality
- `==` on two `int` values
- `==` on two `string` values
- `==` on `any<Trait>` values → expect `compile error`
- `==` on two `func(int) -> int` values → expect `compile error`

### 2.8 Type Aliases
```
alias UserId = int
alias Callback = func(string) -> bool
```
- Use `UserId` and `int` interchangeably in a function signature
- Confirm no new type is created (a `func(int) -> void` accepts `UserId`)

---

## Part 3 — Expressions

### 3.1 Arithmetic and Division
- `10 / 3` → truncates toward zero → result: `3`
- `10 % 3` → result: `1`
- `10 / 0` → runtime panic
- `10.0 / 0.0` → `Infinity` (IEEE 754)
- `-(-5)` → `5`

### 3.2 Comparison Chaining
- `a < b and b < c` → valid
- `a < b < c` → expect `compile error` (chaining not allowed)

### 3.3 Short-Circuit Boolean
Write a test where the right operand of `and` has a visible side effect.
Verify the side effect does NOT run when the left operand is `false`.
Same for `or` when left is `true`.

### 3.4 Error Propagation `?`
- `?` on `result<T, E>` inside a function returning `result<_, E>` → unwraps
- `?` on `optional<T>` inside a function returning `optional<_>` → unwraps
- `?` on `result<T, E>` inside a function returning `void` → expect a compile error
- `?` on `result<T, string>` inside a function returning `result<_, int>` → expect a type mismatch error

### 3.5 Pipe Operator `|>`
Write a pipeline:
```
users
|> iter.filter(do(u: User) => u.active)
|> iter.map(do(u: User) => u.name)
```
Verify the result is a `list<string>` of active user names.

### 3.6 Inline `if` Expression
- `const label: string = if score > 50 then "pass" else "fail"` → valid
- `const label: string = if score > 50 then "pass"` (no else) → expect a compile error
- Both branches returning different types → expect a type error

### 3.7 Struct Literal — Anonymous Form
```
struct Vec2
    x: float
    y: float
end
```
- `const v: Vec2 = .{x: 1.0, y: 2.0}` → valid
- `const v: Vec2 = .{x: 1.0}` — missing field, no `Default` impl → expect `type.anon_struct_field_mismatch` or similar
- `.{x: 1.0, y: 2.0}` with no context type → expect `type.anon_struct_type_unknown`

### 3.8 Struct Update `with`
```
const a: Config = Config(timeout: 30, retries: 3, verbose: false)
const b: Config = a with {
    verbose: true,
} end
```
- Verify `a.verbose == false` and `b.verbose == true` (no mutation)
- Verify `b.timeout == 30` (unchanged fields copied)

### 3.9 Collection Literals
- `list<string>` with 3 elements
- `map<int, string>` with 2 pairs
- `set<int>` with 3 elements (including a duplicate — verify deduplication)
- Empty `list<int>` with explicit type annotation
- Empty `list` with no annotation → expect a type inference error

### 3.10 Index and Slice
- `items[0]` on a 3-element list → valid
- `items[5]` on a 3-element list → runtime panic (out of bounds)
- `items[1..3]` → 2-element slice because end is exclusive
- `text[0..3]` on a `string` → 3-character slice because end is exclusive

---

## Part 4 — Statements and Control Flow

### 4.1 `const` vs `var`
- Attempt `const x: int = 0; x = 1` → expect a compile error
- `var x: int = 0; x += 5` → valid, `x` is `5`
- Shadow a binding in an inner scope → valid
- Shadow a binding in the same scope → expect a compile error

### 4.2 `if some` Binding
- Bind from a function returning `optional<User>` that returns `some(user)`
- Bind from a function returning `none`, with and without `else`
- Verify the bound variable is NOT accessible outside the `if` block

### 4.3 `while some` Loop
```
while some(line) = reader.next_line()
    process(line)
end
```
Verify it stops correctly when `next_line()` returns `none`.
Verify `line` is scoped per-iteration.

### 4.4 `loop` with `break`/`continue`
- A `loop` that increments a counter and `break`s at 10 → verify final value
- A `loop` with `continue` that skips odd numbers → verify only evens are processed
- `break` outside any loop → expect a compile error

### 4.5 `for` with Index
```
for item, index in ["a", "b", "c"]
    io.print(f"{index}: {item}")
end
```
Expected output: `0: a`, `1: b`, `2: c`.

### 4.6 `repeat` / `repeat N times`
- `repeat 3` runs the body exactly 3 times
- `repeat 3 times` is identical
- `repeat 0` runs zero times
- `repeat -1` → runtime panic
- `times` used as a variable name elsewhere → valid (contextual keyword)

### 4.7 `match` Exhaustiveness
- A `match` on a 4-variant enum with all 4 cases covered → valid
- A `match` missing one case → expect a compile error (non-exhaustive)
- A `match` with `case else:` covering the missing case → valid
- A guarded `case n if n > 0:` — verify it does NOT satisfy exhaustiveness alone

### 4.8 `using` Cleanup Order
Declare three `using` bindings A, B, C and verify dispose is called in C, B, A order.
Write a test where `?` propagation triggers cleanup of an already-bound resource.

### 4.9 `check`
- `check 1 + 1 == 2` → no panic
- `check 1 + 1 == 3` → panic
- `check false, "custom message"` → panic with message
- `check` vs field contract: show both in the same program and verify they are distinct

---

## Part 5 — Functions and Closures

### 5.1 Named Parameters and Defaults
```
func connect(host: string, port: int = 80) -> result<void, string>
```
- Call with only `host` → uses default port `80`
- Call with `host:` and `port:` named → valid in any order
- Call with positional arg after a named arg → expect a compile error

### 5.2 Value Contracts on Parameters
```
func sqrt(value: float if it >= 0.0) -> float
```
- Call with `4.0` → valid
- Call with `-1.0` → runtime panic `contract.param_violation`

### 5.3 Variadic Parameters
```
public func log(prefix: string, values: any<Displayable>...)
```
- Call with zero variadic args → valid
- Call with 3 variadic args of different types → valid
- Spread a `list<string>` using `..parts` → valid
- Declare a variadic that is not the last parameter → expect a compile error

### 5.4 `mut func` on Structs
```
struct Counter
    value: int

    mut func increment()
        self.value = self.value + 1
    end

    func get() -> int
        return self.value
    end
end
```
- `var c: Counter = Counter(value: 0); c.increment(); c.increment()` → `c.get()` is `2`
- `const c: Counter = Counter(value: 0); c.increment()` → expect a compile error

### 5.5 Closures — Capture Rules
- Capture a `const` binding by copy → valid
- Try to capture a `var` binding directly → expect a compile error
- Extract the current value of a `var` into a `const`, then capture that → valid
- Pass a named function where `func(T) -> R` is expected → valid

### 5.6 Async Functions
```
async func delayed_value() -> int
    await task.sleep(1)
    return 42
end

async func main()
    const n: int = await delayed_value()
    check n == 42
end
```
- Verify `await` is rejected outside `async func` → expect a compile error
- Verify `using` inside `async func` → expect `async.using_unsupported`

### 5.7 `todo`, `unreachable`, `panic`
- `todo()` in a branch that is not reached at runtime → compiles, no panic
- `todo()` in a branch that IS reached → panics
- `unreachable()` in a branch that is reached → panics

---

## Part 6 — Traits and Implement

### 6.1 Required vs Default Methods
```
trait Greetable
    func name() -> string

    func greet() -> string
        return f"Hello, {self.name()}!"
    end
end
```
- Implement only `name()` on a struct → `greet()` uses default → valid
- Override `greet()` on a struct → custom behavior → valid
- Implement without providing `name()` → expect a compile error

### 6.2 Operator Traits
Implement `Equatable` for a `Point` struct:
```
implement Equatable for Point
    func equals(other: Point) -> bool
        return self.x == other.x and self.y == other.y
    end
end
```
- `point_a == point_b` → uses `equals()` → valid
- Try to overload `*` → expect a compile error (not in the allowed set)

### 6.3 `Comparable` Ordering
Implement `Comparable` for a `Version` struct.
Verify `<`, `<=`, `>`, `>=` all derive from `compare()`.
Call `compare()` on a type that does NOT implement `Comparable` via `<` → expect a compile error.

### 6.4 `any<Trait>` — Dynamic Dispatch
```
const shapes: list<any<Drawable>> = [Circle(radius: 1.0), Rectangle(width: 2.0, height: 3.0)]
for shape in shapes
    shape.draw(canvas)
end
```
- `==` on two `any<Drawable>` values → expect a compile error
- Call a method NOT declared in `Drawable` through `any<Drawable>` → expect a compile error

### 6.5 Method Ambiguity
Define two traits `Alpha` and `Beta` both declaring `func output() -> string`.
Implement both for the same struct.
- Call `shape.output()` → expect `compile error` (ambiguous)
- Call `Alpha.output(shape)` → valid (explicit disambiguation)

### 6.6 `Iterable` — Custom Iterator
Implement `core.Iterable` for a struct `Countdown`.
The implementation must expose `mut func next() -> optional<int>`.
Use it in a `for` loop and verify values are produced in descending order.

---

## Part 7 — Errors and Propagation

### 7.1 `result<T, E>` — Type Mismatch on `?`
```
func a() -> result<int, string>
func b() -> result<int, int>
```
Inside `b`, use `a()?` → expect a type error (error type mismatch).

### 7.2 Error Enum (Union)
```
enum AppError
    Io(error: IoError)
    Validation(error: ValidationError)
end

implement Error for AppError
    func message() -> string
        match self
        case .Io(error):     return error.message()
        case .Validation(error): return error.message()
        end
    end
end
```
Write a function returning `result<void, AppError>` and verify:
- Both variants can be returned
- The `match` at the call site is exhaustive

### 7.3 Panic Sources
Trigger each of the following at runtime and verify the program terminates with a panic:
- `panic("explicit")`
- `check false`
- `todo()`
- `unreachable()` on a reached branch
- Integer division by zero `10 / 0`
- Out-of-bounds index `items[99]` on a 3-element list
- Field contract violation (see Part 2.2)
- Parameter contract violation (see Part 5.2)

### 7.4 No Exceptions Guarantee
Confirm there is no `throw`, `catch`, or `try` keyword in the language.
Write a program that chains 5 fallible operations using only `?` and `result`.

---

## Part 8 — Memory and Cleanup

### 8.1 Value Semantics
```
const a: Point = Point(x: 1, y: 2)
var b: Point = a
b.x = 99
check a.x == 1   -- a is unaffected
```

### 8.2 `using` — All Exit Paths
Write a function with a `using` binding where cleanup must run through:
1. Normal `return`
2. `?` propagation (error path)
3. `break` inside a loop

Verify for each case that `dispose()` is called exactly once.

### 8.3 Multiple `using` — LIFO Order
Declare `using a`, `using b`, `using c` with side effects in `dispose()`.
Verify cleanup order is C → B → A.

### 8.4 `using` on Non-`Disposable` Type
Try `using x: int = 5` → expect a compile error (no `Disposable` implementation).

### 8.5 `using` Inside `async func`
```
async func load()
    using file: File = open_read(path)?
    ...
end
```
Expect: `async.using_unsupported`

---

## Part 9 — Generics

### 9.1 Type Inference at Call Sites
```
func wrap<T>(value: T) -> optional<T>
    return some(value)
```
- `wrap(42)` → infers `T = int` → valid
- `wrap("hello")` → infers `T = string` → valid
- `wrap([])` with no annotation → expect an inference failure

### 9.2 `where` Constraints
```
func max<T>(a: T, b: T) -> T
    where T is Comparable
```
- Call with `int` → valid
- Call with a struct that does NOT implement `Comparable` → expect `generic.constraint_not_satisfied`

### 9.3 Multiple Constraints
```
func sorted_keys<K, V>(m: map<K, V>) -> list<K>
    where (
        K is Hashable
        and K is Comparable
    )
```
- Call with `map<int, string>` → valid
- Call with `map<Point, string>` where `Point` implements only `Hashable` → expect a constraint error

### 9.4 Negative Constraint
```
func raw_copy<T>(src: T, dst: T) where T is not Disposable
```
- Call with `int` → valid
- Call with a type implementing `Disposable` → expect a compile error

### 9.5 Generic Struct
```
struct Pair<A, B>
    first: A
    second: B
end
```
- `Pair<int, string>` and `Pair<string, int>` are different types
- A function accepting `Pair<int, string>` must reject `Pair<string, int>`

### 9.6 Monomorphization — Circular Instantiation
Write a generic function that would require infinite instantiation:
```
func recurse<T>(value: T) -> T
    return recurse(value)
end
```
Expect: compile error for circular/infinite generic instantiation.

### 9.7 v1 Limitations — Compile Errors Expected
- Higher-kinded type parameter: `trait Functor<F<_>>` → expect unsupported
- Associated type in trait: `type Item` inside a trait body → expect unsupported
- Const generic: `struct Matrix<const N: int>` → expect unsupported

---

## Part 10 — Cross-Cutting Scenarios

### 10.1 Full Pipeline Program
Write a complete, valid Ori program that uses:
- `namespace` and `import`
- `struct` with field contracts
- `enum` with named variants
- A `trait` with a default method
- `implement Trait for Type`
- A `generic func` with a `where` constraint
- `result<T, E>` with `?` propagation
- `optional<T>` with `if some` binding
- `using` for a resource
- A closure passed to a higher-order function
- `for` loop with index
- `match` with a guard
- `check` assertion

The program must compile and produce deterministic output.

### 10.2 Visibility Boundary
- A `public func` in namespace `app.util` is callable from `app.main` → valid
- A private func in `app.util` is called from `app.main` → expect `name.private`
- `public import` re-exports an alias; plain `import` does not

### 10.3 Import Cycle
- File A imports B, B imports A → expect `bind.import_cycle`

### 10.4 Namespace Mismatch
- File declares `namespace app.foo` but is imported as `app.bar` → expect `bind.import_namespace_mismatch`

### 10.5 LSP Sanity (`ori-lsp`)
- Open a valid `.orl` file; verify no spurious diagnostics
- Introduce a type error; verify the correct error code appears in the diagnostic
- Test go-to-definition for a function call
- Test hover over a struct field showing its type and contract

---

## Scoring Rubric (per test case)

| Criterion | Weight |
|---|---|
| Correct accept/reject decision | 40% |
| Exact diagnostic error code when invalid | 25% |
| Correct runtime behavior / output when valid | 25% |
| No extraneous errors on unrelated valid code | 10% |

---

## Notes for Test Runner

- Source files use `.orl` extension and UTF-8 encoding.
- Run valid programs with `ori run <file>` or `ori test` for `@test`-marked functions.
- Run type-check only with `ori check <file>`.
- Format with `ori fmt <file>` and verify the output is unchanged for spec-compliant formatting.
- Extract docs with `ori doc <file>` and verify Markdown output for documentation comment tests.
