# Specs

> Audience: maintainer, contributor  
> Status: current  
> Surface: **S3** (`0.3.0` cutover)

This directory contains implementation-facing specifications for Ori.

Use:

- [`00-manifesto.md`](00-manifesto.md) for identity and purpose (**study**,
  **AI-assisted programming**, **ND readability**). **Ori is not market
  competition.**
- `01-overview.md` through `13-error-catalog.md` for the language contract
  under the **S3** surface.
- `13-error-catalog.md` for **emitted** diagnostics, including pre-S3 form
  rejections (`parse.*_removed`, `parse.poetic_call_nested`, …).
- `14-backend-support.md` for the feature × backend matrix.
- `15-stdlib-maintenance.md` for the stdlib update flow.
- `16-runtime-ffi-safety.md` for runtime FFI safety contracts.
- `17-project-and-docs.md` for `ori.proj` and `.oridoc`.
- `18-stability-and-compatibility.md` for pre-1.0 stability rules.
- `19-abi.md` for ABI notes.

Surface S3 product decisions and ADR:

- [`docs/planning/ori-surface-s3-auk9.md`](../planning/ori-surface-s3-auk9.md)
- [`docs/planning/adr-ori-surface-s3-auk9.md`](../planning/adr-ori-surface-s3-auk9.md)
- [`docs/planning/pr-plan-ori-surface-s3.md`](../planning/pr-plan-ori-surface-s3.md)

Breaking list: repository root [`CHANGELOG.md`](../../CHANGELOG.md) section
`[0.3.0]`. Migration helper: `ori migrate-syntax`.

Do not place public tutorials here. User-facing tutorials may live under
`docs/guides/`. Implementation docs stay in `docs/spec/` and `docs/planning/`.
