# Ori Language Specification — Chapter 13: Diagnostic Error Catalog

> Status: normative
> Audience: compiler implementers, tool authors

---

## Overview

Every diagnostic emitted by the Ori compiler has a unique code of the form:

```
category.specific_name
```

Categories map to compiler phases and language areas.
Diagnostics have a severity: `error` (blocks compilation) or `warning` (advisory).

All diagnostics follow the format:

```
severity[code]: short description
  --> file.ori:line:col
   |
N  | source line
   | ^^^^^^^^^^ annotation
   |
   = why: explanation of the rule
   = action: what to do to fix it
```

---

## Category: `parse`

Errors produced by the parser.

| Code | Severity | Description |
|---|---|---|
| `parse.namespace_missing` | error | File does not start with a `namespace` declaration |
| `parse.namespace_not_first` | error | `namespace` appears after other declarations |
| `parse.unexpected_token` | error | Unexpected token at this position |
| `parse.unterminated_block` | error | Block is not closed with `end` |
| `parse.unterminated_string` | error | String literal is not closed |
| `parse.invalid_escape` | error | Unknown escape sequence in string literal |
| `parse.import_after_declaration` | error | `import` appears after non-import declarations |
| `parse.invalid_range` | error | Range expression has incompatible endpoint types |
| `parse.variadic_not_last` | error | Variadic parameter `...` is not the last parameter |
| `parse.chained_comparison` | error | Comparison chaining is not allowed (e.g. `a < b < c`) |
| `parse.missing_else_in_if_expr` | error | Inline `if` expression requires an `else` branch |

---

## Category: `type`

Errors produced by the type checker.

| Code | Severity | Description |
|---|---|---|
| `type.mismatch` | error | Expression type does not match expected type |
| `type.undefined` | error | Type name is not defined in scope |
| `type.annotation_required` | error | Type annotation is required here (cannot be inferred) |
| `type.return_mismatch` | error | Returned value type does not match function return type |
| `type.void_return_value` | error | Returning a value from a `void` function |
| `type.missing_return` | error | Function body may not return a value on all paths |
| `type.constraint_not_satisfied` | error | Type argument does not satisfy `where` constraint |
| `type.ambiguous_generic` | error | Type argument cannot be inferred; provide explicitly |
| `type.comparison_not_supported` | error | `==` applied to a type without equality (e.g. `any<Trait>`, `func`) |
| `type.equality_unsupported_field` | error | Struct contains a field type that does not support `==` |
| `type.invalid_is_check` | error | `is` check on a non-dynamic type |
| `type.incompatible_result_error` | error | `?` used but error types are incompatible |
| `type.propagation_context` | error | `?` used in a function that does not return `optional<_>` or `result<_,_>` |
| `type.no_such_field` | error | Struct field does not exist |
| `type.no_such_method` | error | Method not found for this type |
| `type.ambiguous_method` | error | Method name matches multiple traits; use explicit disambiguation |
| `type.callable_mismatch` | error | Argument types do not match closure or function signature |
| `type.index_non_indexable` | error | `[index]` applied to a non-indexable type |
| `type.spread_non_list` | error | `..expr` spread used with a non-list value |
| `type.unused_result` | warning | `result<T, E>` value is discarded without `?` or match |

---

## Category: `bind`

Errors produced by the name resolution / binding phase.

| Code | Severity | Description |
|---|---|---|
| `bind.undefined` | error | Name is not defined in scope |
| `bind.shadowing` | error | Binding shadows an existing binding in the same scope |
| `bind.const_reassignment` | error | Reassigning a `const` binding |
| `bind.import_not_found` | error | Imported namespace does not exist |
| `bind.import_ambiguous` | error | Import alias conflicts with an existing name |
| `bind.self_outside_method` | error | `self` used outside a method or `implement` block |
| `bind.duplicate_field` | error | Struct or enum variant declares the same field name twice |
| `bind.duplicate_variant` | error | Enum declares the same variant name twice |
| `bind.duplicate_param` | error | Function declares the same parameter name twice |
| `bind.duplicate_implement` | error | `implement Trait for Type` already exists in scope |

---

## Category: `match`

Errors produced by exhaustiveness checking.

| Code | Severity | Description |
|---|---|---|
| `match.non_exhaustive` | error | `match` does not cover all possible cases |
| `match.unreachable_case` | warning | Case can never be reached given earlier cases |
| `match.guard_not_exhaustive` | warning | Guarded case does not count toward exhaustiveness |
| `match.duplicate_case` | warning | Two identical patterns in the same `match` |

---

## Category: `mut`

Errors related to mutability.

| Code | Severity | Description |
|---|---|---|
| `mut.const_mutation` | error | Mutating a `const` binding |
| `mut.const_method_call` | error | Calling a `mut func` on a `const` binding |
| `mut.closure_captures_var` | error | Closure attempts to capture a `var` binding |
| `mut.field_mutation_in_func` | error | Assigning to `self` field in a non-`mut func` |
| `mut.using_binding_mutated` | error | `using` binding cannot be reassigned |

---

## Category: `contract`

Errors related to value contracts (`where` on fields and parameters).

| Code | Severity | Description |
|---|---|---|
| `contract.field_violation` | runtime panic | Field contract violated at construction or assignment |
| `contract.param_violation` | runtime panic | Parameter contract violated at call site |
| `contract.check_failure` | runtime panic | `check` assertion failed |

Note: contract violations are runtime panics, not compile errors.
The compiler may statically detect provably violated contracts and emit `error`.

---

## Category: `impl`

Errors related to `implement` blocks and trait resolution.

| Code | Severity | Description |
|---|---|---|
| `impl.missing_method` | error | `implement` block does not implement all required trait methods |
| `impl.wrong_signature` | error | Implemented method signature does not match trait declaration |
| `impl.trait_not_found` | error | Trait being implemented does not exist |
| `impl.type_not_found` | error | Type being implemented for does not exist |
| `impl.mut_mismatch` | error | Trait requires `mut func` but implementation omits `mut`, or vice versa |

---

## Category: `using`

Errors related to `using` and `Disposable`.

| Code | Severity | Description |
|---|---|---|
| `using.not_disposable` | error | Type in `using` does not implement `Disposable` |
| `using.non_result_init` | error | `using` initializer must produce a value (not void) |

---

## Category: `generic`

Errors related to generics.

| Code | Severity | Description |
|---|---|---|
| `generic.constraint_not_satisfied` | error | Type argument does not satisfy `where T is Trait` |
| `generic.negative_constraint_violated` | error | Type argument violates `where T is not Trait` |
| `generic.ambiguous_type_arg` | error | Type argument cannot be inferred |
| `generic.circular_instantiation` | error | Generic instantiation creates an infinite recursive type |
| `generic.unsupported_hkt` | error | Higher-kinded type parameter not supported in v1 |

---

## Category: `extern`

Errors related to `extern c` and FFI.

| Code | Severity | Description |
|---|---|---|
| `extern.managed_type_in_ffi` | error | Managed type used in FFI without explicit ABI annotation |
| `extern.unknown_abi` | error | Unknown ABI label after `extern` |

---

## Category: `project`

Errors related to project structure and `ori.proj`.

| Code | Severity | Description |
|---|---|---|
| `project.no_proj_file` | error | No `ori.proj` found in current or parent directories |
| `project.entry_not_found` | error | Entry file declared in `ori.proj` does not exist |
| `project.circular_import` | error | Circular namespace import detected |
| `project.namespace_file_mismatch` | warning | File path does not match namespace convention |

---

## Diagnostic Format Contract

Every diagnostic must provide:

1. **Code** — machine-readable identifier.
2. **Severity** — `error` or `warning`.
3. **Short message** — one line, present tense, no period.
4. **Span** — file, line, column of the primary location.
5. **`= why:`** — explanation of the rule (one sentence).
6. **`= action:`** — what the programmer should do (imperative sentence).

Optional:
- **Secondary spans** — additional source locations referenced by the diagnostic.
- **`= note:`** — additional context (migration hints, related rules).

Example:

```
error[type.constraint_not_satisfied]: T does not satisfy constraint
  --> src/app/sort.ori:8:14
   |
8  |    const sorted: list<User> = iter.sort(users)
   |                               ^^^^^^^^^
   |
   = why: iter.sort requires T to implement Comparable
   = action: add 'implement Comparable for User' with func compare(other: User) -> Order
```
