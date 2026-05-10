# Zenith Post-v1 `any` Dispatch Backend Stabilization (Wave 7.5)

> Audience: contributor, maintainer, language designer
> Status: closed (current subset)
> Surface: compiler/runtime/backend validation
> Last updated: 2026-05-05

This document closes Wave 7.5 from `post-v1-implementation-plan.md` for the current executable subset.
It records what is stabilized today for `any` dispatch in the C-backend oracle.

## Stabilized Subset

Current `any` dispatch guarantees:

- user syntax: `any Trait` / `any<Trait>`;
- heterogeneous collection baseline validated with `list<any<TextRepresentable>>`;
- user-defined object-safe trait collection baseline validated with `list<any<Drawable>>`;
- checker-enforced dispatch safety constraints:
  - non-generic traits only;
  - max 8 trait methods;
  - no mutating trait methods;
  - copyable method parameters/returns only;
  - trait implementation required via `apply Trait to Type`;
- `any` is rejected across `extern c` signatures.

The validated collection baseline now covers both built-in formatting trait
objects and user-defined object-safe trait objects:

- non-empty `list<any<TextRepresentable>>` literals can be checked, built, iterated, and dispatched through `to_text()`;
- non-empty `list<any<Drawable>>` literals can be checked, built, iterated, sliced, indexed, appended with `std.list.append`, mutated with indexed assignment/list-set, and dispatched through the trait vtable.

Remaining work is broader language hardening, not a syntax decision.
The user-facing contract remains `any<Trait>`.
Unsupported trait shapes must fail in the checker with an `any.*` diagnostic;
they should not surface as late C-emitter limitations.

## Backend/Runtime Contract (Current)

- HIR/checker lowering keeps dynamic dispatch in `ZT_TYPE_DYN` internally while user-visible rendering is canonicalized as `any`.
- C backend/runtime continue to use internal `zt_dyn_*` representation and helpers.
- This internal naming does not change user syntax, diagnostics, or examples.

## Validation Fixtures

Wave 7.5 stabilization is validated by migrated fixtures and diagnostics:

- `tests/behavior/list_dyn_trait_basic` (canonical source now `list<any<TextRepresentable>>`);
- `tests/behavior/list_dyn_textrepresentable` (scalar `any<TextRepresentable>` boxing and method dispatch);
- `tests/behavior/dyn_dispatch_basic` (trait object calls with `any Shape`);
- `tests/behavior/dyn_trait_heterogeneous_collection` (`list<any<Drawable>>` literal, iteration, slice, index, `std.list.append`, set, and vtable dispatch);
- `tests/behavior/dyn_generic_trait_error` (rejects generic trait object usage with `any.generic_trait`);
- `tests/behavior/std_concurrent_boundary_copy_unsupported_error` (transfer boundary rejects non-transferable `any` payload);
- `tests/hardening/test_wave4_transferable_predicate.py` (expects `concurrency.not_transferable` + `any<...>` wording).

## Non-goals / Still Open

Wave 7.5 does not close broader trait coherence or monomorphization topics.
Still handled by later Wave 7 items:

- trait coherence and overlap rules;
- richer `any` safety envelope across advanced shapes;
- generic monomorphization model and controls;
- backend conformance suite across non-C backends.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - roadmap and Wave statuses.
- `post-v1-closure-matrix.md` - operational tracker (`7.1.11`).
- `post-v1-any-migration.md` - Wave 7.4 naming and deprecation policy.
- `post-v1-completeness-discussion.md` - risk and closure rationale.
- `language-reference.md` - user-facing semantics for `any`.
