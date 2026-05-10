# Final Language + Implementation Audit — 2026-05-03

> Status atual: historico/superseded.
> Para a decisao atual de RC, use
> `docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md`
> e `docs/internal/reports/audit/rc-public-release-gap-closure-2026-05-08.md`.
>
> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: draft audit  
> Surface: internal report / language closure audit  
> Source of truth: no — recommendations for consolidation

## Purpose

This report audits the current Zenith repository against the language decisions and implementation work made so far. It focuses on:

- discrepancies between design decisions, specs, docs, tests, and implementation;
- incongruences with Zenith's reading-first / explicit / predictable philosophy;
- weak and strong points in the current language design;
- files, directories, and documents that appear dead, stale, duplicated, or mergeable.

This report should be treated as a cleanup and finalization agenda before declaring the final structural implementation of the language.

## Executive Summary

Zenith's core identity is now strong: explicit types, qualified imports, `result`/`optional`, textual blocks, traits/apply, controlled `any`, and library-level concurrency/memory all fit the manifesto well.

The main risk is not lack of design. The main risk is **document drift**:

- `v1-surface-contract.md` still says some features are deferred/rejected even though post-v1 closure has implemented or accepted them.
- `post-v1-trait-stability.md` contains older `any` restrictions that conflict with the newly approved final `any` design.
- `post-v1-surface-contract.md` still has Exploration sections for topics already decided in `post-v1-remaining-language-work.md`.
- Several old tests and root-level `.zt` fixtures use obsolete syntax such as `await`, `Outcome.Success`, implicit `var x =`, `pub`, `native lua`, dynamic `any` field access, and old stdlib APIs.
- The docs tree contains multiple roadmap/checklist/readiness layers that now overlap with Wave 7 closure artifacts.

The implementation direction is coherent, but the repository needs a **normative source hierarchy** and a **stale artifact purge**.

## Recommended Source-of-Truth Hierarchy

Use this order when conflicts exist:

1. `docs/spec/language/post-v1-remaining-language-work.md` for decisions made after Wave 7.20.
2. Wave 7 closure artifacts, especially:
   - `post-v1-syntax-freeze.md`
   - `post-v1-idiom-pass.md`
   - `post-v1-any-migration.md`
   - `post-v1-any-dispatch-stabilization.md`
   - `post-v1-trait-stability.md`
   - `post-v1-monomorphization-closure.md`
   - `post-v1-runtime-abi-ownership-audit.md`
   - `post-v1-final-language-closure-review.md`
3. `post-v1-implementation-plan.md` for wave order and validation evidence.
4. `post-v1-surface-contract.md` for post-v1 scope after it is updated.
5. `v1-surface-contract.md` only for historical v1 boundary, not final post-v1 language.
6. `docs/internal/decisions/language/*` as rationale/history, not current syntax if newer specs supersede them.
7. `docs/internal/planning/*` as planning history, not normative implementation truth.

## High-Risk Discrepancies

### 1. `v1-surface-contract.md` is stale versus current implementation

Observed contradictions:

- It lists `Generic argument inference` as deferred, but Wave 2.2 / Wave 7.7 show it implemented for arg-position inference.
- It lists `async/await, tasks, channels` as deferred. `async/await` remains rejected, but jobs/channels/shared/atomic are now implemented as Wave 4 concrete `int` surfaces with typed facades.
- It says broad `Overloading` is rejected, but operator overloading Level 2 is accepted and implemented. The wording should distinguish arbitrary overloading from fixed operator traits.
- It says `group` may be considered later, but `group` alias is implemented in Wave 1.4.
- It lists struct literal shorthand `{ fields }` as accepted but not implemented, while `post-v1-remaining-language-work.md` now explicitly rejects bare struct literal shorthand as a non-goal.

Recommendation:

- Mark `v1-surface-contract.md` as historical v1 snapshot, or update it to point readers to post-v1 closure as the final current language contract.
- Remove or rewrite lines that now contradict final closure.

### 2. `post-v1-trait-stability.md` conflicts with final `any` design

Current older constraints in `post-v1-trait-stability.md`:

- `any<GenericTrait<T>>` is rejected.
- mutating methods are not allowed.
- managed params/returns are not allowed.
- all params/returns must be copyable.
- max 8 methods is fixed.
- future work still lists generic traits, explicit apply precedence, and self-referential traits as deferred.

Newer approved decisions in `post-v1-remaining-language-work.md`:

- Generic traits with semantic parameters are accepted for the first advanced trait model.
- `any<Stream<bytes>>` is allowed when all parameters are concrete or resolved.
- managed params/returns are part of the final design.
- mutating trait methods are allowed through explicit mutability and mutable `any` bindings.
- `Self` is allowed for static dispatch, with restricted object safety for `any`.
- overlapping applies remain rejected; no precedence syntax.

Recommendation:

- Split `post-v1-trait-stability.md` into:
  - current implemented Wave 7.6 subset;
  - final intended trait/`any` contract from `post-v1-remaining-language-work.md`.
- Or update it directly with a section: `Superseded by post-v1-remaining-language-work.md for final generic trait and advanced any policy`.

### 3. `post-v1-surface-contract.md` still has Exploration sections for decided topics

Examples:

- Custom allocator hooks / hot-reload are still Exploration there, but `post-v1-remaining-language-work.md` now decides `std.mem` advanced allocation and hot reload scope.
- FFI user structs, extern variables, variadics, conditional externs are still open/exploration there, but are decided in the remaining-work doc.
- Tooling extension system, `zt bench`, editor config policy, and Borealis Studio are now decided, but older post-v1 surface contract still treats several as open.
- `std.crypto`, `std.image`, `std.db`, `std.regex` expansion are decided, while older post-v1 surface contract lists them as exploration.

Recommendation:

- Update `post-v1-surface-contract.md` to reference `post-v1-remaining-language-work.md` as follow-up closure.
- Remove or convert outdated Exploration tables to `Decided in follow-up` tables.

### 4. `post-v1-final-language-closure-review.md` says no nebulous language topics remain, but `post-v1-remaining-language-work.md` was later used for more decisions

This is not a fatal contradiction if interpreted as Wave 7.20 closure only. But wording like “no language topic remains without decision” is now too broad because subsequent sessions added/refined final decisions for reflection, generic traits, `any`, stdlib boundaries, tooling extension, etc.

Recommendation:

- Add note: Wave 7.20 closed the mandatory compiler/runtime closure gate; later follow-up design decisions are recorded in `post-v1-remaining-language-work.md` and should be folded into final contracts.

### 5. Root-level and legacy `.zt` tests contain obsolete syntax

Tracked old tests include patterns that are no longer canonical or likely no longer compile:

- `tests/test_stdlib.zt` uses `Outcome.Success`, `Outcome.Failure`, old fs names like `write_text_file`, and implicit `var path =`.
- `tests/test_os_time_compat.zt` uses `native lua`, implicit `var`, and old time/os API names.
- `tests/test_os_time.zt` and `tests/test_fase8.zt` use old time API names like `get_cpu_time`, `get_timestamp`, `format_date`.
- `tests/stdlib/test_udp.zt`, `test_tcp_server.zt`, `test_http.zt` use `pub async func`, `await`, `Success`/`Failure`, dynamic `any` field access, and APIs beyond current accepted stdlib surface.
- `tests/stdlib/test_reflect.zt` and `tests/stdlib/test_math.zt` use `var x: any = ...` and dynamic field/method access inconsistent with final `any<Trait>` and reflection decisions.

Recommendation:

- Move legacy exploratory `.zt` files into `docs/internal/archive/legacy-tests/` or delete if not used by any runner.
- Keep only behavior projects under `tests/behavior/*` and C/Python hardening tests as authoritative.
- If some legacy tests are valuable, convert them into `tests/behavior/<feature>/src/app/main.zt` with canonical syntax.

## Medium-Risk Discrepancies

### 1. `any` terminology migration is incomplete in filenames and internal names

The source has user-facing content migrated to `any`, but internal fixture names still use `dyn_*`:

- `tests/behavior/dyn_dispatch_basic`
- `tests/behavior/dyn_trait_heterogeneous_collection`
- `tests/behavior/dyn_generic_trait_error`
- `tests/behavior/list_dyn_textrepresentable`
- `tests/behavior/list_dyn_trait_basic`
- diagnostic enum names like `ZT_DIAG_DYN_*`

This may be acceptable as internal compatibility, but it creates cognitive noise.

Recommendation:

- Rename fixtures to `any_*` when practical.
- Keep `dyn` only in explicit migration/deprecation tests.
- Internally, consider aliasing diagnostic enum names or renaming to `ZT_DIAG_ANY_*` in a controlled migration.

### 2. `std.mem` final design is broader than current implementation

Current implementation:

- `own_text`, `view_text`, `edit_text`
- list-text equivalents

Final direction:

- monomorphized `own<T>`, `view<T>`, `edit<T>`
- `mem.Temp`
- `mem.Pool`

This is not inconsistent if tracked as future work, but active docs should avoid implying generic APIs already exist.

Recommendation:

- Add implementation-status labels to `std.mem` docs: current concrete anchors vs final generic direction.

### 3. Reflection design is final but not implemented

Final design:

- builtin derivable `Reflect`
- `@derive(Reflect, expose: ...)`
- `@reflect` and `@reflect(hidden)`
- explicit `std.reflect` API
- no dynamic field access

Potential stale tests:

- `tests/stdlib/test_reflect.zt` appears to assume runtime metadata and dynamic `any` access patterns.

Recommendation:

- Delete/archive old reflection test or rewrite as future spec fixture once attributes/derive exist.
- Add a `post-v1-reflection-contract.md` or fold the current decision into `language-reference.md` / `post-v1-surface-contract.md`.

### 4. `std.net` and `std.http` docs must distinguish shipped foundation vs final scope

Current implementation:

- `std.net`: blocking TCP client only.
- `std.http`: blocking HTTP client get/post only, no TLS.

Final decisions:

- `std.net`: TCP client/server + stream/sink adapters + TLS wrapper.
- `std.http`: client/server foundation, request/response/headers, blocking core, async helpers via jobs/channels.

Recommendation:

- Add “Current executable subset” and “Final accepted scope” subsections to stdlib docs.
- Avoid saying final APIs are shipped before implementation exists.

### 5. `Transferable` and concurrency surfaces are conceptually final but runtime-limited

Current implementation uses `int` concrete runtime handles. Final direction wants `Job<T>`, `Channel<T>`, `Shared<T>`, `Atomic<T>`.

Recommendation:

- Keep current `int` APIs documented as C-backend executable subset.
- Do not expose them as the final ergonomic public API if final goal is typed handles.
- Add backend capability diagnostics for unsupported non-`int` payloads.

## Philosophy Alignment Audit

### Strong alignment

- **Explicit type declarations:** strongly aligned with reading-first and low neural friction.
- **Qualified imports:** very aligned; source origin is visible.
- **`result<T,E>` / `optional<T>`:** aligned; separates recoverable failure from absence.
- **No `try/catch`, no null coalescing, no safe-navigation punctuation:** aligned with predictable failure semantics and low symbol density.
- **Traits + `apply`:** aligned with composition over inheritance.
- **`mut func`:** excellent alignment; mutation capability is visible at declaration.
- **`using`:** aligned when deterministic cleanup is clearly documented.
- **No macros:** strongly aligned; avoids hidden expansion and cognitive traps.
- **No full local type inference:** aligned with reading-first philosophy.
- **Generic traits with semantic parameter names instead of associated types:** strongly aligned; avoids projection syntax like `Self.Item` while retaining expressiveness.
- **Foundation stdlib + official packages:** aligned; core stays understandable and portable.
- **External tooling hooks instead of compiler plugins:** aligned; avoids hidden compiler behavior.

### Moderate alignment / watch carefully

- **Pipe operator `|>`:** useful for readability in pipelines, but adds symbolic syntax. Acceptable because it has one clear meaning.
- **Operator overloading Level 2:** useful but philosophically risky. Acceptable only because it is fixed to known operator traits and not arbitrary.
- **`@field` self shorthand:** concise but symbol-heavy. Acceptable only if restricted to `apply` and formatted consistently.
- **`group` alias for `tuple`:** possible readability gain, but creates two names for same concept. This weakens “one form” unless docs clearly define canonical preference.
- **`any<Trait>`:** powerful and necessary, but should remain object-safe and explicit. Avoid dynamic “object” behavior.
- **`@derive(Reflect)` / attributes:** attributes introduce meta-syntax. Acceptable because opt-in and explicit, but avoid attribute proliferation.
- **`zt migrate`:** good for evolution, but should not become a reason to churn syntax.

### Weak alignment / potential incongruence

- **Bare struct literal shorthand `{ fields }`:** correctly rejected. It hides type context and weakens readability.
- **Associated types:** correctly deferred. They add type projection and coherence complexity that feels less Zenith.
- **Generic methods in `any`:** correctly rejected. They imply dynamic monomorphization/runtime generic dispatch.
- **In-process compiler plugin API:** correctly rejected. It would violate predictability and toolchain inspectability.
- **Dynamic field access through `any`:** should be rejected. Some old tests imply this; they should be deleted or archived.
- **`native lua` / old experimental bridges:** not coherent with final C/runtime/FFI story unless explicitly isolated as historical.
- **Old `Outcome.Success`/`Failure` naming:** conflicts with final `result`/`success`/`error` style and should be purged from active tests.

## Language Design Strengths

- **Clear identity:** Zenith now has a distinct language personality rather than being a blend of Rust/Go/Python.
- **Strong failure model:** `result`, `optional`, `?`, `.or_return`, `.or_wrap`, and panic boundaries create a teachable model.
- **Composition model:** traits/apply avoid inheritance while keeping behavior modular.
- **Pragmatic systems path:** C backend, runtime ABI, FFI, ORC hooks, and future LLVM/WASM are staged realistically.
- **Readable generic direction:** semantic generic parameters (`Stream<Item>`, `Parser<Output>`) provide a strong middle ground.
- **Controlled dynamic dispatch:** `any<Trait>` is powerful without becoming universal runtime reflection.
- **Stdlib boundary discipline:** keeping crypto/image/db/frameworks mostly out of core avoids stdlib bloat.
- **Accessibility-first philosophy:** the manifesto is unusually concrete and useful as a design filter.

## Language Design Weaknesses

- **Too many closure documents:** Wave 7 produced many precise artifacts, but readers now need to know which document wins.
- **Terminology drift:** `dyn`, `any`, `Outcome`, `result`, `pub`, `public`, and old APIs coexist in tests/docs.
- **Feature status ambiguity:** some docs say “Done” while meaning “executable subset done,” not final surface done.
- **Symbol budget pressure:** `?`, `@field`, `|>`, attributes, generic angle brackets, and potential derive syntax are acceptable individually, but together need careful formatting/teaching.
- **Two names for same concept:** `tuple` and `group` may create unnecessary choice unless one is clearly canonical.
- **Operator overloading tension:** even restricted operator traits complicate the “no hidden behavior” story.
- **Runtime/backend subset naming:** `*_int` APIs for jobs/channels/atomic are implementation-friendly but not final-user elegant.
- **Docs are ahead and behind simultaneously:** final decisions exist, implementation subsets exist, but consolidated public/reference docs do not yet express the exact current/final distinction.

## Recommended Improvements Before Final Structural Implementation

### 1. Create a single `final-language-contract.md`

Create one normative consolidation document that lists final decisions with implementation status:

- syntax
- types/generics
- traits/apply
- `any`
- memory/ownership
- concurrency
- FFI
- stdlib boundary
- tooling boundary

Each row should have:

- `Final decision`
- `Current implementation`
- `Gap`
- `Canonical doc`

### 2. Add explicit labels: `Final Contract`, `Current Executable Subset`, `Historical`

Many contradictions disappear if docs state their layer clearly.

Suggested labels:

- `Final Contract`: approved language shape.
- `Current Executable Subset`: what C backend currently executes.
- `Historical`: old decision/rationale only.
- `Migration Context`: old spelling retained intentionally.

### 3. Collapse old planning documents into indexes

Keep detailed history, but do not let old planning docs compete with closure specs.

### 4. Enforce docs syntax lint

There is already `tools/check_docs_current_syntax.py`. Expand it to flag:

- public `dyn<Trait>` outside migration context;
- `Outcome.Success` / `Outcome.Failure`;
- `pub` if `public` is canonical;
- `async` / `await` in active docs/tests;
- implicit `var x =` in active `.zt` fixtures;
- `native lua` outside archive;
- dynamic `any` field access examples.

### 5. Purge or archive obsolete tests

Tests should either be authoritative or explicitly historical. Active stale fixtures are dangerous because they teach the wrong language.

## Candidate Files / Directories To Delete Or Archive

Do not delete blindly. First verify whether any runner, docs link, or release process still references them.

### Strong candidates: generated/ignored artifacts

These are safe cleanup targets if not needed for local debugging:

- `website/node_modules/`
- `website/dist/`
- `website/.astro/`
- `website/.vscode/`
- root binaries:
  - `zt.exe`
  - `zt-lsp.exe`
  - `zt-next.exe`
  - `zpm.exe`
  - `zt`
  - `zt-lsp`
  - `zpm`
- `tools/vscode-zenith/bin/`
- `tools/vscode-zenith/runtime/`
- `tools/vscode-zenith/stdlib/`
- `tools/vscode-zenith/lsp.log`
- `tests/**/build/`
- `tests/**/.ztc-tmp/`
- `tests/**/*.exe`
- `tests/**/__pycache__/`
- `tools/**/__pycache__/`
- `tests/tmp/`
- `tests/behavior/tmp_list_assign_probe/`

These are already mostly ignored according to `git status --ignored`, but local cleanup would reduce noise.

### Strong candidates: deleted/untracked temporary project remnants

`git status` shows deleted tracked files:

- `tmp_test/src/app/main.zt`
- `tmp_test/zenith.ztproj`

Recommendation:

- If this was a temporary probe, remove the directory from tracking via a normal cleanup commit.
- If it was intended as a fixture, move it under `tests/behavior/<name>/` with canonical syntax.

### Strong candidates: legacy root `.zt` tests

These appear to be old standalone experiments and many use obsolete syntax/API:

- `tests/test_stdlib.zt`
- `tests/test_os_time_compat.zt`
- `tests/test_os_time.zt`
- `tests/test_fase8.zt`
- `tests/test_diagnostics.zt`
- `tests/test_v025_grammar.zt`
- `tests/phase5_bootstrap.zt`
- `tests/phase5_control_flow.zt`
- `tests/phase5_data_structures.zt`
- `tests/debug_if_global.zt`
- `tests/debug_if_struct.zt`
- `tests/debug_list_mutation.zt`

Recommendation:

- Archive or delete after confirming they are not used by `run_all_tests.py`, `run_suite.py`, or CI.
- Convert only valuable cases into behavior fixtures.

### Strong candidates: stale `tests/stdlib/*.zt`

Many `tests/stdlib/*.zt` files predate the current behavior-project test structure and contain obsolete syntax or APIs:

- `tests/stdlib/test_udp.zt`
- `tests/stdlib/test_tcp_server.zt`
- `tests/stdlib/test_http_server.zt`
- `tests/stdlib/test_http.zt`
- `tests/stdlib/test_reflect.zt`
- `tests/stdlib/test_math.zt`
- `tests/stdlib/test_time.zt`
- `tests/stdlib/test_text.zt`
- others in `tests/stdlib/`

Recommendation:

- Treat `tests/behavior/std_*` as authoritative.
- Archive the old `tests/stdlib/*.zt` directory unless a runner still uses it.

### Strong candidates: stale semantic exploratory tests

The following directories contain many old language experiments:

- `tests/semantic/*.zt`
- `tests/semantic_tests/*.zt`
- `tests/core/*.zt` files with old features such as spread/async/union experiments
- `tests/ascension/*.zt`

Recommendation:

- Keep only if actively used by a named runner.
- Otherwise archive as historical compiler-learning material.
- Convert still-relevant tests to behavior fixtures with canonical syntax.

### Candidate duplicate/heavy fuzz trees

`git ls-files` shows both:

- `tests/heavy/fuzz/semantic/...`
- `tests/heavy/tests/heavy/fuzz/semantic/...`

This looks like duplicated nested heavy fixtures.

Recommendation:

- Audit whether `tests/heavy/tests/heavy/...` is accidental duplication.
- If duplicate, delete nested copy or move to archive.

## Documentation That Can Be Deleted, Archived, Or Marked Historical

### Superseded specs already identified by `docs/spec/language/README.md`

The README says these are superseded support material:

- `docs/spec/language/surface-syntax.md`
- `docs/spec/language/closures.md`
- `docs/spec/language/dyn-dispatch.md`
- `docs/spec/language/callables.md`

Recommendation:

- Move to `docs/spec/language/archive/` or add `> Status: historical` headers.
- Keep only if they contain rationale not yet folded into current docs.

### Planning docs likely mergeable into indexes

The docs tree has many overlapping planning docs:

- `docs/internal/planning/language-readiness-roadmap.md`
- `docs/internal/planning/language-readiness-checklist.md`
- `docs/internal/planning/language-readiness-completeness-discussion.md`
- `docs/internal/planning/roadmap-v7.md`
- `docs/internal/planning/checklist-v7.md`
- `docs/internal/planning/tier-7-decision-reconciliation.md`
- `docs/internal/planning/editor-surface-contract-v1.md`
- `docs/internal/planning/editor-implementation-plan-v1.md`
- `docs/internal/planning/editor-checklist-v1.md`
- `docs/internal/planning/editor-completeness-discussion-v1.md`

Recommendation:

- Keep a single `planning-index.md` with links and statuses.
- Mark older ones `historical` or archive after key decisions are folded into `docs/spec/language/` or `docs/reference/`.

### Borealis planning docs conflict with “Borealis Studio external only”

There are many Borealis/Studio planning docs:

- `docs/internal/planning/borealis-checklist-v1.md`
- `docs/internal/planning/borealis-roadmap-v1.md`
- `docs/internal/planning/borealis-roadmap-v2.md`
- `docs/internal/planning/borealis-engine-roadmap-v2.md`
- `docs/internal/planning/borealis-studio-checklist-v1.md`
- `docs/internal/planning/borealis-studio-roadmap-v1.md`
- `docs/internal/planning/borealis-engine-studio-checklist-v3.md`
- `docs/internal/planning/borealis-engine-studio-roadmap-v3.md`

Recommendation:

- Move Borealis Studio docs out of core language planning or mark them as external ecosystem planning.
- Keep only package-level Borealis docs under `packages/borealis/` if still relevant.

### Public docs that may need update, not deletion

- `docs/public/guides/editor-vscode.md`
- `docs/public/guides/editor-lsp-configs.md`
- Japanese equivalents under `docs/public/jp/`

They still describe VSCode as the most complete path. That is fine operationally, but final policy is now LSP-first. Update wording to avoid implying VSCode is the official semantic path.

### `tools/vscode-zenith/README.md` needs terminology update

It mentions completion for `any<Trait>`/`dyn<Trait>`. Keep `dyn` only in migration context or remove from user-facing README.

## Docs That Should Be Merged

### Merge into `final-language-contract.md`

- `post-v1-remaining-language-work.md`
- relevant final sections of `post-v1-surface-contract.md`
- summary rows from `post-v1-implementation-plan.md`
- final decisions from `post-v1-trait-stability.md`
- final decisions from `post-v1-any-dispatch-stabilization.md`
- final decisions from `post-v1-runtime-abi-ownership-audit.md`

### Merge or cross-link language readiness docs

- `docs/spec/language/language-readiness-surface-contract.md`
- `docs/internal/planning/language-readiness-*`
- `post-v1-closure-matrix.md`
- `post-v1-final-language-closure-review.md`

Recommendation:

- Make one readiness/closure index.
- Archive older detailed plans after linking.

### Merge tooling policy docs

- `tools/decisions/*`
- `docs/spec/language/tooling-model.md`
- the new plugin/extension decision in `post-v1-remaining-language-work.md`

Recommendation:

- Ensure `tooling-model.md` reflects external declarative hooks, `zt bench` core-minimal, LSP-first, and Borealis Studio external-only.

## Implementation Gaps To Track Before Final Structural Lock

### Must clarify final-vs-current

- generic traits with semantic parameters: final design accepted; implementation status unclear/incomplete.
- `any<GenericTrait<Concrete>>`: final design accepted; current Wave 7.6 implementation rejects generic traits in `any`.
- managed returns/params through `any`: final design accepted; current implementation likely rejects or only supports copyable shapes.
- mutating methods through `any`: final design accepted with explicit mutability; current implementation rejects.
- reflection derive/attributes: final design accepted; not implemented.
- `std.net` server/listener: final accepted; not implemented.
- `std.http` server/request/headers/body model: final accepted; not implemented.
- `std.mem` generic helpers and `mem.Temp`/`mem.Pool`: final accepted; not implemented.
- typed non-`int` jobs/channels/shared/atomic: final route accepted; current runtime limited.
- `zt bench`: final accepted as core minimal; implementation status not verified in this audit.
- external declarative tooling hooks: final accepted; implementation status not verified.

### Should probably remain future

- full associated types;
- generic methods in `any`;
- broad operator overloading;
- compiler plugin API;
- `async/await` keywords;
- language-level lifetimes/borrow checking;
- JS backend.

## Suggested Cleanup Plan

### Phase A — Documentation truth repair

1. Add/update headers marking historical docs.
2. Update `post-v1-surface-contract.md` from `post-v1-remaining-language-work.md`.
3. Update `post-v1-trait-stability.md` to avoid conflict with final generic trait/advanced any decision.
4. Add `final-language-contract.md` as a single map from final decision to implementation status.
5. Update `docs/spec/language/README.md` reading order so post-v1/final closure docs come before old readiness docs.

### Phase B — Test tree cleanup

1. List which `.zt` fixtures are used by CI/suite runners.
2. Archive/delete unused root and legacy `.zt` tests.
3. Rename `dyn_*` fixtures to `any_*` except migration tests.
4. Convert valuable old stdlib tests into behavior projects.
5. Remove generated ignored build artifacts locally.

### Phase C — Implementation gap tickets

Create tracked issues or roadmap rows for:

- generic traits implementation;
- `any<GenericTrait<Concrete>>`;
- managed `any` ABI;
- mutable `any` ABI;
- reflection derive;
- std.net server/listener;
- std.http server foundation;
- std.mem generic helpers / Temp / Pool;
- typed concurrency runtime;
- `zt bench` minimal runner;
- external declarative tooling hooks.

### Phase D — Philosophy compliance pass

For every final feature, answer:

- Is behavior visible at call/declaration site?
- Does it introduce hidden inference or overload ranking?
- Does it increase symbol density?
- Can diagnostics explain it in ACTION/WHY/NEXT style?
- Is there one canonical form in docs?

## Final Opinion

Zenith's final language direction is coherent and distinctive. The strongest structural decision is choosing explicit, readable middle paths instead of maximal power: semantic generic trait parameters over associated types, object-safe `any` over universal objects, jobs/channels over `async/await`, foundation stdlib over batteries-included framework, and external tools over compiler plugins.

The weak point is repository hygiene. Old experiments, obsolete tests, and overlapping specs now obscure the language's final shape. The next strategic move should not be another feature. It should be a consolidation sprint: one final contract, stale docs marked historical, old tests archived, and implementation gaps tracked explicitly.
