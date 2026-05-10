# Learn Zenith in 30 Minutes

> Audience: user
> Status: current
> Surface: public

## 1. Create The Smallest Program

```zt
namespace app.main

import std.io as io

func main() -> result<void, core.Error>
    io.write("Hello, Zenith")?
    io.write("\n")?
    return success()
end
```

Run:

```powershell
.\zt.exe check examples/hello-world/zenith.ztproj
.\zt.exe run examples/hello-world/zenith.ztproj
```

What to notice:

- every file starts with a namespace;
- the import has an alias;
- `main` returns `result<void, core.Error>`;
- `?` makes errors visible.

## 2. Use Explicit Local Types

```zt
const label: text = "score"
var score: int = 0

score = score + 1
```

This is intentional.

Zenith avoids full local inference so readers do not need to guess the type.

## 3. Model Absence Without Null

```zt
func display_name(input: optional<text>) -> text
    match input
        case some(name):
            return name
        case none:
            return "anonymous"
    end
end
```

There is no `null`.

Use `optional<T>` when a value may be missing.

## 4. Return Errors Explicitly

```zt
func parse_count(raw: text) -> result<int, core.Error>
    const value: optional<int> = int.parse(raw)
    match value
        case some(count):
            return success(count)
        case none:
            return error(core.Error.Invalid)
    end
end
```

Use `result<T, E>` when the caller must handle failure.

## 5. Keep Control Flow Simple

```zt
if score > 10
    io.write("high")?
else
    io.write("normal")?
end
```

```zt
for item, index in values
    io.write(to_text(index))?
    io.write(": ")?
    io.write(item)?
end
```

Use simple branches first.

Split long expressions before they become hard to scan.

## 6. Use Traits For Behavior

```zt
trait Named
    func name() -> text
end

struct User
    id: int
    display_name: text
end

apply Named to User
    func name() -> text
        return self.display_name
    end
end
```

Traits define behavior.

`apply` connects behavior to a type.

## 7. Format Before Sharing

```powershell
.\zt.exe fmt examples/hello-world/zenith.ztproj --check
```

If the check fails:

```powershell
.\zt.exe fmt examples/hello-world/zenith.ztproj
```

Formatting is part of readability, not a cosmetic step.

## 8. What To Read Next

- `docs/public/language/language-reference.md` for compact rules.
- `docs/public/learn/cookbook.md` for task recipes.
- `docs/public/packages/tooling-guide.md` for CLI workflow.
