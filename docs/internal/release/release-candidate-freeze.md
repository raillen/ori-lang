# Release Candidate Freeze

> Audience: maintainer
> Surface: release engineering
> Status: Phase 7 policy

A release candidate starts when maintainers freeze user-visible behavior.

After the freeze, changes should be narrow and easy to review.

## Allowed During Freeze

Allowed changes:

- P0/P1 bug fixes;
- documentation corrections;
- diagnostic wording that improves clarity without changing behavior;
- release notes;
- test fixes that do not hide real failures.

Avoid during freeze:

- new syntax;
- broad refactors;
- new package APIs;
- formatter output changes;
- lockfile format changes.

## Required Gates

Before publishing an RC:

- no known open P0/P1 bugs;
- `python build.py`;
- `zt check zenith.ztproj --all --ci`;
- `python run_suite.py smoke --no-perf`;
- `python run_suite.py pr_gate --no-perf`;
- Phase 7 release-engineering driver;
- Borealis package dogfood;
- clean install on Windows x64, Linux x64 and macOS arm64.

The current Windows workspace can verify the local gates. The three-platform
clean install is an external release gate and must be recorded in the release
report before a public tag.

