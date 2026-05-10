# Zenith Diagnostics Model Spec

- Status: canonical closure spec
- Date: 2026-04-18
- Scope: compiler diagnostics, runtime diagnostics, stable codes and rendering

## Purpose

Diagnostics are part of Zenith's accessibility model.

A diagnostic must reduce confusion, not merely report failure.

## Information Model

Each diagnostic carries:

- severity
- stable code
- stage
- primary message
- file when available
- source span when available
- related spans when useful
- notes
- help
- machine-readable metadata when useful

Severity levels:

- `error`
- `warning`
- `note`
- `help`

## Canonical Terminal Shape

```text
error[type.mismatch]
Expected int, but found text.

where
  src/app/main.zt:8:21

code
  8 | const age: int = "18"
    |                  ^^^^
    |                  this is text

help
  Convert the value explicitly or change the variable type.
```

Rules:

- no color-only meaning
- no dense banners
- stable section labels
- small code excerpts
- plain language
- actionable help only when a concrete action exists

## Diagnostic Stages

Required stages:

- project
- lexer
- parser
- semantic
- lowering
- zir
- backend
- formatter
- runtime
- internal

Internal failures must be wrapped as compiler bugs, not leaked as raw implementation state.

## Stable Code Categories

Initial categories:

- `project.*`
- `syntax.*`
- `name.*`
- `type.*`
- `trait.*`
- `generic.*`
- `mutability.*`
- `result.*`
- `optional.*`
- `control_flow.*`
- `format.*`
- `style.*`
- `runtime.*`
- `internal.*`

## Required Runtime Codes

Current alpha runtime codes are:

- `runtime.check`
- `runtime.check`
- `runtime.contract`
- `runtime.index`
- `runtime.io`
- `runtime.math`
- `runtime.panic`
- `runtime.platform`
- `runtime.todo`
- `runtime.unreachable`
- `runtime.unwrap`

Notes:

- `runtime.index` is the current stable code for bounds/index failures.
- `runtime.math` currently covers divide-by-zero and arithmetic overflow.
- `runtime.platform` currently covers allocation failures, invalid UTF-8 invariants, and other host/runtime platform failures.
- Older proposal names such as `runtime.bounds`, `runtime.divide_by_zero`, `runtime.utf8`, `runtime.allocation`, and `runtime.map_key` are historical and are not the current alpha contract.

## Warning Policy

Warnings must be sparse and actionable.

Allowed initial warnings:

- visually confusing identifier names
- very similar local names in the same scope
- deeply nested blocks that are hard to scan
- functions with too many statements to scan quickly
- `case else` on a known enum when explicit variants are possible
- missing public ZDoc
- unresolved ZDoc link
- deprecated symbol in future

Formatting-only preferences do not belong in compiler-core warnings.
Readability warnings must point to a concrete risk and a clear next action.

Normal mode reports warnings without failing the build.

Strict diagnostics mode may promote selected warnings to errors.

## Renderer Requirement

The compiler must have one structured diagnostic model and multiple renderers later.

Required first renderer:

- detailed terminal renderer

Future renderers:

- short CI renderer
- JSON renderer
- LSP renderer

## Definition Of Done

Diagnostics are complete only when:

- all compiler stages use the structured model
- terminal renderer matches this spec
- stable codes are used in tests
- common syntax/type/mutability/result/optional/project/runtime errors have golden fixtures
- diagnostics avoid cascades where one root cause is enough
