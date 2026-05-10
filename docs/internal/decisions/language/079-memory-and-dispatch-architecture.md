# Decision 079 - Memory and Dispatch Architecture

- Status: accepted
- Date: 2026-04-18
- Type: runtime / backend / architecture
- Scope: memory model, concurrency boundary, dispatch strategy, panic safety, FFI boundary

## Summary

Zenith adopts a performance-first runtime architecture for the C backend while keeping safety and predictability visible at the language level.

This decision closes five base directions: non-atomic RC with isolates, dual dispatch (monomorphization + dyn), UTF-8 text boundary, controlled runtime panics, and explicit FFI boundary handling.

## Decision

### 1. Non-Atomic RC plus Isolates

Zenith uses non-atomic reference counting for ordinary managed heap values.

Cross-thread sharing of ordinary mutable references is not a default language path.

Concurrency uses isolate/message-passing boundaries. When shared mutable cross-thread state is needed, it must use explicit shared wrappers with stronger synchronization guarantees.

The user-facing concurrency follow-up is worker/job/channel oriented, not a raw shared-thread model for ordinary managed values.

### 2. Dual Dispatch Model

Default generic execution uses monomorphization for zero-cost specialization.

For heterogeneous collections and interface-oriented runtime polymorphism, Zenith supports `dyn` with fat-pointer/vtable style dispatch.

Canonical direction:

- homogeneous performance paths: `list<T>` (monomorphized)
- heterogeneous interface paths: `list<dyn Trait>`

### 3. Text and Bytes Safety Boundary

`text` is UTF-8 validated data.

Raw external data from files/network should first be represented as `bytes`.

Explicit UTF-8 conversion (`text.from_utf8(bytes)`) is the boundary where recoverable decoding failure is handled via `result<text, utf8_error>`.

### 4. Controlled Panic Safeguards

Zenith does not treat undefined behavior as normal control flow.

Runtime guards (for example bounds checks and division-by-zero checks) should fail in controlled panic paths with usable diagnostics, instead of silent memory corruption.

Hot-path unsafe escape hatches can exist later behind explicit unsafe boundaries.

### 5. Explicit FFI Boundary Handling

At `extern("C")` boundaries, generated code must protect managed runtime invariants while native code is executing.

The backend is responsible for safe transition mechanics (pin/retain/release/lock strategy as required by runtime invariants), so user code does not rely on implicit UB-prone behavior.

## Consequences

- predictable high performance for common single-isolate runtime paths
- reduced hidden synchronization overhead in default code paths
- clear separation between monomorphic speed paths and dynamic polymorphic paths
- safer text/binary boundary behavior for IO-heavy workloads
- explicit runtime/backend obligations for FFI correctness
