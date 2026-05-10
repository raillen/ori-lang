# RC Public Docs Consolidation - 2026-05-07

> Audience: maintainer, docs writer
> Status: current RC evidence
> Surface: internal report
> Source of truth: no

## Decision

The RC public documentation uses the current top-level `docs/public/` pages.

Translated public trees are post-RC unless they are rebuilt from the current
contract and marked current.

## Canonical Sources For RC

Read in this order:

1. `docs/spec/language/final-language-contract.md`
2. `docs/spec/language/zenith-language-spec.md`
3. `docs/spec/language/syntax-semantics-by-topic.md`
4. `docs/spec/language/stdlib-model.md`
5. `docs/spec/language/stdlib-reference-by-topic.md`
6. `docs/public/README.md`
7. `docs/public/learn-zenith-in-30-minutes.md`
8. `docs/public/language-reference.md`
9. `docs/public/cookbook.md`
10. `docs/public/stdlib-reference.md`
11. `docs/public/tooling-guide.md`
12. `docs/reference/README.md`
13. `docs/internal/release/docs-canonical-policy.md`

## Historical Or Non-Blocking Sources

These remain useful, but they do not define current public behavior:

- `docs/internal/decisions/language/`
- old roadmap/checklist files;
- old audit reports;
- stale public translation plans;
- generated local reports under `reports/`.

Historical references may mention paths that no longer exist. For RC, the
important rule is: active public docs and active reference docs must not route
users to missing files.

`tools/check_docs_paths.py` now follows that RC rule. It checks active docs and
skips historical decisions, historical reports, generated outputs and selected
old planning files that intentionally preserve obsolete paths.

## Public Syntax Audit

The public docs must not teach these as current syntax:

- `dyn<Trait>`;
- `fmt "..."`;
- `assert`;
- `case default`;
- `uint8`, `uint16`, `uint32`, `uint64`;
- global `size_of`.

Migration tables may mention old spellings only when they clearly point to the
current replacement.

## Result

Validation on 2026-05-07:

| Check | Result |
| --- | --- |
| Initial `python tools/check_docs_paths.py` | failed with 300 missing paths, mostly historical reports/plans/decisions |
| Updated `python tools/check_docs_paths.py` | passed |
| `python tools/check_docs_current_syntax.py` | passed |
| `.\zt.exe doc check zenith.ztproj` | passed |
| public historical-term search | only matched `std.debug.size_of(value)`, which is scoped stdlib API, not global `size_of` |
| manual initial user path | `docs/public/README.md` routes to existing learn, language reference, cookbook, stdlib reference and tooling guide pages |

Decision: Etapa 2 is complete for RC-public documentation consolidation.
