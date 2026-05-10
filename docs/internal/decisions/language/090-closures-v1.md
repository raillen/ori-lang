# Decision 090 - Closures v1 (Immutable Capture)

- Status: accepted
- Date: 2026-04-24
- Type: language / semantics / codegen
- Scope: closures, local function declarations, anonymous function expressions, variable capture, fat pointers

## Summary

Zenith introduces closures in R3.M6 ("Closures v1"). A closure is a function (either named or anonymous) that captures variables from its enclosing lexical scope.

To adhere to Zenith's safe and predictable semantics:
1. **Immutable By-Value Capture**: Closures only capture variables by value (an immutable snapshot). Mutating a captured variable inside a closure is a semantic error.
2. **Fat Pointer Representation**: The `func(T...) -> R` type is internally upgraded to a **Fat Pointer** (`struct { void* fn; void* ctx; }`). This universally handles both pure functions and capturing closures under a single, unified type.
3. **ARC Integration**: Captured dynamically-allocated variables (like `list`, `map`, `text`) are automatically retained (`zt_retain`) when the closure is created and released (`zt_release`) when the closure falls out of scope, preventing memory leaks while strictly enforcing safe memory management without a borrow checker.

## Motivation

- Enable Higher-Order Functions (HOF) like `map`, `filter`, and `reduce`, which are essential to modern software composition.
- Prevent the scope from being polluted with one-off helper functions by supporting locally-scoped function declarations and expressions.
- Keep the language unified: instead of fracturing the type system into `func` vs `closure`, we choose a unified "Fat Pointer" approach where any callable can hold state natively.

## Syntax

Closures can be defined in two ways:

**1. Anonymous Function Expression**
```zt
const factor = 10
const multiply = func(val: int) -> int
    return val * factor
end
```

**2. Named Local Function**
```zt
func process()
    const config = get_config()

    func internal_helper(data: text)
        print(config) -- Capturing 'config'
        print(data)
    end

    internal_helper("hello")
end
```

## Immutable Capture Rules

Variables captured from the outer scope are strictly immutable.

```zt
func test_capture()
    var count = 0
    const counter = func()
        count = count + 1   -- ERROR: closure.mut_capture_unsupported
    end
end
```

To mutate state across boundaries, explicit synchronization or messaging must be used (e.g., Channels / Tasks from R3.M2).

## Fat Pointer Convergence

In R3.M5, `func` variables were bare C function pointers. Starting in R3.M6, `func` variables become Fat Pointers.

When a closure is created, the compiler generates:
1. A C struct to hold the captured variables (the context).
2. A heap allocation for that struct, initializing it with copies of the captured values.
3. A `zt_closure` fat pointer containing the generated C function pointer and the allocated context.

If a `func` is assigned from a pure, non-capturing function, the fat pointer simply contains `ctx = NULL`.

## FFI Integration and the Bare Pointer Fallback

Because Zenith `func` types are now Fat Pointers, they are no longer trivially compatible with C ABI function pointers.

When a `func` is passed to an `extern c` parameter:
- The compiler statically allows it, **BUT** it will enforce that the passed `func` must be a pure, non-capturing function reference (i.e. resolving directly to a top-level function).
- If the user attempts to pass a closure (an anonymous function or local function with captures) into an `extern c` API, the compiler emits a diagnostic: `callable.extern_c_closure_unsupported`.

## ARC Integration

Captured objects that manage memory via ARC (like `text` or `list`) are fully tracked.
- Upon closure creation, `zt_retain` is called on every ARC-managed captured variable.
- The closure generates an internal disposal routine.
- When the closure is no longer reachable, `zt_release` is invoked on all captured constraints before freeing the closure context itself.

## Future Milestones
- `R3.M7`: Will introduce syntax sugar for Lambdas (`x => x * 2`).
- `Mutable Captures`: Explict opt-in mutable captures may be evaluated in future cycles, but are forbidden in the v1 implementation to avoid hidden alias side-effects.
