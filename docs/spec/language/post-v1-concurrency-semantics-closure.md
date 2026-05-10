# Zenith Wave 7.11: Concurrency Semantics Closure

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: design session artifact  
> Surface: spec  
> Last updated: 2026-05-10

## Purpose

This document closes the Wave 7.11 concurrency semantics questions for jobs, channels, cancellation, panic propagation, `Transferable`, and typed payload strategy.

## Scope

This closure covers:
- `std.jobs` spawn/join semantics;
- `std.channels` create/send/receive/close semantics;
- capacity and backpressure policy;
- cancellation and panic boundary policy;
- `Transferable` as the cross-boundary predicate;
- non-`int` payload implementation strategy.

## Decisions

### C1: Transferable Boundary

Only `Transferable` values may cross job, channel, worker, shared, or future isolate boundaries.

`Transferable` is compiler-known and structural:
- scalar primitives, `text`, `bytes`, and `void` are transferable;
- `optional<T>`, `result<T,E>`, `list<T>`, `set<T>`, `map<K,V>`, and tuples are transferable only when all components are transferable;
- structs and enums are transferable only when every field or payload is transferable;
- callable/closure values and `any<Trait>` are not transferable in the current subset.

Implementation anchor:
- `compiler/semantic/types/checker.c` `zt_checker_type_is_transferable_inner()`.

### C2: Jobs

`std.jobs.spawn(function)` accepts only non-generic top-level function references.
`std.jobs.spawn(function, value)` additionally requires exact argument type compatibility and a transferable value.
`std.jobs.join(job)` blocks until the job completes and returns the job payload.

Current executable C oracle supports `int` and `text` job payloads via runtime handles.
The generic `Job<T>` surface is the semantic contract; backend support must exist before any wider payload is considered executable.

Closures/captured functions are rejected at the jobs boundary.

### C3: Channels

`std.channels.Channel<T>` is a typed single-payload communication handle.

Current executable C oracle supports `Channel<int>` and `Channel<text>`.

Semantics:
- `create()` creates an open channel;
- `send(channel, value)` succeeds only while the channel is open and empty;
- current runtime capacity is `1`;
- sending to a full channel is a contract error in the C oracle, not silent overwrite;
- `receive(channel)` returns `optional<T>`;
- receive from empty or closed-empty channel returns `none`;
- `close(channel)` closes and releases the handle;
- send after close is a contract error.

Backpressure policy:
- current surface is non-blocking/single-slot;
- future bounded blocking channels require an explicit API addition such as `create_bounded(capacity)` or `send_blocking`;
- no hidden scheduler or implicit async behavior is introduced.

### C4: Cancellation

Cancellation is not implicit.

Jobs and channels do not receive automatic cancellation from lexical scope exit, panic, or caller return.
Future cancellation must be explicit via a token/handle API and must preserve cleanup semantics defined by Wave 7.10.

### C5: Panic Propagation

Panic inside a job is a runtime boundary event.
The current C oracle treats runtime contract failures as process-level runtime errors.
A future richer job result may encode panic payloads, but Wave 7.11 rejects implicit checked-exception style propagation.

### C6: Non-`int` Payload Strategy

The canonical strategy is monomorphized typed runtime storage.

For each executable `Job<T>`, `Channel<T>`, `Shared<T>`, or `Atomic<T>` payload shape, the backend must either:
- generate or select a type-specialized runtime representation and extern ABI; or
- reject the use with a backend capability diagnostic.

Erased `void*` payload storage is not canonical for safe language semantics.
It may only appear behind generated, type-aware runtime wrappers.

## Validation Envelope

Minimum validation set:
- `tests/behavior/std_jobs_int_basic`
- `tests/behavior/std_jobs_text_basic`
- `tests/behavior/std_jobs_spawn_closure_error`
- `tests/behavior/std_channels_text_basic`
- `tests/behavior/wave4_concurrency_surface`
- `tests/behavior/wave4_concurrency_generic_surface`
- `tests/behavior/wave4_concurrency_generic_type_error`
- `tests/behavior/std_concurrent_boundary_copy_basic`
- `tests/behavior/std_concurrent_boundary_copy_unsupported_error`
- `tests/runtime/c/test_concurrency_text.c`
- `python tests/hardening/test_wave4_transferable_predicate.py`

## Closure Result

Wave 7.11 is closed as a semantic contract.
The executable C oracle now includes typed `int` and `text` job/channel handles.
`Shared<T>` and `Atomic<T>` remain restricted to the current executable scalar subset while wider payload strategy stays defined for future monomorphized backend expansion.

## Relationship To Other Documents

- `post-v1-trait-stability.md` defines `Transferable`.
- `post-v1-callable-closure-abi.md` defines callable/job boundary restrictions.
- `post-v1-using-cleanup-semantics.md` defines cleanup behavior that cancellation must preserve.
- `post-v1-runtime-abi-ownership-audit.md` records runtime ABI implications.
