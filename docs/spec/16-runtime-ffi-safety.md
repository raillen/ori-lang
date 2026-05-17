# Runtime FFI safety contracts

> Audience: runtime maintainer, backend maintainer
> Status: current
> Surface: native runtime ABI

This file documents the shared safety rules for `unsafe extern "C"` runtime
functions. It is the domain-level contract used while `ori-runtime/src/lib.rs`
is still being split into smaller modules.

## General ABI rules

- All raw pointers passed to runtime functions must be either null when the
  function explicitly accepts null, or valid for the full operation.
- Pointers returned by allocation functions are owned by the caller according to
  the ARC contract in `10-memory.md`.
- Managed values stored inside another managed value must register an ARC edge.
- Removing or replacing managed children must unregister the old ARC edge.
- Runtime functions must not keep borrowed C pointers after the call returns
  unless the API explicitly copies the payload first.

## String functions

Ori strings currently use a nul-terminated UTF-8 representation.

- `*const c_char` string inputs must point to valid nul-terminated UTF-8.
- Functions that create strings allocate a new managed string payload.
- Functions that return borrowed internal string pointers keep ownership in the
  source object. Callers must not free those pointers directly.
- String length, slice, and index APIs use character positions, not byte
  offsets, unless the function name states that it works on bytes.
- Inputs containing interior NUL are not valid Ori strings. Use `bytes` APIs for
  binary payloads.

## Bytes functions

Ori bytes are length-aware binary payloads.

- Bytes APIs must preserve `0x00` and must not use `CStr` to compute payload
  length.
- Inputs are valid when the data pointer is non-null for `len > 0`.
- A null data pointer is valid only when `len == 0`.
- UTF-8 decoding into `string` must reject interior NUL while strings remain
  nul-terminated.
- File APIs that read or write bytes must use the explicit bytes length.

## Collection functions

Collections own ARC edges to managed children.

- Insert paths must register managed children with the collection as owner.
- Remove, pop, clear, and replacement paths must unregister removed children.
- Map entries register both managed keys and managed values.
- Tree and graph node storage register managed node payloads in the runtime,
  not in backend-specific code.
- Iterator and snapshot APIs that expose managed values must retain or preserve
  ownership according to the returned value contract.

## Heap comparator functions

Custom heap comparison calls may retain managed values temporarily before
calling user code.

- Each temporary retain must have a matching release after the comparison.
- Comparator failures must not skip release cleanup.
- Repeated comparisons must not increase the refcount of heap items.

## Source-level rustdoc policy

Critical ARC and memory functions should keep local `# Safety` rustdoc near the
function. For broad FFI families, this file is the current shared contract until
runtime modules are split and each domain can own smaller rustdoc blocks.
