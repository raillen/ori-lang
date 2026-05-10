# Phase 0 Stabilization Gate Evidence

> Audience: maintainer
> Date: 2026-04-28
> Scope: roadmap-v7 Phase 0 and global gate criteria

This report records the current stabilization evidence.

## Local Windows Evidence

Commands run in this workspace:

- `python build.py` — passed.
- `zt check zenith.ztproj --all --ci` — passed.
- `python run_suite.py smoke --no-perf` — passed, 37/37.
- `python run_suite.py pr_gate --no-perf` — passed, 235/235.
- `git diff --check` — passed, with Windows CRLF warnings only.

## Clean Clone Evidence

Phase 0 adds `.github/workflows/ci.yml`.

The workflow uses a clean `actions/checkout@v4` workspace on:

- `windows-latest`;
- `ubuntu-latest`.

Each platform runs:

- `python build.py`;
- `zt check zenith.ztproj --all --ci`;
- `python run_suite.py smoke --no-perf`.

The GitHub-hosted workflow run is the release evidence for Linux/Windows clean
clone once pushed.

## Public Surface Test Rule

For each new public surface:

- add a positive behavior test;
- add a negative behavior test when the surface can fail with a user error;
- update spec, decision or public docs in the same change.

Current coverage is tracked in:

- `tests/behavior/MATRIX.md`;
- `docs/spec/language/surface-implementation-status.md`;
- `run_suite.py`.

## Residual Risk

- macOS clean install is still a Phase 7/Phase 8 release gate.
- This local run cannot prove GitHub Actions status until the changes are pushed.
- The current workspace has uncommitted changes; release tagging must happen from a clean tree.

