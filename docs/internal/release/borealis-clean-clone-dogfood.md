# Borealis Clean Clone Dogfood

> Audience: maintainer
> Surface: release engineering
> Status: Phase 7 policy and local evidence

Borealis is the main package dogfood for the 0.9 release line.

The clean-clone goal is:

- a fresh checkout can validate `packages/borealis`;
- the package does not rely on local generated files;
- beginner-facing examples keep compiling through the public package surface.

## Local Surrogate

This repository run cannot create a separate network clone. The local surrogate
therefore uses only repo-root paths and deletes no user files.

The Phase 7 driver checks:

- `zt check packages/borealis/zenith.ztproj --all`;
- `zt run tests/behavior/borealis_foundations_stub/zenith.ztproj`;
- `zt run tests/behavior/borealis_scene_entities_stub/zenith.ztproj`;
- required Borealis package files and manifests are present.

This is enough to catch accidental dependency on one developer's current shell,
generated build folders or unstated package paths.

## External Release Gate

Before tagging a public release candidate, run the same commands in clean clones
on:

- Windows x64;
- Linux x64;
- macOS arm64.

If one platform fails because a native optional backend is missing, record that as
a backend limitation. Core Borealis package checks must still pass.

