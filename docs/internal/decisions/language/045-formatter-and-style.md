# Decision 045 - Formatter And Style

- Status: accepted
- Date: 2026-04-17
- Type: language / tooling / style
- Scope: official formatter, indentation, line width, multiline formatting, match layout, naming conventions, comments

## Summary

Zenith has an official deterministic formatter.

Formatting is part of the language's accessibility and reading-first design, not cosmetic polish.

The formatter should have few options, preserve visual stability and avoid style fragmentation.

## Decision

Normative rules for this cut:

- Zenith should have an official formatter
- formatting is deterministic
- formatter options should be minimal
- indentation uses 4 spaces
- tabs are not used for indentation
- target line width is 100 columns
- `end` aligns with the construct that opened the block
- top-level declarations are separated by one blank line
- long signatures use multiline formatting
- signatures with multiple parameter `where` clauses should use multiline formatting
- long calls use one argument per line
- long list/map literals use one item/entry per line
- simple short enums may be inline
- payload enums and long enums use block form
- `case` is not indented under `match`
- attributes use one `attr` line per attribute
- comments use `--` and `--- ... ---`
- long public documentation belongs in ZDoc, not comments
- vertical alignment is not used
- official naming conventions are part of style

## Indentation

Use 4 spaces.

```zt
func main() -> int
    if ready
        return 1
    end

    return 0
end
```

`end` aligns with the opening construct.

Tabs are not canonical.

## Line Width

The target line width is 100 columns.

This is a readability target, not necessarily a hard semantic limit.

The formatter should break long constructs when they exceed the target or when multiline form improves scanning.

## Blank Lines

Top-level declarations are separated by one blank line.

```zt
func first() -> int
    return 1
end

func second() -> int
    return 2
end
```

Inside functions, blank lines may separate logical steps.

```zt
func calculate(price: int, quantity: int) -> int
    const subtotal: int = price * quantity

    if subtotal <= 0
        return 0
    end

    return subtotal
end
```

## Functions

Short signature:

```zt
func add(a: int, b: int) -> int
    return a + b
end
```

Long signature:

```zt
func create_user(
    name: text,
    email: optional<text>,
    age: int where it >= 0
) -> User
    ...
end
```

A signature should use multiline form when:

- it exceeds the target line width
- it has several parameters
- it has multiple `where` clauses
- multiline form materially improves readability

## Where Clauses

Field contract may stay inline when short:

```zt
hp: int where it >= 0
```

Parameter contracts should prefer multiline function signatures when more than one parameter exists or contracts are long:

```zt
func transfer(
    amount: int where it > 0,
    from_balance: int where from_balance >= amount
) -> int
    return from_balance - amount
end
```

## Structs

One field per line.

```zt
struct Player
    name: text
    hp: int where it >= 0
end
```

## Enums

Short simple enum:

```zt
enum Direction = North, South, East, West
```

Payload or longer enum:

```zt
enum Event
    Click(x: int, y: int)
    KeyPress(key: text)
    Close
end
```

The formatter may expand inline enum form when it becomes too long.

## Match

`case` aligns with `match`.

```zt
match event
case Event.Click(x, y):
    return x + y
case Event.Close:
    return 0
end
```

This avoids excessive indentation and keeps cases as strong visual anchors.

Nested statements inside each case are indented one level.

## Control Flow

```zt
if condition
    ...
else if other
    ...
else
    ...
end
```

```zt
while condition
    ...
end
```

```zt
repeat 5 times
    ...
end
```

```zt
for key, value in scores
    ...
end
```

## Function Calls

Short call:

```zt
create_user(name: "Julia", age: 18)
```

Long call:

```zt
create_user(
    name: "Julia",
    email: none,
    age: 18
)
```

When a call is multiline, prefer one argument per line.

Named arguments remain in declaration order.

## List And Map Literals

Short list:

```zt
const numbers: list<int> = [1, 2, 3]
```

Long list:

```zt
const players: list<Player> = [
    Player(name: "Julia", hp: 100),
    Player(name: "Talyah", hp: 90)
]
```

Map:

```zt
const scores: map<text, int> = {
    "Julia": 10,
    "Talyah": 20
}
```

## Attributes

One attribute per line.

```zt
attr deprecated(message: "use new_name")
attr doc_hidden
func old_name()
    ...
end
```

No blank line should separate attributes from the declaration they modify.

## Comments

Line comment:

```zt
-- Short note.
const value: int = compute()
```

Block comment:

```zt
---
Reason this algorithm is unusual.
---
func compute()
    ...
end
```

Comments should be short implementation notes.

Long public documentation belongs in ZDoc.

Decorative comment banners are not canonical.

## Naming Conventions

Types use `PascalCase`:

```zt
User
LoadResult
```

Enum cases use `PascalCase`:

```zt
Success
NotFound
```

Functions use `snake_case`:

```zt
load_user
calculate_total
```

Variables, parameters and fields use `snake_case`:

```zt
user_id
created_at
```

Namespaces use lower `snake_case` segments:

```zt
namespace app.users
```

Generic parameters use descriptive `PascalCase`:

```zt
Item
Key
Value
Error
```

Single-letter generic names such as `T`, `U` and `E` are not canonical style for Zenith.

Public constants, if introduced later, should be discussed separately. Local `const` values use ordinary variable naming.

## No Vertical Alignment

Do not align declarations by adding extra spaces.

Non-canonical:

```zt
const a_long_name: int  = 1
const b:          text = "x"
```

Canonical:

```zt
const a_long_name: int = 1
const b: text = "x"
```

Vertical alignment creates noisy diffs and makes edits harder.

## Formatter Options

The formatter should avoid broad configuration.

Allowed future configuration may include:

- diagnostic mode
- check-only mode
- include/exclude paths

Style choices such as indentation width, match layout and naming conventions should not be per-project preferences in the MVP.

## Non-Canonical Forms

Tabs for indentation:

```zt
func main() -> int
	return 0
end
```

Indented `case` style:

```zt
match event
    case Event.Close:
        return 0
end
```

Dense long call:

```zt
create_user(name: "Julia", email: none, age: 18, role: "admin", active: true)
```

Decorative comment banner:

```zt
-----------------------------
-- User loading
-----------------------------
```

Vertical alignment:

```zt
const short:      int = 1
const long_name:  int = 2
```

## Rationale

Consistent formatting reduces cognitive load.

Four-space indentation creates clearer hierarchy for many readers.

A 100-column target gives room for explicit types without forcing excessive wrapping.

Unindented `case` labels preserve visual anchors in `match`.

Few formatter options prevent fragmented style and keep examples, docs and codebases visually consistent.

Naming conventions support scanning and reduce ambiguity between values, types, functions and namespaces.

## Out of Scope

This decision does not yet define:

- exact formatter CLI command
- formatter implementation
- full parse-preserving rewrite rules
- comment movement rules
- formatter check mode
- import sorting
- automatic line wrapping thresholds for every construct
- public constant naming
- lint rules beyond formatting
