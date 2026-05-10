# Documentation Spec Migration - 2026-05-09

> Audience: maintainer, contributor
> Status: current
> Surface: internal

## Summary

The language documentation now lives inside the unified `docs/` tree.

Current entry points:

- `docs/spec/language/README.md`: normative language specs.
- `docs/spec/language/final-language-contract.md`: active language contract.
- `docs/internal/decisions/language/README.md`: language decisions and rationale.
- `branding/`: canonical branding assets.

## Moves

| From | To |
| --- | --- |
| language specs | `docs/spec/language/` |
| language decisions | `docs/internal/decisions/language/` |
| language branding assets | removed from docs tree; canonical copies remain in `branding/` |
| root language implementation status | `docs/spec/language/surface-implementation-status.md` |
| root language current-state pointer | `docs/spec/language/current.md` |
| root language historical/generated notes | removed as temporary migration material |

## Rule After Migration

Do not recreate `language/`, `docs/generated/`, or `docs/assets/` for ordinary
documentation.

Use:

- `docs/public/` for user-facing guides;
- `docs/reference/` for stable lookup pages;
- `docs/spec/language/` for implementation-facing language truth;
- `docs/internal/decisions/language/` for language rationale;
- `docs/internal/` for plans, reports, governance, and archives.
