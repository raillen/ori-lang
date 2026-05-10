# Ori

Ori is a reading-first, explicitly typed programming language designed for clarity and accessibility.

> *ori* (אורי)  Hebrew for "my light"

## Status

Early development. Compiler being written in Rust.

## Philosophy

Ori optimizes for reading, not writing. Code should make visible:

- where a file belongs (
amespace)
- what each value is (explicit types)
- where absence and errors can happen (optional, esult)
- when resources are cleaned up (using)
- when behavior comes from a trait (implement)

## Quick Example

```ori
namespace app.main

import ori.io as io

func main() -> result<void, ori.Error>
    io.write("hello from Ori")?
    return success()
end
```

## License

MIT OR Apache-2.0
