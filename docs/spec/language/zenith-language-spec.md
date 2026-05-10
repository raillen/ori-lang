# Zenith Language Specification

> Audience: user, contributor, maintainer
> Status: current
> Surface: spec
> Source of truth: yes; `final-language-contract.md` prevails for status and gap distinctions

## Purpose

Zenith is a reading-first systems language.

It aims to make native programs easier to read, review, test, and maintain.
The language prefers explicit intent over clever compression.

Core goals:

- clear syntax;
- predictable runtime behavior;
- no null;
- no exceptions as ordinary control flow;
- composition before inheritance;
- useful diagnostics;
- accessible documentation and examples.

## Mental Model

A Zenith program is a set of namespaces.

Each namespace contains values, types, functions, traits, and `apply` blocks.

The common path is:

1. define data with `struct` and `enum`;
2. define behavior with functions and traits;
3. attach behavior with `apply`;
4. return `optional<T>` for absence;
5. return `result<T, E>` for recoverable failure;
6. use `check` for programmer assertions and impossible states.

## File Shape

Every source file starts with one namespace.

```zt
namespace app.main

import std.io as io

func main()
    io.print("Hello, Zenith!")
end
```

Rules:

- `namespace` is the first real declaration;
- imports come after `namespace`;
- imports bring namespaces, not individual symbols;
- aliases use `as`;
- `import *` is not part of the language.

## Comments

Line comments use `--`.

Block comments use `---`.

```zt
-- One line.

---
Many lines.
Still a comment.
---
```

Public API documentation belongs in ZDoc, not in long inline comments.

## Names And Visibility

Declarations are private by default.

Use `public` to expose a namespace-level declaration.

```zt
public func area(width: int, height: int) -> int
    return width * height
end
```

Rules:

- `public` means visible outside the namespace;
- `public` does not mean global mutable state;
- `pub` is not canonical;
- shadowing is rejected;
- external code cannot write to another namespace's `public var`.

## Values

Bindings use `const` or `var`.

```zt
const name: text = "Ada"
var score: int = 0

score = score + 1
```

Rules:

- `const` cannot be reassigned;
- `var` can be reassigned;
- assignment is a statement;
- local types are explicit in v1;
- `let` is not canonical.

## Primitive Types

Current public primitive types:

| Type | Use |
| --- | --- |
| `bool` | true/false values |
| `int` | default signed integer |
| `int8`, `int16`, `int32`, `int64` | sized signed integers |
| `u8`, `u16`, `u32`, `u64` | sized unsigned integers |
| `float`, `float32`, `float64` | floating point numbers |
| `text` | valid UTF-8 text |
| `bytes` | raw binary data |
| `void` | no useful value |

Prefer `u8`, `u16`, `u32`, and `u64` over legacy long names.

## Collection And Wrapper Types

Common generic types:

```zt
list<T>
map<Key, Value>
set<T>
optional<T>
result<Success, Error>
lazy<T>
```

Examples:

```zt
const names: list<text> = ["Ana", "Bo"]
const ages: map<text, int> = {"Ana": 31}
const maybe_name: optional<text> = some("Ana")
const loaded: result<text, text> = success("ok")
```

## Functions

Functions use `func` and close with `end`.

```zt
func add(a: int, b: int) -> int
    return a + b
end

func log(message: text)
    print(message)
end
```

Rules:

- return types use `->`;
- no-return functions may omit `-> void`;
- `func main()` may omit a return type;
- parameters use `name: Type`;
- returning a value requires `return`.

## Parameters And Defaults

Default values use `=`.

Named call arguments use `name: value`.

```zt
func connect(host: text, port: int = 80) -> result<void, text>
    ...
end

connect("localhost", port: 8080)?
```

Rules:

- required parameters come before defaulted parameters;
- once a named argument is used, later provided arguments are named;
- parameter names are public API for public functions.

## Text Interpolation

Use `f"..."`.

```zt
const user: text = "Ada"
const message: text = f"hello {user}"
```

`fmt "..."` is a migration spelling only. Do not use it in new public docs.

## Type Aliases

Use `type` to name a type.

```zt
public type io_result = result<void, core.Error>
```

Aliases improve readability. They do not create a new runtime representation.

## Structs

Use `struct` for product data.

```zt
struct Player
    name: text
    hp: int where it >= 0
end

const player: Player = Player(name: "Mira", hp: 100)
```

Rules:

- fields are explicit;
- field contracts use `where`;
- construction names fields;
- expected-type shorthand may use `{ field: value }` where supported.

## Enums And Match

Use `enum` for closed sets of states.

```zt
enum LoadState
    loading
    ready(data: text)
    failed(message: text)
end
```

Use `match` for branching over known states.

```zt
match state
case loading:
    print("loading")
case ready(data):
    print(data)
case failed(message):
    print(message)
end
```

Fallback uses `case else:` where fallback is intentional.

```zt
match code
case 200:
    print("ok")
case else:
    print("other")
end
```

Prefer explicit enum variants over `case else:` when the enum is known.

## Optional Without Null

Zenith does not use `null`.

Absence is `optional<T>`.

```zt
func find_name(id: int) -> optional<text>
    if id < 0
        return none
    end

    return some("Ada")
end
```

Handle absence explicitly:

```zt
match find_name(1)
case some(name):
    print(name)
case none:
    print("missing")
end
```

## Result Without Exceptions

Recoverable failure is `result<T, E>`.

```zt
func read_config(path: text) -> result<text, text>
    if path == ""
        return error("empty path")
    end

    return success("config")
end
```

Use `?` to propagate failure from a function returning a compatible result.

```zt
func start(path: text) -> result<void, text>
    const config: text = read_config(path)?
    print(config)
    return success()
end
```

## Check

Use `check` for programmer assumptions that must hold.

```zt
check count >= 0
```

Use `result` for expected recoverable failures. Do not use `check` as ordinary
business error handling.

## Traits

Traits describe behavior.

```zt
trait Drawable
    func draw() -> text
end
```

Use `apply` to attach a trait to a type.

```zt
apply Drawable to Circle
    func draw() -> text
        return "circle"
    end
end
```

This gives Zenith interface-like behavior without class inheritance.

## Dynamic Dispatch With `any`

Use `any<Trait>` when one collection must hold different concrete types that
share a trait.

```zt
const shape: any<Drawable> = Circle(radius: 10)
```

`any Trait` is also accepted where supported.

Rules:

- prefer generics for homogeneous fast paths;
- use `any<Trait>` for heterogeneous values;
- do not write public docs with `dyn<Trait>`;
- older `dyn` material is historical or migration context.

## Generics

Use generic type parameters for reusable code.

```zt
func first<T>(items: list<T>) -> optional<T>
    if items.length == 0
        return none
    end

    return some(items[0])
end
```

Use `where ... is ...` constraints for trait requirements.

```zt
func render<T>(item: T) -> text
where T is Drawable
    return item.draw()
end
```

Use additional `where` clauses for more complex constraints.

## Composition Instead Of Class Inheritance

Zenith does not need class inheritance for reuse.

Use:

- structs for data;
- traits for required behavior;
- `apply` for implementation;
- enums for closed alternatives;
- functions for operations;
- modules for grouping.

Example:

```zt
struct Position
    x: int
    y: int
end

struct Sprite
    name: text
    position: Position
end

trait Movable
    func move(dx: int, dy: int) -> Sprite
end

apply Movable to Sprite
    func move(dx: int, dy: int) -> Sprite
        return Sprite(
            name: self.name,
            position: Position(
                x: self.position.x + dx,
                y: self.position.y + dy
            )
        )
    end
end
```

## Closures And Callables

Functions can be passed as values through callable/delegate support.

```zt
func twice(value: int, f: func(int) -> int) -> int
    return f(f(value))
end
```

Closures can capture persistent state with `capture` where supported.

```zt
func counter() -> func() -> int
    return func() -> int
        capture count: int = 0
        count = count + 1
        return count
    end
end
```

## Control Flow

Basic control flow:

```zt
if ready
    start()
else
    wait()
end

while running
    tick()
end

for item in items
    print(item)
end
```

`if` can be an expression where the expression form is supported:

```zt
const label: text = if active then "on" else "off"
```

## Modules And Packages

Namespaces are the language-level module boundary.

Project files describe package metadata and build roots.

Rules:

- source layout follows namespace intent;
- public APIs should stay small;
- package docs should not define semantics that contradict `docs/spec/language/`;
- generated or reference docs should point back to this spec.

## Runtime Model

The C backend owns runtime details such as reference counting, managed values,
cleanup, and generated helper functions.

User-facing rules:

- managed values are cleaned up by the generated runtime protocol;
- collections and text are safe language values;
- bounds checks and contract failures use controlled diagnostics;
- FFI boundaries must preserve runtime invariants;
- ordinary code should not rely on generated C internals.

## Standard Library Model

The standard library should be small, explicit, and layered.

Common areas:

- `std.io`;
- `std.fs`;
- `std.text`;
- `std.bytes`;
- `std.format`;
- `std.math`;
- `std.time`;
- `std.json`;
- `std.net`;
- `std.test`;
- `std.debug`.

Public docs must describe implemented APIs only, or clearly mark examples as
planned/pseudocode.

## Diagnostics

Diagnostics should be action-first.

A useful diagnostic tells the reader:

1. what happened;
2. where it happened;
3. what to do next;
4. why the rule exists when needed.

Examples should avoid stale syntax. A docs validation gate checks this.

## Complete Example

```zt
namespace app.main

import std.io as io

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

func load_user(id: int) -> result<User, text>
    if id < 0
        return error("invalid id")
    end

    return success(User(name: "Ada", age: 36))
end

func main() -> result<void, text>
    const user: User = load_user(1)?
    io.print(user.display())
    return success()
end
```

## Compatibility Notes

Some old docs and decisions mention syntax that existed during design.

Use this mapping:

| Old | Current |
| --- | --- |
| `dyn<Trait>` | `any<Trait>` |
| `case default:` | `case else:` |
| `fmt "..."` | `f"..."` |
| `<T: Trait>` or `given` constraints | `where T is Trait` |
| `assert` as a current feature | `check` |
| `uint8` style names | `u8` style names |

Historical decisions may still show old spelling. Current public docs should
not teach old spelling as canonical.
