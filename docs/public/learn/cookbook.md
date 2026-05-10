# Zenith Cookbook

> Audience: user
> Status: current
> Surface: public

## Read A File Safely

```zt
namespace app.main

import std.fs as fs
import std.io as io

func main() -> result<void, core.Error>
    const content: text = fs.read_text("input.txt")?
    io.write(content)?
    return success()
end
```

Use `?` when the enclosing function returns a compatible `result`.

## Parse A Number

```zt
func parse_or_zero(raw: text) -> int
    const parsed: optional<int> = int.parse(raw)
    match parsed
        case some(value):
            return value
        case none:
            return 0
    end
end
```

Use `optional<T>` for expected absence.

Use `result<T, E>` when the caller needs an error.

## Build A Clear Error Boundary

```zt
func require_name(input: optional<text>) -> result<text, core.Error>
    match input
        case some(name):
            return success(name)
        case none:
            return error(core.Error.Invalid)
    end
end
```

Keep the branch that creates the error close to the rule it protects.

## Iterate A List

```zt
func total(values: list<int>) -> int
    var sum: int = 0

    for value in values
        sum = sum + value
    end

    return sum
end
```

Use a named variable when it improves scan speed.

## Iterate With Index

```zt
func first_non_empty(values: list<text>) -> optional<int>
    for value, index in values
        if len(value) > 0
            return some(index)
        end
    end

    return none
end
```

The second binding is the index for list-like collections.

## Use A Map

```zt
func lookup_score(scores: map<text, int>, name: text) -> optional<int>
    return scores.get(name)
end
```

Prefer `optional<T>` for missing keys.

## Create A Small Domain Type

```zt
struct Task
    title: text
    done: bool
end

func complete(task: Task) -> Task
    return Task(title: task.title, done: true)
end
```

Use a struct when field names carry meaning.

Use `tuple<T1, T2>` for small positional data.

## Convert Values To Text

```zt
const line: text = f"count = {count}"
```

Interpolation uses the same readable surface as normal expressions.

## Keep Symbol-Heavy Code Readable

Prefer this:

```zt
const raw: text = fs.read_text(path)?
const cleaned: text = text.trim(raw)

if len(cleaned) == 0
    return error(core.Error.Invalid)
end

return success(cleaned)
```

Avoid packing validation, conversion, and error mapping into one long line.

More guidance: `docs/reference/language/expression-readability.md`.
