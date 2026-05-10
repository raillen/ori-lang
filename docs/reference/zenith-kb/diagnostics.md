# Zenith Diagnostics

## Severity Levels

| Severity | Meaning |
|----------|---------|
| `error` | Compilation or runtime failure |
| `warning` | Sparse, actionable issue |
| `note` | Additional context |
| `help` | Concrete suggested action |

## Terminal Output Shape

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
- No color-only meaning
- No dense banners
- Stable section labels
- Small code excerpts
- Plain language
- Actionable help only when concrete action exists

## Diagnostic Stages

| Stage | Responsibility |
|-------|---------------|
| `project` | Manifest, project structure |
| `lexer` | Tokenization errors |
| `parser` | Syntax errors |
| `semantic` | Name resolution, type checking |
| `lowering` | HIR/ZIR lowering errors |
| `zir` | IR invariant violations |
| `backend` | C emission errors |
| `formatter` | Formatting errors |
| `runtime` | Runtime panics and checks |
| `internal` | Compiler bugs (wrapped, not leaked) |

## Compiler Diagnostic Codes

### Project/Manifest (`project.*`)

| Code | Meaning |
|------|---------|
| `project.error` | General project error |
| `project.import_cycle` | Circular imports |
| `project.missing_entry` | Missing `app.entry` namespace |
| `project.invalid_namespace` | Namespace/directory mismatch |
| `project.unresolved_import` | Imported namespace doesn't exist |
| `project.invalid_input` | Path/manifest readability issue |
| `project.invalid_section` | Unsupported manifest section |
| `project.unknown_key` | Unsupported key in section |
| `project.invalid_assignment` | Invalid `key = value` |
| `project.invalid_string` | Invalid quoted string |
| `project.missing_field` | Required field missing |
| `project.invalid_kind` | `project.kind` must be `app` or `lib` |
| `project.invalid_target` | Unsupported `build.target` |
| `project.invalid_profile` | Unsupported `build.profile` |
| `project.path_too_long` | Path exceeds limits |
| `project.too_many_dependencies` | Too many dependencies |
| `project.invalid_monomorphization_limit` | Invalid monomorphization limit |
| `project.monomorphization_limit_exceeded` | Generic instantiations exceed limit |

### Syntax (`syntax.*`)

| Code | Meaning |
|------|---------|
| `syntax.error` | General syntax error |
| `syntax.unexpected_token` | Unexpected token |

### Semantic/Name/Type (`name.*`, `type.*`, `trait.*`, `generic.*`, `mutability.*`)

| Code | Meaning |
|------|---------|
| `name.duplicate` | Duplicate declaration |
| `name.shadowing` | Shadowing outer name (rejected in MVP) |
| `name.unresolved` | Undeclared/unimported name |
| `name.confusing` | Identifier mixes visually confusing characters |
| `name.similar` | Local names in the same scope are too similar |
| `type.invalid` | Invalid type name or generic args |
| `type.mismatch` | Type mismatch |
| `type.invalid_call` | Invalid function call arity |
| `type.invalid_argument` | Invalid argument (name/order/arity) |
| `type.invalid_operator` | Operator used with incompatible operands |
| `type.invalid_map_key` | Map key or set element lacks materialized hash/equality support |
| `type.invalid_conversion` | Unsupported explicit conversion |
| `type.integer_overflow` | Integer overflow |
| `generic.constraint_target` | Constraint on non-generic parameter |
| `mutability.invalid_assignment` | Invalid mutable target |
| `mutability.const_update` | Reassigning `const` |
| `mutability.invalid_update` | Cross-namespace `public var` write |

### Readability (`style.*`)

| Code | Meaning |
|------|---------|
| `style.block_too_deep` | Nested block depth is hard to scan |
| `style.function_too_long` | Function has too many statements to scan quickly |

### Control Flow (`control_flow.*`)

| Code | Meaning |
|------|---------|
| `control_flow.invalid_return` | Return doesn't match function contract |
| `control_flow.invalid_condition` | Condition is not `bool` |
| `control_flow.enum_default_case` | `case else` on known enum hides explicit variant coverage |

### ZIR/Backend (`zir.*`, `backend.*`)

| Code | Meaning |
|------|---------|
| `zir.parse` | Textual ZIR syntax error |
| `zir.verify` | Verifier invariant violation |
| `backend.c.emit` | C emission error |
| `backend.c.legalize` | C sequence legalization error |

### ZDoc (`doc.*`)

| Code | Meaning |
|------|---------|
| `doc.malformed_block` | Invalid ZDoc block structure |
| `doc.missing_target` | Missing `@target` in paired doc |
| `doc.missing_page` | Missing `@page` in guide doc |
| `doc.unresolved_target` | @target points to non-existent symbol |
| `doc.unresolved_link` | Broken `@link` reference |
| `doc.invalid_guide_tag` | Guide tag outside `zdoc/guides` |
| `doc.invalid_paired_tag` | Paired tag outside source-paired doc |
| `doc.missing_public_doc` | Public symbol missing ZDoc |

## Runtime Diagnostic Codes

| Code | Trigger |
|------|---------|
| `runtime.check` | Assertion failure (`check`) |
| `runtime.check` | Check failure |
| `runtime.contract` | `where` contract violation |
| `runtime.index` | Index out of bounds / map key missing |
| `runtime.io` | IO failure |
| `runtime.math` | Divide by zero / arithmetic overflow |
| `runtime.panic` | Explicit `panic()` call |
| `runtime.platform` | Allocation failure / invalid UTF-8 / platform failure |
| `runtime.todo` | Explicit `todo()` call |
| `runtime.unreachable` | Explicit `unreachable()` call |
| `runtime.unwrap` | Unwrap failure |

## Test Runtime Codes

| Code | Meaning |
|------|---------|
| `test.fail` | Test failed |
| `test.skip` | Test skipped |

## Warning Policy

Allowed initial warnings (sparse and actionable):
- Visually confusing identifier names
- Very similar local names in the same scope
- Deeply nested blocks that are hard to scan
- Functions with too many statements to scan quickly
- `case else` on a known enum, because explicit variants are clearer
- Missing public ZDoc
- Unresolved ZDoc link
- Deprecated symbol (future)

Formatting-only preferences do NOT belong in compiler-core warnings.
Readability warnings must point to a concrete risk and a clear next action.

Warnings do not block normal builds.

Set `[diagnostics] profile = "strict"` to promote warnings to errors.

## Renderers

| Renderer | Flag | Description |
|----------|------|-------------|
| Detailed terminal | (default) | Full diagnostic with code excerpt |
| Short CI | `--ci` | One-line stable diagnostics |
| JSON | (future) | Machine-readable output |
| LSP | (future) | IDE integration |
