# Standard Library Reference

> Audience: package-author, advanced-user
> Surface: reference
> Status: current
> Source of truth: no

## Goal

Consult the implemented public-RC standard library without reading internal
plans.

Normative model: `docs/spec/language/stdlib-model.md`.

Primary generated/user-facing API source: `stdlib/zdoc/`.

## Pages

| Page | Use |
| --- | --- |
| `modules.md` | module list and responsibility |
| `io-json.md` | IO and JSON |
| `text-bytes-format.md` | text, bytes, and formatting |
| `filesystem-os-time.md` | filesystem, paths, OS, process, and time |
| `collections.md` | `std.collections`, `std.list`, and `std.map` |
| `math-random-validate.md` | math, regex, random, and validation |
| `concurrency-lazy-test-net.md` | concurrency base, lazy, test, and net |

## ZDoc Coverage

Current ZDoc roots:

- `stdlib/zdoc/std/`
- `stdlib/zdoc/en/std/`

The Tier 7 reset treats ZDoc as the API reference source. Public guides should
link here instead of copying every function.

## Rule

Stdlib is public-RC surface with documented post-RC limits.

Before publishing a new example:

```powershell
python tools/check_docs_current_syntax.py
python tools/check_docs_paths.py
git diff --check
```

Examples must either pass `zt check` or be marked as illustrative.
