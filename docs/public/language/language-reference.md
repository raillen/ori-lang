# Zenith Language Reference

> Audience: user
> Status: current
> Surface: public

## What Zenith Optimizes

Zenith is reading-first.

That means code should make these things visible:

- where a file belongs;
- what each local value is;
- where absence and errors can happen;
- when resources are cleaned up;
- when behavior comes from a trait.

## File Shape

```zt
namespace app.main

import std.io as io

func main() -> result<void, core.Error>
    io.write("hello")?
    io.write("\n")?
    return success()
end
```

Rules:

- `namespace` comes first.
- Imports are qualified: `import std.io as io`.
- Blocks close with `end`.
- Local declarations use explicit types.

## Values

```zt
const name: text = "Ada"
var count: int = 0

count = count + 1
```

Use `const` by default.

Use `var` only when the value must change.

## Main Types

| Shape | Example |
| --- | --- |
| primitive | `int`, `float`, `bool`, `text`, `bytes` |
| sequence | `list<int>` |
| mapping | `map<text, int>` |
| absence | `optional<text>` |
| success/error | `result<int, core.Error>` |
| product | `tuple<int, text>` |
| behavior | `trait` and `apply` |
| dynamic trait value | `any<TextRepresentable>` |

## Functions

```zt
func add(left: int, right: int) -> int
    return left + right
end
```

Function parameters and return type are explicit.

Named arguments are useful when a call has several meanings:

```zt
io.print("done", to: io.stderr)?
```

## Generics

Use generics when one small function works for more than one type.

```zt
func identity<T>(value: T) -> T
    return value
end

const name: text = identity("Ada")
```

For this beta, generic runtime support is capability-based. Supported
collection shapes work; unsupported shapes fail during `zt check` with a clear
message.

Repeated trait bounds use the readable `and` form:

```zt
func accept<T>(value: T) -> int where T is Addable and T is Comparable
    return 0
end
```

## Control Flow

```zt
if count > 0
    io.write("has items")?
else
    io.write("empty")?
end
```

```zt
match value
    case some(text_value):
        return success(text_value)
    case none:
        return error(core.Error.Invalid)
end
```

Use `case else:` as the fallback form.

Use `case pattern if condition:` when a case needs a guard.

## Errors And Absence

Use `optional<T>` when a value may be absent.

Use `result<T, E>` when an operation may fail.

```zt
func read_name(path: text) -> result<text, io.Error>
    const content: text = io.read_text(path)?
    return success(content)
end
```

`?` exits early when the value is an error or absent, depending on the enclosing return type.

## Cleanup

Use `using` for deterministic cleanup.

```zt
using file: io.File = io.open_read(path)?
const content: text = io.read_all(file)?
return success(content)
```

Cleanup runs on normal exit, `return`, `?`, `break`, `continue`, and panic paths.

## Traits And Operators

Zenith allows only the small operator trait set:

| Operator | Trait |
| --- | --- |
| `+` | `Addable` |
| `-` | `Subtractable` |
| `<`, `<=`, `>`, `>=` | `Comparable` |

Do not rely on hidden broad overloads.

## Text Interpolation

```zt
const message: text = f"hello {name}"
```

Values inside `{...}` must be text-representable.

## Rejected Surface

Do not use removed or rejected syntax:

- the removed tuple alias;
- old interpolation spelling;
- old match fallback spelling;
- old guard spelling;
- old dynamic dispatch spelling.

Use `tuple`, `f"..."`, `case else:`, `case ... if ...:`, and `any<Trait>`.

## Deeper References

- `docs/reference/grammar/syntax.md`
- `docs/reference/language/types.md`
- `docs/reference/language/functions-and-control-flow.md`
- `docs/spec/language/final-language-contract.md`
