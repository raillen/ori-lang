# Decision 064 - Stdlib Fs Module

- Status: accepted
- Date: 2026-04-22
- Type: stdlib / filesystem
- Scope: current alpha `std.fs` cut

## Summary

`std.fs` is the synchronous filesystem module for the current alpha cut.

Paths are represented as `text`.

The module now reflects the real host/runtime surface that is implemented today, including metadata timestamps exposed as Unix milliseconds.

## Decision

Canonical import:

```zt
import std.fs as fs
```

Current alpha surface:

- `fs.read_text(path: text) -> result<text, fs.Error>`
- `fs.write_text(path: text, content: text) -> result<void, fs.Error>`
- `fs.append_text(path: text, content: text) -> result<void, fs.Error>`
- `fs.exists(path: text) -> result<bool, fs.Error>`
- `fs.is_file(path: text) -> result<bool, fs.Error>`
- `fs.is_dir(path: text) -> result<bool, fs.Error>`
- `fs.create_dir(path: text) -> result<void, fs.Error>`
- `fs.create_dir_all(path: text) -> result<void, fs.Error>`
- `fs.list_dir(path: text) -> result<list<text>, fs.Error>`
- `fs.remove_file(path: text) -> result<void, fs.Error>`
- `fs.remove_dir(path: text) -> result<void, fs.Error>`
- `fs.remove_dir_all(path: text) -> result<void, fs.Error>`
- `fs.copy_file(from: text, to: text) -> result<void, fs.Error>`
- `fs.move(from: text, to: text) -> result<void, fs.Error>`
- `fs.metadata(path: text) -> result<fs.Metadata, fs.Error>`
- `fs.size(path: text) -> result<int, fs.Error>`
- `fs.modified_at(path: text) -> result<int, fs.Error>`
- `fs.created_at(path: text) -> result<optional<int>, fs.Error>`

Current alpha metadata layout:

```zt
public struct Metadata
    size_bytes: int
    modified_at_ms: int
    created_at_ms: optional<int>
    is_file: bool
    is_dir: bool
end
```

Examples:

```zt
import std.fs as fs

fs.write_text("log.txt", "started\n")?
fs.append_text("log.txt", "running\n")?

const names: list<text> = fs.list_dir(".")?
const info: fs.Metadata = fs.metadata("log.txt")?
const changed_ms: int = fs.modified_at("log.txt")?
```

Normative rules for this cut:

- paths are represented as `text`
- path manipulation belongs in `std.fs.path`, not in `std.fs`
- `std.fs` is synchronous in this cut
- text file operations use UTF-8
- invalid UTF-8 causes text-reading failure
- `exists`, `is_file` and `is_dir` return `result<bool, fs.Error>`
- directory listing returns entry names only
- timestamps are exposed as Unix milliseconds in the current alpha cut
- `created_at(...)` is optional because creation time is not uniformly available

## Rationale

The older draft described a richer metadata surface than the implemented alpha.

The delivered host/runtime layer is now stable enough for text IO, entry checks, create/remove flows, copy/move and basic metadata.

Publishing the real timestamp shape avoids pretending that `time.Instant` is already wired through this module when it is not.

## Deferred Work

Still outside this cut:

- `read_bytes(...)`
- `write_bytes(...)`
- file handles or streaming file IO
- async filesystem APIs
- symlink-specific APIs
- permission APIs
- richer directory entry objects
