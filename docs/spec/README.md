# Specs

> Audience: maintainer, contributor  
> Status: **current (2026-07-13)**  
> Surface: **S3** (`0.3.0`) + local inference **`0.3.1` / option B** + pipe `|>`  
> Package / workspace living line: **`0.3.4`** (Linux tar.gz + deb; FREEZE-1)  
> Milestones closed: **M2** stdlib · **M3** ABI · **M1** Rust-free install path  
> Language-first implementation queue: **empty** (see [`../planning/BACKLOG.md`](../planning/BACKLOG.md))  
> QA: skill **`ori-lang-qa`** · stages `tools/qa/` · matrix [`../planning/qa/test-matrix-ori.md`](../planning/qa/test-matrix-ori.md)

This directory contains **normative** implementation-facing specifications for
Ori. Language is **English only** (single source of truth). User tutorials live
under [../guides/](../guides/) and [../language/](../language/) (EN + PT).

Product docs index: [../README.md](../README.md).

### Living product facts (keep in sync)

| Fact | Value |
|------|--------|
| Canonical surface | S3: `module`, `public`, `import path = alias`, `list[T]`, `end` blocks, `apply`/`use` |
| Identifiers | **snake_case** functions/modules; **PascalCase** types (not camelCase product default) |
| Visibility | **`public`** (not `pub`) |
| Memory | ARC + cooperative cycle collection |
| Execution | AOT native primary; `ori run` may JIT when cdylib staged |
| Freeze | FREEZE-1 on **0.3.x** — additive/fix only without freeze exit |
| ABI | `ori-native-abi-1` — [`19-abi.md`](19-abi.md) |
| Residuals | [`14-backend-support.md`](14-backend-support.md) · [`../planning/historico/lang-res-closure.md`](../planning/historico/lang-res-closure.md) |

Use:

- [`00-manifesto.md`](00-manifesto.md) for identity and purpose (**study**,
  **AI-assisted programming**, **ND readability**). **Ori is not market
  competition.**
- `01-overview.md` through `13-error-catalog.md` for the language contract
  under the **S3** surface (plus inference B and pipe as living features).
- `04-types.md` / `05-expressions.md` / `06-statements.md` for local inference
  rules and the pipe operator.
- `13-error-catalog.md` for **emitted** diagnostics, including pre-S3 form
  rejections (`parse.*_removed`, `parse.poetic_call_nested`, …) and **message quality**.
- `14-backend-support.md` for the feature × backend matrix.
- `15-stdlib-maintenance.md` for the stdlib update flow.
- `16-runtime-ffi-safety.md` for runtime FFI safety contracts.
- `17-project-and-docs.md` for `ori.proj` and `.oridoc`.
- `18-stability-and-compatibility.md` for pre-1.0 stability rules.
- `19-abi.md` for the **native ABI contract** (`ori-native-abi-1`, M3): layouts,
  ARC header, mangling, link versioning.

Surface S3 product decisions and ADR:

- [`docs/planning/ori-surface-s3-auk9.md`](../planning/ori-surface-s3-auk9.md)
- [`docs/planning/adr-ori-surface-s3-auk9.md`](../planning/adr-ori-surface-s3-auk9.md)
- [`docs/planning/pr-plan-ori-surface-s3.md`](../planning/historico/pr-plan-ori-surface-s3.md)

Breaking list: repository root [`CHANGELOG.md`](../../CHANGELOG.md) section
`[0.3.0]`. Migration helper: `ori migrate-syntax`.

Do not place public tutorials here. User-facing tutorials may live under
`docs/guides/`. Implementation docs stay in `docs/spec/` and `docs/planning/`.
