# Decision 066 - Stdlib Json Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / json
- Scope: typed JSON value model, parse/serialize, file convenience, inspection, conversion and numeric representation

## Summary

`std.json` uses its own JSON-specific types instead of exposing raw `map` and `list` as the public surface.

The MVP includes parse, serialization, file convenience, inspection and typed conversion helpers.

JSON remains strict in this cut and does not accept comments, trailing commas or non-standard extensions.

## Decision

Canonical import:

```zt
import std.json as json
```

Accepted principal types:

- `json.Value`
- `json.Object`
- `json.Array`
- `json.Kind`
- `json.Error`

Accepted first-wave API direction:

- `json.parse(text: text) -> result<json.Value, json.Error>`
- `json.stringify(value: json.Value) -> text`
- `json.pretty(value: json.Value, indent: int = 2) -> text`
- `json.read(path: text) -> result<json.Value, json.Error>`
- `json.write(path: text, value: json.Value) -> result<void, json.Error>`
- `json.kind(value: json.Value) -> json.Kind`
- `json.has_key(obj: json.Object, key: text) -> bool`
- `json.get_key(obj: json.Object, key: text) -> optional<json.Value>`
- `json.as_text(value: json.Value) -> optional<text>`
- `json.as_int(value: json.Value) -> optional<int>`
- `json.as_float(value: json.Value) -> optional<float>`
- `json.as_bool(value: json.Value) -> optional<bool>`
- `json.as_array(value: json.Value) -> optional<json.Array>`
- `json.as_object(value: json.Value) -> optional<json.Object>`

Deferred after this first wave:

- `json.set(...)`
- `json.remove_key(...)`
- `json.append(...)`
- `json.merge(...)`
- `json.get_path(...)`

Examples:

```zt
import std.json as json

const root: json.Value = json.parse(source)?
const obj: json.Object = json.as_object(root) or return error(config_error)

if json.has_key(obj, "name")
    const name_value: json.Value = json.get_key(obj, "name") or return error(config_error)
    const name: text = json.as_text(name_value) or return error(config_error)
end

const pretty_text: text = json.pretty(root, indent: 2)
json.write("config.json", root)?
```

Normative rules for this cut:

- `std.json` uses JSON-specific public types rather than exposing raw `map<text, ...>` and `list<...>` as the module surface
- `json.read(...)` and `json.write(...)` are accepted as JSON-specific file conveniences in the MVP
- `json.Error` must be able to represent at least parse failures and file-IO failures
- JSON parsing is strict in this cut
- comments are not accepted in input JSON
- trailing commas are not accepted in input JSON
- type-mismatch conversion helpers such as `as_text(...)` return `optional<T>`, not `result<T, json.Error>`
- `json.has_key(...)` and `json.get_key(...)` operate on `json.Object`, not on arbitrary `json.Value`
- the public conversion name is `as_text`, not `as_string`

## Json Kind Direction

`json.Kind` is accepted as the public discriminator returned by `json.kind(...)`.

The accepted public direction is a JSON-shaped kind model with a single numeric family rather than separate public `int` and `float` kind names.

The intended public cases include the usual JSON concepts:

- object
- array
- text
- number
- bool
- null

The exact enum spelling is implementation work, but the single public `number` family is accepted.

## Numeric Representation

Internally, `json.Value` distinguishes integer-like and floating-point numeric representations.

This distinction exists to avoid collapsing all JSON numbers into `float` and silently losing fidelity for large integral values.

Accepted numeric direction for this cut:

- public `json.Kind` stays at the JSON level with a single `number` family
- internal numeric representation may distinguish integer and floating-point forms
- `json.as_int(...)` only succeeds when the numeric value is representable as a Zenith `int` without implicit rounding or truncation
- `json.as_float(...)` converts numeric JSON values to Zenith `float` when representable

A separate public `json.Number` wrapper is deferred.

## File Convenience

`json.read(...)` and `json.write(...)` are accepted because JSON is a common file-backed configuration format and the module now has a proper typed core.

These functions are conveniences over explicit `fs.read_text(...) + json.parse(...)` and `json.stringify(...) + fs.write_text(...)` flows.

The existence of these convenience functions does not change the boundary that `std.fs` remains the general filesystem module.

## Boundary With Collections

`json.Object` and `json.Array` are JSON-specific types even if their runtime implementation internally reuses map-like or list-like structures.

This preserves a clean JSON boundary and keeps future JSON-specific helpers coherent.

## Rationale

Using dedicated JSON types keeps the API semantically honest and prevents JSON handling from dissolving into generic collection manipulation.

Allowing `json.read(...)` and `json.write(...)` in the MVP improves ergonomics for configuration-heavy programs without making `std.json` the general filesystem layer.

Returning `optional<T>` from `as_*` helpers keeps ordinary type mismatch separate from structural parse failure.

Separating internal integer and floating-point numeric representation avoids precision loss that would occur if every number were forced into `float`.

## Non-Canonical Forms

Treating public JSON values as raw generic collections only:

```zt
const obj: map<text, any> = ...
```

Using `as_string(...)` instead of Zenith `text` vocabulary.

Making every JSON number a `float` in the public or internal model.

Growing the MVP around a textual mini-language for deep lookup:

```zt
json.get_path(root, "user.addresses[0].street")
```

## Out of Scope

This decision does not yet define:

- mutation helpers such as `set`, `remove_key` and `append`
- deep-path query syntax
- whether serialization preserves original textual number spelling
- the exact public field and method surface of `json.Object` and `json.Array`
- schema validation
- JSON5-style or comment-tolerant parsing

