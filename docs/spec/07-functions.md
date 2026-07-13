# Ori Language Specification — Chapter 07: Functions and Closures

> Status: normative  
> Audience: compiler implementers  
> Surface: **S3** (`0.3.0`)

---

## Function Declarations

There is **no** declaration keyword `func`. A function is a name, parameter
list, optional return type, and body:

```ori
add(a: int, b: int) -> int
    return a + b
end

public greet(name: string) -> string
    return f"hello {name}"
end
```

Single-expression body with `=>`:

```ori
double(x: int) -> int => x * 2

greet(name: string) -> string => f"hi, {name}"
```

Rules:
- Parameter types are explicit on public APIs and ordinary declarations.
- Return type may be omitted when it is `void` (same omission rules as before S3).
- Prefer `alias` for long/repeated return types (`alias UserResult = result[User, string]`).
- `public` makes the function visible outside its module.
- Multi-statement bodies close with `end` (optional label: `end` / construct name).
- Writing `func name(...)` as a **declaration** is a hard error: `parse.func_removed`.
- The word `func` remains only in **callable types**: `func(int) -> int`.

---

## Async Functions

```ori
import ori.task = task

async load_count() -> int
    await task.sleep(1)
    return 41
end

async main()
    const count: int = await load_count()
end
```

Rules:
- `async name(...) -> T` has call type `future[T]`.
- Inside the body, `return value` still returns a `T`.
- `await expr` may appear only inside an `async` function.
- `await` requires `expr` to have type `future[T]` and produces `T`.
- `async main()` is allowed in the native backend.
- `using` is allowed inside `async` functions (resources stored in the async frame).

Implementation status:
- Native runtime: pollable futures, failed/cancelled states, FIFO executor,
  non-blocking timers for `task.sleep`.
- Unsupported async shapes fail with `backend.native_unsupported` before Cranelift.
- Cooperative cancellation via `task.CancelToken`.

---

## Parameters

### Required Parameters

```ori
connect(host: string, port: int) -> result[void, string]
```

### Default Values

```ori
connect(host: string, port: int = 80) -> result[void, string]
```

- Parameters with defaults must come after required parameters.
- The default expression is evaluated at each call site.

### Named Arguments

```ori
connect(host: "localhost", port: 8080)
```

Rules:
- Once a named argument is used, all subsequent arguments must also be named.
- Named arguments may be given in any order.
- For public functions, parameter names are part of the public API.

### Value Contracts (`if` on parameters)

```ori
sqrt(value: float if it >= 0.0) -> float
clamp(v: int, lo: int, hi: int if it >= lo) -> int
```

`it` refers to the parameter value. A violation is a runtime panic
(`contract.param_violation` — planned/runtime).

### Variadic Parameters

```ori
public log(prefix: string, values: any[Displayable]...)
```

- Inside the body, `values` is typed as `list[any[Displayable]]`.
- Only the last parameter may be variadic.
- Spread a list with `..`:

```ori
const parts: list[string] = ["a", "b", "c"]
concat(..parts)
```

---

## Return Types

### Explicit Return

```ori
area(w: int, h: int) -> int
    return w * h
end
```

### Void Return

```ori
print_all(items: list[string])
    for item in items
        io.print(item)
    end
end
```

When a function returns `void`, bare `return` exits early.

### `result[T, E]` and propagation

```ori
read_file(path: string) -> result[string, string]
    const file: ori.fs.File = try ori.fs.open_read(path)
    return ori.fs.read_all(file)
end
```

Only `try expr` propagates. Postfix `expr?` is removed (`parse.question_propagate_removed`).
See Chapter 09.

---

## Mutating Methods (`mut`)

```ori
struct Counter
    value: int

    mut increment()
        self.value = self.value + 1
    end

    get() -> int
        return self.value
    end
end
```

Rules:
- `mut` methods may assign to `self` or its fields.
- Non-`mut` methods may not modify `self`.
- Calling a `mut` method on a `const` binding is a compile error.

---

## Methods on Structs

Methods may be declared inside a `struct` body (inherent) or via `apply Type`
(free methods / trait methods). See Chapter 08 for traits.

```ori
struct Rectangle
    width: float
    height: float

    area() -> float
        return self.width * self.height
    end

    scale(factor: float) -> Rectangle
        return Rectangle(width: self.width * factor,
            height: self.height * factor)
    end
end
```

---

## Closures (S3)

Canonical forms — **no** `do` / `fn` / `given`:

```ori
const double: func(int) -> int = (x: int) => x * 2
const is_even: func(int) -> bool = (n: int) => n % 2 == 0

users.map((u) => u.name)
users.filter((u: User) => u.active)

-- multi-statement
users.map((u: User)
    const n: string = u.name
    return n.to_upper()
end)
```

Rules:
- `(params) => expr` — single expression.
- `(params) … end` — statement block.
- Parameter types may be omitted when the checker context provides them.
- Prefer a named function when the body is large.
- `do(...)` is rejected with `parse.do_removed`.

### Callable type

```ori
const f: func(int) -> int = double
```

---

## Poetic calls

A call may omit parentheses when there is **exactly one** argument on the same
line (juxtaposition):

```ori
print name
print greet("hello")    -- argument is a parenthesized/call form; not nested poetic
print user.name()
```

Nested poetic juxtaposition is rejected (`parse.poetic_call_nested`):

```ori
print greet name        -- error
```

Mental rule: at most one “verb without parentheses” per expression.

---

## Generics on functions

```ori
max for T: Comparable (a: T, b: T) -> T
    if a.compare(b) >= 0
        return a
    end
    return b
end
```

See Chapter 11. Removed: `func max<T>(...) where T is Comparable`.

---

## Labeled `end`

Optional construct labels improve navigation:

```ori
if ok
    ...
end if

match shape
    case Circle(radius: r):
        ...
    case else:
        ...
end match
```

Mismatch between label and opening construct → `parse.end_label_mismatch`.
