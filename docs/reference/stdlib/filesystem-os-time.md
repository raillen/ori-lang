# Filesystem, OS and Time Reference

> Surface: reference
> Status: current

## `std.fs`

Types:

| Type | Description |
| --- | --- |
| `fs.Error` | Filesystem error category. |
| `fs.Metadata` | File or directory metadata returned by `fs.metadata`. |

Functions:

| API | Description |
| --- | --- |
| `fs.read_text(file_path: text) -> result<text, fs.Error>` | Reads a whole file as text. |
| `fs.write_text(file_path: text, content: text) -> result<void, fs.Error>` | Writes text to a file, replacing existing content. |
| `fs.append_text(file_path: text, content: text) -> result<void, fs.Error>` | Appends text to the end of a file. |
| `fs.exists(target_path: text) -> result<bool, fs.Error>` | Checks whether a file or directory exists. |
| `fs.is_file(file_path: text) -> result<bool, fs.Error>` | Checks whether a path is a file. |
| `fs.is_dir(dir_path: text) -> result<bool, fs.Error>` | Checks whether a path is a directory. |
| `fs.create_dir(dir_path: text) -> result<void, fs.Error>` | Creates one directory. |
| `fs.create_dir_all(dir_path: text) -> result<void, fs.Error>` | Creates a directory and any missing parent directories. |
| `fs.list_dir(dir_path: text) -> result<list<text>, fs.Error>` | Lists directory entries as text paths or names. |
| `fs.remove_file(file_path: text) -> result<void, fs.Error>` | Removes a file. |
| `fs.remove_dir(dir_path: text) -> result<void, fs.Error>` | Removes an empty directory. |
| `fs.remove_dir_all(dir_path: text) -> result<void, fs.Error>` | Removes a directory tree. Use carefully. |
| `fs.copy_file(from: text, to: text) -> result<void, fs.Error>` | Copies a file from one path to another. |
| `fs.move(from: text, to: text) -> result<void, fs.Error>` | Moves or renames a file or directory. |
| `fs.metadata(target_path: text) -> result<fs.Metadata, fs.Error>` | Reads metadata for a path. |
| `fs.size(target_path: text) -> result<int, fs.Error>` | Returns file size in bytes. |
| `fs.modified_at(target_path: text) -> result<int, fs.Error>` | Returns last modified timestamp in milliseconds. |
| `fs.created_at(target_path: text) -> result<optional<int>, fs.Error>` | Returns creation timestamp when the host exposes it. |

## `std.fs.path`

| API | Description |
| --- | --- |
| `path.join(parts: list<text>) -> text` | Joins path parts using the platform separator. |
| `path.normalize(value: text) -> text` | Normalizes separators and redundant path segments. |
| `path.is_absolute(value: text) -> bool` | Checks whether a path is absolute. |
| `path.is_relative(value: text) -> bool` | Checks whether a path is relative. |
| `path.absolute(value: text, base: text) -> text` | Resolves a path against a base directory. |
| `path.relative(value: text, from: text) -> text` | Computes a relative path from another path. |
| `path.base_name(value: text) -> text` | Returns the final path component. |
| `path.name_without_extension(value: text) -> text` | Returns the final path component without extension. |
| `path.extension(value: text) -> text` | Returns the extension without changing the path. |
| `path.parent(value: text) -> text` | Returns the parent directory. |
| `path.has_extension(value: text, expected: text) -> bool` | Checks whether a path has a specific extension. |
| `path.change_extension(value: text, new_ext: text) -> text` | Replaces or adds an extension. |

Underscore-prefixed path helpers are internal compatibility helpers.

## `std.os`

Types:

| Type | Description |
| --- | --- |
| `os.Platform` | Host operating system category. |
| `os.Arch` | Host CPU architecture category. |
| `os.Error` | OS boundary error category. |

Functions:

| API | Description |
| --- | --- |
| `os.args() -> list<text>` | Returns process command-line arguments. `args()[0]` is the executable name/path received from the host. With `zt run`, values after `--` are forwarded as program args. |
| `os.env(name: text) -> optional<text>` | Reads one environment variable. |
| `os.pid() -> int` | Returns the current process id. |
| `os.platform() -> os.Platform` | Returns the current platform. |
| `os.arch() -> os.Arch` | Returns the current CPU architecture. |
| `os.current_dir() -> result<text, os.Error>` | Returns the current working directory. |
| `os.change_dir(dir_path: text) -> result<void, os.Error>` | Changes the current working directory. |

## `std.os.process`

Types:

| Type | Description |
| --- | --- |
| `process.ExitStatus` | Exit code information from a child process. |
| `process.CapturedRun` | Captured output and exit information from a child process. |
| `process.Error` | Process execution error category. |

Functions:

| API | Description |
| --- | --- |
| `process.run(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.ExitStatus, process.Error>` | Runs a program and returns only exit status. |
| `process.run_capture(program: text, args: list<text> = [], cwd: optional<text> = none) -> result<process.CapturedRun, process.Error>` | Runs a program and captures stdout/stderr metadata. |

## `std.time`

Types:

| Type | Description |
| --- | --- |
| `time.Instant` | Timestamp-like instant. |
| `time.Duration` | Duration in time units. |

Functions:

| API | Description |
| --- | --- |
| `time.now() -> time.Instant` | Returns the current instant. |
| `time.now_ms() -> int` | Returns the current Unix timestamp in milliseconds. |
| `time.sleep(duration: time.Duration) -> result<void, core.Error>` | Sleeps for a duration. |
| `time.sleep_ms(ms: int) -> result<void, core.Error>` | Sleeps for a number of milliseconds. |
| `time.since(start: time.Instant) -> time.Duration` | Returns elapsed time since `start`. |
| `time.until(target: time.Instant) -> time.Duration` | Returns time from now until `target`. |
| `time.diff(a: time.Instant, b: time.Instant) -> time.Duration` | Returns the difference between two instants. |
| `time.elapsed(start: time.Instant, finish: time.Instant) -> int` | Returns elapsed milliseconds between two instants. |
| `time.add(at: time.Instant, duration: time.Duration) -> time.Instant` | Adds a duration to an instant. |
| `time.sub(at: time.Instant, duration: time.Duration) -> time.Instant` | Subtracts a duration from an instant. |
| `time.from_unix(ts: int) -> time.Instant` | Creates an instant from Unix seconds. |
| `time.from_unix_ms(ts: int) -> time.Instant` | Creates an instant from Unix milliseconds. |
| `time.to_unix(at: time.Instant) -> int` | Converts an instant to Unix seconds. |
| `time.to_unix_ms(at: time.Instant) -> int` | Converts an instant to Unix milliseconds. |
| `time.milliseconds(n: int) -> time.Duration` | Creates a duration in milliseconds. |
| `time.seconds(n: int) -> time.Duration` | Creates a duration in seconds. |
| `time.minutes(n: int) -> time.Duration` | Creates a duration in minutes. |
| `time.hours(n: int) -> time.Duration` | Creates a duration in hours. |
