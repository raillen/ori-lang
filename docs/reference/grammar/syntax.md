# Syntax Reference

> Surface: reference
> Status: current

## File Shape

```zt
namespace app.main

import std.io as io

public func main() -> int
    io.print("hello")
    return 0
end
```

Rules:

- `namespace` must be the first declaration in the file.
- Only blank lines and comments may appear before `namespace`.
- `import` appears after `namespace`.
- Imports bring namespaces, not individual symbols.
- Blocks close with `end`.
- Recommended indentation: 4 spaces.

## Namespace First

The namespace is the identity of the file inside a Zenith project.

Valid:

```zt
-- File comment is allowed.
namespace app.inventory

import std.text as text

public func name() -> text
    return "inventory"
end
```

Invalid:

```zt
public const version: text = "0.1.0"

namespace app.inventory -- error: namespace is not first
```

Why this rule exists:

- the reader sees the module identity immediately;
- the compiler does not need hidden module inference;
- package code stays predictable when files move;
- tooling can index the project with less ambiguity.

This matches the Zenith philosophy: explicit structure first, hidden behavior last.

## Comments

```zt
-- line comment
--- block comment ---
```

## Reserved Words

```text
namespace import as public
func return end const var
if else while for in repeat times break continue match case
struct trait apply enum where to is
and or not true false none success error
some mut self attr extern any optional result list map
void
```

Contextual forms and builtins:

```text
f"..."
check(...)
todo(...)
unreachable(...)
panic(...)
len(...)
to_text(...)
int(...) float(...) int64(...)
it in where-contract explanations
```

Important distinction:

- reserved words are tokenized by the lexer;
- contextual forms are recognized by the parser or semantic checker in specific positions;
- `c` and `host` are contextual ABI labels after `extern`, not general reserved words;
- `f"..."`, `check`, `todo`, `unreachable`, `panic`, `len`, `to_text`, `int` and `float` are not ordinary namespace declarations.

## Operators

| Group | Operators |
| --- | --- |
| access/call | `.field`, `call()`, `[index]` |
| unary | `-`, `not` |
| multiply | `*`, `/`, `%` |
| add | `+`, `-` |
| compare | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| boolean | `and`, `or` |
| propagation | `?` for supported optional/result propagation |

Small absence/error helpers:

| Helper | Meaning |
| --- | --- |
| `value.or(fallback)` | unwrap `optional<T>` or use `fallback` |
| `value.or_return(return_value)` | unwrap `optional<T>` or return `return_value` from the enclosing function |
| `result.or_wrap(context)` | keep `success`, or add text context to `core.Error` on failure |

Not canonical in Zenith:

```text
&& || ! ++ -- ** // ?: ?. ??
```

## Declaration Order

Recommended order inside a namespace:

1. imports;
2. public types;
3. public constants and public vars;
4. public functions;
5. private helpers.

The order is not only style. It keeps package files readable for people and easier to scan with tooling.
