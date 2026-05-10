# Zenith Legibility Evaluation Protocol

- Status: supporting closure protocol
- Date: 2026-04-18
- Scope: metrics, reading tasks, approval criteria and release gates for Zenith's reading-first manifesto

## Purpose

This document defines how to judge whether Zenith is legible in practice, not only coherent on paper.

The goal is to measure the user-facing effect of syntax, formatter, diagnostics and tooling on actual reading work.

## Evidence Base

The protocol is informed by the following findings:

- code readability and software complexity are negatively correlated in empirical studies;
- formatting elements affect code legibility, especially indentation, line length, block delimiters and spacing;
- the literature is still mixed on some formatting choices, so a single universal rule set is not fully supported;
- neurodiverse software engineers report real friction from overload, ambiguity and weak feedback loops;
- visual crowding and text spacing affect reading behavior, and dyslexic readers can be more sensitive to crowding;
- sustained attention is important for reading comprehension in ADHD.

## Inference

My engineering inference is that Zenith should optimize for:

- stable visual layout;
- low ambiguity;
- short feedback loops;
- explicit diagnostics;
- low context switching;
- minimal competing canonical forms.

These are implementation policy choices based on the evidence above. They are not universal scientific thresholds.

## Metrics

Use a small human-centered evaluation set with the following metrics:

- first-pass comprehension rate;
- time to first correct interpretation;
- error localization time;
- diagnostic comprehension rate;
- formatter convergence rate;
- lookup-free completion rate;
- subjective cognitive load;
- canonical-form consistency across examples and docs.

## Reading Tasks

Use short, concrete tasks that mirror actual Zenith work:

- explain what a snippet does in one sentence;
- locate the first error in a snippet;
- explain a `where` violation and how to fix it;
- read a `zenith.ztproj` file and identify `kind`, `source.root` and entrypoint;
- compare two equivalent snippets and choose the more legible one;
- read a diagnostic and state `what`, `where`, `why` and `how to fix`;
- read a `match` and determine whether it is exhaustive;
- read a function signature with named args and defaults and state the effective call shape.

## Approval Criteria

Release-facing language and tooling should pass the following thresholds:

- at least 80 percent of participants answer the core comprehension tasks correctly;
- at least 90 percent can localize the source of a diagnostic correctly;
- no canonical feature has two competing "official" forms in docs or examples;
- the formatter is idempotent;
- diagnostics explain what happened, where it happened, why it happened and how to fix it when that is possible;
- ordinary usage of the language does not require consulting the spec for basic interpretation;
- canonical examples in docs and fixtures match formatter output.

## Release V1 Gates

Zenith v1 should not release until the following are true:

- `zt fmt` exists and is mandatory in the canonical workflow;
- formatter golden tests cover imports, attrs, `where`, named args, structs, enums, `match` and comments;
- diagnostics are uniform across project, lexer, parser, semantic, lowering, ZIR, backend and runtime;
- diagnostics use stable codes and structured rendering;
- `zenith.ztproj` and the CLI model are coherent and user-facing;
- runtime behavior for `where`, collections, `optional`, `result`, bytes and UTF-8 is not ambiguous;
- conformance covers observable behavior, not just parsing;
- ZDoc is functional enough to keep public code clean;
- no release-critical feature remains with two conflicting canonical spellings or two conflicting docs.

## Notes

The thresholds above are deliberately conservative.

They are meant to protect the manifesto, not to claim scientific universal truth.

If a future validation round shows a better threshold for Zenith's audience, the protocol should be updated.

## References

- [A systematic literature review on the impact of formatting elements on code legibility](https://www.sciencedirect.com/science/article/pii/S0164121223001231)
- [An Empirical Study of the Relationships between Code Readability and Software Complexity](https://doi.org/10.48550/arXiv.1909.01760)
- [Understanding the Challenges Faced by Neurodiverse Software Engineering Employees](https://www.microsoft.com/en-us/research/publication/understanding-the-challenges-faced-by-neurodiverse-software-engineering-employees-towards-a-more-inclusive-and-productive-technical-workforce/?lang=ja)
- [The role of visual crowding in eye movements during reading: Effects of text spacing](https://link.springer.com/article/10.3758/s13414-023-02787-1)
- [Sustained attention plays a critical role in reading comprehension of adults with and without ADHD](https://www.sciencedirect.com/science/article/pii/S1041608023000444)