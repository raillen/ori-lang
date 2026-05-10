# Decision 088 - Dyn Dispatch Minimum Subset (R3.M4)

- Status: accepted
- Date: 2026-04-23
- Type: language / compiler / runtime
- Scope: dynamic dispatch via `dyn<Trait>`, vtable generation, fat pointers, heterogeneous collections
- Upstream: `docs/internal/decisions/language/079-memory-and-dispatch-architecture.md` (dual dispatch model)
- Downstream: `docs/spec/language/dyn-dispatch.md`, `compiler/targets/c/emitter.c`, `runtime/c/zenith_rt.h`

## Summary

Zenith supports `dyn<Trait>` for user-defined traits with vtable-based dynamic dispatch, enabling heterogeneous collections and runtime polymorphism.

This decision defines the **minimum official subset** for R3.M4: which traits can be used with `dyn`, how vtables are generated, the fat-pointer representation, and the diagnostic boundaries for unsupported cases.

## Problem

Prior to R3.M4, `dyn<Trait>` was limited to `dyn<TextRepresentable>` only, implemented as a tagged union in the C runtime. Any `dyn<UserTrait>` produced a targeted diagnostic deferring support.

This blocked heterogeneous collections of user-defined types, a core requirement for UI elements, game objects, plugin systems, and any domain where runtime polymorphism is needed.

## Decision

### 1. Fat-Pointer Representation

`dyn<Trait>` values are represented as **fat pointers**: a pair of (data pointer, vtable pointer).

```c
typedef struct zt_vtable_header {
    zt_header header;
    void (*drop)(void *data);
    void (*clone_out)(void *dest, const void *src);
} zt_vtable_header;

typedef struct zt_dyn_value {
    zt_header header;
    void *data;
    zt_vtable_header *vtable;
} zt_dyn_value;
```

The vtable contains:
- `drop`: frees the boxed data (RC decrement for managed types)
- `clone_out`: deep-copies the boxed data (for value semantics)
- One function pointer per trait method

### 2. Minimum Subset Rules

A trait `T` can be used with `dyn<T>` if:

1. **All methods are non-mutating** (`mut func` is not allowed in dyn traits for this subset)
2. **All method parameters are copyable** (scalars, `text`, `bytes`, or structs/enums with copyable fields)
3. **All return types are copyable** (same constraint as parameters)
4. **The trait has no more than 8 methods** (vtable size bound for R3.M4)
5. **The trait has no type parameters** (generic traits cannot be dyn'd in this subset)
6. **The trait is public or in the same namespace** (visibility rules apply)

Violations produce targeted diagnostics with `ACTION/WHY/NEXT` format.

### 3. Boxing Rules

When a concrete value `x` of type `S` is used where `dyn<T>` is expected:

1. The compiler verifies `apply T to S` exists
2. A box is allocated containing:
   - A copy of `x` (deep copy for managed fields)
   - A pointer to the vtable for `S` implementing `T`
3. The box is reference-counted like any managed value
4. When RC reaches zero, `vtable.drop` is called

### 4. Method Dispatch

Calling a method on `dyn<T>`:

1. Load the vtable pointer from the fat pointer
2. Load the function pointer for the method from the vtable
3. Call the function with the data pointer as the receiver argument
4. Return the result

This is an indirect call through a function pointer, with predictable overhead.

### 5. Vtable Generation

For each `apply T to S`, the C backend generates:

```c
static zt_vtable_header zt_vtable_S__T = {
    .drop = zt_drop_S,
    .clone_out = zt_clone_S,
    .method1 = zt_S__T_method1,
    .method2 = zt_S__T_method2,
    ...
};
```

Function pointers in the vtable point to the monomorphized implementations of each method for type `S`.

### 6. Heterogeneous Collections

`list<dyn<T>>` is the primary use case. Elements are boxed fat pointers:

```zt
var shapes: list<dyn Drawable> = []
shapes.append(Circle(radius: 10))
shapes.append(Rectangle(width: 5, height: 3))

for shape in shapes
    shape.draw()  -- dynamic dispatch
end
```

### 7. Ownership and Lifetime

- Boxing takes ownership of the value (the original is moved into the box)
- The box is RC-managed; cloning the box clones the inner value
- `dyn<T>` cannot be used in `extern c` boundaries (FFI safety)
- `dyn<T>` values cannot escape their isolate (concurrency boundary)

### 8. Diagnostics

Unsupported cases produce clear diagnostics:

| Case | Diagnostic Code | Message |
|------|----------------|---------|
| `mut func` in dyn trait | `dyn.mut_method` | "dyn dispatch does not support mutating methods in R3.M4 subset" |
| Generic trait | `dyn.generic_trait` | "dyn dispatch does not support generic traits in R3.M4 subset" |
| >8 methods | `dyn.too_many_methods` | "dyn trait '%s' has %d methods; maximum is 8 for R3.M4 subset" |
| Uncopyable param/return | `dyn.uncopyable` | "dyn method '%s' has parameter/return type '%s' which is not copyable" |
| No apply found | `dyn.no_apply` | "no apply of trait '%s' found for type '%s'" |

## Consequences

- Heterogeneous collections are now possible for user-defined traits
- Predictable overhead: one indirect call per dyn method invocation
- Memory overhead: one vtable per (type, trait) pair + one box per dyn value
- Clear boundaries: what is supported vs deferred is documented and enforced
- Foundation for future expansion: mutable methods, generic traits, larger vtables

## Deferred to Future Cycles

- `mut func` in dyn traits (requires careful ownership semantics)
- Generic traits with `dyn` (complex vtable instantiation)
- Traits with >8 methods (arbitrary limit for R3.M4)
- `dyn<Trait>` in FFI boundaries (requires ABI stability guarantees)
- `dyn<Trait>` cross-isolate transfer (requires serialization or shared vtable registry)
- Inline caching or other dyn dispatch optimizations
