# Tier 7 Documentation Inventory

> Audience: maintainer, docs writer
> Status: draft
> Surface: internal
> Source of truth: no

## Snapshot

Current documentation volume is high enough to create authority drift.

Count from the local tree:

| Area | Files |
| --- | ---: |
| `docs/internal/` | 121 |
| `docs/public/` | 237 |
| `docs/reference/` | 32 |
| `docs/wiki/` | 8 |
| `docs/internal/decisions/language/` | 96 |
| `docs/spec/language/` | 26 |
| Other language docs | 9 |

## Main Problem

The project does not only need more docs.

It needs fewer authoritative surfaces:

- one current language spec;
- one public learning path;
- one reference layer;
- one organized decision history;
- archived historical material.

## Keep During Reset

Keep these as implementation evidence until their content is represented in the
new canonical spec:

| Area | Action | Reason |
| --- | --- | --- |
| `docs/spec/language/v1-surface-contract.md` | keep | current v1 surface contract |
| `docs/spec/language/implementation-status.md` | keep | current feature status |
| `docs/spec/language/compiler-model.md` | keep | compiler-facing rules |
| `docs/spec/language/runtime-model.md` | keep | runtime-facing rules |
| `docs/spec/language/stdlib-model.md` | keep | stdlib architecture |
| `docs/spec/language/project-model.md` | keep | project/package behavior |
| `docs/spec/language/diagnostic-code-catalog.md` | keep | stable diagnostic mapping |
| `docs/spec/language/decision-conflict-audit.md` | keep until resolved | known contradictions list |
| code-local maps and READMEs | keep | maintenance docs close to implementation |
| behavior tests and fixtures | keep | strongest evidence of actual behavior |

## Rewrite Candidates

These should not stay as separate competing sources after Tier 7:

| Area | Action | Target |
| --- | --- | --- |
| `docs/spec/language/language-reference.md` | rewrite | canonical language spec |
| superseded surface-syntax notes | merge/rewrite | `docs/spec/language/zenith-language-spec.md` and `docs/spec/language/syntax-semantics-by-topic.md` |
| `docs/reference/language/*` | regenerate/rewrite | short reference derived from spec |
| `docs/public/language/language-reference.md` | rewrite | accessible public language guide |
| `docs/public/learn/learn-zenith-in-30-minutes.md` | rewrite | new tutorial path |
| `docs/public/learn/cookbook.md` | rewrite | task-oriented recipes |
| `docs/public/stdlib/stdlib-reference.md` | rewrite | current stdlib reference |
| translated public trees | post-RC/retranslate | current public docs first |

## Archive Candidates

Archive before deletion:

| Area | Action | Reason |
| --- | --- | --- |
| old planning docs | archive | implementation facts already moved forward |
| old audit/report files | archive | useful history, not user docs |
| duplicate translated docs | archive or mark best-effort | risk of stale semantics |
| `docs/wiki/` snapshots | archive/rebuild | wiki should not become parallel spec |
| stale HTML monoliths | archive unless selected canonical | avoid two specs |
| superseded decisions | keep historical, remove from active path | context only |

Suggested archive root:

`docs/internal/archive/tier7-doc-reset/`

## Known Contradiction Evidence

Fast local search found active drift:

| Topic | Evidence |
| --- | --- |
| `dyn` vs `any<Trait>` | decisions 079/088 and older comparison docs still use `dyn`; current contract and public cookbook use `any<Trait>`. |
| `case default` vs `case else` | reference docs, decisions 010/029/078, and compiler docs still mention `case default`; decision 094 and conflict audit say `case else` is the newer canonical spelling. |
| sub-integer removal vs current shipped types | old implementation-plan item 2.10 conflicted with current `u8/u16/u32/u64` surface and tests. |

These are enough to justify the reset.

## Proposed Canonical Output

Current RC canonical output:

- `docs/spec/language/zenith-language-spec.md`
- `docs/internal/decisions/language/INDEX.md`
- `docs/public/language/language-reference.md`
- `docs/public/learn/learn-zenith-in-30-minutes.md`
- `docs/public/learn/cookbook.md`
- `docs/public/stdlib/stdlib-reference.md`
- `docs/public/packages/tooling-guide.md`
- `docs/public/language/language-comparison.md`

Post-RC translation and comparison expansion remains useful, but it must not be
linked as a current public route until files exist and examples are validated.

## Next Work

1. Build `docs/internal/decisions/language/INDEX.md`.
2. Mark each decision as `Current`, `Superseded`, `Historical`, or `Implementation-only`.
3. Use that index to resolve contradictions before writing the public docs.
4. Add docs validation after canonical syntax is settled.
