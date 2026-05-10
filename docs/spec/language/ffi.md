# Zenith Foreign Function Interface (FFI) Model

- Status: authoritative spec (cut R3.M3 + Wave 5.1 callbacks + Wave 5.2 ABI/name attrs + 0.4.2 C-repr structs, extern const, and target attrs)
- Date: 2026-05-10
- Scope: `extern c`, `extern host`, ABI contract, symbol name attrs, ownership rules at boundary, allowed/blocked type matrix
- Upstream: `docs/internal/decisions/language/011-extern-c-and-extern-host.md`, `docs/spec/language/runtime-model.md`, `docs/spec/language/concurrency.md`
- Related runtime implementation: `compiler/targets/c/emitter.c` (`c_emit_ffi_shielded_zir_call_statement`, `c_type_to_c`, `c_extern_call_expected_arg_type`)

## Purpose

Formalize the current Zenith FFI boundary so that contributors can reason about what `extern c` does at runtime, what the emitter guarantees, and what the user must not attempt today.

This document is normative. Deviation from it is a bug.

## Scope Of This Cut

FFI 1.0 ships:

- `extern c` as the only target-native binding form;
- `extern host` as the abstract boundary to runtime/embedder capabilities (used internally by stdlib; not the user-facing C interop surface);
- automatic reference-count shielding for managed arguments crossing the boundary;
- a curated allowed type matrix for extern signatures;
- callback parameters in `extern c` declarations for top-level Zenith functions with primitive signatures;
- per-function `attr name("symbol")` and `attr abi("cdecl"|"stdcall")` inside `extern c` blocks;
- explicit C representation structs with `attr repr("c")` for by-value C ABI arguments and returns;
- read-only `extern const` declarations for FFI-safe C globals.

Not in this cut:

- raw pointer surface types;
- mutable `extern` variables;
- variadic `extern` functions;
- library discovery or linker-flag inference from extern declarations.

These remain `Decision 011` out-of-scope and are revisited in a later FFI revision.

## Syntax

Canonical form, at namespace scope only:

```zt
namespace app.platform

attr repr("c")
struct Point
    x: int
    y: int
end

extern c
    func puts(message: text) -> int

    attr name("zt_ffi_apply_i64")
    func apply_i64(value: int, callback: func(int) -> int) -> int

    attr abi("stdcall")
    func zt_ffi_add_i64_stdcall(left: int, right: int) -> int

    attr name("zt_ffi_point_sum")
    func point_sum(point: Point) -> int

    attr name("zt_ffi_answer")
    const ANSWER: int

    attr target("windows")
    attr name("zt_ffi_windows_value")
    const PLATFORM_VALUE: int

    attr target("unix")
    attr name("zt_ffi_unix_value")
    const PLATFORM_VALUE: int
end
```

Rules:

- `extern c` and `extern host` are block forms; each block may contain one or more function declarations or read-only constants;
- declarations inside the block have no body;
- by default, the declared function name is the external C symbol name;
- `attr name("symbol")` before a function declaration overrides the external C symbol while preserving the Zenith call name;
- `attr name("symbol")` before an `extern const` overrides the external C symbol while preserving the Zenith constant name;
- `attr target("any"|"windows"|"unix"|"linux"|"macos")` keeps the next extern item active only for that compiler target;
- `attr abi("cdecl")` and `attr abi("stdcall")` before a function declaration select the emitted C prototype calling convention;
- `attr abi(...)` is valid only for extern functions, not constants;
- `attr repr("c")` may be placed before a `struct` declaration to opt into by-value C layout checks;
- the declaration is addressable using its unqualified Zenith name after import;
- callable parameters may be declared as `func(T, ...) -> R` when `T`/`R` are boundary-safe callback shapes.

## ABI Contract

### Target binding

- `extern c` uses the C ABI of the host toolchain. The Zenith compiler currently emits through the host C compiler (`gcc` on the reference build), so the effective ABI is whatever that toolchain implements.
- `attr abi("cdecl")` emits an explicit `ZT_EXTERN_CDECL` prototype; `attr abi("stdcall")` emits an explicit `ZT_EXTERN_STDCALL` prototype. On non-Windows targets these macros intentionally collapse to the platform default calling convention.
- `attr name("symbol")` also causes the C emitter to write an `extern` prototype for the renamed symbol. This is required for custom C shims that are linked through `build.linker_flags`.

### Cross-platform guarantee

- The same Zenith `extern c` declaration is portable across any platform whose C toolchain accepts the emitted C declaration with equivalent semantics.
- Platform-specific symbols may use `attr target(...)` on the extern item. This is a selection gate only: it does not discover libraries or add linker flags.

### No silent conversion

- The compiler does not invent casts. A mismatched argument type at the call site produces `type.mismatch` at the Zenith call site (see the negative fixture).
- The emitter refuses to lower to C if the Zenith type cannot be mapped (`C_EMIT_UNSUPPORTED_TYPE`).

## Allowed / Blocked Type Matrix

Types allowed at the FFI boundary in this cut:

| Zenith type | C representation | Allowed as arg? | Allowed as return? | Notes |
|---|---|---|---|---|
| `int` | `zt_int` (64-bit signed) | yes | yes | primitive roundtrip |
| `float` | `zt_float` (double) | yes | yes | primitive roundtrip |
| `bool` | `zt_bool` | yes | yes | primitive roundtrip |
| `text` | `zt_text *` | yes (shielded) | yes (managed return) | passed as managed pointer; see Ownership |
| `bytes` | `zt_bytes *` | yes (shielded) | yes (managed return) | passed as managed pointer |
| `list<int>` | `zt_list_i64 *` | yes (shielded) | yes (managed return) | passed as managed pointer |
| `list<text>` | `zt_list_text *` | yes (shielded) | yes (managed return) | passed as managed pointer |
| `list<float>` | `zt_list_f64 *` | yes (shielded) | yes (managed return) | passed as managed pointer |
| `map<text,text>` | `zt_map_text_text *` | yes (shielded) | yes (managed return) | passed as managed pointer |
| runtime-approved opaque wrapper | private runtime pointer/struct | yes | yes | stdlib/runtime boundary only; not a general user FFI shape |
| `optional<T>` where `T` allowed | value with discriminator | yes | yes | inline `zt_optional_*` structs |
| `result<T,E>` / `outcome<T,E>` | generated struct | yes | yes | see `compiler/targets/c/emitter.c` type mapping |
| `attr repr("c") struct S` with only FFI-safe fields | generated C struct | yes | yes | by value; fields must be primitive scalar or nested C-repr struct |
| `func(P...) -> R` callback parameter | C function pointer | yes, parameter only | no | `P` must be primitive/text/bytes; `R` may also be `void`; only top-level function refs may be passed |

Types **blocked** at the FFI boundary in this cut:

- user-defined `struct` without `attr repr("c")` as direct `extern c` argument or return;
- C-repr struct fields containing managed or private runtime values (`text`, `bytes`, `list`, `map`, `set`, `optional`, `result`, `any`, callables, and non-C-repr structs);
- user-defined `enum` with payload (no stable C layout for payloads);
- function types outside the callback-parameter subset above;
- raw pointers (no surface syntax);
- `any Trait` values (the boxed representation is private to the runtime);
- any type whose `c_type_to_c` mapping fails (`C_EMIT_UNSUPPORTED_TYPE` at emit time).

If a managed or non-C-repr struct is required on the C side, the user must expose a helper in the Zenith runtime / stdlib that accepts allowed primitives or approved handles, and keep the private managed layout inside runtime code.

## C-Repr Structs By Value

0.4.2-beta.rc1 adds the first stable by-value struct ABI:

```zt
attr repr("c")
struct Point
    x: int
    y: int
end

extern c
    attr name("zt_ffi_make_point")
    func make_point(x: int, y: int) -> Point
end
```

Rules:

- the marker spelling is `attr repr("c")`;
- the marker is valid only on `struct`;
- generic C-repr structs are rejected in this beta;
- fields may be `bool`, integral types, floating types, or another C-repr struct with FFI-safe fields;
- fields may not be `text`, `bytes`, containers, `optional`, `result`, `any`, callable values, or unannotated user structs;
- unannotated structs are rejected at `extern c` parameter and return positions before C emission.

The C emitter already emits stable field order from the Zenith declaration. The
semantic checker is the gate: if a field shape is not FFI-safe, the project
fails during check instead of producing a C compiler error.

The only non-C-repr user-shaped values allowed across the current `extern c`
surface are runtime-approved opaque wrappers used by stdlib/runtime modules,
for example `net.Connection`. They are not ordinary by-value structs: the C
type mapping resolves them to a private runtime representation such as
`zt_net_connection *`.

## Extern Const

0.4.2-beta.rc1 adds read-only C globals:

```zt
extern c
    attr name("zt_ffi_answer")
    const ANSWER: int
end
```

The C emitter writes an external declaration:

```c
extern const zt_int zt_ffi_answer;
```

Rules:

- `extern const` has no initializer in Zenith;
- the C side owns the storage and initialization;
- the value is read-only from Zenith;
- allowed types are primitive scalars and `attr repr("c")` structs with FFI-safe fields;
- managed values such as `text`, `bytes`, collections, `optional`, `result`, `any`, and callables are rejected;
- mutable extern globals remain out of scope for this cut.

## Target Attributes

`attr target(...)` is the minimal conditional extern model for
0.4.2-beta.rc1:

```zt
extern c
    attr target("windows")
    attr name("zt_ffi_selected_windows")
    const SELECTED: int

    attr target("unix")
    attr name("zt_ffi_selected_unix")
    const SELECTED: int
end
```

Rules:

- supported selectors are `any`, `windows`, `unix`, `linux`, and `macos`;
- a missing selector behaves like `any`;
- inactive items are still parsed and formatted, but they do not enter the binder, checker, HIR/ZIR lowering, or C emission;
- duplicate Zenith names are allowed only when inactive alternatives are excluded by the current target;
- target selection does not manage external libraries, headers, or linker flags.


## Symbol Names And ABI Attributes

Wave 5.2 supports per-function attributes inside `extern c` blocks:

```zt
extern c
    attr name("zt_ffi_apply_i64")
    func apply_i64(value: int, callback: func(int) -> int) -> int

    attr abi("stdcall")
    func zt_ffi_add_i64_stdcall(left: int, right: int) -> int
end
```

Rules:

- `attr name("...")` affects the next extern function or `extern const` only;
- `attr target("...")` affects the next extern function or `extern const` only;
- `attr abi("cdecl"|"stdcall")` affects the next extern function only;
- unsupported ABI strings are rejected during parsing;
- ABI annotations emit a C prototype so the host C compiler sees the call convention before calls are emitted;
- name annotations also emit a C prototype for custom linked symbols.

## Callback Parameters

Wave 5.1 supports the narrow callback subset from Decision 089:

- `extern c` functions may declare callable parameters with primitive/text/bytes parameters and primitive/text/bytes/`void` return;
- only top-level Zenith function references may be passed at the call site;
- callback APIs that need state must pass that state as an explicit `user_data` parameter in the C callback signature;
- closure literals, captured closures, local callable variables, generic delegates, callable returns, and callable fields remain unsupported.

The C backend emits a small static trampoline for each eligible top-level Zenith function. The trampoline has the raw C callback signature and immediately invokes the Zenith function with a null runtime context. This supports C functions that call the callback during the `extern c` call and do not store it. Long-lived callback storage by C remains out of scope.

## Ownership And Lifetime At The Boundary

### Managed arguments: automatic shielding

When a managed value (`text`, `bytes`, `list<T>`, `map<K,V>`, ...) is passed to an `extern c` function, the emitter inserts a shielding block around the call:

```
{
    zt_text *zt_ffi_arg0 = greeting;
    if (zt_ffi_arg0 != NULL) { zt_retain(zt_ffi_arg0); }
    length = zt_text_len(zt_ffi_arg0);
    if (zt_ffi_arg0 != NULL) { zt_release(zt_ffi_arg0); }
}
```

Guarantees:

- the managed value's reference count is bumped before the call;
- the managed value is released after the call returns (even if the call stores the pointer internally);
- the value remains alive for the duration of the call;
- the caller's original binding is unaffected.

Implementation entry points:

- `c_emit_ffi_shielded_zir_call_statement` (statement form);
- `c_emit_ffi_shielded_zir_return` (return form);
- legacy variants `c_emit_ffi_shielded_legacy_call_statement` / `c_emit_ffi_shielded_legacy_return`.

### Managed returns

A managed return value (`zt_text *`, `zt_list_*_t *`, etc.) is treated as newly owned by the caller. The runtime ARC path then takes over normal lifetime management.

### What the C side must NOT do

- do not store the received managed pointer beyond the call duration unless the C code is part of the Zenith runtime and uses `zt_retain`/`zt_release` explicitly;
- do not free the managed pointer directly (ARC is managed by Zenith);
- do not mutate the internal layout of `zt_text`, `zt_bytes`, `zt_list_*`, `zt_map_*` directly; use the runtime helpers.

### Raw `owned` across the boundary

- there is no surface `owned<T>` in this cut, so the concept of "transferring ownership to C" is handled implicitly by the copy/shielding contract;
- the Phase 4 move-based transfer described in `docs/spec/language/concurrency.md` is **not** available for FFI in this cut.

## Diagnostics

Expected diagnostics at the FFI boundary:

- `type.mismatch` at the call site when an argument type does not match the declared signature (covered by `tests/behavior/extern_c_struct_arg_error`);
- `type.invalid_call` / `type.invalid_argument` for arity mismatches at an `extern c` call site;
- `backend.c.emit` (`C_EMIT_UNSUPPORTED_TYPE`) when a Zenith type cannot be lowered to C (emitted during code generation, not during semantic check);
- `project.unresolved_import` when the namespace that declares the `extern c` block is not importable;
- `syntax.error` for unsupported extern attrs or ABI strings;
- `type.invalid` for unannotated structs at `extern c` boundaries, non-FFI-safe C-repr struct fields, and non-FFI-safe `extern const` types;
- `callable.extern_c_signature` when a callback parameter or return shape is not boundary-safe;
- `callable.extern_c_closure_unsupported` when the call site passes a closure/local callable instead of a top-level function reference.

All diagnostics use the action-first renderer (ACTION / WHY / NEXT) per `R3.M1`.

## Tests

Current coverage:

- positive, primitive return: `tests/behavior/extern_c_puts_e2e` (`puts(text) -> int`);
- positive, managed-arg shielding + primitive return: `tests/behavior/extern_c_text_len_e2e` (`zt_text_len(text) -> int`);
- working external-library example: `examples/c-bindings-sqlite3` uses a C shim around SQLite so raw pointers stay outside Zenith;
- negative, struct-as-arg: `tests/behavior/extern_c_struct_arg_error` (user struct where `text` expected; `type.mismatch` at call site);
- positive, callback: `tests/behavior/extern_c_callback_basic` (`func(int) -> int` passed to immediate C callback helper);
- positive, callback with explicit user data: `tests/behavior/extern_c_callback_user_data_basic`;
- negative, closure callback: `tests/behavior/extern_c_callback_closure_error`;
- negative, non-primitive callback signature: `tests/behavior/extern_c_callback_signature_error`;
- positive, symbol renaming: `tests/behavior/extern_c_attr_name_basic`;
- positive, stdcall ABI prototype: `tests/behavior/extern_c_abi_stdcall_basic`;
- negative, unsupported ABI string: `tests/behavior/extern_c_abi_unsupported_error`;
- positive, C-repr struct argument: `tests/behavior/extern_c_struct_arg_basic`;
- positive, C-repr struct return: `tests/behavior/extern_c_struct_return_basic`;
- positive, extern const scalar: `tests/behavior/extern_c_const_basic`;
- positive, extern const C-repr struct: `tests/behavior/extern_c_const_struct_basic`;
- positive, target-selected extern const: `tests/behavior/extern_c_target_const_basic`;
- negative, unannotated struct at FFI boundary: `tests/behavior/extern_c_struct_unannotated_error`;
- negative, managed field in C-repr struct: `tests/behavior/extern_c_struct_managed_field_error`;
- negative, managed extern const: `tests/behavior/extern_c_const_managed_error`;
- negative, unsupported target selector: `tests/behavior/extern_c_target_unsupported_error`;
- negative, non-transferable at related `std.concurrent` boundary: `tests/behavior/std_concurrent_boundary_copy_unsupported_error` (for the concurrent helper wrapper, not the direct `extern c` surface).

Deferred (FFI 1.x follow-ups):

- arity negative fixture at `extern c` call site;
- invalid return type negative fixture;
- wider binding suite (libcurl, platform helpers).

## Implementation Phases (FFI)

| Phase | Scope | Status |
|---|---|---|
| 1 | `extern c` declaration + emit + shielding + spec + positive/negative minimal tests | delivered (this cut) |
| 2 | Arity and invalid return negative fixtures; explicit runtime helper matrix | follow-up in R3 |
| 3 | Callback parameters / function pointers | delivered for immediate top-level primitive callbacks in Wave 5.1; long-lived storage deferred |
| 4 | ABI annotations and symbol renaming | delivered in Wave 5.2 for per-function `attr name`, `attr abi("cdecl")`, and `attr abi("stdcall")` |
| 5 | C-repr structs by value | delivered in 0.4.2-beta.rc1 for non-generic FFI-safe structs |
| 6 | `extern const` | delivered in 0.4.2-beta.rc1 for primitive and C-repr read-only C globals |
| 7 | Target attributes | delivered in 0.4.2-beta.rc1 as item-level selection only |

## Cross References

- `docs/internal/decisions/language/011-extern-c-and-extern-host.md` (canonical syntax decision)
- `docs/spec/language/runtime-model.md` (ARC + boundary policy)
- `docs/spec/language/concurrency.md` (transferable shapes, R3.P1.A integration)
- `docs/internal/reports/release/R3.M3-ffi-1.0-report.md` (milestone report)
- `docs/internal/archive/reports/legacy-main/R3-risk-matrix.md` (`R3-RISK-020`, `R3-RISK-021`, `R3-RISK-022`)

## Residual Risk

- ABI stability across compilers (MSVC vs MinGW vs gcc on Linux) is still limited: `cdecl`/`stdcall` are expressed through C macros and verified on the reference MinGW/Windows path. See `R3-RISK-020`.
- Managed-return ownership is handled by the runtime, but a long-lived C pointer cached in a C static struct would be unsafe. The spec forbids it, but no static analysis catches it. See `R3-RISK-021`.
- Callback interop is limited to immediate C calls of top-level primitive callbacks. Capturing closures, generic delegates, and long-lived callback storage remain deferred. See `R3-RISK-022`.
