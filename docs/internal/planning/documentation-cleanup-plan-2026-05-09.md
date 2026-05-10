# Documentation Cleanup Migration Plan - 2026-05-09

> Audience: maintainer, contributor
> Status: completed plan
> Surface: internal
> Goal: reorganize the language documentation into one clear structure without losing current implementation truth.

This plan must be executed in order. Each stage has a gate. Do not advance to
the next stage until every checkbox in the current stage is complete.

The writing standard for the final public docs is:

- short sections;
- direct examples;
- small steps;
- current facts only;
- clear separation between public guidance, reference, specs, internal reports,
  archive material, and generated output.

## Target Structure

The desired documentation layout is:

```text
README.md
CHANGELOG.md
CONTRIBUTING.md
SECURITY.md
TRADEMARK_POLICY.md
docs/
  public/
    get-started/
    learn/
    language/
    stdlib/
    packages/
    licensing/
  reference/
    cli/
    stdlib/
    diagnostics/
    grammar/
  internal/
    architecture/
    decisions/
    planning/
    reports/
    release/
    standards/
    archive/
  spec/
    language/
```

`docs/spec/language/` is the canonical language-contract source after the spec
migration. The old top-level `language/`, duplicate docs asset folders, and
empty generated-doc placeholders were removed after the follow-up cleanup.

## Non-Negotiable Rules

- [x] Do not delete any documentation before it appears in the inventory.
- [x] Do not move stale material into public docs.
- [x] Do not keep historical reports beside current reports without a clear
      historical label.
- [x] Do not let README-only public folders count as complete documentation.
- [x] Do not treat generated logs, benchmark output, or temporary run output as
      authored documentation.
- [x] Do not move `.github/`; it is project infrastructure, not documentation
      clutter.
- [x] Keep public wording accessible for readers with TDAH and dyslexia.

Gate to start Stage 1:

- [x] Maintainer accepts these rules as the cleanup contract.

## Stage 1 - Full Documentation Inventory

Purpose: know what exists before moving or deleting anything.

- [x] List all Markdown, text, HTML, and generated doc-like files in the repo.
- [x] Mark root-level files as one of:
  - public root;
  - internal root candidate;
  - generated output;
  - delete candidate;
  - needs review.
- [x] Mark each file under `docs/` as one of:
  - public;
  - reference;
  - internal;
  - report;
  - archive;
  - generated;
  - duplicate;
  - stale.
- [x] Mark each file under `language/` as one of:
  - canonical spec;
  - decision record;
  - implementation status;
  - historical evidence;
  - stale.
- [x] Identify files with misleading status labels such as "current", "active",
      or "final" when they are no longer current.
- [x] Identify public folders that contain only `README.md` and need real pages.
- [x] Write the inventory to a new report named
      documentation-inventory-2026-05-09.md under `docs/internal/reports/`.

Gate to Stage 2:

- [x] Inventory exists.
- [x] Every doc-like file has one classification.
- [x] Delete candidates are listed but not deleted yet.

## Stage 2 - Define Sources Of Truth

Purpose: prevent conflicting docs after the migration.

- [x] Confirm root-level files that may stay in the repo root.
- [x] Confirm `docs/public/` as user-facing documentation.
- [x] Confirm `docs/reference/` as stable lookup documentation.
- [x] Confirm `docs/internal/` as planning, reports, standards, release notes,
      and maintainer-only documentation.
- [x] Confirm the generated docs area as generated documentation output only.
- [x] Confirm `docs/spec/language/` as the normative language specification source.
- [x] Confirm `docs/internal/decisions/language/` as the decision-record source.
- [x] Define where release reports live after RC and stable releases.
- [x] Define where old reports go when they are kept for history.
- [x] Add this truth map to the inventory report.

Gate to Stage 3:

- [x] No active topic has two competing source-of-truth locations.
- [x] Historical material has an archive destination or delete decision.

## Stage 3 - Create Destination Skeletons And Indexes

Purpose: create the structure before moving content.

- [x] Ensure `docs/public/README.md` explains who the public docs are for.
- [x] Ensure `docs/reference/README.md` explains lookup/reference scope.
- [x] Ensure `docs/internal/README.md` explains maintainer-only scope.
- [x] Ensure `docs/internal/archive/README.md` explains archival rules.
- [x] Ensure the generated docs README explains generated-output rules after the
      folder exists.
- [x] Add index pages for public sections:
  - [x] public get-started README.
  - [x] public learn README.
  - [x] public language README.
  - [x] public stdlib README.
  - [x] public packages README.
  - [x] public licensing README.
- [x] Add index pages for reference sections:
  - [x] `docs/reference/cli/README.md`
  - [x] `docs/reference/stdlib/README.md`
  - [x] reference diagnostics README.
  - [x] reference grammar README.

Gate to Stage 4:

- [x] Every destination folder has a short scope note.
- [x] No migrated content has been moved yet.

## Stage 4 - Migrate Public Documentation

Purpose: make the public docs useful and aligned with current implementation.

- [x] Move or rewrite getting-started content into the public get-started
      section.
- [x] Move or rewrite tutorial content into the public learn section.
- [x] Move or rewrite language overview content into the public language
      section.
- [x] Move or rewrite stdlib overview content into the public stdlib section.
- [x] Move or rewrite package/project content into the public packages section.
- [x] Move or rewrite license/trademark guidance into the public licensing
      section.
- [x] Replace README-only public sections with concrete pages and examples.
- [x] Remove or rewrite public claims that still describe pre-RC gaps as current
      blockers.
- [x] Verify all public examples against the current compiler or mark them as
      illustrative.
- [x] Keep post-RC technical debt visible only where it helps users understand
      limits, not as stale release blockers.

Gate to Stage 5:

- [x] Public docs can be read from start to finish without jumping to internal
      reports.
- [x] Public docs describe the current compiler, not older audit findings.
- [x] Public docs use short, accessible explanations.

## Stage 5 - Migrate Reference Documentation

Purpose: give advanced users stable lookup pages without mixing them with
planning reports.

- [x] Move CLI command reference into `docs/reference/cli/`.
- [x] Move stdlib reference into `docs/reference/stdlib/`.
- [x] Move diagnostic reference into the reference diagnostics section.
- [x] Move grammar/reference tables into the reference grammar section.
- [x] Cross-check stdlib reference against implemented modules.
- [x] Label post-RC collection gaps explicitly:
  - [x] `grid2d<T>` remains post-RC technical debt.
  - [x] `pqueue<T>` remains post-RC technical debt.
  - [x] `circbuf<T>` remains post-RC technical debt.
  - [x] `btreemap<K,V>` remains post-RC technical debt.
- [x] Ensure concurrency docs distinguish typed facade from narrower runtime
      behavior.
- [x] Ensure FFI docs state the current narrow scope.

Gate to Stage 6:

- [x] Reference pages do not contradict `docs/spec/language/`.
- [x] Reference pages do not hide known post-RC limits.
- [x] Each reference page has an owner source or validation command.

## Stage 6 - Consolidate Language Specs And Decisions

Purpose: protect the language contract while the docs move around it.

- [x] Keep `docs/spec/language/README.md` as the entry point for normative language
      truth.
- [x] Review `docs/spec/language/language-reference.md` for stale RC language.
- [x] Review `docs/spec/language/surface-implementation-status.md` for stale blockers.
- [x] Review `docs/spec/language/post-v1-remaining-language-work.md` for current
      post-RC debt.
- [x] Review `docs/spec/language/post-v1-implementation-plan.md` and close or archive
      completed RC material.
- [x] Add cross-links from public/reference docs to `docs/spec/language/` only where
      useful.
- [x] Do not duplicate full spec text into `docs/public/` or `docs/reference/`.

Gate to Stage 7:

- [x] There is one current normative spec path.
- [x] Completed RC implementation notes are no longer presented as open work.
- [x] Public docs can link to specs without depending on internal plans.

## Stage 7 - Consolidate Internal Reports And Planning

Purpose: stop old reports from looking like current release blockers.

- [x] Keep one current release-readiness report.
- [x] Keep one current technical-debt backlog.
- [x] Move historical audit reports into `docs/internal/archive/` or delete them
      if they no longer provide value.
- [x] Rename archived reports with explicit historical status when retained.
- [x] Update `docs/internal/reports/README.md` to explain current vs archived
      reports.
- [x] Update `docs/internal/planning/README.md` to point to active plans only.
- [x] Remove stale language-readiness planning entries from active indexes.
- [x] Keep RC/stable release evidence in `docs/internal/release/` or a clearly
      named release-report folder.

Gate to Stage 8:

- [x] A maintainer can find the current release status in one place.
- [x] Old reports cannot be mistaken for active blockers.
- [x] Planning indexes list only active plans or clearly historical material.

## Stage 8 - Clean Root-Level Documentation

Purpose: keep the repository root small and intentional.

- [x] Keep only approved root public files.
- [x] Move internal root-level docs into `docs/internal/`.
- [x] Move generated root output into the generated docs area only if it has
      value.
- [x] Delete generated root output that has no durable value.
- [x] Review likely generated/delete candidates:
  - [x] `emit_debug.txt`
  - [x] `emit_stdout.txt`
  - [x] `out.txt`
  - [x] `test_output.txt`
  - [x] `tmp_*.txt`
  - [x] `warnings_full.txt`
- [x] Review root workflow docs and either keep, move, or delete:
  - [x] `CAVEMAN_WORKFLOW.md`
  - [x] `HEAVY_TESTS_BRIEFING.md`
- [x] Update `.gitignore` if cleanup reveals new generated-output patterns.

Gate to Stage 9:

- [x] Root docs are intentional.
- [x] Temporary or generated files are gone or explicitly contained.
- [x] No public entry point link is broken.

## Stage 9 - Remove Duplicates And Stale Material

Purpose: delete only after migration and validation evidence exists.

- [x] Compare moved/reworked docs against inventory source paths.
- [x] Delete duplicate files whose current content has a new canonical home.
- [x] Delete stale reports that conflict with current release status.
- [x] Delete README-only placeholders replaced by real pages.
- [x] Archive historical files only when they explain a decision or audit trail.
- [x] Keep Git history as the source for low-value old planning details.

Gate to Stage 10:

- [x] Every deletion is traceable to an inventory entry.
- [x] No deleted file is the only source of current implementation truth.
- [x] `git status --short` has only intentional changes.

## Stage 10 - Update Cross-Links And Tooling

Purpose: make the new structure navigable and enforceable.

- [x] Update root `README.md` links to the new docs structure.
- [x] Update `CONTRIBUTING.md` links if contributor docs move.
- [x] Update package docs links if package references move.
- [x] Update website/docs build inputs if they point to old paths.
- [x] Update docs path checks if new folders need allow-list entries.
- [x] Add or update a documentation map in `docs/README.md` if useful.
- [x] Search for stale old paths after moves:

```powershell
rg "old/path/or/file-name"
```

Gate to Stage 11:

- [x] No active docs link to deleted paths.
- [x] Tooling knows the new documentation layout.
- [x] Navigation from root README to public docs works.

## Stage 11 - Validate The Migration

Purpose: prove the cleanup did not break documentation or release signals.

- [x] Run the docs path checker.
- [x] Run the public examples or examples smoke suite if public examples changed.
- [x] Run the main quick compiler validation if examples/spec snippets changed.
- [x] Run markdown formatting or lint checks if available.
- [x] Run `git diff --check`.
- [x] Confirm no ignored local-agent/cache folders are staged.
- [x] Confirm `.github/` was not removed or hidden from version control.
- [x] Write validation evidence to the inventory or final cleanup report.

Suggested commands:

```powershell
python tools/check_docs_paths.py
python tools/check_examples.py
git diff --check
git status --short
```

Gate to Stage 12:

- [x] Validation commands pass or have documented, accepted exceptions.
- [x] Final report lists moved, deleted, archived, and rewritten docs.

## Stage 12 - Close The Migration

Purpose: finish with one clear state and no ambiguous leftovers.

- [x] Create a final report named documentation-cleanup-final-2026-05-09.md
      under `docs/internal/reports/`.
- [x] Summarize final structure.
- [x] List files moved.
- [x] List files deleted.
- [x] List files archived.
- [x] List docs intentionally left for a later pass.
- [x] Update this plan checklist with completed boxes.
- [x] Commit the cleanup in one or more reviewable commits.
- [x] Push after validation passes.

Done criteria:

- [x] Public docs are current and useful.
- [x] Reference docs are separated from tutorials.
- [x] Normative specs remain in one canonical place.
- [x] Internal reports cannot be mistaken for public documentation.
- [x] Root docs are minimal.
- [x] Generated output is removed or contained.
- [x] Validation evidence is recorded.
