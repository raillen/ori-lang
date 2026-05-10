# Modules and Visibility Reference

> Surface: reference
> Status: current

## Namespace

```zt
namespace app.inventory
```

A namespace is the module boundary.

`namespace` must be the first declaration in the file. Comments and blank lines may appear before it, but declarations may not.

Files with the same effective namespace share private symbols.

## Import

```zt
import std.io as io
import app.inventory.items as items
```

Rules:

- imports are namespace imports;
- use qualified access through the namespace or alias;
- `from ... import ...` is not canonical;
- `import *` is not canonical;
- relative imports are not canonical.

## Private by Default

```zt
func helper() -> int
    return 1
end
```

Top-level declarations are private unless marked `public`.

## Public Const and Func

```zt
public const version: text = "0.1.0"

public func answer() -> int
    return 42
end
```

External namespaces can read/call public declarations through qualified access.

## Public Var

```zt
namespace app.counter

public var value: int = 0

public func next() -> int
    value = value + 1
    return value
end
```

External read is allowed:

```zt
import app.counter as counter

const current: int = counter.value
```

External write is rejected:

```zt
counter.value = 0 -- error
```

Rules:

- `public var` is allowed only at namespace scope.
- `public` means visibility, not global scope.
- Zenith has no `global` syntax.
- Writes are allowed only inside the owner namespace.
- Use functions when mutation needs validation or invariants.

## Public Is Not Global

A public declaration belongs to its namespace.

```zt
app.counter.value
```

It does not become an unqualified global name.

This protects package authors from accidental name collisions and keeps imports readable.

## Relation To `mut`

`public var` exposes namespace state for reading.

`mut func` allows a method to mutate its receiver.

They do not replace each other.

```zt
struct Counter
    value: int
end

apply Counter
    mut func bump()
        self.value = self.value + 1
        return
    end
end
```

If a public API mutates internal namespace state, prefer a function with a clear name:

```zt
public func reset()
    value = 0
    return
end
```
