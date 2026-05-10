# Release Engineering

> Audience: maintainer
> Surface: internal release process

This folder keeps the release rules that must stay stable during the 0.9 line.

Read in this order:

1. `semver-policy.md`
2. `deprecation-policy.md`
3. `docs-canonical-policy.md`
4. `borealis-clean-clone-dogfood.md`
5. `release-candidate-freeze.md`

The goal is simple:

- make breaking changes visible before they ship;
- keep old syntax usable for one release when possible;
- make English public docs the canonical docs;
- prove Borealis works from repo-root paths;
- freeze release candidates with clear gates.

