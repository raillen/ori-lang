# Phase 6 Dogfood Report

Status: current.

Phase 6 starts with three small tools written in Zenith:

- `tools/selfhost/link-checker`
- `tools/selfhost/manifest-validator`
- `tools/selfhost/fixture-index`

## What worked

- `std.fs.exists`, `read_text`, `write_text`, `create_dir_all`, and `list_dir` were enough for useful repository checks.
- `std.text.contains` was enough for simple manifest validation.
- `std.console.write_line` and `error_line` gave readable CLI output without reaching for `std.io` directly.

## Gaps found

- Recursive directory walking is still manual.
- CLI argument parsing exists only as raw `std.os.args`.
- Text scanning is enough for simple checks, but richer Markdown and TOML parsing would reduce ad hoc validation.
- The C backend still needs `result` values materialized before `?` in some direct function-call cases.
- Text concatenation with `to_text(...)` can still lower through the numeric add path in some compound expressions; `std.text.join` is the supported workaround.

## Current decision

The gaps are not blockers for Phase 6 bootstrap.
The tools use the supported materialized-result style and `std.text.join` today.
Keep the tools small, use them as regression checks, and only add stdlib APIs when repeated tool code proves the need.
