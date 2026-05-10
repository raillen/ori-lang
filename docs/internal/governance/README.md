# Governance 2.0 (R2.M0)

This folder stores the governance baseline for cycle 2.0.

## Fast start

Run one command for cycle triage:

```powershell
python tools/triage_cycle.py
```

Freeze baseline (quality + perf):

```powershell
python tools/triage_cycle.py --freeze-baseline
```

## Main files

- `docs/internal/governance/r2-m0-governance.md`: owners, SLA, bug flow, P0 block rule.
- `docs/internal/governance/baselines/quality-baseline.json`: frozen quality baseline.
- `docs/internal/governance/baselines/perf-baseline.json`: frozen perf baseline snapshot.
- `tests/perf/baselines/<platform>/*.json`: per-benchmark perf baselines.
- `CONTRIBUTING.md`: mandatory bug report fields and regression checklist.
- `tools/triage_cycle.py`: single triage command.

## Triage artifacts

Each run writes:

- `docs/internal/reports/triage/latest.md`
- `docs/internal/reports/triage/latest.json`
- `docs/internal/reports/triage/triage-<timestamp>.md`
- `docs/internal/reports/triage/triage-<timestamp>.json`
