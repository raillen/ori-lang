# Zenith Wave 7.16: Source Mapping Contract

> Audience: compiler/runtime implementer, tooling implementer  
> Status: audit artifact  
> Surface: compiler/tooling contract  
> Last updated: 2026-05-03

## Purpose

This document closes Wave 7.16 source mapping expectations for diagnostics, generated C, textual IR, and future debug information.

## Scope

This closure covers:
- source span preservation;
- diagnostic locations;
- generated C `#line` expectations;
- ZIR textual span stability;
- future debug-info expectations.

## Decisions

### S1: Source Spans

Every user-visible diagnostic for parser, semantic, lowering, verifier, and backend stages should carry the best available source span.

If a compiler-generated node has no direct source, it should point to the source construct that caused generation.

### S2: Diagnostic Locations

Diagnostics must prefer user source locations over generated artifacts.
Generated C locations are secondary evidence and must not replace the original Zenith span when available.

### S3: Generated C `#line`

Generated C may emit `#line` mappings for debugging, but language conformance depends on Zenith source spans.
`#line` output must be deterministic and must not expose unstable temporary paths where avoidable.

### S4: ZIR Spans

Textual ZIR dumps should preserve stable span metadata for golden fixtures and verifier reports.
Span syntax should be stable enough for tests to compare canonical output.

### S5: Future Debug Info

Future native debug info should map:
- function bodies to source functions;
- statement-level control flow to original statements;
- generated cleanup paths to the source resource statement;
- monomorphized instances to the generic declaration plus call-site instance metadata.

## Validation Envelope

Minimum validation set:
- parser invalid fixtures with expected span fragments;
- semantic invalid fixtures with expected span fragments;
- generated C smoke build for closure features;
- ZIR verifier failure paths with spans where available.

## Closure Result

Wave 7.16 is closed as a compiler/tooling contract.
Follow-up work is implementation hardening, not semantics design.

## Relationship To Other Documents

- `post-v1-diagnostic-contract.md`
- `post-v1-zir-consolidation.md`
- `post-v1-backend-conformance-suite.md`
