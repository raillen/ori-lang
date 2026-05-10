# Zenith Language Implementation Audit - 2026-04-29

Status: historical/superseded snapshot, audit-first, not a design decision file.

For current RC decisions, use
`docs/internal/reports/audit/implementation-review-rerun-2026-05-07.md` and
`docs/internal/reports/audit/rc-public-release-gap-closure-2026-05-08.md`.

Purpose: create one implementation-grounded view of what the language actually
supports today, what is only documented, what is only tested, and where docs,
tests, tooling, and compiler behavior disagree.

This report was created because v8 planning found large divergence between
documentation and implementation, especially around callables, closures,
collections, and tooling.

## Executive Summary

The language is more implemented than several reference documents say.

The largest confirmed divergence is:

- `docs/reference/zenith-kb/not-implemented.md` still lists first-class
  functions and lambda expressions as deferred.
- The compiler, runtime path, stdlib, and behavior tests already support
  callable types, named function references, indirect calls, block closures,
  expression lambdas, immutable by-value captures, and escape checks.

There are also real current failures:

- `tests/conformance/test_m16.exe` currently reports `114/122 passed`.
- `multifile_private_access` builds successfully even though the fixture expects
  access to a private imported symbol to fail.
- `value_semantics_struct_managed` fails during backend/legalization with
  `unable to legalize structured list_len operand`.
- `list_dyn_trait_basic` fails in the M16 harness and needs focused reproduction
  after the audit.
- `result_optional_propagation_error` now reports `check ok`, while the M16
  harness still expects an older backend error. This is likely stale expected
  behavior, but must be reclassified deliberately.

The behavior matrix is not complete:

- `tests/behavior` currently has 199 project directories.
- At least 38 behavior project directories do not appear as project entries in
  `tests/behavior/MATRIX.md`.

The stdlib zdoc tree is also incomplete:

- `stdlib/std` has 23 `.zt` modules.
- `stdlib/zdoc/std` has 21 `.zdoc` files.
- Missing direct zdoc files: `lazy.zdoc`, `set.zdoc`.

The VSCode TextMate grammar is behind the LSP:

- LSP treats `print` as a builtin.
- `tools/vscode-zenith/syntaxes/zenith.tmLanguage.json` does not list `print`
  in its builtin function pattern.

## Validation Evidence

Commands run in this audit:

```powershell
python build.py
.\tests\conformance\test_m16.exe
python tests\driver\test_cli_output_clean.py
python tests\driver\test_repl.py
python tests\driver\test_create_scaffold.py
.\zt.exe run tests/behavior/list_dyn_trait_basic/zenith.ztproj --ci
.\zt.exe run tests/behavior/value_semantics_struct_managed/zenith.ztproj --ci
.\zt.exe build tests/behavior/multifile_private_access/zenith.ztproj --ci
.\zt.exe verify tests/behavior/result_optional_propagation_error/zenith.ztproj --ci
```

Observed results:

| Check | Result | Notes |
| --- | --- | --- |
| `python build.py` | pass | Rebuilt `zt.exe` and `zpm.exe`. |
| `test_m16.exe` | fail | `114/122 passed`. |
| `test_cli_output_clean.py` | pass | CLI output remains compact. |
| `test_repl.py` | pass | REPL quiet-mode behavior passes. |
| `test_create_scaffold.py` | pass | Scaffold creates/checks/runs current hello-world style. |
| `list_dyn_trait_basic` direct run | fail | Needs focused reproduction; M16 also failed it. |
| `value_semantics_struct_managed` direct run | fail | Backend/legalization failure. |
| `multifile_private_access` direct build | pass unexpectedly | Visibility check regression or fixture expectation mismatch. |
| `result_optional_propagation_error` direct verify | pass unexpectedly | Likely stale negative fixture after semantics changed. |

## Current Implementation Surface

### Lexer and Syntax

Confirmed from `compiler/frontend/lexer/token.h`,
`compiler/frontend/parser/parser.c`, formatter support, and behavior fixtures.

Implemented syntax families include:

- `namespace`
- `import ... as ...`
- `public`
- `const`, `var`
- `func`
- `struct`
- `trait`
- `apply`
- `enum`
- `extern`
- `type` aliases
- `where`
- `using`
- `if` statement
- `if` expression
- `while`
- `repeat ... times`
- `for ... in ...`
- `break`, `continue`
- `match`, `case`, `case else`
- `some`, `none`, `success`, `error`
- `optional<T>`, `result<T,E>`
- `list<T>`, `map<K,V>`, `set<T>`
- `any<Trait>` / `dyn<Trait>`
- callable type syntax: `func(T) -> R`
- block closures: `func(...) -> T ... end`
- expression lambdas: `func(...) => expr`
- same-line expression closure form: `func(...) expr`
- `fmt`/`f` interpolation
- `hex bytes "..."` literals
- list literals: `[a, b]`
- map literals: `{ key: value }`
- set literals: `{ a, b }`
- index and slice: `value[i]`, `value[start..end]`
- field assignment and index assignment

Deferred or rejected syntax still listed in docs includes:

- `unless`
- pipe operators
- `?.`
- `??`
- wildcard imports
- relative imports
- selective imports
- implicit `self.`
- C-style `for`
- numeric range `for`
- async/await
- macros
- nested functions
- user-facing C interop
- conditional compilation

Audit note: some "deferred" entries are stale. Callable/lambda items in
`not-implemented.md` are no longer accurate.

### AST, Formatter, and LSP Coverage

`compiler/frontend/ast/model.h` exposes a broad AST:

- declarations: file, namespace, import, func, struct, trait, apply, enum,
  extern, type alias;
- types: simple, generic, dyn/any, callable;
- statements: block, if, while, for, repeat, return, var, const, assignment,
  index assignment, field assignment, match, using, break, continue, expr;
- expressions: binary, unary, call, field, enum-dot, index, slice, int, float,
  string, bytes, bool, none, success, error, list, map, set, struct literal,
  ident, fmt, grouped, if expression, closure, bindings.

Formatter support exists in `compiler/tooling/formatter.c` for the same broad
set, including callable types and closure expressions.

LSP support exists in `compiler/driver/lsp.c` for:

- parse-backed symbols;
- local scopes;
- closure parameters;
- hover;
- go-to-definition-like local lookup;
- completions for keywords, builtins, imports, aliases, module members,
  struct fields, enum variants, trait/apply methods, match cases;
- signature help;
- semantic tokens from the official lexer.

Tooling mismatch:

- LSP knows `print` as builtin.
- TextMate grammar does not highlight `print` as builtin yet.

### Type System and Semantics

Confirmed type kinds in `compiler/semantic/types/types.h`:

- primitives: bool, signed ints, unsigned ints, float family, text, bytes, void;
- `core.Error`;
- user types and type params;
- optional/result wrappers;
- `list`, `map`, `set`;
- extended collections: `grid2d`, `pqueue`, `circbuf`, `btreemap`,
  `btreeset`, `grid3d`;
- `dyn`/`any`;
- `lazy`;
- `callable`.

Confirmed semantic support:

- function calls, named args, default params;
- module-level `public const` and `public var`;
- local mutability and const reassignment diagnostics;
- structs, field defaults, field reads and updates;
- methods through `apply Type`;
- trait implementations through `apply Trait to Type`;
- `dyn<Trait>` dispatch and heterogeneous lists;
- enum payload construction and matching;
- `optional<T>` and `result<T,E>`, including `?` propagation paths;
- `optional.or_return`;
- `result.or_wrap`;
- `where` contracts for params, construction, and field assignment;
- noncanonical syntax diagnostics for legacy/foreign forms;
- readable diagnostics with ACTION/WHY/NEXT;
- warning promotion under strict profiles;
- callable signatures and callable escape checks.

Confirmed callable/closure support:

- callable type syntax: `func(...) -> ...`;
- named function as callable value;
- local callable variables;
- indirect calls through callable values;
- anonymous block closures;
- expression lambdas;
- immutable by-value captures;
- rejection of assignment to captured outer variables;
- rejection of callables in public vars, struct fields, and containers.

### Backend and Runtime

Confirmed backend/runtime coverage from behavior tests and source inspection:

- native C backend is the current backend.
- runtime supports managed `text`, `bytes`, `list`, `map`, optional/result,
  closures, lazy, and selected stdlib host wrappers.
- value semantics/COW exist for important collection paths.
- runtime diagnostics cover index bounds, panic, todo, unreachable, check, and
  where contract failures.
- C backend has structured ZIR and legacy paths mixed together.

Current backend risk:

- `value_semantics_struct_managed` fails with:
  `unable to legalize structured list_len operand`.
- This suggests a legalization/backend gap rather than a parser/typechecker
  gap.

### Standard Library

Current stdlib modules under `stdlib/std`:

- `bytes`
- `collections`
- `concurrent`
- `console`
- `format`
- `fs`
- `fs.path`
- `io`
- `json`
- `lazy`
- `list`
- `map`
- `math`
- `net`
- `os`
- `os.process`
- `random`
- `regex`
- `set`
- `test`
- `text`
- `time`
- `validate`

Confirmed behavior coverage exists for:

- `std.bytes`
- `std.text`
- `std.math`
- `std.regex`
- `std.random`
- `std.format`
- `std.fs`
- `std.fs.path`
- `std.io`
- `std.json`
- `std.test`
- `std.time`
- `std.validate`
- `std.os`
- `std.os.process`
- `std.net`
- `std.console`
- `std.concurrent`
- `std.collections`
- `std.lazy`
- `std.set`

Known stdlib documentation gaps:

- `stdlib/zdoc/std/lazy.zdoc` missing.
- `stdlib/zdoc/std/set.zdoc` missing.
- `std.map` and `std.set` public docs exist, but must be reconciled with v8
  decisions for generic API names and safe lookup behavior.
- `std.bytes` docs still describe `from_list(...) -> bytes`, while v8 design
  now wants `result<bytes, core.Error>` for invalid `0..255` checks.

## Source-of-Truth Conflicts

| Area | Implementation says | Documentation says | Audit result |
| --- | --- | --- | --- |
| callables | Implemented and tested | Some docs say deferred/partial | Docs stale. |
| lambdas/closures | Implemented and tested locally | `not-implemented.md` says deferred | Docs stale. |
| `std.set` zdoc | Module and tests exist | zdoc missing | Documentation gap. |
| `std.lazy` zdoc | Module and tests exist | zdoc missing | Documentation gap. |
| behavior matrix | 199 behavior dirs exist | at least 38 dirs missing from matrix | Matrix stale/incomplete. |
| private import access | Fixture expects failure | build currently passes | Possible semantic regression. |
| result optional propagation error | Fixture expects backend error | verify currently passes | Likely stale negative fixture. |
| TextMate builtin list | LSP knows `print` | grammar omits `print` | Extension grammar stale. |

## High-Priority Findings

### A1 - Public documentation is not a reliable implementation map

Examples:

- `not-implemented.md` says first-class functions are deferred.
- `feature-matrix.md` says closure/lambda docs are missing while tests prove the
  feature exists.
- v8 decisions for `map`, `set`, `list`, `text`, `bytes`, and callables are
  ahead of public docs.

Action:

- Create a single implementation-status matrix.
- Mark each feature with: parser, checker, HIR/ZIR, backend, runtime, stdlib,
  LSP, formatter, tests, docs.

### A2 - M16 conformance is not green

Current result:

- `114/122 passed`

Failing areas:

- `list_dyn_trait_basic`
- `value_semantics_struct_managed`
- `multifile_private_access`
- `result_optional_propagation_error`

Action:

- Reproduce each failing case directly.
- Split into:
  - real compiler/backend regression;
  - stale fixture;
  - stale conformance expectation.

### A3 - Visibility of imported private symbols may be broken

Evidence:

```powershell
.\zt.exe build tests/behavior/multifile_private_access/zenith.ztproj --ci
```

Observed:

```text
build ok
```

Expected by M16 fixture:

- diagnostic containing `member 'secrets.HIDDEN' is not public`.

Action:

- Audit binder/typechecker import-member visibility.
- Add a direct driver test if missing.

### A4 - Backend/legalization still has structured expression gaps

Evidence:

```powershell
.\zt.exe run tests/behavior/value_semantics_struct_managed/zenith.ztproj --ci
```

Observed:

```text
error[backend.c.emit] <zir>:1:1 stage=backend.c.emit effort=requires thinking unable to legalize structured list_len operand
```

Action:

- Inspect generated ZIR/HIR for managed struct list field access.
- Fix legalization for structured `list_len` operand or lower earlier.

### A5 - Behavior matrix is incomplete

At least these behavior projects are not listed as project entries in
`tests/behavior/MATRIX.md`:

- `callable_basic`
- `callable_escape_container_error`
- `callable_escape_public_var_error`
- `callable_escape_struct_field_error`
- `callable_invalid_func_ref_error`
- `callable_signature_mismatch_error`
- `check_intrinsic_basic`
- `check_intrinsic_type_error`
- `core_error_construction`
- `dyn_dispatch_basic`
- `dyn_trait_heterogeneous_collection`
- `enum_match_non_exhaustive_error`
- `extern_c_puts_e2e`
- `extern_c_struct_arg_error`
- `extern_c_text_len_e2e`
- `functions_param_ordering_error`
- `list_dyn_textrepresentable`
- `methods_mutating_const_receiver_error`
- `optional_match_value`
- `optional_result_helpers_absence_error`
- `optional_result_helpers_pass`
- `panic_basic`
- `panic_with_message`
- `range_builtin_basic`
- `std_collections_basic`
- `std_concurrent_boundary_copy_determinism`
- `std_io_basic`
- `std_os_args_basic`
- `std_os_process_capture_basic`
- `u_alias_basic`
- `using_basic`
- `where_contract_param_where_invalid_error`
- `where_contract_param_where_non_bool_error`

Action:

- Regenerate or manually refresh `tests/behavior/MATRIX.md`.
- Avoid using the current matrix as exhaustive coverage proof.

## Recommended Next Audit Work

Do this before continuing v8 API decisions:

1. Fix or classify the M16 failures.
2. Create `docs/reference/language/implementation-status.md` as the current
   source of truth.
3. Update `not-implemented.md` to remove implemented items.
4. Refresh `feature-matrix.md` from actual tests.
5. Add missing zdocs for `std.set` and `std.lazy`.
6. Align TextMate grammar with LSP builtins and current types.
7. Re-run:
   - `python build.py`
   - `.\tests\conformance\test_m16.exe`
   - driver Python tests
   - focused behavior tests for callables, collections, imports, optional/result
8. Return to `COL.06` only after the callable/doc/test state is reconciled.

## Audit Status

This is a broad implementation snapshot, not the final exhaustive matrix.

Completed in this pass:

- Source inventory.
- Current docs inventory.
- Parser/AST/token inventory.
- Formatter/LSP surface scan.
- Stdlib/zdoc inventory.
- Behavior project inventory.
- Build and selected executable validation.
- Initial mismatch report.

Still needed for a fully exhaustive audit:

- Per-feature parser/checker/backend/runtime matrix.
- Per-stdlib API signature matrix.
- Full driver/test runner matrix.
- LSP protocol feature test matrix.
- VSCode extension packaging/runtime asset matrix.
- Public docs page-by-page freshness check.
