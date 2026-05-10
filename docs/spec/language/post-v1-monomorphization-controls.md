# Zenith Wave 7.8: Monomorphization Controls

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: closure artifact  
> Surface: spec  
> Last updated: 2026-05-03

## Purpose

Close Wave 7.8 by defining hard controls that keep generic specialization bounded and observable.

## Control Set

## C1: Canonical Type Keys

Monomorphization keys use canonical type text before binding/lookup.
This prevents duplicate instances created by spelling variance.

Implementation anchors:
- `compiler/targets/c/emitter.c` canonicalization and substitution helpers.

## C2: Instance Cache + Dedup

Specialization registry is authoritative per compile session.
If same specialization exists, emitter reuses existing instance instead of regenerating code.

Implementation anchors:
- `c_registry_find_specialization`
- `c_registry_add_specialization`

## C3: Recursion Guard

Generic binding recursion is depth-guarded.
Excess recursive pattern expansion fails with explicit backend emit error.

Implementation anchors:
- `c_bind_generic_params_from_type_pair_impl` depth guard path.

## C4: Capacity Guard

Specialization registry has explicit hard ceiling to avoid unbounded code growth in one compile run.
Limit breach produces deterministic diagnostic.

Implementation anchors:
- `C_GENERIC_SPECIALIZATION_MAX`
- capacity check in `c_registry_add_specialization`.

## C5: Project Monomorphization Limit

Project manifest exposes build-level guard:
- `build.monomorphization_limit`
- default: `ZT_PROJECT_DEFAULT_MONOMORPHIZATION_LIMIT`.

Compiler enforces post-lowering instance count and fails when exceeded.
The count includes generic runtime type shapes and concrete generic function
specializations collected by the C emitter's specialization registry.

Implementation anchors:
- `compiler/project/ztproj.h`
- `compiler/driver/pipeline.c` (`zt_enforce_monomorphization_limit`).

Diagnostic contract:
- `project.invalid_monomorphization_limit`
- `project.monomorphization_limit_exceeded`

## Build Report Contract (Current Scope)

Current control plane exposes count + examples through deterministic diagnostics when limit is exceeded.
For v1 closure this is accepted as minimum report surface.
Optional richer JSON/text report remains future enhancement, not closure blocker.

## Validation Envelope

Minimum checks:

- Positive generic specialization fixtures:
  - `generic_arg_inference_basic`
  - `generic_monomorphization_nested_call`
  - `generic_monomorphization_text_basic`
- Negative monomorphization limit checks:
  - project parser/driver tests for invalid or exceeded `build.monomorphization_limit`;
  - `monomorphization_limit_error`;
  - `monomorphization_function_limit_error`.
- Positive many-instance check:
  - `monomorphization_many_instances_basic`.
- Full compile sanity:
  - `python build.py`

## Closure Result

Wave 7.8 control contract is defined and implemented for C oracle subset:
canonical keying, dedup registry, recursion guard, capacity guard, and manifest-level limit enforcement.

## Relationship To Other Documents

- `post-v1-monomorphization-closure.md` - Wave 7.7 specialization model.
- `post-v1-implementation-plan.md` - Wave 7.8 status.
- `backend-scalability-risk-model.md` - required backend scalability controls.
- `post-v1-closure-matrix.md` - operational closure tracker entry 7.1.15.
