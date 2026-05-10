# Deprecation Policy

> Audience: maintainer
> Surface: release engineering
> Status: Phase 7 policy

Deprecation is the default path for old syntax and old public APIs.

The rule is:

> Keep the old form accepted for at least one release and emit a warning.

## Warning Shape

A deprecation warning must be action-first.

It should include:

- what is deprecated;
- the canonical replacement;
- when removal can happen;
- a stable diagnostic code when the compiler has one.

Good wording:

```text
warning[syntax.deprecated]: `case default` is deprecated. Use `case else:`.
```

Avoid wording that only says "invalid soon" without a replacement.

## Removal Rule

Removal is allowed after one release that shipped the warning.

Before removal:

- the replacement must be documented;
- at least one fixture must cover the replacement;
- release notes must mention the removal;
- migration notes must stay easy to scan.

## Exceptions

Skip the one-release warning only when keeping the old form would create:

- data loss;
- a security risk;
- a P0/P1 compiler or runtime defect;
- a contradiction with the canonical spec that blocks other work.

The exception must be documented in the release note.

