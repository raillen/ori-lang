# Zenith Wave 7.18: Runtime ABI and Ownership Audit

> Audience: runtime implementer, backend implementer, compiler maintainer  
> Status: audit + design artifact  
> Surface: runtime ABI contract  
> Last updated: 2026-05-10

## Purpose

This document closes Wave 7.18 runtime ABI and ownership audit scope for values, functions, closures, ARC/ORC, cleanup, FFI, concurrency, collections, lazy, net, and time under the C oracle.

## Scope

This closure covers:
- function ABI;
- closure/callable ABI;
- managed value ownership;
- deterministic cleanup ABI;
- collections and optional/result layout expectations;
- FFI boundaries;
- concurrency runtime handles;
- stdlib runtime-backed modules.

## Decisions

### R1: C Oracle ABI

The C runtime ABI is the executable oracle.
Other backends may use different internal layouts, but language-visible behavior must match C oracle results.

### R2: Function ABI

Functions lower to explicit C-callable shapes with typed parameters and typed return values.
Top-level function references used as callables or jobs are represented through the closure ABI when a runtime callback boundary requires it.

### R3: Closure ABI

Stored callable values use:
- function pointer;
- context pointer;
- optional context drop function.

Escaping callable positions remain restricted by Wave 7.9.

### R4: Managed Ownership

Managed values (`text`, `bytes`, collections, optional/result wrapping managed values, lazy values) must obey retain/release or move/sink rules.

Backend lowering must preserve:
- last-use moves;
- copy-on-write isolation;
- cleanup on all control-flow exits;
- no double-free or leaked owned values on normal paths.

### R5: Cleanup ABI

`using` lowering must emit cleanup on:
- normal scope exit;
- return;
- `?` propagation;
- panic path where supported;
- loop `break`/`continue`.

### R6: Collections and Sum Types

Collections, `optional<T>`, and `result<T,E>` must preserve value semantics and managed payload ownership.
Typed helper generation is accepted where required by the C backend.

### R7: FFI Boundary

Extern C functions use explicitly declared signatures.
Managed values crossing FFI require explicit supported ABI shapes.
Closures cannot be passed to extern C callbacks unless the boundary explicitly supports a pure top-level function wrapper.

### R8: Concurrency Runtime Handles

Jobs, channels, shared values, and atomics use runtime handles in the current C oracle.
The current executable subset is `int`-backed; generic surface expansion must lower to typed runtime wrappers or reject unsupported payloads.

### R9: Stdlib Runtime Modules

Runtime-backed stdlib modules such as lazy, net, time, fs/os/process, regex, random, and debug must document any ownership-bearing return values and cleanup responsibilities as they expand.

### R10: `std.mem` Ownership Intent Facade

`std.mem.own`, `std.mem.view`, and `std.mem.edit` are library-level ownership
intent APIs. They are compiler-known, but they are not ownership keywords.

The C oracle accepts only shapes with proven clone/retain/move/destroy/edit
behavior:
- primitive scalars;
- `text`;
- safe tuples and structs;
- primitive/text lists;
- `list<safe tuple>` and `list<safe struct>`;
- `set<int>` and `set<text>`;
- maps with `int` or `text` keys and scalar/text values.

Unsupported nested mutable managed shapes must fail during checking. The ABI
must never pretend that an editable owned copy exists when the runtime would
actually return a shared alias.

### R11: Callback Cleanup Boundary

The stable FFI callback path remains top-level and immediate. Explicit
`user_data` parameters are allowed when declared in the callback signature.
Captured callbacks remain rejected until a future ABI defines context lifetime
and drop hooks for C-owned callback storage.

Cleanup inside the Zenith callback body follows ordinary `using` lowering.
The callback fixture with `user_data` is part of the ownership validation
envelope.

## Validation Envelope

Minimum validation set:
- ORC hardening tests;
- value semantics collection fixtures;
- optional/result managed fixtures;
- using cleanup fixtures;
- `std.mem` positive and negative ownership intent fixtures;
- FFI callback cleanup fixture with explicit `user_data`;
- callable fixtures;
- concurrency fixtures;
- stdlib runtime-backed fixtures for lazy/net/time/fs/os/process where available.

## Closure Result

Wave 7.18 is closed as the runtime ABI audit contract.
Open implementation work must now be filed as concrete ABI gaps rather than nebulous runtime uncertainty.

0.4.2-beta.rc1 addendum:

- `std.mem` generic ownership intent is closed for the safe executable subset.
- `mem.Temp`, `mem.Pool<T>`, weak references and broad cycle collection remain
  explicit future library/runtime work.
- Thread-safe ARC stays scoped to approved concurrency wrappers, not the
  default managed value path.

## Relationship To Other Documents

- `post-v1-callable-closure-abi.md`
- `post-v1-using-cleanup-semantics.md`
- `post-v1-concurrency-semantics-closure.md`
- `post-v1-zir-consolidation.md`
- `post-v1-backend-conformance-suite.md`
