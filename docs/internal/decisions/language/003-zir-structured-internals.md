# Decision 003 - ZIR Structured Internals

- Status: accepted
- Date: 2026-04-16
- Type: architecture / compiler
- Scope: ZIR, backend, driver

## Summary

ZIR internally uses structured data types. ZIR textual format (`.zir` files) exists for debug, fixtures, and golden tests â€” not as the primary contract between compiler stages.

## Decision

The canonical ZIR representation is the in-memory `zir_module` struct hierarchy defined in `compiler/zir/model.h`.

The textual format produced by `zir_printer` and consumed by `zir_parser` is a secondary artifact with three purposes:

1. **Debug**: inspecting IR content during development
2. **Fixtures**: writing test cases without running the full frontend
3. **Golden tests**: comparing expected output against actual output

The frontend pipeline (M1â€“M5) will produce structured ZIR directly. It will not emit `.zir` text and re-parse it.

The backend (C target, legalization) will consume structured ZIR. It will not parse expression text out of instruction fields.

## Rationale

Stringly-typed expression fields (`expr_text`, `value_text`, `condition_text`, `message_text`) are an interim artifact from the current bootstrap phase. They create these problems:

- the C emitter parses expression text with `strcmp` chains â€” fragile and unscalable
- the verifier uses text-level token scanning for identifier usage â€” imprecise
- adding new operations requires changes in multiple string-matching locations
- `.zir` becomes a serialization format that competes with the real IR for attention

Moving to structured operands and terminators eliminates these problems. The textual printer and parser remain as dev tools, not as the architectural center.

## Consequences

- `compiler/zir/lowering/from_hir.h` and `from_hir.c` will produce structured ZIR
- `compiler/zir/model.h` will gain structured operand and terminator kinds (replacing `expr_text` as the primary path)
- `compiler/zir/printer.c` and `parser.c` remain maintained but are demoted to dev/debug status
- `compiler/targets/c/emitter.c` will be refactored to consume structured instructions instead of parsing expression text
- `compiler/targets/c/legalization.c` will be refactored similarly
- The driver will accept `.zt` source files; `.zir` file input will remain available as a debug feature
- This decision must be reflected in documentation that currently implies `.zir` is the primary input format
