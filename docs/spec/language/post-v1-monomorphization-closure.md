# Zenith Wave 7.7: Generic Monomorphization Closure

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: closure artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

Close Wave 7.7 for current executable C-backend scope.
Define instance identity, lowering model, supported subset, diagnostics, and validation envelope.

## Decision Summary

- Generic function execution model is monomorphization-first for C oracle.
- Canonical instance identity is:
  - generic function declaration identity;
  - ordered generic parameter binding map (`T -> concrete type`);
  - canonicalized concrete type names.
- Inference source is argument-position evidence only.
  - No return-context inference.
  - No partial inference.
  - No defaulted argument monomorphization path in current emitter slice.
- Specialization naming is deterministic (`<fn>__mono__<param>_<type>...`).
- Repeated request for same instance must reuse existing specialization.

## Lowering Model (Current C Oracle)

### 1) Where monomorphization happens

Monomorphization is resolved in C emission pipeline for executable generic calls.
Current implementation is centered in `compiler/targets/c/emitter.c`.

Key functions:
- `c_infer_generic_specialization_for_call`
- `c_bind_generic_params_from_type_pair`
- `c_build_generic_instance_name`
- `c_registry_find_specialization`
- `c_registry_add_specialization`

### 2) What is monomorphized

Current closed subset:
- Direct generic function calls with complete argument evidence.
- Nested/transitive generic calls where downstream callee bindings are inferable.
- Generic parameter substitution in callable lowering path used by executable C backend.

### 3) What is intentionally outside this closure item

Still mapped to later waves, not blocker for 7.7 closure artifact:
- Full generic HOF surface (`map<T,U>`, generic `reduce<T>` in all shapes).
- Generic lazy iterator families.
- Generic streams and erased adapters.
- Runtime payload storage beyond the current `int`/`text` job/channel and `int` shared/atomic typed-handle constraints.

## Cross-Module Model

- Project compilation builds unified module graph before backend emission.
- Specialization identity remains declaration + canonical binding map.
- Same canonical instantiation across project must deduplicate in single compile session.

## Failure Contract

When specialization cannot be safely inferred/emitted, compilation fails with explicit diagnostics (not silent fallback):
- missing generic evidence;
- argument/parameter shape mismatch during binding;
- unsupported generic slice in current emitter path.

This preserves deterministic behavior and prevents unsound implicit coercion.

## Validation Envelope

Minimum validation set for this closure:

- `tests/behavior/generic_arg_inference_basic`
- `tests/behavior/generic_arg_inference_missing_error`
- `tests/behavior/generic_arg_inference_conflict_error`
- `tests/behavior/generic_monomorphization_nested_call`

And baseline compiler validation:
- `python build.py`

2026-05-03 validation note:
- `.\zt.exe check tests/behavior/generic_arg_inference_basic --all --ci`
- `.\zt.exe check tests/behavior/generic_monomorphization_nested_call --all --ci`
- `.\zt.exe build tests/behavior/generic_arg_inference_basic --ci --native-raw`
- `.\zt.exe run tests/behavior/generic_arg_inference_basic --ci --native-raw` exits `7`.
- `.\zt.exe build tests/behavior/generic_monomorphization_nested_call --ci --native-raw`
- `.\zt.exe run tests/behavior/generic_monomorphization_nested_call --ci --native-raw` exits `9`.
- Negative inference fixtures report expected diagnostics:
  - `generic_arg_inference_missing_error`: `type.invalid_call`
  - `generic_arg_inference_conflict_error`: `type.mismatch`

## Closure Result

Wave 7.7 is closed for executable C-backend monomorphization subset.
Further generic surface expansion continues in dependent waves, but 7.7 decision contract is now defined and auditable.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave 7.7 status and sequencing.
- `post-v1-monomorphization-controls.md` - Wave 7.8 control mechanisms.
- `backend-scalability-risk-model.md` - monomorphization risk requirements.
- `post-v1-closure-matrix.md` - operational closure tracker entry 7.1.14.
