# Documentation Canonical Policy

> Audience: maintainer, docs writer
> Surface: release engineering
> Status: Phase 7 policy

The final language contract is canonical.

Future public docs and translations are best-effort teaching layers. They should
help readers, but they must not define behavior that conflicts with
`docs/spec/language/final-language-contract.md` or topic-specific specs.

## Source Order

When two docs disagree, use this order:

1. `docs/spec/language/final-language-contract.md` for final/current/future status.
2. Topic-specific `docs/spec/language/` files for detailed compiler/runtime behavior.
3. `docs/reference/` for generated or knowledge-base reference material.
4. `docs/public/` for current public learning docs after rewrite.
5. Future translated public docs, only after they are rebuilt from the current
   contract.
6. Internal planning docs.

Historical decisions remain useful context. They do not override current
canonical specs.

## Translation Rule

Translated docs may lag behind the current public docs after the public rewrite.

When updating translations:

- keep examples close to the English version;
- avoid adding new semantics only in a translation;
- link back to the final language contract when a page is partial;
- prefer short sections and clear examples.
- mark the translation as post-RC or best-effort if it was not rebuilt for the
  current release.

## Release Check

Before a release candidate:

- new public behavior must exist in the final language contract or a topic-specific spec first;
- translated pages must not claim stronger guarantees than the final language contract;
- known translation gaps can ship if they are marked as best-effort.
