# Zenith Wave 7.20: Final Language Closure Review

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: final gate artifact  
> Surface: post-v1 closure  
> Last updated: 2026-05-03

## Purpose

This document records the Wave 7.20 final language closure review. Its goal is to ensure no language, ZIR, compiler, runtime, diagnostics, or backend-conformance topic remains nebulous after Wave 7.

## Closure Checklist

### Language Surface

Closed by:
- `post-v1-syntax-freeze.md`
- `post-v1-idiom-pass.md`
- `post-v1-any-migration.md`
- `post-v1-trait-stability.md`
- `post-v1-error-model-closure.md`
- `post-v1-pattern-matching-closure.md`

Result: no syntax or idiom topic remains without a decision document.

### Generics and Callables

Closed by:
- `post-v1-monomorphization-closure.md`
- `post-v1-monomorphization-controls.md`
- `post-v1-callable-closure-abi.md`

Result: executable generic monomorphization, inference boundaries, callable ABI, and callable escape restrictions are documented.

### Resources and Runtime Semantics

Closed by:
- `post-v1-using-cleanup-semantics.md`
- `post-v1-runtime-abi-ownership-audit.md`

Result: cleanup semantics and runtime ownership obligations are explicit.

### Concurrency

Closed by:
- `post-v1-concurrency-semantics-closure.md`

Result: jobs/channels semantics, `Transferable`, cancellation policy, panic boundary, and non-`int` payload strategy are defined.

### Compiler, ZIR, Diagnostics, Backends

Closed by:
- `post-v1-zir-consolidation.md`
- `post-v1-backend-conformance-suite.md`
- `post-v1-source-mapping-contract.md`
- `post-v1-diagnostic-contract.md`
- `post-v1-optimization-boundary.md`

Result: compiler/backend contracts are explicit enough to unblock future Zig/LLVM/WASM planning after conformance work.

## Deferred But No Longer Nebulous

The following remain future implementation work, but their route is now defined:
- non-`int` executable concurrency payloads use monomorphized typed runtime storage or capability diagnostics;
- richer job panic capture requires explicit API;
- stream/async IO work builds on jobs/channels without `async/await`;
- WebSocket/TLS remain IO/dataflow scope after closure prerequisites;
- backend optimization quality follows conformance, not vice versa.

## Final Gate Rule

A future work item may start only if it either:
- conforms to one of the Wave 7 closure artifacts; or
- creates a new explicit decision artifact before changing language/compiler/runtime behavior.

## Validation Envelope

Minimum final review validation:
- `python build.py`
- targeted closure fixtures from Waves 7.7-7.18
- selected invalid diagnostics from Wave 7.17
- hardening tests for transferability and ownership where available

2026-05-03 validation note:
- `python build.py`
- Positive closure fixtures passed `check`, `build`, and `run` with expected exits:
  - `std_jobs_int_basic`
  - `wave4_concurrency_surface`
  - `wave4_concurrency_generic_surface`
  - `std_concurrent_boundary_copy_basic`
  - `optional_result_basic`
  - `result_question_basic`
  - `optional_question_basic`
  - `optional_or_return_basic`
  - `result_or_wrap_basic`
  - `value_semantics_optional_result_managed`
  - `optional_result_helpers_pass`
  - `optional_result_helpers_absence_error`
  - `control_flow_match`
  - `enum_match`
  - `match_guard_basic`
  - `multivalue_match_basic`
  - `optional_match_value`
  - `using_basic`
  - `callable_basic`
  - `generic_arg_inference_basic`
  - `generic_monomorphization_nested_call`
- Negative closure fixtures returned expected non-zero `check`:
  - `std_jobs_spawn_closure_error`
  - `wave4_concurrency_generic_type_error`
  - `std_concurrent_boundary_copy_unsupported_error`
  - `result_optional_propagation_error`
  - `optional_question_outside_optional_error`
  - `enum_match_non_exhaustive_error`
  - `match_guard_non_bool_error`
  - `multivalue_match_type_error`
  - `callable_invalid_func_ref_error`
  - `generic_arg_inference_missing_error`
  - `generic_arg_inference_conflict_error`
- `python tests/hardening/test_wave4_transferable_predicate.py`

## Closure Result

Wave 7.20 closes the language-closure review gate.
The post-v1 roadmap can proceed to later IO/dataflow, alternate backend, tooling, registry, and ecosystem work without unresolved language-core ambiguity.

## Relationship To Other Documents

- `post-v1-implementation-plan.md`
- `post-v1-closure-matrix.md`
- `post-v1-completeness-discussion.md`
- `post-v1-surface-contract.md`
