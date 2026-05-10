# Zenith Wave 7.6: Trait Stability Pass

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: implemented subset; advanced trait/any policy superseded by `post-v1-remaining-language-work.md`  
> Surface: spec  
> Last updated: 2026-05-02

## Purpose

This document captures the design session output for Wave 7.6, establishing stable semantics for Zenith's trait system. It resolves open questions about trait coherence, method lookup, default implementations, and diagnostics.

For final generic trait, associated type, `Self`, apply precedence, and advanced
`any<Trait>` policy, the current decision in
`post-v1-remaining-language-work.md` prevails over the limited Wave 7.6
implementation subset described here.

## Scope

This session addresses:
- Trait default method implementations
- `apply` declaration lookup and resolution order
- Overlapping `apply` handling
- Operator traits (Addable, Subtractable, Comparable)
- `Transferable` trait semantics for concurrency boundaries
- Trait-related diagnostics

## Design Decisions

### D1: Trait Default Methods (Default Implementations)

**Status:** Defined and implemented

**Decision:** Trait methods may provide default implementations. Types implementing the trait inherit the default unless they explicitly override it.

**Syntax:**
```zenith
trait Drawable
    func draw() -> void
        -- default implementation
        print("drawing")
    end
end

struct Circle
    radius: float
end

-- Inherits default draw()
apply Drawable to Circle
end

struct Square
    side: float
end

-- Overrides default draw()
apply Drawable to Square
    func draw() -> void
        print("square drawn")
    end
end
```

**Semantics:**
- Default implementations are evaluated at compile time
- Method lookup order: explicit apply implementation > trait default > compiler error
- Default implementations may call other trait methods (including those with defaults)
- Default implementations are not virtual/dynamic; they resolve statically at compile time

**Validation:** Tests at `tests/behavior/methods_trait_apply/` verify both inheritance and override behaviors.

---

### D2: Apply Declaration Lookup and Resolution

**Status:** Defined and implemented

**Decision:** Apply declarations are resolved via a deterministic lookup algorithm with explicit precedence rules.

**Lookup Algorithm:**

Given a method call `receiver.method_name()` where `receiver` has type `T`:

1. **Inherent methods first:** Check if `T` has an inherent method `method_name` (declared directly on the struct/enum)
2. **Apply declarations second:** Search apply declarations where `T` is the target type
3. **Trait methods third:** Check if `T` implements any trait with `method_name`
4. **Error if ambiguous:** If multiple apply declarations or traits match, emit ambiguity diagnostic

**Apply Resolution Specifics:**

```zenith
-- Form 1: Apply methods directly to type (no trait)
apply Type
    func method() -> void ... end
end

-- Form 2: Apply trait to type
apply TraitName to Type
    func trait_method() -> ReturnType ... end
end
```

**Precedence:**
- Inherent methods > apply methods > trait methods via apply
- Multiple apply declarations for the same type are additive (methods accumulate)
- Multiple apply declarations implementing the same trait for the same type = error (duplicate conformance)

---

### D3: Overlapping Apply Resolution

**Status:** Defined with conservative rejection

**Decision:** Overlapping apply declarations (where multiple applies could match a method call) are rejected with a clear diagnostic.

**Definition of Overlap:**

Two apply declarations overlap when:
1. They both target the same concrete type, OR
2. They both implement traits that share a method name for the same type

**Current Policy:**
```zenith
trait Printable
    func print() -> void
end

trait Debuggable
    func print() -> void  -- same name, different semantics
end

struct Item end

-- ERROR: overlapping applies - Item would have two 'print' methods
apply Printable to Item ... end
apply Debuggable to Item ... end
```

**Resolution Strategy:**
- Zenith v1 rejects overlapping applies at check time
- Future versions may introduce explicit precedence syntax (deferred to post-v1)
- Workaround: use distinct method names or wrapper types

**Diagnostic:** `type.overlapping_apply` - "Cannot apply both `TraitA` and `TraitB` to `Type`: method `name` conflicts"

---

### D4: Operator Traits (Level 2 Only)

**Status:** Defined and implemented

**Decision:** Operator overloading is restricted to a fixed set of operator traits. Arbitrary operator overloads are rejected.

**Supported Operator Traits:**

| Trait | Method | Operators | Signature Pattern |
|-------|--------|-----------|-------------------|
| `Addable` | `add` | `+` | `func add(other: Self) -> Self` |
| `Subtractable` | `subtract` | `-` | `func subtract(other: Self) -> Self` |
| `Comparable` | `less` | `<` | `func less(other: Self) -> bool` |
| `Comparable` | `less_or_equal` | `<=` | `func less_or_equal(other: Self) -> bool` |
| `Comparable` | `greater` | `>` | `func greater(other: Self) -> bool` |
| `Comparable` | `greater_or_equal` | `>=` | `func greater_or_equal(other: Self) -> bool` |

**Constraints:**
- Only these three traits are supported for operator overloading
- Method signatures must match the expected pattern exactly
- All operators in a trait family must be implemented together (e.g., all four `Comparable` methods)
- Operator traits must be applied explicitly; no automatic derivation

**Lowering:** Operator expressions lower to explicit method calls:
```zenith
-- Source
a + b

-- Lowered
a.add(b)
```

**Rejection:** Arbitrary operators (`, etc.) cannot be overloaded. Attempts result in `type.invalid_operator`.

---

### D5: Transferable Trait Semantics

**Status:** Defined and implemented

**Decision:** `Transferable` is a compiler-known trait that determines which types may cross concurrency boundaries (jobs, channels, shared memory).

**Transferable Types:**

The following types are automatically `Transferable`:
- All scalar primitives: `bool`, `int`, `int8`..`int64`, `u8`..`u64`, `float`, `float32`, `float64`
- `text` and `bytes`
- `void`
- Generic compositions where all components are `Transferable`:
  - `optional<T>` where `T: Transferable`
  - `result<T, E>` where `T: Transferable`, `E: Transferable`
  - `list<T>` where `T: Transferable`
  - `set<T>` where `T: Transferable` (and `T: Hashable + Equatable`)
  - `map<K, V>` where `K: Transferable` (and `K: Hashable + Equatable`), `V: Transferable`
  - `tuple<T1, T2, ...>` where all elements are `Transferable`

**User-Defined Transferable:**

Structs and enums are `Transferable` if all their fields/payloads are `Transferable` (recursive check):

```zenith
struct Point
    x: int
    y: int
end
-- Point is Transferable (both fields are int)

struct Container
    name: text
    items: list<int>
end
-- Container is Transferable (text and list<int> are Transferable)

struct Bad
    callback: func(int) -> int
end
-- Bad is NOT Transferable (closures are not Transferable)
```

**Checking:** The compiler performs a recursive structural check via `zt_checker_type_is_transferable_inner()` in `checker.c`.

**Concurrency Boundaries:**
- `std.jobs.spawn()` requires `Transferable` for value and return types
- `std.concurrent.copy_*()` requires `Transferable` for the value being copied
- `std.channels.send()`/`receive()` require `Transferable` payloads

**Diagnostic:** `concurrency.not_transferable` - "Only transferable shapes may cross isolate, job, or worker boundaries."

---

### D6: Method Lookup Order Summary

**Final precedence (highest to lowest):**

1. **Inherent methods** (declared directly on struct/enum)
2. **Apply block methods** (apply Type ... end)
3. **Trait methods via apply** (apply Trait to Type ... end)
4. **Default trait methods** (from trait declaration)

**At each level:**
- First match wins
- Ambiguity within a level = compile error
- No dynamic dispatch unless using `any<Trait>`

---

### D7: Visibility and Traits

**Status:** Defined

**Decision:** Trait and apply visibility follows Zenith's module visibility rules.

**Rules:**
- Traits declared `public trait` are visible to importers
- Apply declarations inherit visibility from their containing module
- Methods in apply blocks follow normal visibility: `public func` vs `func`
- Trait methods are always public (trait defines the contract)

**Note:** An apply declaration itself cannot be marked `public`; visibility is determined by:
- Whether the trait is public
- Whether the type is public
- Whether the implementing methods are public

---

### D8: Trait-Related Diagnostics

**Status:** Defined and stable

**Diagnostic Catalog:**

| Code | Stable Name | Message Pattern |
|------|-------------|-----------------|
| `any.mut_method` | `ZT_DIAG_DYN_MUT_METHOD` | "trait 'X' cannot be used as any<X>: mut method 'Y' needs a concrete receiver" |
| `any.generic_trait` | `ZT_DIAG_DYN_GENERIC_TRAIT` | "trait 'X' cannot be used as any<X>: generic traits are not any-safe" |
| `any.too_many_methods` | `ZT_DIAG_DYN_TOO_MANY_METHODS` | "trait 'X' cannot be used as any<X>: it has N methods, but any supports at most 8" |
| `any.uncopyable` | `ZT_DIAG_DYN_UNCOPYABLE` | "trait 'X' cannot be used as any<X>: method 'Y' has non-copyable parameter type Z" |
| `any.no_apply` | `ZT_DIAG_DYN_NO_APPLY` | "Implement the trait for this type using apply Trait to Type." |
| `any.ffi_unsafe` | `ZT_DIAG_DYN_FFI_UNSAFE` | "any types cannot be used in extern c signatures." |
| `concurrency.not_transferable` | `ZT_DIAG_CONCURRENCY_NOT_TRANSFERABLE` | "Only transferable shapes may cross isolate, job, or worker boundaries..." |
| `type.invalid_operator` | `ZT_DIAG_INVALID_OPERATOR` | "Operands incompatible with this operator" / "No operator trait implementation" |

**Help Text Pattern:**
All trait-related diagnostics should include:
- **WHY:** Why the operation is invalid
- **WHAT:** What constraint was violated
- **NEXT:** Specific action to fix (when applicable)

---

### D9: Any<Trait> Safety Constraints (Wave 7.6 Implementation Subset)

**Status:** Defined in `post-v1-any-dispatch-stabilization.md`, integrated here

**Decision:** The Wave 7.6 implementation subset requires conservative constraints for safe vtable generation. The final language policy for advanced `any` shapes is superseded by `post-v1-remaining-language-work.md`.

**Constraints:**
1. **No generic traits in the current subset** - `any<GenericTrait<T>>` is rejected by the Wave 7.6 implementation subset
2. **Max 8 methods** - vtable has fixed 8-slot layout
3. **No mutating methods in the current subset** - dynamic dispatch currently requires consistent ABI
4. **All params/returns must be copyable in the current subset** - no heap-managed types crossing the current vtable boundary

**Rationale:** These constraints ensure the C vtable layout is stable and calls across the `any` boundary are safe without complex runtime coordination.

---

## Implementation Status

| Component | Status | Location |
|-----------|--------|----------|
| Trait parsing | Implemented | `parser.c:zt_parser_parse_trait_decl()` |
| Trait method parsing | Implemented | `parser.c:zt_parser_parse_trait_method()` |
| Apply parsing | Implemented | `parser.c:zt_parser_parse_apply_decl()` |
| Default method support | Implemented | AST model + checker resolution |
| Method lookup | Implemented | `checker.c:zt_checker_find_apply_method()` |
| Operator lowering | Implemented | `from_ast.c:zt_lower_operator_trait_call()` |
| Transferable check | Implemented | `checker.c:zt_checker_type_is_transferable_*()` |
| Diagnostics | Implemented | `diagnostics.c` |

## Testing

**Positive fixtures:**
- `methods_trait_apply` - basic trait apply with default + override
- `operator_overloading_level2_basic` - all supported operators
- `concurrency_transferable_predicate_basic` - Transferable check acceptance

**Negative fixtures:**
- `operator_overloading_missing_trait_error` - operator without trait implementation
- `concurrency_transferable_predicate_error` - non-Transferable type rejection
- `dyn_generic_trait_error` - generic trait in `any<>` rejection
- `std_concurrent_boundary_copy_unsupported_error` - `any<Trait>` not Transferable

## Future Work (Post-v1)

The following were deferred by Wave 7.6 but have later final decisions in
`post-v1-remaining-language-work.md` unless otherwise noted:

1. **Generic traits** - accepted as semantic generic parameters for the first advanced trait model
2. **Associated types** - still deferred unless generic trait parameters prove insufficient
3. **Explicit apply precedence** - rejected; overlapping applies remain errors
4. **More operator traits** - `Mul`, `Div`, `Mod`, `BitAnd`, etc.
5. **Trait bounds on generics** - `where T: TraitA + TraitB`
6. **Self-referential traits** - accepted for static dispatch; restricted for `any` object safety

## Relationship To Other Documents

- `post-v1-implementation-plan.md` - Wave 7.6 item, now marked "Done"
- `post-v1-closure-matrix.md` - 7.1.12 "Trait coherence" now "Defined"
- `post-v1-any-dispatch-stabilization.md` - `any<Trait>` safety constraints
- `language-reference.md` - Trait and apply syntax documentation
- `zenith-language-spec.md` - Core trait semantics

---

**Design Session Closure Date:** 2026-05-02  
**Next Action:** Audit implementation against this document, then mark Wave 7.6 complete.
