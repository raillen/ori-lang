# Decision 089 - Callable Delegates v1

- Status: accepted
- Date: 2026-04-24
- Type: language / semantics / codegen / FFI
- Scope: first-class function references, `func(T,...) -> R` type, signature
  compatibility, escape rules, FFI callback interop

## Summary

Zenith gains a first-class callable type `func(T1, ..., Tn) -> R` and
allows top-level named functions to be used as values of that type.

This is the R3.M5 Phase 1 ("Callables v1") surface. Closures with captures
are out of scope here and remain on the R3.M6 track.

## Motivation

- Unblock R3.M2 Phase 3 (`jobs.spawn(func, arg)` / `jobs.join`) without
  inventing an ad-hoc registry or synthetic jobs surface.
- Provide a surface for strategy-style callbacks (sort, map, FFI callbacks)
  that is narrow, explicit, and analyzable.
- Keep ownership and escape reasoning simple: a delegate produced by a
  top-level `func` has no captured state, so it has no lifetime coupling
  to any object.

## Surface Syntax

A callable type expression:

```zt
func(int, text) -> bool
func() -> int
func(int)
```

Rules:

- `func(...)` with no `-> R` defaults to `void` return.
- Parameter list is a comma-separated list of type expressions.
- Named parameters and default values are NOT part of the callable type
  (a callable loses argument names).
- Type parameters are NOT allowed in callable types in this cut
  (generic delegates are deferred).

A value-position function reference:

```zt
func add(a: int, b: int) -> int
    return a + b
end

const op: func(int, int) -> int = add
```

Rules:

- An identifier that resolves to a top-level `func` declaration can
  appear in value position when a callable type is expected.
- The identifier must refer to a plain (non-generic, non-method) top-level
  function.
- The identifier must be visible (normal visibility rules for `public`
  apply).

Calling through a callable-typed variable uses the standard call syntax:

```zt
const r = op(40, 2)
```

## Signature Compatibility

Two callable types are compatible iff:

- they have the same parameter count
- each parameter type pair is `zt_type_equals`-compatible
- the return types are `zt_type_equals`-compatible

No variance. No coercion. Mismatches emit `callable.signature_mismatch`
with ACTION/WHY/NEXT.

## Escape and Ownership (v1)

Because v1 delegates carry no captured state, ownership is simple.

Allowed positions:

- local `let`/`var` declaration
- function parameter
- function return value
- argument to another callable or regular call
- argument to `extern c` with a matching function-pointer parameter

Rejected positions (Phase 1 cut, may be relaxed later):

- `public var` at namespace scope (diagnostic `callable.escape_public_var`)
- struct field (diagnostic `callable.escape_struct_field`)
- collection element types: `list<func...>`, `map<K, func...>`,
  `optional<func...>`, `result<func..., E>`, etc.
  (diagnostic `callable.escape_container`)

Rationale: v1 delegates are effectively function pointers. A later phase
with closures must extend escape analysis; Phase 1 pins the invariant
that a delegate never outlives a heap cell it does not own, which is
trivial when it is a raw function pointer.

## FFI Integration

This decision lifts the MVP restriction from `Decision 011` that
"callbacks in extern declarations are not part of the MVP", but only for
the narrow v1 surface.

Permitted:

- an `extern c` function may declare a parameter of callable type, e.g.:

```zt
extern c
    func qsort(base: bytes, count: int, size: int, cmp: func(int, int) -> int)
end
```

- passing a v1 delegate (top-level function reference) to such a call is
  allowed.

Rejected (diagnostic `callable.extern_c_signature`):

- callable parameter with non-primitive, non-text, non-bytes parameter
  or return types (structs, enums, optional, result, list, map, dyn).
  Allowed boundary shapes: `int`, `int32`, `int64`, `float`, `float64`,
  `bool`, `void` return, `text` (by pointer), `bytes` (by pointer).
- callable return type in an `extern c` declaration (returning a
  function pointer from C into Zenith is deferred).
- callable inside a struct-typed `extern c` argument.

Rationale: the permitted shapes map cleanly to C function pointers with
primitive signatures and avoid exposing ARC or managed-memory contracts
across the FFI boundary.

## Codegen Model

At the C emitter:

- each distinct callable signature used in the program emits one typedef
  of the form
  `typedef <ret> (*zt_callable__<ret>__<p1>__<p2>__...)(<args>);`
  where the mangling uses short fixed tokens (`i64`, `f64`, `bool`,
  `text`, `bytes`, `void`, `u_<name>`).
- a function reference value used as a Zenith callable is emitted as an immortal closure value.
- a top-level function reference passed to an `extern c` callback parameter is emitted as a raw C function pointer to a generated static trampoline (`<mangled_func_name>__ffi_cb`) that immediately calls the Zenith function with a null runtime context.
- an indirect call is emitted as `callee(args...)` where `callee` is
  the callable-typed expression.

This avoids any per-call vtable and preserves the "zero-overhead when
you do not opt in" property.

## Runtime Model

A Zenith callable value uses the closure representation already used by the C backend. At the `extern c` callback boundary, the emitter exposes a raw C function pointer through a generated trampoline so C code sees the exact declared function-pointer signature.

## Out Of Scope For v1

- closures with captures (R3.M6)
- lambda literals (R3.M7)
- generic delegates `func<T>(T) -> T`
- variadic delegates
- partial application / method values
- returning function pointers from `extern c`
- callable as struct field / container element (escape v2)

## Consequences

- `func(T,...) -> R` becomes a legal type expression wherever a type is
  expected, except as a struct field, public var, or container element
  (rejected explicitly).
- A top-level function name used as a value resolves to a callable.
- `extern c` can declare function-pointer parameters for primitive
  shapes; passing a Zenith delegate to such a parameter is safe.
- R3.M2 Phase 3 (`jobs.spawn/join/cancel`) can be built on top of this
  surface.

## Related

- `docs/internal/decisions/language/011-extern-c-and-extern-host.md` (relaxed for v1
  callback shapes)
- `docs/internal/decisions/language/087-concurrency-workers-and-transfer-boundaries.md`
  (Phase 3 consumer)
- `docs/spec/language/callables.md` (authoritative surface + phases)
- `docs/internal/planning/checklist-v3.md` (`R3.M5`)
