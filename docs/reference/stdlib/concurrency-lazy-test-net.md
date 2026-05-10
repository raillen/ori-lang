# Concurrency, Lazy, Test and Net Reference

> Surface: reference
> Status: current

## `std.concurrent`

Current delivered surface is explicit copy helpers.

| API | Description |
| --- | --- |
| `concurrent.copy_int(value: int) -> int` | Copies an integer for an explicit boundary. |
| `concurrent.copy_bool(value: bool) -> bool` | Copies a boolean for an explicit boundary. |
| `concurrent.copy_float(value: float) -> float` | Copies a float for an explicit boundary. |
| `concurrent.copy_text(value: text) -> text` | Copies text for an explicit boundary. |
| `concurrent.copy_bytes(value: bytes) -> bytes` | Copies bytes for an explicit boundary. |
| `concurrent.copy_list_int(value: list<int>) -> list<int>` | Copies a list of integers. |
| `concurrent.copy_list_text(value: list<text>) -> list<text>` | Copies a list of text values. |
| `concurrent.copy_map_text_text(value: map<text,text>) -> map<text,text>` | Copies a text-to-text map. |

Notes:

- Typed facades are public surface: `jobs.Job<T>`, `channels.Channel<T>`, `shared.Shared<T>`, `atomic.Atomic<T>`.
- Current executable subset supports `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>`, and `Atomic<int>`.
- Unsupported payloads produce explicit diagnostics (for example: `std.jobs.spawn(...) currently supports only int or text payloads`).
- Engine/runtime hosts may use internal scheduling without exposing language-level async.
- Boundary-copy helpers remain explicit and are still recommended for transfer clarity.

Quick examples:

```zt
import std.jobs as jobs
import std.channels as channels

func produce() -> int
    return 41
end

func produce_label() -> text
    return "ready"
end

func main() -> int
    const handle: jobs.Job<int> = jobs.spawn(produce)
    const value: int = jobs.join(handle)

    const queue: channels.Channel<int> = channels.create()
    channels.send(queue, value)
    const received: optional<int> = channels.receive(queue)
    channels.close(queue)

    const label_job: jobs.Job<text> = jobs.spawn(produce_label)
    const label: text = jobs.join(label_job)

    const labels: channels.Channel<text> = channels.create()
    channels.send(labels, label)
    const received_label: optional<text> = channels.receive(labels)
    channels.close(labels)

    match received
        case some(n):
            return n
        case none:
            return -1
    end
end
```

## `std.lazy`

| API | Description |
| --- | --- |
| `lazy.once_int(thunk: func() -> int) -> lazy<int>` | Creates a one-shot lazy integer from a thunk. |
| `lazy.force_int(value: lazy<int>) -> int` | Evaluates and consumes the lazy integer. |
| `lazy.is_consumed_int(value: lazy<int>) -> bool` | Checks whether the lazy integer was already forced. |
| `lazy.once_float(thunk: func() -> float) -> lazy<float>` | Creates a one-shot lazy float. |
| `lazy.force_float(value: lazy<float>) -> float` | Evaluates and consumes the lazy float. |
| `lazy.is_consumed_float(value: lazy<float>) -> bool` | Checks whether the lazy float was already forced. |
| `lazy.once_bool(thunk: func() -> bool) -> lazy<bool>` | Creates a one-shot lazy bool. |
| `lazy.force_bool(value: lazy<bool>) -> bool` | Evaluates and consumes the lazy bool. |
| `lazy.is_consumed_bool(value: lazy<bool>) -> bool` | Checks whether the lazy bool was already forced. |
| `lazy.once_text(thunk: func() -> text) -> lazy<text>` | Creates a one-shot lazy text value. |
| `lazy.force_text(value: lazy<text>) -> text` | Evaluates and consumes the lazy text value. |
| `lazy.is_consumed_text(value: lazy<text>) -> bool` | Checks whether the lazy text value was already forced. |

The alpha surface is specialized for `int`, `float`, `bool`, and `text`.

Every lazy helper is one-shot. Creating a lazy value does not run the thunk.
The matching `force_*` helper runs it once and consumes the lazy value.
Generic managed `lazy<T>` remains future work until clone/drop semantics are safe for every `T`.
In `0.4.2-beta.rc1`, unsupported lazy payloads fail during `zt check`;
they should not reach C emission.

## `std.test`

| API | Description |
| --- | --- |
| `test.fail(message: text = "test failed") -> void` | Fails the current test with a message. |
| `test.skip(reason: text = "") -> void` | Skips the current test with a reason. |
| `test.throws(body: func() -> void) -> void` | Fails when the body finishes without a runtime error. |
| `test.is_true(value: bool) -> void` | Fails when the value is false. |
| `test.is_false(value: bool) -> void` | Fails when the value is true. |
| `test.equal_int(actual: int, expected: int) -> void` | Compares ints and reports expected/received values. |
| `test.equal_text(actual: text, expected: text) -> void` | Compares text and reports expected/received values. |
| `test.not_equal_int(actual: int, expected: int) -> void` | Fails when both ints are equal. |
| `test.not_equal_text(actual: text, expected: text) -> void` | Fails when both text values are equal. |
| `test.zt_test_fail(message: text) -> void` | Low-level exported fail helper used by the runtime/test bridge. |
| `test.zt_test_skip(reason: text) -> void` | Low-level exported skip helper used by the runtime/test bridge. |
| `test.zt_test_throws_closure(body: func() -> void) -> bool` | Low-level helper used by `test.throws`. |

Prefer `test.fail` and `test.skip` in user code.
Prefer `throws` for expected fatal paths.
Prefer `equal_*` and `not_equal_*` when expected/received values make the failure easier to fix.

## `std.net`

Types:

| Type | Description |
| --- | --- |
| `net.Error` | Network error category. |
| `net.Connection` | Network connection handle. |

Functions:

| API | Description |
| --- | --- |
| `net.connect(host: text, port: int, timeout_ms: int = 0) -> result<net.Connection, core.Error>` | Opens a connection to a host and port. |
| `net.read_some(connection: net.Connection, max_bytes: int) -> result<bytes, core.Error>` | Reads up to `max_bytes` from a connection. |
| `net.write_all(connection: net.Connection, data: bytes) -> result<void, core.Error>` | Writes all bytes to a connection. |
| `net.close(connection: net.Connection) -> result<void, core.Error>` | Closes a connection. |
| `net.is_closed(connection: net.Connection) -> bool` | Checks whether a connection is closed. |
| `net.kind(err: core.Error) -> net.Error` | Maps a core error to a network error category. |

Networking remains alpha. Validate behavior with project tests before relying on it in packages.
