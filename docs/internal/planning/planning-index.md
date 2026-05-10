# Planning Document Index

> Audience: contributor, maintainer
> Status: current
> Surface: internal
> Last updated: 2026-05-03

This index explains which planning documents remain active after the language
planning cleanup. Current language direction lives in `docs/spec/language/`, not in
legacy roadmap/checklist files. The `0.4.2-beta.rc1` implementation plan exists
only to sequence the remaining approved implementation gaps.

## Active Documents

| Document | Role | Status |
|----------|------|--------|
| `documentation-cleanup-plan-2026-05-09.md` | Documentation cleanup and migration plan | Active |
| `0.4.2-beta.rc1-language-gap-implementation-plan.md` | Language gap implementation plan for advanced generics, concurrency, FFI, ownership, and stdlib platform foundations | Active |
| `editor-roadmap-v1.md` | Editor roadmap for Keter Micro and Zenith IDE | Active |
| `editor-checklist-v1.md` | Editor execution checklist split by track | Active |
| `selfhosted-roadmap-v1.md` | Self-hosted compiler roadmap | Future (post language readiness) |
| `selfhosted-checklist-v1.md` | Self-hosted execution checklist | Future |

## Removed Planning Documents

Language-readiness, v7, and Borealis planning files were removed from the active
documentation tree. Old editor planning files were replaced by
`editor-roadmap-v1.md` and `editor-checklist-v1.md`. Use Git history for older
planning details.

## Other Documents

| Document | Role | Status |
|----------|------|--------|
| `lsp-1.0-roadmap.md` | LSP implementation roadmap | Active |
| `cascade-v1.md` | Historical cascade document | Historical |
| `cascade-v2.md` | Cascade v2 | Historical |
| `r3-m5-progress.txt` | R3 milestone progress notes | Historical |

## Canonical Sources

For the current state of the language, read these in order:

1. `docs/spec/language/surface-implementation-status.md` â€” what the compiler does today
2. `docs/spec/language/language-reference.md` â€” normative unified spec
3. `docs/spec/language/post-v1-remaining-language-work.md` - current follow-up decisions
4. `docs/spec/language/post-v1-implementation-plan.md` - implementation waves and validation evidence
5. `docs/spec/language/v1-surface-contract.md` - historical baseline only
