# Zenith Language Complete Analysis - 2026-05-08

> Audience: maintainer
> Scope: full language, compiler, runtime, stdlib, docs, tooling and release readiness
> Method: current repository evidence, not inherited conclusions from older audits

## Executive Summary

Current local result: **approved for public RC preparation, not yet "stable
release published"**.

There is no current local P0/P1 blocker in the language implementation. The
main gates are green:

- build passes;
- root check/test/doc gates pass;
- release suite passes 365/365;
- heavy semantic suite passes;
- documentation path and current-syntax checks pass.

The language is coherent and has a clear identity:

- explicit local types;
- no null;
- recoverable errors through `result<T,E>`;
- expected absence through `optional<T>`;
- deterministic cleanup through `using`;
- restricted, explicit composition through traits/apply;
- C backend as current behavior oracle.

The remaining work is not one big hidden blocker. It falls into four clear
groups:

1. release execution outside this local step;
2. public documentation maturity;
3. post-RC generic/runtime expansion;
4. future ecosystem and backend maturity.

## Evidence Run In This Analysis

| Command | Result | Evidence |
|---|---:|---|
| `python build.py` | PASS | `zt.exe` and `zpm.exe` rebuilt |
| `.\zt.exe check zenith.ztproj --all --ci` | PASS | `check ok` |
| `.\zt.exe test zenith.ztproj --ci` | PASS | `test ok (pass=1 skip=0)` |
| `.\zt.exe doc check zenith.ztproj` | PASS | `doc check ok` |
| `python tools\check_docs_paths.py` | PASS | `docs path check ok` |
| `python tools\check_docs_current_syntax.py` | PASS | `docs current syntax check ok` |
| `python run_suite.py release` | PASS | 365/365, `reports/suites/release__20260508T124944Z.json` |
| `python tests\heavy\run_heavy_tests.py --suite all` | PASS | `tests/heavy/reports/heavy-tests-all-20260508T125007Z.json` |

Known external release steps still not performed in this analysis:

- remote GitHub Actions matrix on final commit;
- commit/push of all RC changes;
- public tag;
- public release notes / announcement.

## Source Of Truth Assessment

Canonical implementation-facing documents are now reasonably clear:

- `docs/spec/language/final-language-contract.md`
- `docs/spec/language/zenith-language-spec.md`
- `docs/spec/language/v1-surface-contract.md`
- `docs/spec/language/syntax-semantics-by-topic.md`
- `docs/spec/language/stdlib-model.md`
- `docs/spec/language/stdlib-reference-by-topic.md`
- `docs/spec/language/runtime-model.md`
- `docs/spec/language/compiler-model.md`
- `docs/spec/language/tooling-model.md`
- `tests/behavior/MATRIX.md`

Current risk: old reports can confuse readers if treated as current truth.
The reference-page wording issues found here were closed in the follow-up
release-gap pass on 2026-05-08.

Examples:

- `reports/pending-language-issues-current.md` is historical and should not be
  used as the current RC blocker list.
- `reports/deep-analysis-report.md` is historical/operational background.
- `docs/reference/language/errors-and-results.md` was refreshed to separate
  current helpers from helpers outside the public subset.
- `docs/reference/language/feature-matrix.md` was refreshed as a release matrix
  with explicit post-RC documentation follow-ups.

Recommendation: keep one authoritative "current language status" page and mark
old reports explicitly as historical, corrected, or superseded.

## Language Design Assessment

### Strengths

The language direction is internally consistent.

- It favors explicitness over hidden behavior.
- It avoids common ambiguous features: `null`, broad exceptions, C-style `for`,
  broad overloads, magic imports, hidden async.
- It has a stable error philosophy: expected absence is `optional<T>`;
  recoverable failure is `result<T,E>`; fatal invariant failure is `panic`.
- It has a concrete readability goal for TDAH/dyslexia: shorter constructs,
  explicit flow, diagnostics with action guidance, and reduced hidden state.

The syntax is also mostly teachable:

- `namespace` and qualified imports make origin clear.
- `const`/`var` make mutability visible.
- `case else` is clearer than mixing `default`/`else`.
- `f"..."` for interpolation is familiar and avoids the removed `fmt"..."`.
- `any<Trait>` is clearer than selling universal dynamic behavior.

### Weaknesses

The language has a large surface for a young implementation.

The current compiler already covers many advanced features:

- traits/apply;
- generic functions and argument-position inference;
- `any<Trait>`;
- closures/callables;
- pattern matching;
- result/optional propagation;
- ORC/ARC ownership hooks;
- typed concurrency facades;
- stdlib foundation modules.

This is powerful, but it raises the maintenance bar. The main risk is not that
one feature is broken today. The risk is that future additions may expand the
surface faster than docs, conformance tests, and backend contracts can absorb.

## Implementation Assessment By Area

| Area | Current assessment | Evidence | Main remaining risk |
|---|---|---|---|
| Lexer/parser/syntax | OK for RC local | release suite, syntax coherence fixtures | keep old syntax out of public docs |
| Semantic checker | OK for current surface | behavior + heavy semantic suite | advanced bounds and broader generic trait shapes |
| Generics | Strong but bounded | direct/nested generic inference tests pass | runtime-generic surfaces are not universal |
| Traits/apply | OK for current subset | trait/default/overlap fixtures pass | richer trait shapes need hardening |
| `any<Trait>` | OK for object-safe subset | any/dyn fixtures pass | managed returns, generic traits, cross-thread use |
| Pattern matching | OK for current subset | enum/optional/guard/multivalue fixtures pass | OR/range/rest/deep patterns are deferred |
| Error model | OK for current subset | optional/result/panic fixtures pass | stale docs around helper availability |
| Runtime memory | OK for RC local | sanitizer/Valgrind evidence, release suite | cycle collection and advanced ownership APIs |
| Stdlib core | Broad and mostly coherent | stdlib behavior fixtures pass | some modules are foundation subsets, not full platforms |
| `std.collections` | Correctly documented as subset | collections fixtures pass | advanced structures are not fully generic yet |
| Concurrency | Useful facade, not mature general concurrency | wave4/jobs/channel/shared/atomic fixtures pass | non-`int` payload runtime storage, cancellation, backpressure |
| FFI | Useful narrow subset | extern C and callback fixtures pass | captured closures, managed values, vars, varargs |
| Tooling | Good local CLI/tooling | help, doc, fmt, release suite | LSP maturity, registry, installer, remote CI |
| Public docs | OK for RC local, with post-RC teaching improvements | docs checks pass, matrix refreshed | keep release notes aligned with accepted limits |

## Current Blockers

### P0

None found in this analysis.

### P1

None found locally for public RC preparation.

The language implementation passed all gates run in this analysis.

## Important Issues To Fix Before Calling It Stable

### P2-01 - Public documentation still lags behind implementation

The implementation and tests are ahead of parts of the public/reference docs.
This is the biggest user-facing risk.

Evidence found during this analysis:

- `docs/reference/language/feature-matrix.md` used to list several implemented
  or tested features as `partial` or `missing`.
- `docs/reference/language/errors-and-results.md` used old first-slice helper
  wording that no longer matched the current public surface.

Follow-up status on 2026-05-08:

- `feature-matrix.md` now separates `release-covered`,
  `reference-covered`, `contract-limited`, and `post-RC`.
- `errors-and-results.md` now separates current helpers from helpers outside
  the current public subset.

Impact:

- users may underuse implemented features;
- users may distrust the RC because docs look older than the code;
- old docs can contradict the clean RC decision.

Remaining post-RC improvement:

1. Add short public examples for closures, callables, `any<Trait>` and `where`.
2. Keep examples small and explicit.

### P2-02 - Advanced collection generics are intentionally incomplete

Current contract is clear and acceptable for RC, but not final language
ambition.

Current supported public subset:

- `list<T>`, `map<K,V>` and `set<T>` have meaningful generic support in the
  current backend subset;
- `queue_values<T>` and `stack_values<T>` are generic list-backed helpers;
- advanced `std.collections` structures are specialized:
  - `grid2d<int>`, `grid2d<text>`;
  - `grid3d<int>`, `grid3d<text>`;
  - `pqueue<int>`, `pqueue<text>`;
  - `circbuf<int>`, `circbuf<text>`;
  - `btreemap<text,text>`;
  - `btreeset<text>`.

Debt:

- `grid2d<T>`;
- `grid3d<T>`;
- `pqueue<T>`;
- `circbuf<T>`;
- `btreemap<K,V>`;
- `btreeset<T>`.

Recommended fix:

Do not rush this before RC. Make it a post-RC wave with:

1. ordering constraints or explicit comparator design;
2. monomorphized runtime storage;
3. COW/ARC tests for managed payloads;
4. positive and negative fixtures for each shape.

### P2-03 - Concurrency is a typed facade over a narrower runtime ABI

The user-facing direction is good: explicit jobs/channels/shared/atomic values,
no hidden scheduler, no `async/await`.

Current limitation:

- many typed surfaces are backed by `int` handles in the current C oracle;
- non-`int` runtime payload storage is not the same thing as the semantic
  generic facade;
- channel capacity/backpressure, cancellation and richer panic capture are
  future work.

Recommended fix:

Keep public docs honest: teach the typed facade, but say clearly which payloads
are executable today. Then implement typed storage or clear backend diagnostics
for unsupported payloads.

### P2-04 - FFI is useful but narrow

Current subset is adequate:

- `extern c`;
- ABI annotations;
- top-level primitive callbacks;
- immediate C calls;
- clear rejection of unsafe unsupported shapes.

Remaining gaps:

- captured closures through FFI;
- managed values crossing C boundaries;
- extern variables;
- varargs;
- conditional extern per target;
- user structs by value without explicit C representation.

Recommended fix:

Treat FFI expansion as a separate post-RC hardening track. Every new ABI shape
needs tests, docs, and clear ownership rules.

### P2-05 - Historical reports still look current

The old reports are useful, but some titles and sections now imply open
problems even when their entries say `Corrigido`.

Recommended fix:

1. Mark historical reports with a banner: "historical; superseded by current
   RC analysis".
2. Keep only one current backlog.
3. Move corrected findings out of "open issues" sections.

## Lower-Priority Technical Debt

### P3-01 - Backend conformance for future targets

C is the current oracle. That is fine. Before Zig/LLVM/WASM becomes real,
Zenith needs an automated backend conformance runner.

### P3-02 - Public docs are passing checks, but not complete as teaching material

Docs path checks prove links and syntax hygiene. They do not prove that a new
user can learn all implemented features smoothly.

Missing or weak teaching areas:

- closures and callables;
- `any<Trait>`;
- formatter rules;
- diagnostics;
- `where` contracts;
- advanced stdlib boundaries.

### P3-03 - Runtime cycle policy remains future work

Current policy is acceptable: RC cycles are leak risk, not undefined behavior.
Full cycle collection should not be implied until public APIs can create and
manage cycles intentionally.

### P3-04 - Ecosystem tooling remains early

Future work:

- `zt bench`;
- `zt migrate`;
- registry and package publishing;
- native installers;
- production LSP;
- VSCode marketplace extension;
- web playground.

## Accessibility And Readability Assessment

The language direction is unusually strong here.

Good choices:

- explicit types reduce inference surprises;
- `optional<T>` and `result<T,E>` make absence/failure visible;
- `case else` is easier to search and teach than multiple fallback names;
- rejection of broad overloads reduces hidden meaning;
- diagnostics and docs explicitly consider lower cognitive load.

Risks:

- the language now has many advanced features, so docs must be organized in
  small steps;
- symbols like `?`, `|>`, `..`, `<T>` and `@field` are acceptable, but need
  repeated examples and a compact cheat sheet;
- public docs should not list too many caveats on the first page.

Recommended teaching order:

1. file shape, namespace, imports;
2. values, `const`/`var`, primitive types;
3. functions and control flow;
4. optional/result;
5. collections;
6. structs/enums/match;
7. traits/apply;
8. generics;
9. closures/callables;
10. runtime/FFI/concurrency only after the core is comfortable.

## Final Recommendation

For **RC public local**: proceed.

For **stable public release**: do not call it stable until:

1. remote CI matrix passes on the final commit;
2. public/reference docs no longer contain stale "not implemented in first
   compiler slice" wording;
3. the feature matrix is updated to reflect current implementation;
4. historical reports are clearly marked historical or superseded;
5. release notes explain the current stdlib/generic/concurrency boundaries.

For **language evolution after RC**: focus on depth, not more surface syntax.

Priority order:

1. docs/status cleanup;
2. backend conformance runner;
3. advanced generic collections;
4. typed concurrency runtime payloads;
5. FFI expansion;
6. package registry/installers/tooling maturity;
7. alternative backends only after conformance is routine.

## Bottom Line

Zenith is no longer in a "does the language hold together?" phase. It does.

The current risk is maturity management: keeping documentation, conformance,
stdlib promises, runtime ABI and release process aligned as the language grows.
