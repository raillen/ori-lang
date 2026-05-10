# Decision 047 - C Target And Interop Boundary

- Status: accepted
- Date: 2026-04-17
- Type: language / backend / interop
- Scope: C backend, runtime C, user-facing C interop, host/runtime boundary, unsafe future

## Summary

Zenith compiles to C as its first platform target.

That does not mean user-facing C interop belongs in the MVP.

The MVP should keep C as an implementation/backend detail and expose safe Zenith APIs to users.

Open C interop is deferred to post-MVP and must be explicitly unsafe if introduced.

## Decision

Normative rules for this cut:

- C is the first backend target
- runtime C is part of the implementation strategy
- generated C is not the user-facing language surface
- user-facing `extern c` is not part of the MVP
- users should not be encouraged to call C directly in ordinary Zenith code
- C interop needed by the runtime/stdlib should stay internal
- `extern host` may exist as an internal or experimental runtime/tooling mechanism
- public C interop is post-MVP
- public C interop, if added later, must be explicitly unsafe or otherwise strongly marked
- safe Zenith wrappers should be the preferred way to expose native capabilities
- C nullable values must not introduce `null` into Zenith
- future C nullable interop must map into `optional<T>`

## Backend C Versus C Interop

Backend C means:

```text
.zt -> compiler -> C -> native executable
```

The user writes Zenith.

The compiler emits C.

The C compiler produces a platform executable.

C interop means the user writes declarations that call external C APIs.

Example future/non-MVP direction:

```zt
unsafe extern c func c_strlen(value: c.string) -> u64
```

These are different concerns.

The MVP needs backend C.

The MVP does not need open user-facing C interop.

## Why User-Facing C Interop Is Deferred

C interop introduces problems that conflict with Zenith's accessibility and safety goals:

- null pointers
- manual ownership
- dangling pointers
- buffer overflows
- ABI differences
- platform-specific behavior
- C string encoding issues
- unclear lifetime rules
- unsafe casts
- difficult diagnostics
- build-system complexity
- crashes outside Zenith's normal error model

Exposing this too early would make ordinary Zenith code less predictable.

## Runtime And Stdlib Boundary

The runtime and future stdlib may use C internally.

That does not require exposing C to users.

Preferred model:

```zt
const text: text = file.read_all(path)?
```

instead of user-facing C calls.

Runtime/stdlib code can wrap platform details and return safe Zenith types such as:

- `text`
- `optional<T>`
- `result<T, E>`
- managed handles
- domain-specific structs/enums

## Host Boundary

A host/runtime boundary may exist for tooling, embedding or runtime services.

Example conceptual direction:

```zt
extern host func log(message: text)
```

However, this is not ordinary C ABI interop.

`extern host` can speak Zenith runtime types and can be controlled by the compiler/runtime.

It may remain internal or experimental until the host API is designed.

## Future C Interop Direction

If public C interop is added after the MVP, it should have a separate decision.

Likely requirements:

- explicit unsafe marker
- restricted C-compatible types
- explicit ownership annotations
- explicit nullable mapping to `optional<T>`
- no implicit `text` to C string conversion
- no implicit C pointer to Zenith value conversion
- clear target/platform configuration
- strong diagnostics
- wrapper-first guidance

Possible future direction:

```zt
unsafe extern c func c_getenv(name: c.string) -> optional<c.string>
```

Safe wrapper:

```zt
func getenv(name: text) -> optional<text>
    ...
end
```

The wrapper is what ordinary users should call.

## Non-Canonical Forms

User-facing C declarations in MVP are not canonical:

```zt
extern c func malloc(size: u64) -> c.ptr<u8>
```

Using C null directly is never canonical:

```zt
const ptr = null
```

Passing Zenith managed values directly through C ABI without explicit interop rules is not canonical:

```zt
extern c func puts(message: text) -> int32
```

Encouraging users to solve ordinary problems through C imports is not aligned with the language philosophy.

## Diagnostics

Expected diagnostic direction if user-facing C interop syntax appears before support:

```text
error[feature.unsupported]
User-facing extern c is not supported in the MVP.

help
  Use Zenith stdlib/runtime APIs or wait for the post-MVP unsafe C interop design.
```

## Rationale

C is a practical backend target, but it should not leak into the safe Zenith language experience.

Keeping C interop out of the MVP protects the language from premature unsafe complexity.

A safe stdlib can expose platform capabilities without requiring users to reason about pointers, nullability and manual ownership.

Deferring public C interop allows it to be designed with the right warnings, syntax, ownership rules and tooling.

## Out of Scope

This decision does not yet define:

- unsafe syntax
- C ABI type mapping
- C pointer types
- C string types
- C header integration
- linker configuration for external C libraries
- ownership annotations for C APIs
- host API details
- FFI package model
