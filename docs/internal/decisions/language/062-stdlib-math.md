# Decision 062 - Stdlib Math Module

- Status: accepted
- Date: 2026-04-17
- Type: stdlib / math
- Scope: numeric math surface, principal module, future submodules, float semantics, trigonometry, logarithms, exponentials, rounding and constants

## Summary

`std.math` is the pure numeric mathematics module.

The principal module stays small and predictable, but it includes the standard float-domain trigonometric, logarithmic and exponential helpers that users expect from a core math library.

Interpolation helpers, integer/discrete helpers and randomness are separated into future submodules instead of inflating the main `std.math` surface.

## Decision

Canonical import:

```zt
import std.math as math
```

Accepted current principal-module direction:

- `abs`
- `min`
- `max`
- `clamp`
- `pow`
- `sqrt`
- `floor`
- `ceil`
- `round`
- `trunc`
- `deg_to_rad`
- `rad_to_deg`
- `approx_equal`
- `sin`
- `cos`
- `tan`
- `asin`
- `acos`
- `atan`
- `atan2`
- `ln`
- `log10`
- `log2`
- `log`
- `exp`
- `is_nan`
- `is_infinite`
- `is_finite`
- constants: `pi`, `e`, `tau`
- constants considered acceptable in lowercase form when exposed: `infinity`, `nan`

Examples:

```zt
const angle: float = math.deg_to_rad(90.0)
const okay: bool = math.approx_equal(a, b, epsilon: 0.0001)

const clamped: float = math.clamp(value, min: 0.0, max: 1.0)
const root: float = math.sqrt(25.0)
const rounded: float = math.round(-2.5)

const s: float = math.sin(angle)
const p: float = math.exp(2.0)
const bits: float = math.log2(1024.0)
const digits: float = math.log10(1000.0)
const base3: float = math.log(81.0, base: 3.0)
```

Normative rules for this cut:

- `std.math` is pure and deterministic
- the principal `std.math` module is numeric only
- `min`, `max` and `clamp` are numeric only in the MVP direction
- `pow`, `sqrt`, `floor`, `ceil`, `round` and `trunc` operate on `float`
- trigonometric, logarithmic and exponential helpers in this cut operate on `float`
- when integer output is desired after float-domain functions, the user must convert explicitly
- `round` uses `half away from zero` semantics
- trigonometric angles are expressed in radians
- `deg_to_rad` and `rad_to_deg` are the canonical degree-conversion helpers
- float-domain math functions do not return `result` in this cut
- invalid numeric domains are represented through ordinary float semantics such as `nan` and `infinity`
- `is_nan`, `is_infinite` and `is_finite` are the intended inspection helpers for those outcomes
- `random` is not part of the principal `std.math` module
- collection aggregations such as `mean`, `median`, `sum` and `product` are deferred
- `EPSILON` is not standardized as a generic uppercase constant in this cut

## Float-Domain Functions

Accepted float-domain direction:

```zt
math.sin(x)
math.cos(x)
math.tan(x)
math.asin(x)
math.acos(x)
math.atan(x)
math.atan2(y, x)

math.ln(x)
math.log10(x)
math.log2(x)
math.log(x, base: 2.0)
math.exp(x)
```

Notes:

- `atan2(y, x)` is accepted because quadrant-aware angle calculation is common enough to belong in the principal module
- `log(value, base: ...)` is accepted in addition to `ln`, `log10` and `log2`
- `log(...)` uses an explicit `base:` parameter instead of introducing multiple ad-hoc names for every base family

## Domain And Special-Value Semantics

Accepted semantic direction for invalid or boundary float domains:

- `sqrt(-1.0)` yields `nan`
- `asin(2.0)` yields `nan`
- `acos(2.0)` yields `nan`
- `ln(0.0)` yields negative infinity according to float semantics
- `ln(-1.0)` yields `nan`

These cases do not become typed `math.Error` results in this cut.

The accepted model is ordinary floating-point behavior plus explicit inspection helpers such as `math.is_nan(...)`.

## Principal Module vs Future Submodules

Accepted organization direction:

### Principal `std.math`

Keep the broadly useful numeric core here.

This includes scalar arithmetic helpers, scalar float-domain math, conversion helpers, special-value inspection and constants.

### Future `std.math.interpolation`

Recommended future home for:

- `lerp`
- `inverse_lerp`
- `remap`
- `smootherstep`

### Future `std.math.integer`

Recommended future home for:

- `gcd`
- `lcm`
- `is_power_of_two`
- `next_power_of_two`

### Future `std.math.random`

Randomness is accepted as a future submodule, not as part of the principal `std.math` API.

### Future range or wave-oriented submodule

Recommended future home for:

- `wrap`
- `ping_pong`

The exact final submodule name remains open, but these helpers should stay out of the principal module.

## Deferred Or Ambiguous Names

The following names are intentionally not accepted into the principal module now:

- `normalize`
- `saturate`

Rationale:

- `normalize` is ambiguous without additional context
- `saturate` is common in graphics/shader domains but is less transparent for the general Zenith audience
- the simple `0..1` use case remains expressible with `math.clamp(value, min: 0.0, max: 1.0)`

## Constants

Accepted principal constants:

- `math.pi`
- `math.e`
- `math.tau`

Current A.5 implementation decision:

- `math.infinity()` remains a function
- `math.nan()` remains a function
- both use explicit runtime helpers instead of division-by-zero construction

Reason: the current constant model supports finite float literals, but has no true non-finite float literal. Exposing `math.infinity` or `math.nan` as constants would make the public surface imply compile-time support the language does not have yet.

Accepted future naming direction if true non-finite float constants are added:

- `math.infinity`
- `math.nan`

Uppercase forms such as `INF`, `NaN` and `EPSILON` are not the preferred Zenith surface.

`epsilon` remains deferred because its meaning is too ambiguous without fixing the exact numeric model and naming intent first.

## Rationale

A small principal `std.math` module keeps the common path clear and aligns with Zenith readability goals.

Including trigonometry, logarithms and exponentials in the principal module is justified because they are standard scalar math operations, not domain-specific extensions.

Separating interpolation, integer/discrete helpers and randomness improves cohesion and keeps the core math API easy to scan.

Requiring explicit conversion from `float` to `int` preserves type clarity and avoids hidden narrowing.

Using ordinary float-domain `nan` and `infinity` behavior avoids turning common math into result-heavy control flow.

Choosing lowercase constant names maintains visual consistency with the rest of the language.

## Non-Canonical Forms

Putting random generation directly into the principal module:

```zt
math.random()
```

Making `min`, `max` and `clamp` fully generic over every comparable type in this module.

Returning `int` directly from float-domain functions such as `floor` or `round`.

Using uppercase numeric constants as the preferred public Zenith surface.

Turning invalid float domains into typed `result<..., math.Error>` for ordinary scalar math.

## Out of Scope

This decision does not yet define:

- exact overload set per numeric width
- vector or matrix math
- statistics helpers
- decimal arbitrary-precision math
- complex numbers
- the final API of `std.math.random`
- the final API of interpolation and integer submodules
- hyperbolic functions
