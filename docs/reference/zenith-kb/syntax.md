# Zenith Syntax Reference

## File Shape

```zt
namespace app.users

import std.io as io
import app.users.types as types

public func main() -> int
    return 0
end
```

Rules:
- `namespace` is the first real declaration (only blanks/comments before it)
- imports come after namespace
- imports import namespaces, not individual symbols
- `import path.to.namespace as alias` is allowed
- `from ... import ...`, `import *`, relative imports are NOT MVP

## Comments

```zt
-- line comment
--- block comment ---
```

## Keywords

```
namespace  import  as  public
func  return  end
const  var
if  else  while  for  in  repeat  times  break  continue  match  case
struct  trait  apply  enum  where
and  or  not
true  false  none
success  error
mut  self
attr  check
panic
any
some
is
it
```

## Operators (strongest to weakest)

| Precedence | Operators |
|------------|-----------|
| 1 (highest) | `.field`, `call()`, `[index]` |
| 2 | unary `-`, `not` |
| 3 | `*`, `/`, `%` |
| 4 | `+`, `-` |
| 5 | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| 6 | `and` |
| 7 (lowest) | `or` |

NOT in Zenith: `&&`, `||`, `!`, `++`, `--`, `**`, `//`, ternary `?:`, `?.`, `??`

## Functions

```zt
-- With return type
func render_name(user: User) -> text
    return user.name
end

-- No return type. In small functions, prefer omitting -> Type.
func log(message: text)
    io.print(message)
end

-- Explicit void is valid when an API signature benefits from being explicit.
func flush() -> void
    io.print("flushed")
end

-- With default parameters
func open_file(
    path: text,
    mode: text = "read"
) -> result<File, text>
    ...
end

-- Named arguments at call site
const file: File = open_file("data.txt", mode: "write")?

-- Mutating method (receiver via self)
mut func heal(amount: int)
    self.hp = self.hp + amount
end
```

Rules:
- `func` closes with `end`
- `-> Type` for return; omit for no return when the intent is obvious
- `-> void` is valid and means no useful return value
- `return` is explicit
- Parameters: `name: Type`
- Defaults use `=`
- Named call args: `name: expression`
- Required params before defaulted params
- After a named arg, remaining args must be named
- Default expressions evaluated at call time
- Default expressions do NOT reference sibling params or `self` in MVP

## Variables

```zt
const name: text = "Julia"
var hp: int = 100
hp = hp - 10
```

Rules:
- `const` = immutable, `var` = mutable
- Assignment is a statement, not an expression
- `let` is NOT canonical
- Local type inference is NOT MVP

## Namespace-Level State (public var)

```zt
namespace app.runtime.state

public var frame_count: int = 0

public func next_frame() -> int
    frame_count = frame_count + 1
    return frame_count
end
```

Cross-namespace READ is allowed:
```zt
import app.runtime.state as state
func main() -> int
    return state.frame_count
end
```

Cross-namespace WRITE is NOT allowed (error: `mutability.invalid_update`):
```zt
import app.runtime.state as state
func force_reset()
    state.frame_count = 0  -- ERROR
end
```

Rules:
- `public var` only at namespace top level
- Read through qualified imports from anywhere
- Write only inside the owner namespace
- `pub` is NOT canonical
- `global` does NOT exist
- `public` = visibility, not global scope/lifetime

## Types

### Primitives
```
bool
int        (always int64)
int8, int16, int32, int64
u8, u16, u32, u64
float      (always float64)
float32, float64
text       (UTF-8, NOT "string")
bytes      (immutable binary data)
void       (function return or result<void, E>, not for locals/fields/params)
```

### Generics
```zt
list<T>
map<Key, Value>
optional<T>
result<Success, Error>
```

### Numeric Conversions (explicit only)
```zt
const count: int = int(value)
const precise: int64 = int64(value)
const ratio: float = float(value)
const label: text = to_text(value)
```

No implicit numeric conversion. Mixed ops require explicit conversion.

Use target-type calls for numeric conversion: `int(value)`, `float(value)`, `int64(value)`, etc.

`to_text(value)` is separate: it converts values that implement `TextRepresentable<T>` to `text`.

## Text

```zt
const title: text = "Zenith"
const page: text = """
Hello
Zenith
"""

-- Interpolation requires f"..."
const message: text = f"Player {player.name} has {player.hp} HP"

-- Explicit concat (no implicit conversion)
const report: text = "X=" + to_text(x) + ", Y=" + to_text(y)
```

Rules:
- Double quotes for text literals
- Triple double quotes for multiline
- Ordinary text literals do NOT interpolate
- Interpolation requires `f"..."` with `{expr}`
- `+` only for `text + text`
- `len(text)` counts code points

## Bytes

```zt
const magic: bytes = hex bytes "DE AD BE EF"
const packet: bytes = hex bytes """
    DE AD BE EF
    00 01 02 03
"""

const data: bytes = text.to_utf8("Hello")
const decoded: text = text.from_utf8(data)?
```

Rules:
- `hex bytes` prefix
- Hexadecimal data only
- ASCII whitespace and `_` may separate byte pairs
- Indexing yields `u8`, slicing yields `bytes`
- No implicit conversion between `text` and `bytes`

## Collections

```zt
const ports: list<int> = [3000, 3001]
const scores: map<text, int> = {"Julia": 10}

-- Mutation (var only)
var items: list<int> = [1, 2]
items.append(3)
items.prepend(0)

-- Safe lookup
const maybe_score: optional<int> = scores.get("Julia")
const maybe_item: optional<int> = items.get(3)

-- Direct indexing PANICS on miss
const score: int = scores["Julia"]    -- panics if key absent
const item: int = items[0]            -- panics if index OOB
```

Rules:
- `const` collections are observably immutable (including deep)
- `var` collections allow element updates
- Mutations: `list.append(item)`, `list.prepend(item)` (NOT push/unshift)
- COW under the hood
- Indexer `[]` panics on OOB/missing
- Safe lookup: `.get()` returns `optional<T>`
- Map keys require `Hashable` + `Equatable`
- Direct deep mutation (`players[0].hp = 80`) is NOT MVP
- Zero-based indexing, slice end is inclusive

## Optional and Result

```zt
-- Optional match
match maybe_user
case value user:
    return user.name
case none:
    return "anonymous"
end

-- Propagation with ?
const user: User = load_user(id)?
const value: int = read_cached_score()?
```

Rules:
- No `null` in Zenith
- Absence = `optional<T>` + `none`
- Recoverable failure = `result<T, E>`
- `case value name` binds present optional
- `?` propagates early return (function must return optional or result)
- `?` is NOT safe navigation
- `some(value)` is NOT required in MVP
- Panic is NOT caught by result/optional flow

## Control Flow

```zt
-- if/else if/else
if ready
    start()
else if waiting
    queue()
else
    stop()
end

-- while
while index < total
    index = index + 1
end

-- for (collection-oriented)
for player in players
    display(player)
end

-- for with key,value (maps)
for key, value in scores
    show_score(key, value)
end

-- repeat
repeat 5 times
    io.print("tick")
end

-- match
match status
case 10, 20, 30:
    handle_known()
case else:
    handle_other()
end

-- break and continue (nearest loop)
```

Rules:
- ALL blocks close with `end`
- `else if` is canonical (NOT `elif`)
- `unless` is NOT MVP
- `for` is collection-oriented (NOT C-style)
- `repeat` count evaluated once before loop
- Conditions must be `bool` (no truthiness)

## Structs

```zt
struct Player
    name: text
    hp: int where it >= 0
end

-- Construction
var p1: Player = Player(name: "Julia", hp: 100)
p1.hp = 80
```

Rules:
- Construction: `Type(field: value)`
- `new` is NOT canonical
- Fields explicitly typed
- Field defaults allowed (when compiler supports)
- `where` on fields = runtime contract (panics on failure)

## Traits and Apply

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

Rules:
- No classical OOP inheritance
- Composition via structs + traits + apply
- `mut func` = mutating method (receiver may change via `self`)
- `self` is reserved inside method bodies
- Receiver methods do NOT list `self` in signature
- Use `self.field` for receiver access

## Enums

```zt
-- Payload enum (block form)
enum LoadUserError
    NotFound(id: int)
    InvalidName(message: text)
end

-- Unit enum (inline form)
enum Direction = North, South, East, West

-- Construction
const error: LoadUserError = LoadUserError.NotFound(id: 10)

-- Exhaustive match
match error
case LoadUserError.NotFound(id):
    io.print(f"User {id} not found")
case LoadUserError.InvalidName(message):
    io.print(f"Invalid name: {message}")
end
```

Rules:
- Enums may have payloads
- Block and inline forms
- Match without `case else` must be exhaustive
- `case else` opts out of exhaustiveness

## Generics and Constraints

```zt
-- Monomorphized generic function
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable<Item>
    ...
end

-- Dynamic dispatch with any (heterogeneous collections)
var ui_nodes: list<any<Widget>> = []
ui_nodes.append(Button())
ui_nodes.append(Label())
```

Rules:
- Monomorphization is default (zero-cost, homogeneous)
- `any Trait` = fat pointer (vtable + data) for heterogeneous collections
- Generic type args may be omitted when inference is unambiguous
- Constraints: `where Type is Trait<Type>`

Core implicit traits (no import needed):
- `Equatable<Item>`
- `Hashable<Item>`
- `Comparable<Item>`
- `TextRepresentable<Item>`

## Where Contracts (Two Roles)

```zt
-- Runtime value contract (on field/parameter)
struct User
    name: text where len(it) > 0
end

func set_age(age: int where age >= 0)
    ...
end

-- Compile-time type constraint (on generic)
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable<Item>
    ...
end
```

Rules:
- `where` on field/param = runtime contract (panics on failure)
- `where` after generic header = compile-time constraint
- Local variable `where` is NOT MVP
- Return type `where` is NOT MVP
- Failed checks produce `runtime.contract` panic
- `it` refers to the checked value
- Constructor with `where` fields does NOT return `result<T, E>`
- For recoverable validation, write `try_create_*` returning `result`

```zt
struct Player
    hp: int where it >= 0
end

func try_create_player(hp: int) -> result<Player, text>
    if hp < 0
        return error("hp must be >= 0")
    end
    return success(Player(hp: hp))
end
```

## Panic and Errors

```zt
panic(message: text)   -- fatal, not caught by result/optional
```

Rules:
- `panic` = fatal error mechanism
- Not ordinary recoverable control flow
- Recoverable errors use `result<T, E>`
- Bounds checks, contracts, division by zero inject panic

## Attributes

```zt
attr test
func loads_user()
    check(true)
end
```

Rules:
- One `attr` per line
- Applies to next declaration
- Metadata, not macros
- `attr test` = test function (no params, no return)
- `check(...)` = canonical assertion helper
- `check(condition, message)` = assertion helper with custom message
- `todo(message)` = known incomplete path
- `unreachable(message)` = impossible path guard

## UI (Zero-Magic-DSL)

```zt
var interface = Column(
    alignment: Align.Center,
    children: [
        Image(source: "logo.png"),
        Text("Bem-vindo!", size: 24)
    ]
)
```

Rules:
- UI = standard parameterized structs
- Children = `list<any<Widget>>` arrays
- No JSX, no HTML, no DSL magic

## Expressions

Expression forms:
- Function call: `name(args)`
- Field access: `value.field`
- Indexing: `value[index]`
- Grouping: `(expression)`
- Unary: `-value`, `not value`
- Arithmetic: `*`, `/`, `%`, `+`, `-`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `and`, `or`
