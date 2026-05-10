# Release Diff Review - 2026-05-08

> Audience: maintainer
> Status: current RC evidence
> Surface: internal audit evidence
> Source of truth: no

## Scope

This evidence covers the release-gap follow-up after
`docs/internal/reports/audit/rc-public-release-gap-closure-2026-05-08.md`.

It does not approve a public tag. It records the local diff/release-note review
that can be done before the final remote CI run.

## Commands

| Command | Result |
| --- | --- |
| `git status --short` | large intentional RC worktree; no release tag created |
| `git diff --name-status` | reviewed tracked files by area |
| `python build.py` | pass on 2026-05-09 pre-commit validation |
| `python tools/check_docs_paths.py` | pass on 2026-05-09 pre-commit validation |
| `python tools/check_docs_current_syntax.py` | pass on 2026-05-09 pre-commit validation |
| `.\zt.exe check zenith.ztproj --all --ci` | pass on 2026-05-09 pre-commit validation |
| `.\zt.exe test zenith.ztproj --ci` | pass on 2026-05-09 pre-commit validation |
| `.\zt.exe fmt zenith.ztproj --check` | pass on 2026-05-09 pre-commit validation |
| `.\zt.exe doc check zenith.ztproj` | pass on 2026-05-09 pre-commit validation |
| `python run_suite.py release` | pass, 365/365, `reports/suites/release__20260509T005740Z.json` |
| `python tools/build_installers.py --version 1.0.0-rc.1 --target windows --dry-run --skip-build` | pass |
| `python tools/build_installers.py --version 1.0.0-rc.1 --target all --dry-run --skip-build` | expected platform-boundary failure for Linux from Windows |
| `wsl.exe ... python3 tools/build_installers.py --target linux --dry-run --skip-build` | failed because Linux binaries `zt`, `zpm`, `zt-lsp` are not staged in this Windows checkout |
| `wsl.exe ... python3 tools/build_linux_packages.py --dry-run --skip-staging --skip-checksums ...` | pass; emitted `.deb`, `.rpm`, and `.pkg.tar.zst` `fpm` commands |

## Diff Classification

The current worktree contains intentional RC changes in these groups:

- compiler/runtime fixes and hardening;
- stdlib/ZDoc coverage;
- docs/public, docs/reference and docs/internal release cleanup;
- behavior, hardening, heavy and performance test evidence;
- performance baselines with justification already recorded in
  `docs/internal/reports/audit/evidence/diff-cleanup-2026-05-08.md`;
- audit/release reports under `docs/internal/reports/`.

Historical reports that could look current were marked historical or
superseded:

- `reports/pending-language-issues-current.md`;
- `reports/deep-analysis-report.md`;
- `docs/internal/reports/audit/R2.M7-spec-vs-implementation-audit.md`;
- `docs/internal/reports/audit/language-implementation-audit-2026-04-29.md`;
- `docs/internal/reports/audit/final-language-implementation-audit-2026-05-03.md`.

## Artifact Decision

Installer/package tooling is present and dry-run validated, but published
artifacts are not generated in this pass.

Reason:

- Linux packages require Linux/WSL binaries for `zt`, `zpm` and `zt-lsp`;
- public artifacts should be generated only from the final commit after the
  remote matrix passes;
- attaching unsigned or locally stale artifacts would create more release risk
  than value.

## External Gates

- final remote CI matrix - completed for commit `3db2a30`;
- final clean-tree review after staging/commit - completed;
- tag creation - completed as `v1.0.0-rc.1`;
- GitHub Release publication - completed as a pre-release;
- artifact upload - not applicable for this RC because binary installers and
  packages were explicitly excluded.
