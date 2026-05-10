# Decision Conflict Audit (M21)

- Status: completed audit
- Date: 2026-04-18
- Scope: conflicts between older decision documents and canonical specs

## Purpose

This file records explicit reconciliations where older accepted decisions still contain examples or rules that conflict with current canonical specs.

Canonical specs remain the source of truth.

## Reconciled Conflicts

### C-001: Value-level `where` on return types

Conflict source:

- `decisions/014-contracts-refinement-and-field-invariants.md` includes return-level `where`.

Canonical rule:

- `surface-syntax.md` and `decisions/031-runtime-where-contracts.md` disallow value-level `where` on function return types in the MVP.

Resolution:

- return-level `where` from Decision 014 is superseded.
- runtime `where` applies only to fields and parameters in MVP.

### C-002: `map[key]` returning optional

Conflict source:

- `decisions/013-mvp-semantic-validation.md` states `map[key]` has type `optional<V>`.

Canonical rule:

- `surface-syntax.md` and `decisions/032-generic-collections-and-update-block.md` define strict indexing:
  - `map[key]` returns `V`
  - missing key panics
  - safe lookup belongs to explicit API (`map.get` direction)

Resolution:

- Decision 013 map-lookup typing is superseded by Decision 032 and current specs.

### C-003: Manifest schema `[layout]`, `source_root`, `zdoc_root`

Conflict source:

- `decisions/015-zenith-ztproj-and-package-model.md` uses `[layout]`, `source_root`, `zdoc_root`.
- `decisions/016-zdoc-and-doc-linking.md` also references `source_root` and `zdoc_root`.

Canonical rule:

- `project-model.md` and `decisions/051-zenith-ztproj-project-manifest.md` define:
  - `[source] root`
  - `[zdoc] root`
  - `[app] entry`
  - `[lib] root_namespace`

Resolution:

- Decision 015/016 manifest field names are superseded by Decision 051 and canonical project spec.

### C-004: Optional match notation using `some(...)` case labels

Conflict source:

- `decisions/030-question-propagation-for-result-and-optional.md` and `decisions/037-panic-fatal-errors-and-attempt.md` contain examples with `case some(value)`.

Canonical rule:

- `surface-syntax.md` defines optional pattern as:
  - `case value name:`
  - `case none:`
- `some(value)` is not canonical MVP syntax.

Resolution:

- optional match examples using `case some(...)` are historical and superseded by current surface syntax.

### C-005: Entrypoint error channel narrowing

Conflict source:

- `decisions/022-entrypoint-and-program-model.md` examples emphasize `result<..., text>`.

Canonical rule:

- `decisions/057-main-entrypoint-typed-error-results.md` amends entrypoint error handling:
  - typed error channels are allowed when renderable at process boundary.

Resolution:

- Decision 022 should be read as amended by Decision 057.

### C-006: Mutating method marker migration to `mut func`

Conflict source:

- older decisions and examples use legacy suffix-based mutating method names.

Canonical rule:

- `surface-syntax.md` and `decisions/082-syntax-accessibility-ergonomics.md` define mutating method declarations with `mut func`.

Resolution:

- suffix-based mutating method examples are historical and superseded by `mut func`.
- the compiler does not provide transition compatibility for suffix-based mutating declarations or calls.

### C-007: Interpolation model (`fmt`) versus escaped interpolation in ordinary strings

Conflict source:

- older examples use escaped interpolation inside ordinary string literals.

Canonical rule:

- `surface-syntax.md` and `decisions/036-text-concatenation-and-interpolation.md` define `fmt "..."` as the canonical interpolation form.

Resolution:

- escaped interpolation examples in ordinary strings are superseded by `fmt` interpolation.

### C-008: Text conversion surface (`to_text(value)`) versus method-style conversion

Conflict source:

- older examples use method-style value conversion.

Canonical rule:

- `surface-syntax.md` defines explicit conversion as `to_text(value)` in user-facing source.

Resolution:

- method-style conversion examples are historical and superseded for user-facing docs.

### C-009: Collection append naming (`append/prepend`) versus `push`

Conflict source:

- older examples and deferred notes mention `push`.

Canonical rule:

- `surface-syntax.md` defines list mutation names as `append` and `prepend`.

Resolution:

- `push` examples are non-canonical and superseded by `append/prepend`.
### C-010: Optional match syntax — `case value name` to `case some(name)`

Conflict source:

- `surface-syntax.md` lines 370-391 define `case value name` as canonical optional match.
- C-004 above states `some(value)` is not canonical.

Canonical rule:

- Decision 094 makes `case some(name)` the canonical form. Adds `some` as keyword.

Resolution:

- `case value name` is superseded by `case some(name)`.
- C-004 is itself superseded by Decision 094.
- Implementation note: parser accepts `case some(name):` as canonical and keeps legacy `case value name` as transition syntax.

### C-011: Interpolation model — `fmt "..."` to `f"..."`

Conflict source:

- C-007 above references `fmt "..."` as canonical interpolation form.

Canonical rule:

- Decision 093 and Decision 094 make `f"..."` the canonical form. `fmt` is deprecated alias.

Resolution:

- C-007 is superseded by Decision 093.

### C-012: `dyn` keyword renamed to `any`

Conflict source:

- `dyn-dispatch.md` and `surface-syntax.md` use `dyn` throughout.
- `surface-syntax.md` line 844 lists `any` as "Explicitly Not MVP".

Canonical rule:

- Decision 094 renames `dyn` to `any`. All dynamic dispatch uses `any Trait` / `any<Trait>`.

Resolution:

- All `dyn` references in specs are superseded by `any`.
- `any` is removed from the "Explicitly Not MVP" list.
- Unified spec: `docs/spec/language/language-reference.md`.
- Implementation note: lexer/parser/checker accept `any Trait` and `any<Trait>`; `dyn` remains accepted as legacy syntax during migration.

### C-013: Generic constraints — `where T is Trait` to `<T: Trait>`

Conflict source:

- `surface-syntax.md` lines 604-627 use `where Item is Equatable` as constraint syntax.
- `dyn-dispatch.md` lines 88-97 suggest `where T is Drawable`.

Canonical rule:

- Decision 094 makes `<T: Trait>` the primary form. `given` as optional trailing clause.
- `where` is reserved for value-level runtime contracts only.

Resolution:

- Generic constraint examples using `where T is Trait` are superseded by `<T: Trait>`.
- Implementation note: parser supports inline `<T: Trait>` and trailing contextual `given T is Trait`.

### C-014: Match default case — `case default:` to `case else:`

Conflict source:

- `surface-syntax.md` lines 585-593 use `case default:`.

Canonical rule:

- Decision 094 replaces `case default:` with `case else:`. `default` is removed as keyword.

Resolution:

- All `case default:` examples are superseded by `case else:`.
- Implementation note: parser accepts `case else:` as canonical and keeps legacy `case default` as transition syntax. `default` is no longer emitted as a lexer keyword.

### C-015: Mutable closure capture syntax

Conflict source:

- Decision 093 line 90 specifies mutable capture with `[mut var]` syntax.
- `closures.md` line 104 says mutable capture is "a future feature".

Canonical rule:

- Decision 094 specifies `capture name: Type = init` (without `mut`).

Resolution:

- Decision 093 `[mut var]` syntax is superseded by `capture` from Decision 094.
- `closures.md` "future feature" note is superseded.

## Non-Conflicts (Clarified)

- `-> void` appears in older decisions as non-canonical examples; current spec keeps omission as canonical and may diagnose explicit `-> void`.
- `pub` examples in older decisions are explicitly listed as non-canonical and do not conflict with `public`.
- strict map indexing and safe lookup API are intentionally split between core syntax and future stdlib APIs.

## Operational Rule

When an older decision conflicts with canonical specs:

1. canonical specs under `docs/spec/language/` win.
2. explicit later decisions that amend earlier ones win.
3. this audit document acts as reconciliation index for maintainers.

## Follow-up

Maintainers should prefer updating historical decision headers with `Superseded in part by ...` notes when touching those files, but that annotation pass is not required to keep the canonical rules valid.

