# Math, Regex, Random and Validate Reference

> Surface: reference
> Status: current

## `std.math`

Constants:

| API | Description |
| --- | --- |
| `math.pi: float` | Pi constant. |
| `math.e: float` | Euler number constant. |
| `math.tau: float` | Tau constant, equal to `2 * pi`. |

Functions:

| API | Description |
| --- | --- |
| `math.infinity() -> float` | Returns positive infinity. This remains a function because the current constant model has no safe non-finite float literal. |
| `math.nan() -> float` | Returns a NaN floating-point value. This remains a function because the current constant model has no safe non-finite float literal. |
| `math.abs(value: float) -> float` | Returns the absolute value. |
| `math.min(a: float, b: float) -> float` | Returns the smaller value. |
| `math.max(a: float, b: float) -> float` | Returns the larger value. |
| `math.clamp(value: float, min: float, max: float) -> float` | Restricts a value to the inclusive range. |
| `math.pow(base: float, exponent: float) -> float` | Raises `base` to `exponent`. |
| `math.sqrt(value: float) -> float` | Returns the square root. |
| `math.floor(value: float) -> float` | Rounds down. |
| `math.ceil(value: float) -> float` | Rounds up. |
| `math.round(value: float) -> float` | Rounds to the nearest integer value. |
| `math.trunc(value: float) -> float` | Removes the fractional part. |
| `math.deg_to_rad(x: float) -> float` | Converts degrees to radians. |
| `math.rad_to_deg(x: float) -> float` | Converts radians to degrees. |
| `math.approx_equal(a: float, b: float, epsilon: float) -> bool` | Compares floats with tolerance. |
| `math.sin(value: float) -> float` | Returns sine. |
| `math.cos(value: float) -> float` | Returns cosine. |
| `math.tan(value: float) -> float` | Returns tangent. |
| `math.asin(value: float) -> float` | Returns arc sine. |
| `math.acos(value: float) -> float` | Returns arc cosine. |
| `math.atan(value: float) -> float` | Returns arc tangent. |
| `math.atan2(y: float, x: float) -> float` | Returns arc tangent using `y` and `x`. |
| `math.ln(value: float) -> float` | Returns natural logarithm. |
| `math.log_ten(value: float) -> float` | Returns base-10 logarithm. |
| `math.log2(value: float) -> float` | Returns base-2 logarithm. |
| `math.log(value: float, base: float) -> float` | Returns logarithm in an explicit base. |
| `math.exp(value: float) -> float` | Returns `e` raised to `value`. |
| `math.is_nan(value: float) -> bool` | Checks whether a value is NaN. |
| `math.is_infinite(value: float) -> bool` | Checks whether a value is infinite. |
| `math.is_finite(value: float) -> bool` | Checks whether a value is finite. |

## `std.regex`

Types:

| API | Description |
| --- | --- |
| `regex.Regex` | Validated pattern wrapper. |
| `regex.Error.InvalidPattern` | Returned by `compile` for invalid patterns. |

Functions:

| API | Description |
| --- | --- |
| `regex.compile(pattern: text) -> result<regex.Regex, regex.Error>` | Validates a pattern. |
| `regex.is_valid(pattern: text) -> bool` | Checks whether a pattern is valid. |
| `regex.is_match(pattern: text, input: text) -> bool` | Checks whether the pattern matches at least once. |
| `regex.contains(pattern: text, input: text) -> bool` | Alias for `is_match`. |
| `regex.matches(pattern: text, input: text) -> result<bool, regex.Error>` | Match helper with explicit invalid-pattern errors. |
| `regex.full_match(pattern: text, input: text) -> result<bool, regex.Error>` | Checks whether the pattern covers the whole input. |
| `regex.first(pattern: text, input: text) -> optional<text>` | Returns the first match. |
| `regex.count(pattern: text, input: text) -> int` | Counts non-empty matches. |
| `regex.find_all(pattern: text, input: text) -> list<text>` | Returns all non-empty matches. |
| `regex.split(pattern: text, input: text) -> list<text>` | Splits input by non-empty matches. |
| `regex.replace_all(pattern: text, input: text, replacement: text) -> text` | Replaces all non-empty matches. |
| `regex.escape(input: text) -> text` | Escapes basic regex metacharacters. |

Current pattern support is intentionally small: literals, `.`, `^`, `$`, `*`, `+`, `?`, classes/ranges and `\d`, `\w`, `\s`.
Groups, captures, flags and full Unicode regex are deferred.

## `std.random`

State:

| API | Description |
| --- | --- |
| `random.seeded: bool` | Public read-only-from-outside flag showing whether `seed` was called. |
| `random.last_seed: int` | Public read-only-from-outside value with the last seed. |
| `random.draw_count: int` | Public read-only-from-outside count of generated draws. |

Functions:

| API | Description |
| --- | --- |
| `random.seed(seed: int) -> void` | Sets the random seed and resets draw tracking. |
| `random.next() -> int` | Returns the next pseudo-random integer. |
| `random.between(min: int, max: int) -> result<int, text>` | Returns a pseudo-random integer in the inclusive range. |
| `random.stats() -> random.Stats` | Returns a snapshot of the random state. |

Notes:

- `std.random` uses namespace state in this alpha.
- External namespaces may read public vars but cannot write them.
- Prefer `seed(...)` for controlled mutation.

## `std.validate`

| API | Description |
| --- | --- |
| `validate.between(value: int, min: int, max: int) -> bool` | Checks whether `value` is inside the inclusive range. |
| `validate.positive(value: int) -> bool` | Checks whether `value` is greater than zero. |
| `validate.non_negative(value: int) -> bool` | Checks whether `value` is zero or greater. |
| `validate.negative(value: int) -> bool` | Checks whether `value` is below zero. |
| `validate.non_zero(value: int) -> bool` | Checks whether `value` is not zero. |
| `validate.one_of(value: int, candidates: list<int>) -> bool` | Checks whether an integer appears in a candidate list. |
| `validate.one_of_text(value: text, candidates: list<text>) -> bool` | Checks whether text appears in a candidate list. |
| `validate.not_empty(value: text) -> bool` | Checks whether text is not empty. |
| `validate.not_empty_text(value: text) -> bool` | Clear text-specific alias for `not_empty`. |
| `validate.min_length(value: text, min: int) -> bool` | Checks whether text has at least `min` characters. |
| `validate.min_len(value: text, min: int) -> bool` | Short alias for `min_length`. |
| `validate.max_length(value: text, max: int) -> bool` | Checks whether text has at most `max` characters. |
| `validate.max_len(value: text, max: int) -> bool` | Short alias for `max_length`. |
| `validate.length_between(value: text, min: int, max: int) -> bool` | Checks whether text length is inside an inclusive range. |

Additional executable helpers:

| API | Description |
| --- | --- |
| `validate.between_float(value: float, min: float, max: float) -> bool` | Inclusive float range check. |
| `validate.positive_float(value: float) -> bool` | Checks whether a float is greater than zero. |
| `validate.non_negative_float(value: float) -> bool` | Checks whether a float is zero or greater. |
| `validate.negative_float(value: float) -> bool` | Checks whether a float is below zero. |
| `validate.non_zero_float(value: float) -> bool` | Checks whether a float is not zero. |
| `validate.one_of_float(value: float, candidates: list<float>) -> bool` | Checks whether a float appears in a candidate list. |
| `validate.is_true(value: bool) -> bool` | Returns the bool value as a predicate. |
| `validate.is_false(value: bool) -> bool` | Checks whether a bool is false. |
| `validate.one_of_bool(value: bool, candidates: list<bool>) -> bool` | Checks whether a bool appears in a candidate list. |
| `validate.is_some_int/is_none_int` | Checks `optional<int>` state. |
| `validate.is_some_float/is_none_float` | Checks `optional<float>` state. |
| `validate.is_some_bool/is_none_bool` | Checks `optional<bool>` state. |
| `validate.is_success_int_text/is_error_int_text` | Checks `result<int, text>` state. |
| `validate.*_list_int`, `*_list_text`, `*_list_float`, `*_list_bool` | Non-empty, min, max, and inclusive length checks for supported list shapes. |
| `validate.*_map_text_int`, `*_map_text_text`, `*_map_int_text`, `*_map_int_int` | Non-empty, min, max, and inclusive size checks for supported map shapes. |

Notes:

- `std.validate` stays pure: no IO, parsing, sanitizing, or domain validators.
- Fully generic public helpers such as `validate.not_empty_list<T>` are deferred until public stdlib generics are stable through import and C emission.
