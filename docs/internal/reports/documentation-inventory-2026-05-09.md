# Documentation Inventory - 2026-05-09

> Audience: maintainer
> Status: current
> Surface: internal
> Stage: documentation cleanup plan, Stage 1

This inventory classifies the current doc-like files before any migration,
archive move, or deletion.

No file was moved or deleted in this stage.

## Scope

Inventory command used for tracked files:

```powershell
git ls-files | Where-Object { $_ -match '\.(md|txt|html?|rst)$' }
```

Result:

- tracked doc-like files: 635
- untracked doc-like files: 1
- untracked file: `docs/internal/planning/documentation-cleanup-plan-2026-05-09.md`

The untracked file is the active migration plan created for this cleanup.

## Classification Model

Every doc-like file is classified by the first matching rule below.

| Rule | Classification | Action |
| --- | --- | --- |
| root README / changelog / contributing / security / trademark policy | public root | keep in root |
| root workflow notes not useful to public users | internal root candidate | move or archive later |
| root generated warning/log output | generated output / delete candidate | delete in Stage 8 after gate |
| `.github/` markdown | project infrastructure | keep outside docs migration |
| `docs/public/` | public | migrate into clearer public sections |
| `docs/reference/` | reference | keep, then fill missing reference sections |
| `docs/wiki/` | public wiki source | update after public docs are stable |
| `docs/internal/archive/` | archive | keep, but avoid linking as current |
| `docs/internal/planning/` | internal planning | keep only active plans in active indexes |
| `docs/internal/reports/` | report | consolidate current vs historical reports |
| `docs/internal/release/` | internal release policy | keep as internal policy |
| `docs/internal/governance/` | internal governance | keep |
| `docs/internal/standards/` | internal standard | keep |
| `docs/spec/language/` | canonical spec | keep canonical |
| `docs/internal/decisions/language/` | decision record | keep |
| `docs/spec/language/surface-implementation-status.md` | implementation status | keep, review stale blocker wording |
| old `language/` top-level docs | migrated or removed | do not recreate after Stage 6 cleanup |
| `compiler/*_MAP.md` and compiler map docs | colocated maintenance docs | keep near code |
| `runtime/`, `stdlib/`, `tests/`, `tools/` README/map docs | colocated maintenance docs | keep near code |
| `tests/**/*.txt` fixture outputs | validation fixture | keep with tests |
| `reports/` | generated/local or operational report | consolidate policy in Stage 7/8 |
| `packages/` docs | package documentation | keep with package unless public language docs duplicate it |
| `benchmarks/` docs | benchmark documentation | keep with benchmarks |
| `examples/` docs | example documentation | keep with examples |
| `installer/README.md` | installer documentation | keep with installer |
| `website/README.md` | website documentation | keep with website |

## Top-Level Coverage

| Top-level area | Count | Classification |
| --- | ---: | --- |
| `.github/` | 1 | project infrastructure |
| root public files | 5 | public root |
| root internal candidates | 2 | internal root candidate |
| root generated files | 1 | generated output / delete candidate |
| `benchmarks/` | 3 | benchmark documentation |
| `compiler/` | 18 | colocated maintenance docs |
| `docs/` | 180 | public/reference/internal/wiki docs |
| `examples/` | 6 | example documentation |
| `installer/` | 1 | installer documentation |
| historical `language/` root | 161 | migrated into `docs/spec/language/`, `docs/internal/decisions/language/`, or removed |
| `packages/` | 78 | package documentation |
| `reports/` | 6 | generated/local or operational reports |
| `runtime/` | 3 | colocated runtime docs |
| `stdlib/` | 5 | colocated stdlib docs |
| `tests/` | 153 | test docs and validation fixtures |
| `tools/` | 11 | tool docs and tool decisions |
| `website/` | 1 | website documentation |

## Root-Level Files

| File | Classification | Stage action |
| --- | --- | --- |
| `README.md` | public root | keep |
| `CHANGELOG.md` | public root | keep |
| `CONTRIBUTING.md` | public root | keep |
| `SECURITY.md` | public root | keep |
| `TRADEMARK_POLICY.md` | public root | keep |
| `CAVEMAN_WORKFLOW.md` | internal root candidate | move to internal archive or delete after review |
| `HEAVY_TESTS_BRIEFING.md` | internal root candidate / stale report | move to archive or delete after review |
| `warnings_full.txt` | generated output / delete candidate | delete in Stage 8 after gate |

Notes:

- `HEAVY_TESTS_BRIEFING.md` is tied to `0.3.0-alpha.1` and contains stale
  risk language. It must not remain as an apparent current root document.
- `CAVEMAN_WORKFLOW.md` describes an agent/GSD workflow, not public language
  usage. It also has mojibake in headings.
- `warnings_full.txt` is compiler warning output and is not authored
  documentation.

## Docs Folder

| Area | Count | Classification | Stage action |
| --- | ---: | --- | --- |
| `docs/README.md` | 1 | docs index | keep and update after migration |
| `docs/DOCS-STRUCTURE.md` | 1 | docs policy | keep and update after migration |
| `docs/public/` | 7 | public | restructure in Stage 4 |
| `docs/reference/` | 33 | reference | fill missing sections in Stage 5 |
| `docs/wiki/` | 8 | public wiki source | update after public docs are current |
| `docs/internal/` | 130 | internal | consolidate in Stage 7 |

### Public Docs

Current public files:

- `docs/public/README.md`
- `docs/public/cookbook.md`
- `docs/public/language-comparison.md`
- `docs/public/language-reference.md`
- `docs/public/learn-zenith-in-30-minutes.md`
- `docs/public/stdlib-reference.md`
- `docs/public/tooling-guide.md`

Classification:

- public and current-intended;
- not yet organized by target section;
- should be split into `get-started`, `learn`, `language`, `stdlib`,
  `packages`, and `licensing` in Stage 4.

Risk:

- all public files use `Status: current`; this is acceptable only if Stage 4
  revalidates examples and release wording.

### Reference Docs

Current reference sections:

- `docs/reference/api/`
- `docs/reference/cli/`
- `docs/reference/language/`
- `docs/reference/stdlib/`
- `docs/reference/zenith-kb/`

Classification:

- reference docs;
- `docs/reference/api/` has only one README and should be treated as a
  placeholder/generated API entry point;
- diagnostics content currently lives under CLI and KB, not a dedicated
  diagnostics section;
- grammar content is spread across language reference pages, not a dedicated
  grammar section.

### Internal Reports

Report folders:

- `docs/internal/reports/audit/`
- `docs/internal/reports/compatibility/`
- `docs/internal/reports/fuzz/`
- `docs/internal/reports/overrides/`
- `docs/internal/reports/perf/`
- `docs/internal/reports/raw/`
- `docs/internal/reports/release/`
- `docs/internal/reports/semantic/`
- `docs/internal/reports/triage/`

Classification:

- curated internal evidence and historical reports;
- Stage 7 must separate "current release status" from old audit evidence.

Main risk:

- historical RC and alpha reports are close to current reports and can look
  active unless indexes label them clearly.

## Language Folder

| Area | Count | Classification | Stage action |
| --- | ---: | --- | --- |
| `docs/spec/language/` | 54 | canonical spec | keep canonical, review stale RC wording |
| `docs/internal/decisions/language/` | 98 | decision records | keep |
| old `language/README.md` | 1 | migrated language index | removed after `docs/spec/language/README.md` became canonical |
| `docs/spec/language/current.md` | 1 | current pointer | review against RC/stable state |
| `docs/spec/language/MVP_OUT_OF_SCOPE.md` | 1 | historical or needs review | classify in Stage 6 |
| `docs/spec/language/surface-implementation-status.md` | 1 | implementation status | keep, review stale blockers |
| removed language-root quarantine | 5 | temporary migration archive | removed after useful content was migrated or replaced |

Canonical rule:

- `docs/spec/language/` remains the normative source during this cleanup.
- Public and reference docs may point to it, but must not duplicate it as a
  second source of truth.

## Public Folder Gaps

The target public section folders do not exist yet:

- public get-started section;
- public learn section;
- public language section;
- public stdlib section;
- public packages section;
- public licensing section.

Current state:

- `docs/public/` is useful but flat;
- it is not README-only, but it is still harder to scan than the target
  structure;
- Stage 4 should split or route the existing pages into the target sections.

## Reference Folder Gaps

Missing or incomplete target reference sections:

- diagnostics reference section;
- grammar reference section;
- generated API section is only a README placeholder.

Current state:

- CLI reference exists;
- stdlib reference exists;
- language reference exists;
- diagnostics and grammar are spread across existing pages.

## Misleading Or Stale Current Labels

These items need review before public/stable release docs are considered clean:

| File | Issue | Stage |
| --- | --- | --- |
| `docs/wiki/Home.md` | version label aligned to `0.4.1-alpha.1`; keep synced after each package cut | Stage 4 or 10 |
| `docs/wiki/Roadmap-and-Releases.md` | version label aligned to `0.4.1-alpha.1`; keep synced after each package cut | Stage 4 or 10 |
| `HEAVY_TESTS_BRIEFING.md` | root file for `0.3.0-alpha.1`; stale risk list | Stage 8 |
| `CAVEMAN_WORKFLOW.md` | root workflow note, not language docs, mojibake headings | Stage 8 |
| `warnings_full.txt` | generated warning output in root | Stage 8 |
| `docs/internal/reports/audit/implementation-plan-rc-public.md` | RC plan now historical after RC publication | Stage 7 |
| `docs/internal/reports/audit/rc-public-release-gap-closure-2026-05-08.md` | gap closure report should be historical once final status exists | Stage 7 |
| `docs/internal/reports/audit/language-complete-analysis-2026-05-08.md` | analysis report can look current unless indexed as evidence | Stage 7 |
| `docs/spec/language/implementation-plan.md` | old implementation plan naming can look active | Stage 6 |
| `docs/spec/language/implementation-review.md` | old review naming can look active | Stage 6 |
| `reports/pending-language-issues-current.md` | operational current backlog outside `docs/internal/reports/` | Stage 7 |

## Delete Candidates

Delete candidates are listed only. Nothing is deleted in Stage 1.

| File or pattern | Reason | Earliest stage |
| --- | --- | --- |
| `warnings_full.txt` | generated compiler warning output | Stage 8 |
| root `emit_debug.txt` if present later | generated output | Stage 8 |
| root `emit_stdout.txt` if present later | generated output | Stage 8 |
| root `out.txt` if present later | generated output | Stage 8 |
| root `test_output.txt` if present later | generated output | Stage 8 |
| root `tmp_*.txt` if present later | generated output | Stage 8 |

Review candidates before delete/archive:

- `CAVEMAN_WORKFLOW.md`
- `HEAVY_TESTS_BRIEFING.md`
- old audit reports that duplicate current release status;
- raw reports that no longer add evidence beyond Git history.

## Source Of Truth Map

This closes Stage 2 of the cleanup plan.

| Need | Source of truth | Notes |
| --- | --- | --- |
| Public entry point | `README.md` and `docs/public/README.md` | Root README stays small; public docs teach usage. |
| Changelog | `CHANGELOG.md` | Keep in repo root. |
| Contribution workflow | `CONTRIBUTING.md` | Keep in repo root unless a later contributor-docs split is planned. |
| Security policy | `SECURITY.md` | Keep in repo root. |
| Trademark policy | `TRADEMARK_POLICY.md` | Keep in repo root. |
| Public user guides | `docs/public/` | Must match current implementation. |
| Stable lookup/reference docs | `docs/reference/` | Short, consultable, not normative when it conflicts with specs. |
| Language normative contract | `docs/spec/language/` | Canonical for parser, typechecker, runtime, stdlib, backend, and tooling behavior. |
| Decision records | `docs/internal/decisions/language/` | Canonical for accepted language rationale and tradeoffs. |
| Internal planning | `docs/internal/planning/` | Active plans only; historical plans must be labeled or archived. |
| Internal reports and evidence | `docs/internal/reports/` | Curated reports that need to survive local runs. |
| Release reports | `docs/internal/reports/release/` | Versioned release evidence and release notes. |
| Release policy | `docs/internal/release/` | Policies, freezes, and release process docs. |
| Internal standards | `docs/internal/standards/` | Writing style and templates. |
| Internal architecture index | `docs/internal/architecture/` | Indexes colocated architecture docs. |
| Historical docs | `docs/internal/archive/` | Only retained when they explain history, decisions, or evidence. |
| Generated durable docs | generated on demand or under a named reports/tool output area | Create only if generated output has durable value. |
| Local/generated run output | `reports/` or ignored files | Must not be presented as authored docs. |
| Colocated subsystem docs | near the code | Allowed for compiler, runtime, stdlib, tests, tools, packages, examples, benchmarks, installer, and website docs. |

Root files allowed to stay in root:

- `README.md`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `TRADEMARK_POLICY.md`

Root files not approved as long-term root docs:

- `CAVEMAN_WORKFLOW.md`
- `HEAVY_TESTS_BRIEFING.md`
- `warnings_full.txt`

Historical material policy:

- archive if it explains a decision, release gate, audit trail, or migration
  context;
- delete if it is generated output, duplicated by a current source, or useful
  only through Git history;
- never leave old reports in current indexes without a historical label.

Conflict rule:

- if public/reference docs disagree with `docs/spec/language/`, the spec wins;
- if current release status disagrees with old audit reports, the current
  release-readiness report wins;
- if docs disagree with implementation, docs must be corrected or clearly
  marked as future/historical before release.

## Stage 1 Gate

- inventory exists: yes;
- every doc-like file has one classification: yes, by classification model;
- delete candidates are listed but not deleted: yes.

## Stage 2 Gate

- root-level files that may stay in root are defined: yes;
- public/reference/internal/generated/spec/decision layers are defined: yes;
- release reports and release policies have separate homes: yes;
- historical material has archive/delete criteria: yes;
- no active topic has two competing source-of-truth locations: yes, by conflict
  rule above.
