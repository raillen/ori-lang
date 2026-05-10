# Zenith Post-v1 Remaining Language Work

> Audience: maintainer, language designer, compiler/runtime implementer  
> Status: consolidation tracker  
> Surface: post-v1 follow-up  
> Last updated: 2026-05-10

## Purpose

This document consolidates the work that remains after the post-v1 language closure gate.

It is guided by:
- `post-v1-surface-contract.md`
- `post-v1-completeness-discussion.md`
- `post-v1-closure-matrix.md`
- `post-v1-implementation-plan.md`
- Wave 7 closure artifacts from `post-v1-syntax-freeze.md` through `post-v1-final-language-closure-review.md`

Wave 7 closed the core language, ZIR, compiler, diagnostics, runtime ABI, and backend-conformance contracts. The items below are no longer nebulous language blockers unless marked as needing discussion. They are grouped by current follow-up shape.

## 1. Parcial

These items have an accepted route or current executable subset, but implementation is incomplete, backend-limited, or requires incremental hardening.

### Concurrency

- **Wider concurrency runtime payloads:** `Job<int>`, `Job<text>`, `Channel<int>`, `Channel<text>`, `Shared<int>`, and `Atomic<int>` are executable today. Future work must generate/select monomorphized typed runtime storage for wider `Job<T>`, `Channel<T>`, `Shared<T>`, and `Atomic<T>` payloads, or reject unsupported payloads with backend capability diagnostics.
- **Channel capacity/backpressure:** current channels are single-slot and non-blocking. Future bounded/blocking channels require explicit APIs such as `create_bounded(capacity)` or `send_blocking`.
- **Explicit cancellation:** jobs/channels have no implicit cancellation from scope exit, panic, or caller return. Future cancellation requires token/handle APIs that preserve `using` cleanup semantics.
- **Richer job panic capture:** current panic/contract failures at job boundaries are runtime boundary events. Future panic payload capture requires explicit API design.
- **Generic shared/atomic payloads:** `Atomic<int>` is the current supported atomic payload. Broader `Shared<T>` and `Atomic<T>` behavior depends on typed runtime wrappers and `Transferable`/ownership rules.

### `any` and traits

- **Advanced `any` safety:** current stabilized subset supports non-generic traits, non-mutating methods, small method sets, copyable signatures, and rejects `any` across `extern c`. Managed returns, broader trait shapes, and stronger cross-boundary guarantees remain future implementation/audit work.
- **Trait/operator expansion:** current operator overloading is Level 2 only: `Comparable`, `Addable`, and `Subtractable`. Additional operator traits such as multiplication, division, modulo, and bitwise operators require explicit future work.
- **Trait bounds hardening:** repeated named-trait bounds such as `where T is Addable and T is Comparable` are covered in the current subset. Shorthand compound spellings such as `where T: TraitA + TraitB`, public runtime capability traits, and richer bound expressions remain future implementation/design territory.

### Runtime and ownership

- **ZIR verifier hardening:** the verifier contract is closed, but additional checks can be hardened incrementally.
- **Golden ZIR fixtures:** textual dump stability is specified, but fixture coverage should expand as `emit-zir` matures.
- **Runtime ABI gap tracking:** runtime ABI is documented, but concrete ABI gaps should now be tracked as implementation issues rather than broad design uncertainty.
- **Stack-optimized `result<void,E>`:** accepted as runtime optimization direction, but not a completed general runtime layout feature.
- **Thread-safe ARC expansion:** atomic ARC for shared/concurrent managed values remains tied to future generic shared/concurrency payload expansion.
- **Advanced allocation resources:** `mem.Temp` and `mem.Pool<T>` are reserved library-level API names, but are not exposed in 0.4.2-beta.rc1. They require real API pressure plus deterministic `using` cleanup fixtures before becoming public.
- **Runtime-backed stdlib ownership notes:** modules such as lazy, net, time, fs/os/process, regex, random, and debug must keep ownership-bearing returns documented as they expand.

### Standard library

- **`std.net` expansion:** decided. Final `std.net` scope includes blocking TCP client and server primitives plus `Stream<bytes>`/`Sink<bytes>` adapters. Current implementation already provides a blocking TCP client foundation (`Connection`, `connect`, `read_some`, `write_all`, `close`, `is_closed`) backed by the C runtime; future implementation should reuse that base and extend it with `Listener`, `listen`, `accept`, listener cleanup, stream/sink adapters, and async serving helpers. Async serving is library-level through jobs/channels helpers. There is no hidden scheduler. TLS is layered as an explicit wrapper over TCP streams.
- **`std.http` foundation:** decided. Final `std.http` scope follows the foundation model, not a full framework. It remains in stdlib because HTTP is protocol infrastructure needed by tooling, package workflows, WebSocket handshake, playgrounds, and simple services. Final scope includes HTTP client and server foundation APIs: `Request`, `Response`, `Method`, `Status`, `Headers`, body as `bytes`/`text` and later `Stream<bytes>`, blocking core, and async helpers via jobs/channels. Minimal router helpers may live in stdlib. Full framework features such as middleware stacks, auth, sessions, templating, static file serving, schema binding, and rich routing belong in packages. HTTPS depends on the TLS decision.
- **`std.time` expansion:** MVP `Instant`/`Duration` exists. Calendar/date APIs remain future work.
- **Generic HOFs:** same-type primitive/text list HOFs exist, including `list.reduce<T,T>`. Cross-type `map<T,U>` and `reduce<T,U>` remain future work.
- **Lazy/generic lazy:** current lazy support is limited to executable C-backend subsets, and unsupported payloads are rejected during `zt check`. Full `lazy<T>` and lazy iterators remain future work.
- **`std.console` cursor movement:** accepted as direction, not part of the completed closure set.
- **Generic collections beyond validated C subset:** current C backend covers important list/map specializations. Nested managed payloads in advanced materialized containers remain rejected during `zt check`. Expansion must continue under monomorphization and runtime ABI rules.

### FFI

- **Captured closure callbacks:** current supported callback path is narrow and top-level. Captured closure callbacks remain rejected unless a future explicit ABI supports them.
- **Managed values across FFI:** only explicitly supported ABI shapes may cross `extern c`. Further managed-value interop must be added case-by-case.

### Compiler and backend infrastructure

- **Backend conformance runner automation:** conformance contract exists; automation must be built before activating alternative backends.
- **Debug info beyond `#line`:** source mapping contract exists; DWARF/PDB-level debug info remains future implementation.
- **Incremental compilation:** accepted direction, not current closure work.
- **Optimization passes:** boundaries are defined. Concrete semantic ZIR passes and backend-specific optimization quality remain future implementation after conformance gates.
- **Cross-compilation:** target triple and sysroot model remain future tooling/backend work.
- **Separate compilation units:** split C output / parallel compilation remains exploration-level compiler work.

## 2. Aberto para discussão

These items still need dedicated design sessions or explicit decisions before implementation.

### IO/dataflow

- **Stream design:** decided. `Stream<T>` is push-based. The canonical consumer abstraction is `Sink<T>`, with explicit item, error, and completion behavior. Channel bridge helpers are supported as adapters, not as the core stream abstraction. Async delivery is built by running stream producers in jobs and delivering items through channels.
- **Async IO API:** decided. Async IO is library-level, not syntax. The canonical route is `Stream<T>` async helpers backed by jobs and channels. Helpers return explicit handles for cancellation, completion, and error delivery. There is no hidden scheduler and no `async/await`.
- **WebSocket scope:** decided. Final WebSocket scope includes both client and server APIs. Implementation is phased: first blocking client plus `Stream<T>`/`Sink<T>` async adapters, then server listen/accept and per-connection job helpers. Message APIs support text, binary, close, ping, and pong with result-based errors. `wss://` depends on the TLS decision.
- **TLS ownership:** decided. TLS uses a hybrid provider model. `std.net.tls` is the canonical stdlib interface, while the provider may be runtime-native, OS/host-backed, or an official package/provider selected per platform. `https://` and `wss://` depend on this interface, not on ad-hoc package APIs. Certificate validation, error mapping, cleanup, and provider capability diagnostics are part of the TLS contract.

### Versioning and migration

- **Edition/deprecation model:** decided. Zenith uses semver plus `zt migrate` as the normal compatibility model. `1.x` releases do not break valid v1 code. Breaking changes require a deprecation cycle, stable diagnostics, and migration support where mechanically possible before removal in a major release. Editions are reserved as an edition-lite escape hatch for rare broad syntax or semantic shifts that cannot be handled cleanly by semver plus migration alone.
- **`zt migrate` behavior:** decided. `zt migrate` handles mechanically safe syntax migrations plus safe API/import/stdlib renames. It should provide dry-run output and patch-style review before writing changes. It does not perform speculative semantic rewrites, control-flow restructuring, or migrations that require human intent. Deprecation diagnostics should point to the relevant migration rule when one exists.
- **Deprecation policy enforcement:** decided. Deprecated syntax, aliases, APIs, or diagnostics may warn in minor releases, but removal happens only in a major release. Deprecation diagnostics must include replacement guidance and mention `zt migrate` when a migration rule exists. Compatibility aliases such as deprecated `dyn` may remain accepted for the whole major line, warning until removal in the next major.

### Stdlib and ecosystem boundary

- **Stdlib boundary:** decided. Zenith uses a foundation stdlib plus official packages. The stdlib contains language foundations, primitive data structures, core IO, process/fs/os/time, protocol foundations such as TCP/HTTP/WebSocket/TLS interfaces, diagnostics/testing basics, and runtime-backed APIs required for portability. Higher-level domain libraries, frameworks, integrations, and opinionated workflows belong in packages. Important domains may be maintained as official packages without being merged into the core stdlib.
- **Package graduation policy:** decided. Package maturity uses tiers: community package, official package, platform/foundation package, and only rarely core stdlib. A package may become official after proving maintenance, tests, docs, API stability, security posture, and real usage. A package may move toward platform/foundation status when many projects depend on it or when it bridges runtime/platform behavior. Core stdlib inclusion requires that the API is foundational, portable, low-level enough, and unsuitable to remain versioned independently.
- **Optional dependencies and feature flags:** decided. Packages may declare explicit features and optional dependencies, and projects enable them intentionally in `zenith.ztproj`. Target/platform conditions may select dependencies or providers for cases such as TLS, graphics, OS APIs, or native integrations. There are no hidden default feature surprises for security-sensitive capabilities; defaults must be documented and minimal.
- **`std.crypto`:** decided. A narrow crypto foundation may live in stdlib for common hashes, HMAC, secure random bytes, and primitives needed by TLS/package security. Broader protocols, password hashing policy, key management, JWT, certificates tooling, and high-level cryptography belong in official packages.
- **`std.image`:** decided. Image manipulation belongs in official packages, not core stdlib. The stdlib may expose only generic bytes/path/runtime primitives needed by image packages.
- **`std.db`:** decided. Database abstractions belong in packages, preferably official packages for common drivers or query layers. Core stdlib should not define a universal database abstraction.
- **`std.regex` expansion:** decided. Regex remains a stdlib core utility, but with a deliberately bounded feature set. The stdlib may expand practical matching/search/replace helpers, while full PCRE-like engines, heavy backtracking controls, and domain-specific regex engines can live in packages.

### Runtime and FFI design questions

- **`std.mem` ownership and advanced allocation:** decided. The 0.4.2-beta.rc1 executable subset has closed `std.mem.own`, `std.mem.view`, and `std.mem.edit` for the supported safe shapes: primitive scalars, `text`, safe tuples/structs, primitive/text lists, `list<safe tuple>`, `list<safe struct>`, `set<int>`, `set<text>`, and maps with `int` or `text` keys plus scalar/text values. Unsupported nested mutable managed shapes continue to fail at check time with capability diagnostics. Advanced allocation for engines and high-performance packages such as Borealis remains accepted as opt-in library-level API, not language syntax. Advanced memory resources use intent-oriented names: `mem.Temp` is the canonical temporary region resource for arena/bump/scratch-style allocation and reset-at-once workflows; `mem.Pool<T>` is reserved for reusable fixed-shape object/block pools. They are explicit library values passed to APIs and must not introduce ownership keywords, borrow checking, or lifetimes into Zenith core.
- **Hot-reload runtime:** decided. Hot reload is accepted as experimental opt-in tooling, not a v1 language/runtime contract. `zt run --dev` and Borealis-style workflows may support reload/restart/reinjection strategies where practical, but stable runtime ABI, state migration, and DLL reinjection are not guaranteed by the language core. Production semantics remain normal build/run behavior.
- **Struct runtime generic field access:** decided. Zenith rejects universal runtime reflection and generic field access for every struct. Reflection is opt-in through `Reflect`, a builtin derivable trait. The normal form is `@derive(Reflect)`, which asks the compiler/tooling to generate trusted structural metadata for the selected type. Manual `apply reflect.Reflect to Type` may be allowed for opaque/runtime/FFI wrappers, but a type cannot use both derived and manual reflection. Metadata is accessed through explicit `std.reflect` functions such as `metadata<T>()`, `metadata_of(value)`, `type_name<T>()`, and `fields<T>()`, not through magic pseudo-fields like `value.type_name` or `value.fields`. `Readable` remains separate: metadata types such as `TypeInfo` and `FieldInfo` may implement `Readable`, so printing metadata is a normal readable conversion. Reflection is metadata-first; dynamic field reads/writes are not part of the initial core contract. Reflection must distinguish public and private fields. `@derive(Reflect)` defaults to `@derive(Reflect, expose: public)`. Explicit exposure policies are `expose: public`, `expose: reflect`, `expose: public + reflect`, and `expose: all`. Field-local `@reflect` marks private or selected fields for exposure when the policy includes `reflect`, and `@reflect(hidden)` excludes a field even under broader policies such as `all`.
- **User-defined struct as `extern c` argument:** decided. User-defined structs may cross `extern c` only when explicitly marked with a C representation attribute such as `repr("c")` or equivalent ABI annotation. All fields must be FFI-safe, layout-stable types. Unannotated Zenith structs remain language-layout values and are rejected at FFI boundaries. Opaque handles remain preferred for ownership-bearing or platform-defined resources.
- **`extern` variables:** decided. Read-only C globals may be imported as explicit `extern const` bindings when their type is FFI-safe. Mutable C globals require an unsafe gate such as an unsafe annotation or `std.unsafe` API, because reads and writes may violate thread-safety, initialization, or ownership assumptions. Managed Zenith values cannot be bound directly to mutable C globals.
- **Variadic `extern` functions:** decided. The canonical FFI path is typed wrappers with fixed signatures. Raw C varargs may exist only behind an unsafe gate and cannot accept managed Zenith values directly. Public Zenith APIs should expose typed wrapper functions, not user-facing `...` calls. General Zenith variadic parameters remain rejected.
- **Conditional `extern` per target:** decided. Conditional externs use declarative target attributes or cfg-style blocks for OS, arch, ABI, library name, and symbol availability. Package/project configuration may select platform providers or dependencies per target. Core Zenith should avoid arbitrary build-script execution for FFI resolution; target selection must remain inspectable and deterministic.

### Traits and `any`

- **Generic traits:** decided. Zenith prefers generic traits with semantic parameter names over associated types for the first advanced trait model. Traits such as `Stream<Item>`, `Sink<Item>`, `Parser<Output>`, and `Iterator<Item>` express the important associated concept directly as a generic parameter, keeping the surface readable while avoiding projected types such as `Self.Item` and `T.Item`. Type aliases may name common specializations such as `ByteStream = Stream<bytes>`. The initial contract targets static dispatch and monomorphized generic functions.
- **Associated types:** decided as deferred. Full associated types remain future work unless generic trait parameters prove insufficient. Zenith should not add `type Item` inside traits, `type Item = T` inside `apply`, or projected type syntax such as `Self.Item`/`T.Item` in the first generic-trait contract.
- **Explicit apply precedence:** decided. Overlapping applies remain rejected. A type may have only one implementation of a given trait instance, including generic trait specializations. Zenith does not add explicit precedence or priority syntax for applies. This keeps method lookup deterministic and diagnostics early. Shared behavior should use trait default methods, helper functions, or explicit wrapper types rather than blanket-overlap resolution.
- **Self-referential traits:** decided. Traits may reference the implementing type as `Self` in method signatures for static dispatch and monomorphized generic code. Dynamic dispatch through `any` is restricted: traits whose public methods expose `Self` in parameters or returns are not object-safe by default. Future `any` support may allow specific ABI-safe `Self` patterns only when the concrete type is not required by callers or when the operation is compiler/runtime-defined.
- **Advanced `any` shapes:** decided. `any<Trait>` is a strict object-safe trait object with a stable Zenith runtime ABI, not a universal dynamic object. Object-safe traits may be used as `any` when their public methods have no method-level generics, no unresolved type parameters, no unsupported `Self` exposure, and only ABI-supported parameters/returns. Managed parameters and returns are part of the final design: returned managed values are owned or retained for the caller, and managed parameters are borrowed for the duration of the call unless ownership is made explicit by the API. Generic traits may be used as `any<Stream<bytes>>` or similar only when all trait parameters are concrete or resolved by an enclosing generic context. Mutating trait methods are allowed only through explicit mutability and mutable `any` bindings. `any` does not cross `extern c`; FFI uses opaque handles or typed callbacks. Cross-thread use requires explicit `Transferable`/sendable capability for both the concrete type and the object-safe surface. Generic trait methods remain rejected for `any`.

### Tooling extensibility

- **Plugin/extension system:** decided. Zenith tooling uses external declarative tools rather than in-process compiler plugins. Build/codegen/lint hooks may be declared in project/package configuration and invoked as separate commands with stable inputs/outputs such as JSON, manifests, generated source directories, and diagnostics. The compiler core does not load arbitrary plugin code. Formatter hooks are avoided so canonical formatting remains stable. Extension points should be inspectable, deterministic, sandbox-friendly, and suitable for package registry security review.
- **`zt bench`:** decided. Benchmarking belongs in the core CLI as a minimal stable runner. `zt bench` should support simple benchmark discovery, warmup, iterations, timing summaries, exit status, and machine-readable JSON output. Advanced statistics, charts, historical baseline management, dashboards, and domain-specific profiling belong in packages or external tools.
- **Editor config policy:** decided. Zenith uses an LSP-first editor policy. The official stability contract is the language server, diagnostics protocol, formatter behavior, and semantic tokens/completions where supported. Editor-specific integrations such as VSCode, Helix, Neovim, Zed, and future IDE configs may be maintained as best-effort adapters over the same LSP/formatter contracts. No editor-specific behavior should define language semantics.
- **Borealis Studio integration:** decided. Borealis Studio remains external tooling. It may consume stable Zenith protocols such as `zt`, LSP, formatter output, benchmark JSON, reflection metadata, and package manifests, but it does not define language semantics, editor policy, or core tooling behavior. Zenith remains LSP-first rather than tied to a single official IDE path.

## 3. Futuro grande

These are large post-closure initiatives. They should start only after conformance, runtime ABI, and tooling prerequisites are ready.

### Alternative backends

- **Zig backend:** textual systems backend exploration. Must conform to ZIR/backend contracts and must not reshape Zenith semantics.
- **LLVM backend:** strategic optimized native backend. Requires backend conformance suite, runtime ABI clarity, and source/debug mapping plan.
- **WASM backend:** sandbox/web target for playground and deployment. Requires runtime portability audit and either LLVM or direct ZIR-to-WASM route.
- **Cranelift backend:** fast native backend spike for dev-mode validation after conformance infrastructure exists.
- **C3 backend:** lower-priority textual C-like backend experiment after C oracle and toolchain maturity are stable.

### Tooling maturity

- **Production LSP:** move current LSP from partial/beta toward stable diagnostics, go-to-definition, completions, formatting integration, and project model support.
- **VSCode Marketplace extension:** package and publish official extension after LSP/tooling maturity.
- **Web playground:** browser Zenith REPL or playground, likely after WASM/backend route stabilizes.
- **Formatter/migrator polish:** stable formatting policy and `zt migrate` integration for future syntax/deprecation changes.
- **Diagnostics UX maturity:** apply ACTION/WHY/NEXT consistently across all stable diagnostics and invalid fixtures.

### Ecosystem and packaging

- **ZPM registry web:** late-stage ecosystem item after language/runtime/package model stabilizes.
- **Package registry policy:** ownership, naming, publishing, verification, yanking, and security policy.
- **Optional dependencies:** package feature selection, platform conditions, and lockfile impact.
- **Distribution/installers:** mature cross-platform release, installer, and update channels.
- **Package quality gates:** docs, tests, semantic versioning, deprecation, and compatibility requirements.

### IO/network platform

- **Streams/dataflow foundation:** public generic streams, adapters, lazy integration, and IO backpressure model.
- **Network stack expansion:** TLS, UDP, server APIs, WebSocket, certificate handling, and cross-platform socket behavior.
- **Async-via-jobs ecosystem:** reusable patterns and stdlib helpers for background blocking IO without `async/await`.

### Runtime evolution

- **Full typed concurrency runtime:** generated or selected runtime storage for non-`int` jobs/channels/shared/atomic payloads.
- **ORC/cycle collection expansion:** cycle detector becomes meaningful when future APIs expose cycle-forming managed references.
- **Allocator strategy:** optional allocator APIs if real-world pressure justifies them.
- **Hot reload/dev runtime:** possible long-term developer-experience feature.

## Explicit Non-goals

The following are rejected and should not be reopened without a new high-bar decision:

- `char` type
- `?.` safe navigation
- `??` null coalescing
- implicit return
- `try/catch`
- `async/await` keywords
- `owned<T>` / `borrow<T>` / language lifetimes
- macros
- broad method/function overloading
- standalone `uint`
- rest operator `...`
- postfix guard `return x if cond`
- `unpack` destructuring syntax
- full local type inference (`const x = 42`)
- bare struct literal shorthand (`{ fields }`)
- variadic parameters
- selective imports
- wildcard imports
- `unless`
- math operators such as `**` and `//`
- C-style `for` loops
- named tuple fields
- JavaScript backend

## Relationship To Closure Gate

This document does not reopen Wave 7. The closure gate remains valid:

- no core language/ZIR/compiler/runtime topic should proceed without an explicit decision artifact;
- future implementation work should conform to the Wave 7 closure contracts;
- ecosystem, tooling, and backend work starts after conformance and runtime ABI prerequisites are ready.
