# Ori Language Specification — Chapter 19: ABI and FFI Contract

> Status: normative
> Audience: compiler implementers, runtime maintainers, tooling authors

---

## Overview

This document specifies the Application Binary Interface (ABI) for the Ori language in its `1.0` stability cycle. The ABI guarantees stable memory layouts, name mangling, and calling conventions so that external C libraries can interoperate safely with Ori, and so the Ori compiler can safely interface with its own runtime.

---

## Memory Layouts

Ori data types are designed to be explicitly predictable in memory, closely matching C's `repr(C)` guarantees.

### Primitives
- `bool`: 1 byte (0x00 is false, 0x01 is true).
- `int`: 64-bit signed integer (`int64_t`).
- `float`: 64-bit floating point (`double`).
- Pointers (`*T`, `*const T`, `*mut T` in runtime): size of machine word (64-bit on supported architectures).

### Structs
All `struct` declarations in Ori are guaranteed to have a memory layout identical to a C `struct` with the same fields declared in the same order.
- Alignment follows the standard C ABI for the target architecture.
- There is no automatic field reordering.
- Padding bytes are inserted as dictated by the platform's C compiler.

### Enums
An `enum` without payloads (C-like enum) is represented as a single integer. The default size is the smallest integer type that can hold all variant values (typically 1 byte for < 256 variants), though this is subject to C alignment rules when inside a struct.

An `enum` *with payloads* (tagged union) is represented as:
1. A discriminant tag (typically 1 byte, aligned to the largest alignment requirement of any payload).
2. A union containing the payload of each variant.

This matches Rust's `#[repr(C, u8)]` enum layout.

### Tuples
Tuples are represented exactly like anonymous `struct`s with fields ordered from left to right.

---

## Calling Conventions

The default calling convention for Ori functions and FFI interactions is the standard C calling convention (`cdecl` on Unix-like systems, standard Microsoft x64 calling convention on Windows).

- All `extern "C"` functions imported into Ori assume this calling convention.
- All Ori functions emitted by Cranelift follow the system's default C calling convention. This makes any Ori function directly callable from C without wrapper thunks.

---

## Name Mangling

To avoid naming collisions and to support namespaces, Ori applies a deterministic name mangling scheme to all top-level functions and static variables.

### The Scheme
The mangled name is a concatenation of the namespace, the entity name, and an optional type hash (for monomorphized generics).

Format: `ori_MANGLE_<namespace>_<name>[_<hash>]`

- `namespace`: The fully qualified namespace, with dots `.` replaced by underscores `_`. (e.g., `ori.string.utils` becomes `ori_string_utils`).
- `name`: The function or variable name.
- `hash`: A 16-character hexadecimal hash of the generic type arguments (only present if the function is a monomorphized instance of a generic function).

### Exceptions (No Mangling)
Functions declared as `extern "C"` are NOT mangled. They use the exact name specified in the binding.
The `main` function (entry point of an Ori executable) is emitted without mangling as `main` (or wrapped by the system's CRT `main` entry).

---

## FFI Safety and ARC

When crossing the ABI boundary to external C code, memory ownership must be respected. Ori uses Automatic Reference Counting (ARC).

- **Borrowing**: Passing a managed type (like `string` or `list`) to C passes a borrowed pointer. The C code MUST NOT free it.
- **Ownership**: If C code returns a managed pointer allocated via Ori's runtime allocators, the Ori caller assumes ownership and will release it.

For detailed FFI safety rules regarding the runtime, refer to [Chapter 16: Runtime FFI safety contracts](16-runtime-ffi-safety.md).
