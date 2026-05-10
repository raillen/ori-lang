# Zenith Wave 7.12: Error Model Closure

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: audit artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

This document closes the Wave 7.12 error model audit for `result`, `optional`, `?`, `.or_return`, `.or_wrap`, panic boundaries, and FFI/jobs interop.

## Scope

This closure covers:
- `optional<T>` and `result<T,E>` as ordinary typed values;
- `?` propagation;
- `.or_return` and `.or_wrap` helper semantics;
- panic as runtime failure, not recoverable control flow;
- FFI and concurrency boundaries.

## Decisions

### E1: Primary Error Model

Recoverable failures use `result<T,E>`.
Absence uses `optional<T>`.
Zenith does not add `try/catch`, implicit exceptions, `??`, or `?.`.

### E2: Question Propagation

`expr?` unwraps a success/present payload and short-circuits the enclosing function on failure/none.

Rules:
- `result<T,E>?` is valid only inside a function whose return type can receive the propagated error result;
- `optional<T>?` is valid only inside a function whose return type can receive `none`;
- propagation preserves ownership and cleanup obligations before returning.

### E3: `.or_return`

`optional<T>.or_return(value)` unwraps present values.
If none, it returns `value` from the enclosing function.

This is explicit early-return sugar, not exception handling.

### E4: `.or_wrap`

`result<T, core.Error>.or_wrap(context)` unwraps success.
On failure it returns a `core.Error` with added context.

The helper is reserved for `core.Error` shaped flows and should not create hidden conversions for arbitrary error types.

### E5: Panic

`panic` is unrecoverable runtime failure for contract violations and impossible states.
It is not part of the typed recoverable error model.

Across FFI/jobs boundaries, panic is a boundary failure and must not be silently translated to success, `none`, or arbitrary error values.

### E6: FFI and Jobs

Extern C calls use the declared ABI and do not gain hidden `result` wrapping.
Jobs return their declared payload through `join`; richer panic/error capture requires an explicit future API.

## Validation Envelope

Minimum validation set:
- `tests/behavior/optional_result_basic`
- `tests/behavior/result_question_basic`
- `tests/behavior/optional_question_basic`
- `tests/behavior/optional_or_return_basic`
- `tests/behavior/result_or_wrap_basic`
- `tests/behavior/value_semantics_optional_result_managed`
- `tests/behavior/result_optional_propagation_error`
- `tests/behavior/optional_question_outside_optional_error`
- `tests/behavior/optional_result_helpers_pass`
- `tests/behavior/optional_result_helpers_absence_error`

## Closure Result

Wave 7.12 is closed as an audit-backed language contract.
Remaining work is fixture expansion and diagnostics polish, not semantics discovery.

## Relationship To Other Documents

- `docs/internal/decisions/language/009-optional-result-and-error-flow.md`
- `docs/internal/decisions/language/030-question-propagation-for-result-and-optional.md`
- `post-v1-using-cleanup-semantics.md`
- `post-v1-concurrency-semantics-closure.md`
