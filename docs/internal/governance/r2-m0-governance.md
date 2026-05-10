# R2.M0 Governance Baseline

Status: active
Date: 2026-04-21
Scope: cycle 2.0

## Single triage command

Command:

```powershell
python tools/triage_cycle.py
```

This command must run before closing a cycle checkpoint.

What it runs:

1. `python run_all_tests.py`
2. `python tests/perf/run_perf.py --suite quick --release-gate`

Artifacts:

- `docs/internal/reports/triage/latest.md`
- `docs/internal/reports/triage/latest.json`

## Baseline freeze

Freeze command:

```powershell
python tools/triage_cycle.py --freeze-baseline
```

Frozen files:

- Quality: `docs/internal/governance/baselines/quality-baseline.json`
- Performance snapshot: `docs/internal/governance/baselines/perf-baseline.json`
- Performance per benchmark: `tests/perf/baselines/<platform>/*.json`

## Owners per layer

Current model is single-maintainer ownership.

| Layer | Paths | Owner |
| --- | --- | --- |
| Frontend | `compiler/frontend/**` | `@raillen` |
| Semantic | `compiler/semantic/**` | `@raillen` |
| IR (HIR/ZIR) | `compiler/hir/**`, `compiler/zir/**` | `@raillen` |
| Backend C | `compiler/targets/c/**` | `@raillen` |
| Runtime C | `runtime/c/**` | `@raillen` |
| Stdlib | `stdlib/**` | `@raillen` |
| Tooling and Driver | `compiler/tooling/**`, `compiler/driver/**`, `compiler/project/**` | `@raillen` |
| Tests and Perf | `tests/**`, `run_all_tests.py`, `tools/triage_cycle.py` | `@raillen` |
| Specs and Planning | `docs/spec/language/**`, `docs/internal/planning/**`, `docs/internal/governance/**` | `@raillen` |

Owner mapping is also enforced in `.github/CODEOWNERS`.

## Bug SLA by severity

Time targets are counted in calendar time.

| Severity | First response | Triage complete | Fix or mitigation |
| --- | --- | --- | --- |
| P0 | <= 2h | <= 4h | <= 24h |
| P1 | <= 1 day | <= 2 days | <= 7 days |
| P2 | <= 2 days | <= 5 days | <= 30 days |
| P3 | <= 5 days | <= 30 days | Backlog window |

## Mandatory bug report template

All bug reports must use:

- `CONTRIBUTING.md` (bug report required fields)

Minimum required fields:

- severity (`P0` to `P3`)
- affected layer
- minimal reproducible case
- exact command used
- expected result and actual result
- regression test plan

## Mandatory regression flow per bug

Every bug follows this order:

1. Open issue from bug template.
2. Set severity and owner.
3. Reproduce with minimal case.
4. Add failing regression test first.
5. Implement fix.
6. Run cycle triage command.
7. Open PR with regression checklist filled.
8. Merge only with green triage and linked regression test.

## P0 block rule

Hard rule:

- No merge to release cut with open `P0`.
- No alpha/release publish with open `P0`.

Exception rule:

- Allowed only with explicit risk acceptance file:
  - `docs/internal/reports/overrides/p0-<issue-id>.md`
- This file must include:
  - owner,
  - reason,
  - mitigation,
  - expiry date (max 7 days).

## Real flow evidence

The governance is considered "in use" only when all items below exist in git:

- Latest triage report in `docs/internal/reports/triage/latest.md`
- Frozen quality baseline in `docs/internal/governance/baselines/quality-baseline.json`
- Frozen perf baseline snapshot in `docs/internal/governance/baselines/perf-baseline.json`
- Frozen perf benchmark baselines in `tests/perf/baselines/<platform>/*.json`
- Active issue template and PR checklist in `.github/`



