# Decision 077: Language Coherence Closure

- Status: accepted
- Date: 2026-04-18
- Type: language / implementation / quality
- Scope: final coherence tracks required before Zenith can be considered stable

## Summary

Zenith's central syntax and philosophy are accepted, but the project must close the gap between readable design and production-quality implementation.

The remaining work is organized into closure tracks for specs, formatter, diagnostics, runtime, value semantics, runtime contracts, enums, bytes/UTF-8, safe collections, CLI, ZDoc and conformance.

## Decision

The project accepts `LANGUAGE_COHERENCE_CLOSURE.md` as the active closure plan.

The next roadmap milestones must explicitly cover:

- final specs without ambiguity
- mandatory formatter
- diagnostic renderer
- C runtime hardening
- value semantics conformance
- runtime `where` contracts
- payload enums and strong match
- bytes, UTF-8 and stdlib base
- safe collection APIs
- final CLI
- functional ZDoc
- final conformance matrix

## Normative Clarifications

### `void`

`void` remains part of the type system, but ordinary no-return functions omit the return arrow.

Canonical:

```zt
func log(message: text)
    io.write(message)?
end
```

Canonical for empty success:

```zt
func save(path: text) -> result<void, fs.Error>
    fs.write_text(path, "ok")?
    return success()
end
```

Non-canonical user source:

```zt
func log(message: text) -> void
    ...
end
```

The compiler should eventually diagnose explicit `-> void` in user-authored source and suggest removing it.

`void` is not a normal data value. It must not be used for locals, fields, parameters, lists, maps or optionals.

### `where`

`where` keeps two roles separated by syntax position:

- value-level `where` on fields and parameters is a runtime contract
- generic `where` after a declaration header is a compile-time type constraint

No return-value `where` and no local-variable `where` are part of the MVP.

### `const` collections

`const` collections are observably immutable.

This is invalid:

```zt
const players: list<Player> = [Player(name: "Julia", hp: 100)]
players[0].hp = 80
```

Mutation requires a `var` binding and, for deep indexed updates in the MVP, explicit read-modify-write.

Future `update ... as ... end` sugar remains allowed if it preserves value semantics.

## Rationale

These clarifications keep the language aligned with the manifesto:

- `void` exists only where it reduces ceremony without becoming everyday syntax noise
- `where` remains one keyword, but diagnostics and formatting must make its role clear
- `const` stays strong enough that readers do not need to reason about shallow versus deep mutation

The closure plan prevents new features from being called complete before they support diagnostics, formatting, runtime semantics and conformance coverage.

## Consequences

The roadmap and checklist must grow beyond "feature parses and emits C".

A feature is closed only when it has:

- spec coverage
- implementation
- diagnostics
- formatter behavior when syntax is involved
- runtime behavior when values are involved
- conformance tests

