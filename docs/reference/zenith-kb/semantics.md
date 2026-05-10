# Zenith Semantics

## Visibility and Scope

### Namespace Scope

```zt
namespace app.users

-- Private by default
func helper()
    ...
end

-- Public: visible outside namespace
public func get_user(id: int) -> optional<User>
    ...
end
```

Rules:
- Top-level declarations are namespace-scoped
- Declarations are **private by default**
- `public` exposes outside the namespace
- `pub` is NOT canonical
- `global` does NOT exist
- Shadowing is **rejected** in MVP
- Duplicate names in same scope = error
- Files with same effective namespace share private symbols

### public var (Namespace Mutable State)

```zt
namespace app.runtime.state

public var frame_count: int = 0

public func next_frame() -> int
    frame_count = frame_count + 1   -- OK: same namespace
    return frame_count
end
```

Cross-namespace READ (allowed):
```zt
namespace app.main
import app.runtime.state as state

func main() -> int
    return state.frame_count   -- OK: read through qualified import
end
```

Cross-namespace WRITE (rejected):
```zt
namespace app.main
import app.runtime.state as state

func force_reset()
    state.frame_count = 0   -- ERROR: mutability.invalid_update
end
```

Rules:
- `public var` only at namespace top level
- Read access through qualified imports from anywhere
- Write access ONLY inside owner namespace
- `public` = visibility, NOT global scope or global lifetime

### Import Rules

```zt
import std.io as io
import app.users.types as types
```

- Imports import **namespaces**, not individual symbols
- Imported symbols remain **qualified** through namespace or alias
- `from ... import ...` is NOT MVP
- `import *` is NOT MVP
- Relative imports are NOT MVP
- Namespace aliases written as `as alias`

## Mutability

### Local Bindings

```zt
const name: text = "Julia"   -- immutable, cannot reassign
var hp: int = 100            -- mutable, can reassign
hp = hp - 10                 -- OK
name = "Bob"                 -- ERROR: mutability.const_update
```

Rules:
- `const` = cannot be reassigned
- `var` = can be reassigned
- Assignment is a **statement**, not an expression
- `let` is NOT canonical

### Collections

```zt
const items: list<int> = [1, 2, 3]
items.append(4)              -- ERROR: const collection is immutable

var items: list<int> = [1, 2, 3]
items.append(4)              -- OK
```

Rules:
- `const` collections are **observably immutable** (including deep)
- `var` collections allow explicit element/key updates
- Direct deep mutation (`players[0].hp = 80`) is NOT MVP
- Explicit read-modify-write is the MVP form for deep indexed updates

### Mutating Methods

```zt
apply Healable to Player
    mut func heal(amount: int)
        self.hp = self.hp + amount
    end
end
```

Rules:
- `mut func` = mutating method
- `mut` means receiver may change through `self`
- `self` is reserved inside method bodies
- Receiver methods do NOT list `self` in signature
- Use `self.field` for receiver access

## Value Semantics

Zenith uses **value semantics by default**.

```zt
var a: list<int> = [1, 2, 3]
var b: list<int> = a    -- semantic copy
b.append(4)
-- a is still [1, 2, 3]
```

Implementation may use:
- Direct copy for scalars
- Retain/release for immutable managed values
- **Copy-on-write (COW)** for mutable managed containers
- Internal move when source is provably unused

Observable rule: modifying `b` must NOT change `a`.

## Ownership and Lifetime

### Scope Rules

- Locals live until scope end
- Temporaries live until end of statement
- Cleanup happens on: normal return, early return, `?` propagation, branch exits, loop exits

### Managed Values

Managed values (tracked by ARC):
- `text`, `bytes`
- `list<T>`, `map<K, V>`
- `optional<T>` when T is managed
- `result<T, E>` when either side is managed
- Structs with managed fields
- Enums with managed payloads

### Evaluation Order

**Left-to-right**, always:
- Function arguments evaluate in written order
- Receiver evaluates before method arguments
- Defaults evaluate at call time after provided earlier arguments
- Generated C must NOT rely on unspecified C evaluation order

## Optional and Result Semantics

### optional<T>

```zt
-- Absence
const maybe: optional<int> = none

-- Match
match maybe_user
case value user:
    return user.name
case none:
    return "anonymous"
end

-- Propagation
const user: User = load_user(id)?
```

Rules:
- No `null` in Zenith
- `some(value)` is NOT required in MVP
- `case value name` binds present optional
- `?` = early return (function must return optional or result)
- `?` is NOT safe navigation

### result<T, E>

```zt
func open_file(path: text) -> result<File, io.Error>
    ...
end

const file: File = open_file("data.txt")?

-- Match
match result
case success(value):
    return value
case error(message):
    return 0
end
```

Rules:
- Recoverable failure uses `result<T, E>`
- `success(value)` and `error(value)` for construction
- `?` propagates errors
- Panic is NOT caught by result/optional flow
- Ignoring `result` is an error
- Ignoring standalone `optional` is an error in MVP

## Panic Semantics

```zt
panic("something is broken")
```

Rules:
- Panic = fatal language control flow
- NOT ordinary recoverable control flow
- NOT caught by result/optional flow
- Injected on: bounds violation, contract failure, division by zero, invalid UTF-8, allocation failure
- Panic output uses diagnostics model when source span available
- Future `attempt/rescue/finally` may make result flow more readable but does NOT catch panic

## Where Contracts

Two distinct roles based on syntax position:

### Runtime Value Contract (on field/parameter)

```zt
struct User
    name: text where len(it) > 0
end

func set_age(age: int where age >= 0)
    ...
end
```

- Checked at runtime
- Failed check = `runtime.contract` panic
- `it` refers to the checked value

### Compile-Time Type Constraint (on generic)

```zt
func contains<Item>(items: list<Item>, target: Item) -> bool
where Item is Equatable<Item>
    ...
end
```

- Checked at compile time
- Enforces trait bounds on generic parameters

### NOT MVP

- Local variable `where`
- Return type `where`

## Const Collections Immutability

```zt
const items: list<Player> = [Player(name: "Julia", hp: 100)]
items[0] = Player(name: "Julia", hp: 80)   -- ERROR
items[0].hp = 80                            -- ERROR
```

`const` collections do not permit deep observable mutation through their elements.

## Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Types | PascalCase | `Player`, `LoadUserError` |
| Enum cases | PascalCase | `North`, `NotFound` |
| Functions | snake_case | `render_name`, `try_create_user` |
| Variables | snake_case | `frame_count`, `user_name` |
| Fields | snake_case | `user.name`, `player.hp` |
| Namespaces | lower snake_case segments | `app.users`, `std.io` |
| Generic params | Descriptive PascalCase | `Item`, `Key`, `Value` |
| Traits | PascalCase | `Equatable`, `Healable` |
