---
title: Cline prepared-headless god-file papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-134457-papercuts-cline-prepared-headless-god-file.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, rust]
---

## Objective

Close the Cline Plan god-file papercut by splitting the prepared-headless test
target into focused modules without changing behavior, assertions, target
identity, public API, or test coverage.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `28543770ea61865ba7da3b1af8dc003b35ba59bb`
- **Worker branch:** `worker/papercuts-cline-prepared-headless-god-file`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a source-preserving test-module split,
  not frontier implementation work.
- **Authority:** `AGENTS.md`, the exact PAPERCUTS entry, the repository Rust
  quality profile, current test target, current Effigy god-file result, and
  this handoff.
- **Reproduction:** `tests/prepared_headless_facade.rs` is 395 code lines and
  is one warning in the current 385-finding scan: 7 critical, 42 high, 336
  warning.
- **In scope:** the `prepared_headless_facade` test root and new private test
  modules beneath it; `PAPERCUTS.md` closeout evidence.
- **Out of scope:** production source, Cargo target rename, manifest change,
  public API, fixtures, test semantics, other Cline god-files, other
  papercuts, roadmap/research/log/index edits, or baseline-wide cleanup.
- **Parallel partition:** the Codex currentness worker owns Codex surfaces,
  research/log indexes, and route/version closeout. This lane owns only the
  paths above.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Keep `prepared_headless_facade` as the Cargo test-target name. Split default
mode, Plan mode, and rejection/binding proofs into focused private modules,
with shared fixture builders in the smallest sensible support surface. Prefer
moving existing proof bodies intact over rewriting them.

All four current tests must remain discoverable under their existing names and
must still pass. Argument order, capability assertions, access drift, package
drift, working-resource failure, cleanup, redaction, and Plan-mode policy
agreement must remain load-bearing. No moved proof may be weakened, merged
away, ignored, or replaced by a broad smoke assertion.

The original 395-code-line finding must disappear. No new file may enter the
configured god-file findings, and the total must improve from 385 to 384 with
severity counts 7 critical, 42 high, 335 warning. Stop if a behavior or API
change is needed, another mutable surface is required, or the exact baseline
cannot be reconciled.

Falsify the split by checking the target's test listing and exact count, then
run the target. Compare moved proof bodies against the base so review can
distinguish relocation from semantic edits. Record any unavoidable semantic
edit explicitly; do not hide it as mechanical movement.

## Validation And Completion

Confirm a clean non-`main` worktree, exact branch, `HEAD == origin/main`, and
this handoff from `HEAD` before editing. Read the repository Effigy skill and
the strict everyday Rust authoring instructions selected by `AGENTS.md`.

Run one coherent validation round after the split:

- `cargo fmt -p swallowtail-adapter-cline --check`
- `cargo test -p swallowtail-adapter-cline --test prepared_headless_facade`
- `effigy validate:focused swallowtail-adapter-cline`
- `effigy package:verify-affected swallowtail-adapter-cline`
- `effigy --json scan god-files`
- `effigy qa:docs`
- `git diff --check`

Close the exact PAPERCUTS entry with the measured before/after result. Do not
add a closeout log while the parallel Codex lane owns the shared log index.
Commit, push, and open one PR. Report exact head/base, moved proof grouping,
test count, god-file result, validation, changed files, and PR URL. Do not
merge.
