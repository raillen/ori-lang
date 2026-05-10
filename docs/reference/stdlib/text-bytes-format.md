# Text, Bytes and Format Reference

> Surface: reference
> Status: current

## `std.text`

| API | Description |
| --- | --- |
| `text.to_utf8(value: text) -> bytes` | Converts text to UTF-8 bytes. |
| `text.from_utf8(value: bytes) -> result<text, text>` | Decodes UTF-8 bytes into text, returning an error message when invalid. |
| `text.trim(value: text) -> text` | Removes whitespace from the start and end. |
| `text.trim_start(value: text) -> text` | Removes whitespace from the start. |
| `text.trim_end(value: text) -> text` | Removes whitespace from the end. |
| `text.contains(value: text, needle: text) -> bool` | Checks whether `needle` appears inside `value`. |
| `text.join(parts: list<text>) -> text` | Joins a list of text values into one text value. |
| `text.replace_all(value: text, needle: text, replacement: text) -> text` | Replaces every occurrence of `needle` with `replacement`. |
| `text.starts_with(value: text, prefix: text) -> bool` | Checks whether `value` starts with `prefix`. |
| `text.ends_with(value: text, suffix: text) -> bool` | Checks whether `value` ends with `suffix`. |
| `text.has_prefix(value: text, prefix: text) -> bool` | Alias-style helper for prefix checks. |
| `text.has_suffix(value: text, suffix: text) -> bool` | Alias-style helper for suffix checks. |
| `text.has_whitespace(value: text) -> bool` | Checks whether the text contains whitespace. |
| `text.index_of(value: text, needle: text) -> optional<int>` | Returns the first index of `needle`, or `none` when absent. |
| `text.last_index_of(value: text, needle: text) -> optional<int>` | Returns the last index of `needle`, or `none` when absent. |
| `text.index_of_or_minus_one(value: text, needle: text) -> int` | Compatibility helper that returns `-1` when absent. |
| `text.last_index_of_or_minus_one(value: text, needle: text) -> int` | Compatibility helper that returns `-1` when absent. |
| `text.is_empty(value: text) -> bool` | Checks whether the text has no content. |
| `text.is_blank(value: text) -> bool` | Checks whether the text is empty or contains only whitespace. |
| `text.is_digits(value: text) -> bool` | Checks whether the text contains only digit characters. |
| `text.limit(value: text, max_len: int) -> text` | Truncates text to at most `max_len` characters. |

Migration note: older code that compared `text.index_of(...)` or
`text.last_index_of(...)` with `-1` should either use `.or(-1)` on the
optional result, or call the explicit compatibility helpers.

Underscore-prefixed helpers in `std.text` are internal compatibility helpers and should not be used in public examples.

## `std.bytes`

| API | Description |
| --- | --- |
| `bytes.empty() -> bytes` | Creates an empty byte buffer. |
| `bytes.from_list(values: list<int>) -> bytes` | Creates bytes from a list of integer byte values. |
| `bytes.to_list(value: bytes) -> list<int>` | Converts bytes to a list of integer byte values. |
| `bytes.join(left: bytes, right: bytes) -> bytes` | Concatenates two byte buffers. |
| `bytes.starts_with(value: bytes, prefix: bytes) -> bool` | Checks whether bytes start with `prefix`. |
| `bytes.ends_with(value: bytes, suffix: bytes) -> bool` | Checks whether bytes end with `suffix`. |
| `bytes.contains(value: bytes, part: bytes) -> bool` | Checks whether bytes contain `part`. |

## `std.format`

| API | Description |
| --- | --- |
| `format.number(value: float, decimals: int = 0) -> text` | Formats a number with a fixed decimal count. |
| `format.percent(value: float, decimals: int = 0) -> text` | Formats a number as a percentage. |
| `format.date(millis: int, style: text = "iso") -> text` | Formats a millisecond timestamp as a date. |
| `format.datetime(millis: int, style: text = "short", locale: text = "") -> text` | Formats a millisecond timestamp as date and time. |
| `format.date_pattern(millis: int, pattern: text) -> text` | Formats a date using an explicit pattern. |
| `format.datetime_pattern(millis: int, pattern: text) -> text` | Formats date and time using an explicit pattern. |
| `format.bytes(value: int, style: format.BytesStyle = format.BytesStyle.Binary, decimals: int = 1) -> text` | Formats a byte count using binary or decimal units. |
| `format.hex(value: int) -> text` | Formats an integer as hexadecimal text. |
| `format.bin(value: int) -> text` | Formats an integer as binary text. |

## `format.BytesStyle`

| Variant | Description |
| --- | --- |
| `Binary` | Uses binary units, such as KiB and MiB. |
| `Decimal` | Uses decimal units, such as KB and MB. |
