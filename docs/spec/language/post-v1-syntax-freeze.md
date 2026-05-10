# Zenith Post-v1 Syntax Freeze (Wave 7.2)

> Audience: contributor, maintainer, language designer
> Status: frozen
> Surface: spec
> Last updated: 2026-05-02

This document closes Wave 7.2 from `post-v1-implementation-plan.md`.
It freezes syntax/keyword direction for post-v1 closure and removes open syntax questions.

## Freeze Rule

If change affects user-visible syntax, keyword reservation, shorthand forms, import grammar, or operator surface, it is frozen here.

Non-syntax topics (ABI, trait coherence, `any` safety, monomorphization model, runtime behavior) remain in later Wave 7 items.

## Accepted Syntax (Frozen)

| Topic | Freeze decision | Notes |
|-------|-----------------|-------|
| Dynamic dispatch spelling | User-facing spelling is `any` | Wave 7.4 migration policy is closed in `post-v1-any-migration.md`. |
| Match fallback clause | `case else:` is canonical | `case default:` is not accepted surface syntax. |
| Contextual keywords | `then` is contextual, not reserved | Can be used as an identifier outside keyword positions. |
| Enum shorthand | `.Variant` / `.Variant(payload)` accepted when expected enum type is known | Type-directed shorthand only. |
| Closure shorthand | Single-expression closure syntax accepted:<br>`func(x: int) x * 2` | Multi-statement closure with `end` remains valid. |
| Closure return inference | Accepted according to callable typing rules in language reference | Exact inference/diagnostic behavior audited in Wave 7.9 and 7.17. |
| Callable type syntax | `func(T) -> U` accepted | First-class callable type surface is frozen. |
| Generic argument inference | Argument-position inference accepted | No return-context inference, no partial inference. |
| Import model | Qualified imports only | Selective imports remain rejected. |
| Operator model | Trait-based operator surface only (current Level 2 contract) | No arbitrary symbolic operators or precedence customization. |

## Rejected Syntax (Frozen)

| Topic | Freeze decision | Reason summary |
|-------|-----------------|----------------|
| Full local type inference | Rejected (`const x = 42` remains invalid) | Explicit local typing remains language rule. |
| Struct type omission | Rejected (`{ fields }` without explicit type) | Keep explicit, single-form struct construction. |
| Selective imports | Rejected | Preserve clear symbol provenance via qualified imports. |
| `default` in match | Rejected in favor of `else` | One canonical fallback spelling. |
| Implicit return | Rejected | Explicit return semantics remain language rule. |
| `async/await` keywords | Rejected | Async route remains jobs/channels model, not syntax-level async functions. |
| `unless` keyword | Rejected | `if not` remains canonical form. |
| Rest operator (`...`) | Rejected | Not part of Zenith syntax model. |

## Wave 7.2 Completion Checklist

- accepted/rejected syntax list consolidated in one closure document;
- keyword/contextual-keyword direction frozen;
- shorthand direction frozen (`any`, enum dot shorthand, closure shorthand, struct omission rejected);
- import/operator/inference syntax direction frozen for post-v1 closure;
- remaining uncertainty moved to non-syntax waves (7.4+, 7.9+, 7.17+).

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - roadmap and Wave statuses.
- `post-v1-closure-matrix.md` - operational tracker for remaining closure work.
- `post-v1-surface-contract.md` - canonical accepted/rejected direction.
- `language-reference.md` - grammar, examples, and detailed user-facing specification.
