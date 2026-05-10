# Decision 063 - Stdlib Time Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / time
- Scope: instant and duration model, system clock, unix conversion, sleep, temporal arithmetic and MVP boundaries

## Summary

`std.time` starts as a small temporal module centered on `Instant` and `Duration`.

The MVP uses `time.now()` as the system-clock entrypoint and keeps formatting and parsing outside this module.

Comparison uses ordinary operators instead of helper functions such as `is_before(...)`.

## Decision

Canonical import:

```zt
import std.time as time
```

Accepted principal types:

- `time.Instant`
- `time.Duration`
- `time.Error`

Accepted first-wave API direction:

- `time.now() -> Instant`
- `time.sleep(duration: Duration) -> result<void, time.Error>`
- `time.since(start: Instant) -> Duration`
- `time.until(end: Instant) -> Duration`
- `time.diff(a: Instant, b: Instant) -> Duration`
- `time.add(time: Instant, duration: Duration) -> Instant`
- `time.sub(time: Instant, duration: Duration) -> Instant`
- `time.from_unix(ts: int64) -> Instant`
- `time.from_unix_ms(ts: int64) -> Instant`
- `time.to_unix(time: Instant) -> int64`
- `time.to_unix_ms(time: Instant) -> int64`
- `time.seconds(n: int) -> Duration`
- `time.milliseconds(n: int) -> Duration`
- `time.minutes(n: int) -> Duration`
- `time.hours(n: int) -> Duration`

Examples:

```zt
const start: time.Instant = time.now()
const timeout: time.Duration = time.seconds(5)

time.sleep(timeout)?

const elapsed: time.Duration = time.since(start)
const later: time.Instant = time.add(start, time.minutes(1))

const unix_s: int64 = time.to_unix(start)
const unix_ms: int64 = time.to_unix_ms(start)

const restored: time.Instant = time.from_unix(unix_s)
const restored_ms: time.Instant = time.from_unix_ms(unix_ms)
```

Normative rules for this cut:

- `std.time` starts with `Instant` and `Duration` as the core temporal model
- `time.now()` uses the system clock
- the meaning of `time.now()` is not configurable by project flag or global setting
- if Zenith later exposes a monotonic or steady clock, it must use an explicit separate API
- unix timestamps use `int64` in both second and millisecond forms
- helper shortcuts such as `time.unix()` and `time.unix_ms()` are not part of the accepted surface
- `sleep(...)` returns `result<void, time.Error>`
- comparison should use ordinary operators such as `==`, `<` and `>` where supported
- helper functions such as `is_before(...)`, `is_after(...)` and `equal(...)` are not part of this cut
- formatting is not part of `std.time` in this cut
- parsing is not part of `std.time` in this cut

## Semantic Direction

Accepted semantic direction for the MVP:

- `Instant` represents an absolute moment suitable for system-clock operations and Unix conversion
- `Duration` represents elapsed or relative time
- `since(start)` is a convenience form around the current moment and the given start instant
- `until(end)` is a convenience form around the current moment and the given end instant
- `diff(a, b)` expresses the temporal distance between two instants

The exact runtime representation is implementation work, but the semantic roles above are part of the accepted design.

## Boundaries

The following capabilities are intentionally kept out of the MVP surface of `std.time`:

- calendar-focused types such as `Date`, `ClockTime` and `DateTime`
- formatting helpers such as `time.format(...)`
- parsing helpers such as `time.parse(...)`
- timezone handling
- configurable clock modes inside `time.now()`

Formatting remains the responsibility of `std.format`.

Calendar parsing remains deferred until the language defines a broader calendar model.

## Rationale

Keeping `std.time` centered on `Instant` and `Duration` gives Zenith a small and understandable MVP without immediately dragging in calendar complexity.

Using the system clock for `time.now()` keeps the first version straightforward and makes Unix conversion coherent.

Rejecting a configurable meaning for `now()` preserves readability and prevents APIs such as `to_unix(now())` from depending on hidden project configuration.

Keeping formatting and parsing outside `std.time` preserves clean boundaries between time representation and presentation.

Using ordinary comparison operators is shorter and clearer than proliferating helper predicates for simple temporal comparisons.

## Non-Canonical Forms

Configuring the meaning of `now()` through a project flag or global setting.

Adding helper shortcuts that are only aliases for `to_unix(now())`:

```zt
time.unix()
time.unix_ms()
```

Using helper predicates instead of operators for ordinary comparison:

```zt
time.is_before(a, b)
time.is_after(a, b)
time.equal(a, b)
```

Putting formatting and parsing directly in the first-wave `std.time` surface.

## Out of Scope

This decision does not yet define:

- monotonic or steady-clock APIs
- timezone-aware temporal types
- calendar types such as `Date` and `DateTime`
- temporal formatting syntax
- temporal parsing syntax
- the exact negative-duration policy for `sleep(...)`
- the exact internal representation of `Instant` and `Duration`
