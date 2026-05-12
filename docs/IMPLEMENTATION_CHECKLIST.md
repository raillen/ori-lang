# Ori implementation checklist

This checklist tracks what is implemented and what still needs work before Ori can be considered language-complete.

## Compiler pipeline

- [x] Lexing and parsing entry points for `.orl` source files
- [x] Single-file compile/check/build pipeline
- [x] Multi-file local import loading
- [x] Recursive transitive import loading
- [x] Namespace validation for imported files
- [x] Combined `DefMap` across loaded files
- [x] Combined function signatures across loaded files
- [x] Combined top-level `const`/`var` type signatures across loaded files
- [x] Combined struct field signatures across loaded files
- [x] Import cycle diagnostics
- [x] Ambiguous import path diagnostics
- [x] Project/package root discovery
- [x] Manifest-based package/module configuration
- [x] Stable module path policy for `mod.orl`/`index.orl` if adopted

## Namespaces and imports

- [x] Explicit import aliases, e.g. `import app.util as util`
- [x] Default aliases from last import segment, e.g. `import app.util` -> `util`
- [x] Alias expansion in type lowering
- [x] Alias expansion in type checking
- [x] Alias expansion in HIR lowering
- [x] Imported function calls
- [x] Imported type annotations in function signatures
- [x] Imported top-level constants
- [x] Imported struct field access
- [x] Duplicate import alias diagnostics
- [x] Import/local symbol alias conflict diagnostics
- [x] Unused import warnings
- [x] Public/private visibility enforcement across namespaces
- [x] Re-export/export model if adopted

## Name resolution and type checking

- [x] Top-level definition registration for functions, structs, enums, traits, aliases, consts, vars, externs
- [x] Function return type lookup by `DefId`
- [x] Function call argument count validation
- [x] Function call argument type validation
- [x] Top-level `const`/`var` declaration checking
- [x] Imported top-level `const`/`var` use-site type checking
- [x] Struct field type lookup by `DefId`
- [x] Missing struct field diagnostics
- [x] Field access on non-struct diagnostics
- [x] Complete generic type inference/unification
- [x] Generic substitution for imported generic structs/functions
- [x] List/set/map element consistency validation
- [x] Tuple field/type checking completeness
- [x] Index expression type checking
- [x] Match expression/pattern type checking
- [x] Optional/result propagation type checking completeness
- [x] Method resolution
- [x] Trait constraint checking
- [x] Implementation coherence checks
- [x] Pipe expression (`|>`) type inference
- [x] Struct update expression (`with`) type inference
- [x] `is` type-check expression type inference
- [x] Closure expression type inference and environment capture
- [x] Map literal type inference
- [x] Set literal type inference
- [x] Generic monomorphization pass
- [x] `where` clause constraint enforcement
- [x] Match exhaustiveness checking
- [x] String `+` concatenation special-casing via runtime call
- [x] Value contract validation on function parameters (`name: int if it > 0`)
- [x] Value contract validation on struct fields (`field: int if it > 0`)
- [x] Default parameter codegen (use default value when argument omitted)
- [x] Variadic parameter type checking and desugaring
- [x] `any<Trait>` dynamic dispatch type checking

## HIR lowering

- [x] Function lowering with fully qualified names
- [x] Struct lowering with fully qualified names
- [x] Enum lowering with fully qualified names
- [x] Const lowering with fully qualified names
- [x] Alias-aware expression lowering for imported definitions
- [x] Alias-aware type lowering
- [x] Top-level `var` lowering
- [x] Type alias lowering/resolution
- [x] Trait lowering
- [x] Implement lowering
- [x] Extern lowering completeness
- [x] Struct literal lowering with imported type aliases
- [x] Enum variant lowering with imported enum types
- [x] Statement lowering for `using`, `ifsome`, `whilesome`, and remaining control forms
- [x] Pipe expression (`|>`) lowering (desugars to Call)
- [x] Struct update expression (`x with { field: v } end`) lowering
- [x] `is` type-check expression lowering
- [x] Closure expression lowering with environment capture
- [x] Map literal lowering (`HirExprKind::MapLit`)
- [x] Set literal lowering (`HirExprKind::SetLit`)
- [x] Index range/slicing lowering (desugars to `__slice` method call)
- [x] Type alias full expansion/substitution during resolution

## Native backend

- [x] Cranelift native compile path
- [x] Qualified function symbol mangling
- [x] Entry namespace `main` wrapper
- [x] Imported function calls
- [x] Imported top-level constants as emitted expressions
- [x] For loops over ranges and lists
- [x] Basic struct layout and struct literal emission
- [x] Real global data for top-level constants where needed
- [x] Top-level mutable globals
- [x] Complete enum codegen
- [x] Complete pattern/match codegen
- [x] Complete optional/result ABI handling
- [x] Remove silent fallback values such as zero/null for unsupported expressions
- [x] Runtime ownership/freeing model for native/runtime v1
- [x] Closure codegen (environment capture, call-site invocation)
- [x] Map literal codegen
- [x] Set literal codegen
- [x] Pipe expression codegen (desugared at HIR level)
- [x] Struct update expression codegen
- [x] `is` type-check expression codegen (placeholder `true`; full `any<Trait>` pending)
- [x] Index range/slicing codegen (`__slice` method call)
- [x] `for` loop over `list`, `set`, `string` (set uses same list API; string via `ori_string_chars`)
- [x] `for` loop over `map`
- [x] `for` loop second binding (index variable)
- [x] Tuple destructuring in pattern matching (native backend)
- [x] `using` statement dispose/cleanup call on scope exit
- [x] Value contract runtime checks (parameter and struct field contracts)
- [x] Default parameter value insertion at call site
- [x] Variadic parameter codegen
- [x] Named/labeled argument codegen for function calls
- [x] Spread argument (`..expr`) codegen
- [x] `any<Trait>` vtable generation and dynamic dispatch
- [x] Trait default method dispatch (choosing default vs. overridden)
- [x] Generic monomorphization in emitted code
- [x] String concatenation via `ori_string_concat` runtime call
- [x] String equality/inequality via `strcmp`
- [x] `ori_to_string` multi-value return (currently stubbed as ptr)
- [x] ARC retain/release insertion for managed types
- [x] Cycle detection for reference-counted objects

## C backend

- [x] C source generation entry point
- [x] Qualified function symbol mangling
- [x] Entry namespace `main` wrapper
- [x] `DefId`-based names for structs/enums/named types
- [x] Qualified/imported constants in generated C
- [x] Complete struct literal codegen
- [x] Complete enum value/payload codegen
- [x] String interpolation codegen
- [x] List literal codegen
- [x] Tuple literal/codegen types
- [x] Index assignment codegen
- [x] Pattern matching codegen completeness
- [x] Runtime ABI cleanup for strings/lists/results/options
- [x] Closure codegen (environment capture via captured-env struct)
- [x] Map literal codegen
- [x] Set literal codegen
- [x] Pipe expression codegen (desugared at HIR level)
- [x] Struct update expression codegen
- [x] `is` type-check expression codegen (placeholder; full `any<Trait>` pending)
- [x] Index range/slicing codegen (`__slice` method call)
- [x] Tuple proper C struct emission (anonymous struct)
- [x] Tuple destructuring in pattern matching
- [x] `for` loop over `list`, `set`, `string` (via runtime calls)
- [x] `for` loop over `map`
- [x] `for` loop second binding (index variable)
- [x] `using` statement dispose/cleanup call on scope exit
- [x] Value contract runtime checks (parameter and struct field contracts)
- [x] Default parameter value insertion at call site
- [x] Variadic parameter codegen
- [x] Named/labeled argument codegen for function calls
- [x] Spread argument (`..expr`) codegen
- [x] `any<Trait>` vtable generation and dynamic dispatch
- [x] Trait default method dispatch (choosing default vs. overridden)
- [x] Generic monomorphization in emitted code
- [x] String concatenation via `ori_string_concat` call
- [x] String equality/inequality via `strcmp`
- [x] ARC retain/release insertion for managed types
- [x] Cycle detection for reference-counted objects

## Runtime and standard library

- [x] Basic `ori.io.print` integration
- [x] Basic integer-to-string runtime support
- [x] Basic list runtime hooks for native backend
- [x] Stable standard library module surface
- [x] String runtime completeness
- [x] List/map/set runtime completeness
- [x] Error/result runtime conventions
- [x] File/process/network APIs scoped out of the core distribution

### `ori.string` — missing operations

- [x] `index_of(sub) -> int`
- [x] `join(list, sep) -> string`
- [x] `repeat(s, n) -> string`
- [x] `pad_left(s, n, ch) -> string`
- [x] `pad_right(s, n, ch) -> string`

### `ori.math` — missing operations

- [x] `pow(base, exp) -> float`
- [x] `floor(n) -> int`
- [x] `ceil(n) -> int`
- [x] `round(n) -> int`
- [x] `log(n) -> float`
- [x] `sin(n) -> float`
- [x] `cos(n) -> float`
- [x] `tan(n) -> float`
- [x] `pi` and `e` constants (emitted as float literals in HIR)

### `ori.list` — missing operations

- [x] `pop(list) -> T`
- [x] `remove(list, index)`
- [x] `insert(list, index, value)`
- [x] `sort(list)`
- [x] `reverse(list)`
- [x] `contains(list, value) -> bool`
- [x] `index_of(list, value) -> int`
- [x] `slice(list, start, end) -> list<T>`
- [x] `map(list, fn) -> list<U>` (type-erased fn_ptr/env_ptr ABI, codegen expansion)
- [x] `filter(list, fn) -> list<T>` (type-erased fn_ptr/env_ptr ABI, codegen expansion)

### `ori.map` — missing operations

- [x] `remove(map, key)`
- [x] `keys(map) -> list<K>`
- [x] `values(map) -> list<V>`
- [x] `entries(map) -> list<tuple<K, V>>`
- [ ] Proper hash-based implementation (currently linear-scan paired arrays)

### `ori.set` — missing operations

- [x] `remove(set, value)`
- [x] `union(a, b) -> set<T>`
- [x] `intersection(a, b) -> set<T>`
- [x] `difference(a, b) -> set<T>`
- [ ] Proper hash-based implementation (currently linear-scan list)

### Type conversions

- [x] `float_to_string(n) -> string`
- [x] `bool_to_string(b) -> string`
- [x] `string_to_int(s) -> optional<int>`
- [x] `string_to_float(s) -> optional<float>`

### Planned stdlib modules (not yet implemented)

- [ ] `ori.fs` — file system operations (read_text, write_text, exists, delete, list_dir, etc.)
- [ ] `ori.bytes` — byte-level operations
- [ ] `ori.iter` — functional collection operations (map, filter, reduce, zip, enumerate, etc.)
- [ ] `ori.format` — formatting utilities
- [ ] `ori.time` — time operations (now, sleep, duration, etc.)
- [ ] `ori.random` — random number generation (int, float, choice, shuffle, etc.)
- [ ] `ori.json` — JSON parsing and serialization
- [ ] `ori.os` — OS interactions (env, args, exit, etc.)
- [ ] `ori.test` — testing framework
- [ ] `ori.Error` — standard error type

## Tests and validation

- [x] Multi-file import integration tests
- [x] Default and explicit alias tests
- [x] Transitive import tests
- [x] Missing import diagnostics tests
- [x] Namespace mismatch diagnostics tests
- [x] Imported function arity/type mismatch tests
- [x] Imported const declaration/use-site type tests
- [x] Imported struct field lookup tests
- [x] Same type name in different namespace C backend test
- [x] Import cycle tests
- [x] Duplicate alias tests
- [x] Private item import tests
- [x] Ambiguous import tests
- [x] Imported struct literal tests
- [x] Imported enum variant tests
- [x] Larger project tree tests
- [x] End-to-end compile/run tests for complex imported structs/enums
- [x] Closure expression tests (capture, invoke, pass as argument)
- [ ] Pipe operator tests
- [ ] Struct update expression tests
- [x] `is` type-check expression tests
- [ ] Map/set literal tests
- [ ] Index slicing tests
- [ ] Value contract violation tests
- [x] Default parameter tests
- [x] Generic monomorphization tests
- [ ] `for` over collections tests (list, set, map, string)
- [ ] `using`/dispose tests
- [x] Match exhaustiveness diagnostic tests
- [ ] String concatenation tests
- [x] `any<Trait>` dynamic dispatch tests

## Documentation and tooling

- [ ] Language reference aligned with implementation
- [ ] Module/import documentation
- [ ] Visibility rules documentation
- [ ] Standard library documentation
- [ ] Diagnostics catalog
- [ ] Formatter integration for new syntax
- [ ] LSP/editor support
