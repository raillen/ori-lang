# Zenith Tooling Guide

> Audience: user
> Status: current
> Surface: public

## Daily Commands

| Command | Use |
| --- | --- |
| `zt check <project>` | validate syntax, manifest, imports, and semantics |
| `zt build <project>` | compile a project |
| `zt run <project>` | build and run an app project |
| `zt test <project>` | run `attr test` functions |
| `zt fmt <project>` | rewrite source to canonical formatting |
| `zt fmt <project> --check` | verify formatting without rewriting |
| `zt doc show <symbol>` | show symbol documentation |
| `zpm install <project>` | resolve dependencies and write lockfile |
| `zpm install --locked <project>` | verify lockfile in CI mode |

## Local Loop

Use this while editing one feature:

```powershell
python build.py
.\zt.exe check zenith.ztproj --all --ci
python run_suite.py smoke --no-perf
```

For formatter work:

```powershell
python tests/driver/test_fmt_phase15.py
```

For tooling work:

```powershell
python tests/driver/test_phase16_tooling.py
python tests/driver/test_zpm_lockfile.py
python tests/driver/test_zpm_semver.py
python tests/lsp/test_lsp_smoke.py
```

## Before A Pull Request

Run:

```powershell
python build.py
python run_suite.py pr_gate --no-perf
python tools/check_docs_current_syntax.py
git diff --check
```

If `pr_gate` reports only the known loopback network fixture problem, verify that fixture directly:

```powershell
.\zt.exe build tests/behavior/std_net_basic
powershell -NoProfile -ExecutionPolicy Bypass -File tests/behavior/std_net_basic/run-loopback.ps1 -Executable build/std-net-basic.exe
```

## Formatting Rule

Formatting is a gate.

Use `zt fmt --check` in CI and `zt fmt` only when you want to rewrite files.

## Package Lock Rule

Use `zpm install --locked` in CI.

It should fail when:

- `zenith.lock` is missing;
- the manifest changed but the lockfile was not updated;
- dependency constraints are invalid.

## LSP And Editor Flow

The LSP smoke test covers diagnostics and completion.

Run it after touching:

- parser surface;
- semantic diagnostics;
- stdlib symbols;
- completion logic.

```powershell
python tests/lsp/test_lsp_smoke.py
```

## Deeper References

- `docs/reference/cli/zt.md`
- `docs/reference/cli/zpm.md`
- `docs/reference/diagnostics/cli-diagnostics.md`
- `docs/spec/language/tooling-model.md`
