# Decision 011 - Extern C And Extern Host

- Status: accepted
- Date: 2026-04-16
- Type: language / syntax
- Scope: foreign declarations, host boundary, portability, target integration

## Summary

Zenith Next distinguishes between foreign C bindings and host-provided bindings.

`extern c` defines target-specific bindings to C symbols.

`extern host` defines runtime or embedder-provided capabilities.

Both forms use block syntax and are limited to function declarations in the MVP.

## Decision

The canonical forms are:

```zt
namespace std.io.host

extern host
    public func read_file(path: text) -> result<text, text>
    public func write_stdout(message: text)
    public func write_stderr(message: text)
end
```

```zt
namespace platform.c.stdio

extern c
    public func puts(message: text) -> int32
end
```

Normative rules for this cut:

- `extern c` and `extern host` are top-level block forms
- `extern` blocks may appear only at namespace scope
- the MVP allows only function declarations inside `extern` blocks
- `extern` declarations have no body
- the declared function name is the external name in this cut
- function calls to extern declarations use normal call syntax
- `extern c` is explicitly target-specific and not guaranteed to be portable
- `extern host` is the boundary to host or embedder capabilities
- the language does not treat `extern host` as C FFI
- variadic extern functions are not part of the MVP
- callbacks in extern declarations are not part of the MVP
- raw pointer surface syntax is not part of the MVP
- symbol renaming and ABI annotations are not part of this cut

## Rationale

This keeps two different interop problems separate.

Calling into C is not the same thing as calling host services provided by the runtime or embedder. Giving them different syntactic forms makes that distinction visible.

Using blocks instead of one-line markers groups foreign declarations clearly and keeps the top of the file easier to scan.

Restricting the MVP to functions avoids committing early to variables, callbacks, pointers and lower-level ABI details before the rest of the language surface is stable.

## Canonical Examples

Host boundary:

```zt
namespace std.io.host

extern host
    public func read_file(path: text) -> result<text, text>
    public func write_stdout(message: text)
end
```

C boundary:

```zt
namespace platform.c.stdio

extern c
    public func puts(message: text) -> int32
end
```

Normal use after import:

```zt
import std.io.host
import platform.c.stdio

public func main()
    host.write_stdout("hello")
    const code: int32 = stdio.puts("hello from c")
end
```

## Non-Canonical Forms

These forms are not accepted as the canonical style for Zenith Next:

line-by-line extern marker:

```zt
extern c public func puts(message: text) -> int32
```

mixing host functions into `extern c`:

```zt
extern c
    public func write_stdout(message: text)
end
```

declaring extern variables in the MVP:

```zt
extern c
    public const errno: int32
end
```

using bodies inside extern declarations:

```zt
extern host
    public func write_stdout(message: text)
        ...
    end
end
```

## Out of Scope

This decision does not yet define:

- symbol renaming such as binding one Zenith name to a different external symbol
- explicit ABI strings or calling convention annotations
- extern variables
- callbacks and function-pointer-style interop
- pointer types and manual memory APIs in surface syntax
- conditional extern declarations by target


## Wave 5 amendments

Later post-v1 work narrows several original MVP exclusions:

- Wave 5.1 permits `extern c` callback parameters for top-level primitive function references only.
- Wave 5.2 permits per-function `attr name("symbol")` and `attr abi("cdecl"|"stdcall")` inside `extern c` blocks.

The original MVP exclusions still apply to raw pointer surface syntax, variadic externs, extern variables, generic callbacks, captured closures, and long-lived C callback storage.
