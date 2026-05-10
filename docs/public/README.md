# Zenith Public Docs

> Audience: user
> Status: current
> Surface: public

## Start Here

Use these pages as the current public learning path.

| Need | Page |
| --- | --- |
| First run | `docs/public/get-started/quickstart.md` |
| First working mental model | `docs/public/learn/learn-zenith-in-30-minutes.md` |
| Practical task examples | `docs/public/learn/cookbook.md` |
| Compact language rules | `docs/public/language/language-reference.md` |
| Standard library map | `docs/public/stdlib/stdlib-reference.md` |
| CLI and package workflow | `docs/public/packages/tooling-guide.md` |
| Comparison with other languages | `docs/public/language/language-comparison.md` |
| License and trademark | `docs/public/licensing/license-and-trademark.md` |

## Target Sections

The public docs are being migrated into clearer sections:

| Section | Use |
| --- | --- |
| `get-started/` | first install, first run, first project |
| `learn/` | learning path and small examples |
| `language/` | user-facing language guide |
| `stdlib/` | standard library guide |
| `packages/` | project and package workflow |
| `licensing/` | licensing, trademark, and redistribution notes |

## Reading Order

1. Run `docs/public/get-started/quickstart.md`.
2. Read `docs/public/learn/learn-zenith-in-30-minutes.md`.
3. Keep `docs/public/language/language-reference.md` open while editing.
4. Use `docs/public/learn/cookbook.md` when solving a concrete task.
5. Use `docs/public/packages/tooling-guide.md` before publishing or running CI.

## Source of Truth

Public docs teach current usage.

Normative implementation rules remain in:

- `docs/spec/language/final-language-contract.md`
- `docs/spec/language/zenith-language-spec.md`
- `docs/spec/language/syntax-semantics-by-topic.md`
- `docs/reference/`

If a public example disagrees with the final contract, the final contract wins.

## Release Scope

The public release docs ship from this folder.

Translated trees are post-RC work unless they are rebuilt from the same current
contract and marked as current.
