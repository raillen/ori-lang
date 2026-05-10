# Zenith Post-v1 `any` Terminology Migration (Wave 7.4)

> Audience: contributor, maintainer, language designer
> Status: closed
> Surface: spec + compiler diagnostics/tooling
> Last updated: 2026-05-02

This document closes Wave 7.4 from `post-v1-implementation-plan.md`.
It defines canonical naming for dynamic dispatch and migration behavior for legacy `dyn` spelling.

## Decision

- canonical user-facing spelling is `any` (`any Trait`, `any<Trait>`);
- legacy `dyn` is accepted as a migration alias during post-v1 closure;
- parser emits `warning[deprecated.syntax]` for `dyn<Trait>` with migration guidance to `any<Trait>`;
- formatter and user-facing examples normalize to `any`;
- user-facing stable diagnostic codes use `any.*` (internal enum names may remain `ZT_DIAG_DYN_*`);
- LSP completion surfaces only `any` for dynamic dispatch type snippets.

## Internal Naming Policy

Internal implementation symbols may keep historical `dyn` naming while behavior is canonicalized at the surface.
This includes runtime helpers and internal type tags such as:

- `ZT_TYPE_DYN`;
- `zt_dyn_*` runtime helpers;
- historical fixture/module names not exposed as syntax guidance.

## Migration Guarantees

After Wave 7.4:

- docs/tutorials/spec guidance must not present `dyn` as preferred syntax;
- diagnostics shown to users must use `any.*` code family and `any<...>` wording;
- fixtures that validate user-facing syntax should prefer `any` spelling.

## Deferred Removal Policy

Final removal of `dyn` parser alias is tied to the edition/deprecation policy item (`post-v1-closure-matrix.md` item `7.1.38`).
Until then:

- `dyn` remains parse-compatible with deprecation warning;
- no new feature/docs should require `dyn` spelling.

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - roadmap and Wave statuses.
- `post-v1-closure-matrix.md` - operational tracker (`7.1.10`).
- `post-v1-syntax-freeze.md` - syntax-level freeze that set `any` as canonical spelling.
- `post-v1-any-dispatch-stabilization.md` - Wave 7.5 backend/runtime stabilization envelope.
- `language-reference.md` - canonical language reference examples and constraints.
