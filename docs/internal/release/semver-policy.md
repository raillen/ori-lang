# SemVer Policy

> Audience: maintainer
> Surface: release engineering
> Status: Phase 7 policy

Zenith uses SemVer wording for public releases.

During the 0.x alpha line, the language can still make breaking changes. Those
changes must be explicit, documented and tested.

## Public Surface

Treat these as public surface:

- source syntax accepted by `zt`;
- language semantics in `docs/spec/language/`;
- `zenith.ztproj` project fields;
- `zt` command names and stable flags;
- public stdlib modules under `stdlib/std`;
- package manifests and lockfiles;
- public package APIs, including `packages/borealis`.

Internal compiler structs are not public surface unless a document says they are
part of an integration contract.

## Version Rules

For `0.x` alpha releases:

- `PATCH`: fixes only. No intentional breaking changes.
- `MINOR`: new features, warnings, deprecations and documented breaking changes.
- `MAJOR`: reserved for the future `1.0` stability line.

For `1.0+` releases:

- `MAJOR`: breaking changes.
- `MINOR`: additive compatible changes.
- `PATCH`: compatible fixes.

## Breaking Changes

A change is breaking when existing valid user code can fail, change behavior or
need a different command without an explicit opt-in.

Common examples:

- removing syntax;
- changing type checking rules;
- renaming public stdlib functions;
- changing `zt` flag behavior;
- changing package manifest fields;
- changing generated lockfile meaning.

## Required Evidence

Every breaking change must have:

- a short migration note;
- a test or fixture that covers the old and new behavior;
- a release note entry;
- a deprecation warning for one release, unless the exception policy applies.

Exceptions are allowed only for security fixes, data loss fixes or P0/P1 defects.
The release note must name the exception.

