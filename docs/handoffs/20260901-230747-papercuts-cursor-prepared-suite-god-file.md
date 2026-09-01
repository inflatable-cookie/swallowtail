---
title: Cursor prepared-suite god-file papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-230747-papercuts-cursor-prepared-suite-god-file.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, rust]
---

## Objective

Close the Cursor model-parameter god-file papercut by splitting
`tests/prepared_suite.rs` into focused private modules without changing test
behavior, target identity, public API, diagnostics, or coverage.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `164ab230762e8261911e7c30b7d9e3fcbdd87873`
- **Worker branch:** `worker/papercuts-cursor-prepared-suite-god-file`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a source-preserving integration-test
  module split, not frontier implementation work.
- **Authority:** `AGENTS.md`, the exact `PAPERCUTS.md` entry, strict Rust
  everyday-authoring rules, current test target, Effigy scan, and this handoff.
- **Current reproduction:** repository scan is 379 findings (7 critical / 42
  high / 330 warning); `crates/swallowtail-adapter-cursor/tests/prepared_suite.rs`
  is a 577-code-line high finding. The historical entry's 454-line/error
  terminology predates the current critical/high/warning taxonomy.
- **In scope:** the `prepared_suite` integration-test root and focused private
  modules below it; `PAPERCUTS.md` closeout evidence.
- **Out of scope:** production source, Cargo manifest/target names, public API,
  fixtures, test semantics, other Cursor findings, version/currentness claims,
  roadmap/research/log/index surfaces, and other papercuts.
- **Parallel partition:** Claude Code currentness owns Claude-agent code,
  fixtures, research/log/roadmap, and currentness surfaces. This lane owns only
  Cursor test modules and `PAPERCUTS.md`.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Inventory the current suite before moving code. Group model-parameter
preparation, capability/plan agreement, rejection/drift, and any adjacent
proofs into the smallest coherent private modules; keep shared fixtures in one
bounded support surface. Prefer moving bodies intact. Preserve every test name,
assertion, failure code, access-effect assertion, fixture, and test count.

The original 577-code-line high finding must disappear. No new file may enter
the configured god-file findings. If the current baseline is unchanged, total
findings must improve from 379 to 378 with 7 critical, 41 high, 330 warning. If
main moves first, reconcile against exact current main and prove one net finding
removed with no severity replacement.

Falsify the split by listing the target tests and exact count, running the
target, and comparing moved proof bodies against the base. Any necessary path
or visibility edit must be named; no test may be ignored, combined away, or
replaced by a smoke assertion. Stop if production/API/manifest/diagnostic
changes or new acceptance policy are required.

## Validation And Completion

Confirm a clean non-`main` worktree, `HEAD == origin/main`, required ancestor,
and tracked handoff before edits. Read the Effigy skill and strict everyday
Rust-authoring route.

Run one coherent closeout round:

- `cargo fmt -p swallowtail-adapter-cursor --check`
- `cargo test -p swallowtail-adapter-cursor --test prepared_suite`
- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-adapter-cursor`
- `effigy --json scan god-files`
- `effigy qa:docs`
- `git diff --check`

Close the exact papercut with the current before/after taxonomy and historical
correction. Commit, push, and open one PR. Report exact head/base, module
grouping, test count, moved-body equivalence, scan result, validation, changed
paths, and PR URL. Do not add a log while currentness owns logs/index. Do not
merge.

