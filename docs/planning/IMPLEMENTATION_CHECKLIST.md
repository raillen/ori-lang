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
- [x] Nested field assignment lvalue lowering

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
- [x] Struct field assignment codegen
- [x] `is` type-check expression codegen for native backend
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
- [x] `string(...)` scalar conversion uses length-aware ABI for `int`, `float`, and `bool`
- [x] ARC retain/release insertion for managed types
- [x] Cycle detection for reference-counted objects in the native runtime (registered ARC graph edges)

## Native runtime route correction

Source: `docs/planning/native-runtime-route-correction-plan.md`.

Decision: the native backend and the Rust `ori-runtime` are the primary route.
The C backend remains a debug backend and must not define core language
semantics.

### Native runtime as source of truth

- [x] Treat `compiler/crates/ori-runtime` as the canonical runtime for `ori compile` and `ori test`.
- [x] Stop compiling `ORI_RUNTIME_C` in the native compile/test path.
- [x] Replace `build_runtime_lib()` with native runtime artifact discovery.
- [x] Remove the native path dependency on `ensure_cc_available()`.
- [x] Update tests so native runtime symbol coverage is checked against the Rust runtime, not the embedded C runtime.
- [x] Remove or rewrite tests that require native-runtime symbols to exist in `ORI_RUNTIME_C`.

### Native linker route

- [x] Add a `NativeLinker` abstraction for the Cranelift object -> executable step.
- [x] Stop calling `cc` directly from `ori-codegen::link` for the main native route.
- [x] Support `ORI_NATIVE_LINKER` as an explicit development/diagnostic override.
- [x] Support a packaged linker or `rust-lld` route where available.
- [x] Report missing linker errors as native-linker diagnostics, not as C compiler requirements.
- [x] Keep platform-specific linker flags in a testable structure.

### Runtime packaging

- [x] Stage `ori-runtime` static library artifacts under `runtime/{target-triple}`.
- [x] Generate or maintain `runtime-link.json` for native static libraries required by the Rust `staticlib`.
- [x] Validate Windows GNU/MSVC runtime artifact naming separately.
- [x] Validate Linux runtime artifact naming separately.
- [x] Add a release-layout test or smoke check that `ori compile` works outside the Cargo workspace.

### Documentation and CLI contract

- [x] Update `README.md` so `ori compile` and `ori test` no longer say they require a C toolchain.
- [x] Update `docs/spec/10-memory.md` so Rust `ori-runtime` is documented as the native runtime.
- [x] Update `docs/spec/12-stdlib.md` so native runtime coverage is canonical and C backend coverage is secondary.
- [x] Document the C backend as debug/transpile support with partial feature parity.
- [x] Add a diagnostic for C-debug-backend feature gaps when generating C would be semantically wrong.

## C backend

Status: debug backend with partial feature parity. It may reject features that
the native backend supports when generating C would be semantically wrong.

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
- [x] `is` type-check expression codegen
- [x] Index range/slicing codegen (`__slice` method call)
- [x] Tuple proper C struct emission (named typedefs for reusable tuple values)
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
- [x] String equality/inequality via `ori_string_eq`
- [x] `?` propagation in C backend
- [x] ARC retain/release insertion for managed types
- [x] Cycle detection for reference-counted objects in the C backend inline ARC graph

## Runtime and standard library

- [x] Basic `ori.io.print` integration
- [x] Basic integer-to-string runtime support
- [x] Basic list runtime hooks for native backend
- [x] Stable standard library module surface
- [x] String runtime completeness
- [x] List/map/set runtime completeness
- [x] Error/result runtime conventions
- [x] File/process/network APIs scoped out of the core distribution
- [x] Lazy runtime and stdlib (`lazy<T>`, `lazy.once`, `lazy.force`)

### `ori.string` operations covered

- [x] `trim_start(s) -> string`
- [x] `trim_end(s) -> string`
- [x] `index_of(sub) -> int`
- [x] `join(list, sep) -> string`
- [x] `repeat(s, n) -> string`
- [x] `pad_left(s, n, ch) -> string`
- [x] `pad_right(s, n, ch) -> string`
- [x] `parse_int(s) -> result<int, string>` and `parse_float(s) -> result<float, string>`

### `ori.math` operations covered

- [x] `pow(base, exp) -> float`
- [x] `floor(n) -> int`
- [x] `ceil(n) -> int`
- [x] `round(n) -> int`
- [x] `log(n) -> float`
- [x] `log2(n) -> float`
- [x] `sin(n) -> float`
- [x] `cos(n) -> float`
- [x] `tan(n) -> float`
- [x] `clamp(value, min, max) -> int`
- [x] `is_nan(n) -> bool`
- [x] `is_infinite(n) -> bool`
- [x] `pi`, `e`, `infinity`, and `nan` constants (emitted as float literals in HIR)
- [x] Float overloads for `abs`, `min`, and `max`.

### Runtime and ABI source of truth

- [x] `compiler/crates/ori-types/src/stdlib.rs` owns canonical stdlib paths, compatibility aliases, runtime symbols, and backend coverage flags.
- [x] HIR lowering consults the stdlib manifest before falling back to legacy mappings.
- [x] The checker canonicalizes stdlib aliases through the stdlib manifest before resolving signatures.
- [x] Tests validate that manifest symbols have HIR function types.
- [x] Tests validate that manifest symbols required by native lowering exist in the native backend declarations.
- [x] Tests validate that manifest symbols required by native lowering exist in the Rust runtime.
- [x] Tests validate that manifest symbols marked for the C backend exist in the C backend inline runtime.
- [x] Full ABI parameter/return type generation from the stdlib manifest for runtime symbols. Helper-only native imports remain local to the backend.

### `ori.list` operations covered

- [x] `pop(list) -> T`
- [x] `remove(list, index)`
- [x] `insert(list, index, value)`
- [x] `sort(list)`
- [x] `reverse(list)`
- [x] `contains(list, value) -> bool`
- [x] `contains(list, value)` rejects values that do not match the list element type.
- [x] `index_of(list, value) -> int`
- [x] `slice(list, start, end) -> list<T>`
- [x] `map(list, fn) -> list<U>` (type-erased fn_ptr/env_ptr ABI, codegen expansion)
- [x] `filter(list, fn) -> list<T>` (type-erased fn_ptr/env_ptr ABI, codegen expansion)

### `ori.map` operations covered

- [x] `remove(map, key)`
- [x] `keys(map) -> list<K>`
- [x] `values(map) -> list<V>`
- [x] `entries(map) -> list<tuple<K, V>>`
- [x] Hash-based implementation for `int` keys (open-addressing hash map with dense key/value arrays)
- [x] Hash-based implementation for `string` keys with textual equality in the Rust native runtime and C debug backend inline runtime.
- [x] Checker accepts built-in `int`/`string` keys and user-defined `Hashable`/`Equatable` keys; unsupported key types are rejected with `type.collection_hash_unsupported`.
- [x] Generic user-defined `Hashable`/`Equatable` map keys are accepted behind the core trait gate for native runtime map operations.

### `ori.set` operations covered

- [x] `remove(set, value)`
- [x] `union(a, b) -> set<T>`
- [x] `intersection(a, b) -> set<T>`
- [x] `difference(a, b) -> set<T>`
- [x] Hash-based implementation for `int` elements (open-addressing hash set with dense item array)
- [x] Hash-based implementation for `string` elements with textual equality in the Rust native runtime and C debug backend inline runtime.
- [x] Checker accepts built-in `int`/`string` elements and user-defined `Hashable`/`Equatable` elements; unsupported element types are rejected with `type.collection_hash_unsupported`.
- [x] Generic `Hashable`/`Equatable` set elements are accepted behind the core trait gate for native runtime set operations.

### Type conversions

- [x] `float_to_string(n) -> string`
- [x] `bool_to_string(b) -> string`
- [x] `string_to_int(s) -> optional<int>`
- [x] `string_to_float(s) -> optional<float>`
- [x] Compatibility aliases without `ori.convert` are accepted; docs prefer explicit `ori.convert.*`.

### Stdlib module import status

Implemented and importable:

- [x] `ori.core`
- [x] `ori.io`
- [x] `ori.fs` - file system operations (read_text, write_text, append_text, exists, delete, list_dir, create_dir, is_file, is_dir, copy, rename). `ori.files` remains as a compatibility alias.
- [x] `ori.files` - compatibility alias for `ori.fs`
- [x] `ori.string`
- [x] `ori.bytes` - byte-level operations (len, concat, slice, to_hex, from_hex, decode_utf8, get)
- [x] `ori.list`
- [x] `ori.map`
- [x] `ori.set`
- [x] `ori.math`
- [x] `ori.convert`
- [x] `ori.mem` - memory inspection utilities (`size_of(value)`, `align_of(value)`); type-argument call syntax such as `size_of<T>()` remains planned
- [x] `ori.time` - time operations (`now`, `sleep`, `duration_ms`)
- [x] `ori.format` - formatting utilities (`number`, `percent`, `hex`, `binary`, `date`, `datetime`, `bytes_size`)
- [x] `ori.os` - OS interactions (`args`, `env`, `exit`, `pid`, `platform`, `arch`)
- [x] `ori.random` - scalar random generation (`int`, `float`, `bool`) plus generic `choice<T>`/`shuffle<T>` using the list/optional storage ABI
- [x] `ori.iter` - eager generic list operations `map`, `filter`, `any`, `all`, `count_where`, `take`, `skip`, `reverse`, `reduce`, `find`, `flat_map`, `sort`, `sort_by`, `unique`, `zip`, `partition`, `group_by`, and `flatten`; native runtime covers non-`int` list storage and string sort/unique/group_by specializations
- [x] `ori.lazy` - lazy values (`lazy<T>`, `once`, `force`) with at-most-once evaluation
- [x] `ori.Error` - standard error value type with `code` and `message` fields; richer cause chaining and trait-method integration remain planned

Partially importable modules:

- [x] `ori.test` - assertion helpers (`assert`, `assert_eq`, `assert_ne`, `fail`) plus `ori test`; `assert_eq`/`assert_ne` support generic equality for `int`, `bool`, `float`, `string`, and user values gated by `Equatable`

Implemented native-runtime module:
- [x] `ori.json` - JSON parsing and serialization with `ori.json.Value` currently represented as canonical JSON text (`string`)

### Function-level stdlib gaps

- [x] Generic `ori.test.assert_eq<T>` and `ori.test.assert_ne<T>` support non-`int` values for the current equality ABI.
- [x] Generic `ori.random.choice<T>` and `ori.random.shuffle<T>` for non-`int` element types use the list/optional storage ABI.
- [x] Generic `ori.iter.map<T, R>`, `filter<T>`, `any<T>`, `all<T>`, `count_where<T>`, `take<T>`, `skip<T>`, `reverse<T>`, `reduce<T, R>`, `find<T>`, `flat_map<T, R>`, `flatten<T>`, `sort<T>`, `sort_by<T>`, `unique<T>`, `zip<A, B>`, `partition<T>`, and `group_by<T, K>` are typed and covered by native-runtime tests for non-`int` element/key types.

## Concurrency and async

Source: `docs/planning/async-implementation-plan.md`.

Decision: implement native concurrency primitives first, then `future<T>`,
then `async func` and `await`.

### Concurrency foundation

- [x] Add `ori.concurrent` module.
- [x] Add `Transferable` as the checker/runtime boundary rule for values crossing tasks or channels.
- [x] Mark primitive scalar types as `Transferable`.
- [x] Mark `string` and `bytes` as `Transferable` through safe ownership/copy semantics.
- [x] Mark `list<T>`, `map<K, V>`, and `set<T>` as `Transferable` only when their contents are transferable.
- [x] Mark structs as `Transferable` only when all fields are transferable.
- [x] Reject non-transferable task/channel values with a clear diagnostic.

### `ori.task`

- [x] Add `ori.task` to the stdlib module registry.
- [x] Add `task.Job<T>` type.
- [x] Add `task.JoinError` type.
- [x] Implement `task.spawn<T>(work: func() -> T) -> task.Job<T>` in the native runtime.
- [x] Implement `task.join<T>(job: task.Job<T>) -> result<T, task.JoinError>` in the native runtime.
- [x] Implement `task.detach<T>(job: task.Job<T>) -> void` or document it as planned.
- [x] Require `task.spawn` closures and captured values to satisfy `Transferable`.
- [x] Reject `var` captures consistently with the current closure capture rule.

### `ori.channel`

- [x] Add `ori.channel` to the stdlib module registry.
- [x] Add `channel.Channel<T>` type.
- [x] Add `channel.SendError` and `channel.ReceiveError` types.
- [x] Implement `channel.create<T>() -> channel.Channel<T>`.
- [x] Implement `channel.send<T>(ch: channel.Channel<T>, value: T) -> result<void, channel.SendError>`.
- [x] Implement `channel.receive<T>(ch: channel.Channel<T>) -> result<T, channel.ReceiveError>`.
- [x] Implement `channel.close<T>(ch: channel.Channel<T>) -> void`.
- [x] Use real synchronization in the native runtime; do not rely on unsynchronized shared storage.

### `ori.atomic`

- [x] Add `ori.atomic` to the stdlib module registry.
- [x] Add `atomic.AtomicInt`.
- [x] Implement `atomic.new(value: int) -> atomic.AtomicInt`.
- [x] Implement `atomic.load(value: atomic.AtomicInt) -> int`.
- [x] Implement `atomic.store(value: atomic.AtomicInt, next: int) -> void`.
- [x] Implement `atomic.add(value: atomic.AtomicInt, delta: int) -> int`.
- [x] Defer generic `Atomic<T>` until a concrete need exists.

### `future<T>` and executor

- [x] Add `future<T>` to the type system.
- [x] Add native runtime representation for ready, pending, failed, and cancelled futures.
- [x] Implement a minimal native executor.
- [x] Implement `task.block_on<T>(future: future<T>) -> T`.
- [x] Implement `task.sleep(ms: int) -> future<void>`.
- [x] Define cancellation semantics before exposing public cancellation APIs.
- [x] Define ARC ownership rules for values stored inside futures.

### `async func` and `await`

- [x] Add parser support for contextual `async func`.
- [x] Add parser support for `await expr`.
- [x] Add AST nodes or flags for async functions and await expressions.
- [x] Add HIR representation for async functions and await expressions.
- [x] Make calls to `async func f(...) -> T` type as `future<T>`.
- [x] Reject `await` outside async functions.
- [x] Reject `await` on non-`future<T>` values.
- [x] Support `async func main()` by lowering it through the native executor.
- [x] Define and implement `using` behavior across `await`, or reject it with a clear diagnostic until cleanup is safe.
- [ ] Preserve ARC retain/release correctness for values live across `await`.
- [ ] Generate a native state machine for async functions with real suspension points.

### Async stdlib and tooling

- [x] Add `ori.fs.read_text_async`.
- [x] Add `ori.fs.write_text_async`.
- [x] Add async time/sleep API under `ori.task` or `ori.time`.
- [x] Add `@test async func` support in `ori test`.
- [x] Add formatter support for `async func` and `await`.
- [x] Add diagnostics catalog entries for async/concurrency errors.
- [x] Let the C debug backend reject async/concurrency features with a clear unsupported-backend diagnostic.

### Async/concurrency tests

- [x] Checker tests for `Transferable` acceptance and rejection.
- [x] Checker tests for `task.spawn` capture rules.
- [x] Native runtime tests for `task.spawn` + `task.join`.
- [x] Native runtime tests for `channel.send` + `channel.receive`.
- [x] Native runtime tests for `atomic.AtomicInt`.
- [x] Type checker tests for `future<T>`, `async func`, and `await`.
- [x] Native compile/run tests for `task.block_on`.
- [x] Native compile/run tests for `async main`.
- [x] Native compile/run tests for `await` on a ready future.
- [x] Native compile/run tests for `await task.sleep(...)`.
- [x] Negative tests for `await` outside async and `await` on non-future values.
- [x] C debug backend tests that unsupported async/concurrency features fail clearly.

## Documentation comments and attributes

- [x] Attribute syntax is parsed on top-level declarations and stored in the AST
- [x] Attribute validation (`attr.unknown`, `attr.invalid_target`, `attr.duplicate`, `attr.invalid_arg`)
- [x] `@deprecated` use-site warnings
- [x] `@test` concrete no-arg/no-return functions run through `ori test`; `ori.test.assert` and `ori.test.fail` are importable
- [x] Block comments are lexed and skipped as trivia during normal compilation
- [x] Documentation comment extraction for `ori doc`
- [x] `@param` doc tag validation warns when a tag names a parameter that does not exist on the documented function

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
- [x] Pipe operator tests
- [x] Struct update expression tests
- [x] `is` type-check expression tests
- [x] Map/set literal tests
- [x] Index slicing tests
- [x] Value contract violation tests
- [x] Default parameter tests
- [x] Generic monomorphization tests
- [x] `for` over collections tests (list, set, map, string)
- [x] `using`/dispose tests
- [x] Match exhaustiveness diagnostic tests
- [x] String concatenation tests
- [x] `any<Trait>` dynamic dispatch tests
- [x] `@test` runner tests for passing tests, failing `check`, and invalid test signatures
- [x] `ori.test` assertion helper tests for native runner and C backend generation

## Audit backlog - 2026-05-13

Source: `_reversa_sdd/auditoria-profunda-implementacao-linguagem-2026-05-13.md`.

These items are observable gaps found after the previous correction rounds.
Close them with code, docs, and regression tests together.

### Parser and syntax gaps

- [x] Field assignment parses as a real lvalue: `b.value = 2` must not be discarded during parser recovery.
- [x] Invalid lvalue conversion emits a diagnostic instead of silently dropping the statement and following statements.
- [x] `mut func` method syntax is aligned with the spec: implicit `self` is supported while explicit `self` remains accepted for compatibility.
- [x] Method declarations that pass `check` cannot fail later in native codegen because of missing receiver ABI.
- [x] Variadic parameter syntax accepts the documented `Type...` form or the spec is updated to the implemented form.
- [x] Variadic parameters are rejected when they are not the last parameter.
- [x] Parameters with defaults before required parameters are rejected at declaration time.
- [x] Duplicate struct fields are rejected with a clear diagnostic.
- [x] Duplicate enum variants are rejected with a clear diagnostic.
- [x] F-string diagnostics inside `{expr}` point to the original source span.

### Name resolution and semantic gaps

- [x] Unknown single-segment names fail in `ori check` instead of becoming `_#0`.
- [x] Unknown function calls fail in `ori check` instead of becoming missing backend references.
- [x] Unknown multi-segment paths fail in `ori check` with a path/name diagnostic.
- [x] `panic`, `todo`, and `unreachable` are implemented as language forms or documented as planned.
- [x] `panic`, `todo`, and `unreachable` return `never` and interact correctly with missing-return analysis.
- [x] `and` and `or` require boolean operands.
- [x] `not` requires a boolean operand.
- [x] Closures reject capture of `var` bindings if the spec keeps the current capture rule.
- [x] Discarded `result<T, E>` expression statements emit `type.unused_result` or the spec is changed.

### Lexer and literal edge cases

- [x] `--|` inside normal string literals is treated as string text, not as a block comment start.
- [x] `--|` inside byte strings and f-string literal text is treated as text.
- [x] Unclosed real block comments keep the dedicated `lex.unclosed_block_comment` diagnostic.

### Runtime and backend behavior gaps

- [x] List index out of bounds panics at runtime instead of returning `0`.
- [x] Byte index out of bounds panics at runtime instead of returning `0`.
- [x] String/list/bytes slice bounds follow one documented rule: panic, clamp, or explicit checked behavior.
- [x] Negative `repeat` counts panic at runtime or the spec is changed.
- [x] Native backend rejects invalid HIR before reaching Cranelift verifier errors.
- [x] Rust native runtime and C debug backend inline runtime share the same bounds behavior for covered features.

### Stdlib and core contract gaps

- [x] `.or(fallback)` for `optional<T>` is implemented or moved to planned status.
- [x] `.or_return(value)` for `optional<T>` is implemented or moved to planned status.
- [x] `.or_wrap(context)` for `result<T, E>` is implemented or moved to planned status.
- [x] `ori.core` provides real built-in traits such as `Displayable`, `Equatable`, `Comparable`, `Hashable`, `Disposable`, `Default`, `Error`, and `Cloneable`, or the docs are reduced.
- [x] `using` resolves `Disposable` through the chosen core trait contract, not only a local trait named `Disposable`.
- [x] Stdlib generic signatures preserve type parameters across arguments and returns instead of using unconstrained `Ty::Infer(0)`.
- [x] `ori.list.contains(list<int>, "x")` and similar mismatches fail in the checker.
- [x] `ori.mem` is either implemented or listed consistently as planned in the spec and checklist.
- [x] README quick example compiles with today's stdlib surface.
- [x] README text no longer contains broken control characters in words like `namespace` and `result`.

### Regression tests required for this audit

- [x] Add checker/parser/native tests for field assignment, invalid lvalues, implicit `self`, and `mut func` receiver diagnostics.
- [x] Add checker/parser tests for duplicate fields, duplicate enum variants, variadic syntax, and default-parameter ordering.
- [x] Add checker tests for unknown names, unknown calls, and logical operators.
- [x] Add checker tests for closure capture of `var`.
- [x] Add checker tests for discarded `result`.
- [x] Add checker tests for `panic`/`todo`/`unreachable`.
- [x] Add lexer tests for `--|` inside strings, byte strings, and f-string text.
- [x] Add native runtime tests for out-of-bounds index.
- [x] Add native runtime tests for negative `repeat`.
- [x] Add native runtime tests for invalid slice bounds.
- [x] Add stdlib tests for optional/result helpers, `ori.core` traits, `using` with `Disposable`, list generic mismatches, and `ori.mem`.
- [x] Add documentation tests that extract and check the README quick example.

## Semantic enhancement and architecture follow-up

- [x] G2-01: Negative generic constraints (`where T is not Trait`) emit a dedicated diagnostic at call sites.
- [x] G2-02: Trait method lookup fallback in method-call inference is covered, including default trait methods.
- [x] G2-03: Match exhaustiveness for enums accounts for payload variants and rejects unit/field-shape mismatches.
- [x] G3-01: `check.rs` has initial submodules for generic constraints and match exhaustiveness.
- [x] G3-02: Native `using` cleanup is covered before failing `check` traps.
- [x] G3-03: Operator overloading via `ori.core` traits is implemented for `+`, `-`, `==`, `!=`, `<`, `<=`, `>`, and `>=`.

## Documentation and tooling

- [x] Language reference aligned with implementation
- [x] Module/import documentation
- [x] Visibility rules documentation
- [x] Standard library documentation
- [x] Diagnostics catalog
- [x] Formatter integration for new block/attribute syntax through `ori fmt`
- [x] LSP/editor support is planned; `ori-lsp` is currently a placeholder and exits with a clear not-implemented message
