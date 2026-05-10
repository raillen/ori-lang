# I/O and JSON Reference

> Surface: reference
> Status: current

## `std.io`

| API | Description |
| --- | --- |
| `io.input: io.Input` | Standard input handle. |
| `io.output: io.Output` | Standard output handle. |
| `io.stderr: io.Output` | Standard error handle. |
| `io.read_line(from: io.Input = io.input) -> result<optional<text>, core.Error>` | Reads one line. Returns `none` at end of input. |
| `io.read_all(from: io.Input = io.input) -> result<text, core.Error>` | Reads all available input as text. |
| `io.write(value: text, to: io.Output = io.output) -> result<void, core.Error>` | Writes text without adding a newline. |
| `io.print(value: text, to: io.Output = io.output) -> result<void, core.Error>` | Writes text using the standard print behavior. |

Types:

| Type | Description |
| --- | --- |
| `io.Input` | Represents an input stream handle. |
| `io.Output` | Represents an output stream handle. |
| `io.Error` | Represents I/O errors at the stdlib boundary. |

## `std.json`

Current alpha JSON support is map-oriented.

| API | Description |
| --- | --- |
| `json.parse(input: text) -> result<map<text,text>, core.Error>` | Parses JSON text into a text-to-text map. |
| `json.stringify(value: map<text,text>) -> text` | Converts a text-to-text map to compact JSON text. |
| `json.pretty(value: map<text,text>, indent: int = 2) -> text` | Converts a map to formatted JSON text. |
| `json.read(file_path: text) -> result<map<text,text>, core.Error>` | Reads and parses a JSON file. |
| `json.write(file_path: text, value: map<text,text>) -> result<void, core.Error>` | Serializes a map and writes it to a JSON file. |

Use explicit `result` handling for file and parse failures.
