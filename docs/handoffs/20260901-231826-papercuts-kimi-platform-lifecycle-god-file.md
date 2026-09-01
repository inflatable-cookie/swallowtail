---
title: Kimi Platform lifecycle god-file papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-231826-papercuts-kimi-platform-lifecycle-god-file.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, rust]
---

## Objective

Close the Kimi Platform lifecycle god-file papercut by splitting
`tests/connection_lifecycle.rs` into focused private modules without changing
behavior, assertions, target identity, public API, diagnostics, or coverage.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `302483424111552bf310dc1ed413659ef73c802a`
- **Worker branch:** `worker/papercuts-kimi-platform-lifecycle-god-file`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a source-preserving integration-test
  module split, not frontier implementation work.
- **Authority:** `AGENTS.md`, the exact `PAPERCUTS.md` entry, strict Rust
  everyday-authoring rules, current target, Effigy scan, and this handoff.
- **Current reproduction:** scan is 378 findings (7 critical / 41 high / 330
  warning); `crates/swallowtail-adapter-kimi-platform/tests/connection_lifecycle.rs`
  is a 512-code-line high finding. The entry's 566-line/error wording predates
  current critical/high/warning taxonomy and later edits.
- **In scope:** the `connection_lifecycle` integration-test root and focused
  private modules beneath it; `PAPERCUTS.md` closeout evidence.
- **Out of scope:** production source, manifests/target names, public API,
  fixtures, test semantics, other Kimi/Kimi Platform findings, currentness or
  g05.009/card 034 claims, roadmap/research/log/index surfaces, and other
  papercuts.
- **Parallel partition:** Claude Code currentness/review owns Claude-agent and
  currentness surfaces. This lane owns only Kimi Platform lifecycle tests and
  `PAPERCUTS.md`.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Inventory the current target before moving code. Split admission/preparation,
refresh/catalogue, Contract 047, failure/drift, and shared fixture concerns into
the smallest coherent private modules justified by the existing proofs. Prefer
moving bodies intact. Preserve every test name, assertion, failure code,
ordering/lifecycle check, effect boundary, fixture, and test count.

The 512-code-line high finding must disappear. No new file may enter the
configured god-file findings. If main is unchanged, total findings must improve
from 378 to 377 with 7 critical, 40 high, 330 warning. If main moves first,
reconcile against exact current main and prove one net finding removed without
severity replacement.

Falsify the split by listing the target tests and exact count, running the
target, and comparing moved function/test bodies against the base. Name any
unavoidable path or visibility edit. No proof may be ignored, combined away,
or replaced with a smoke assertion. Stop if production/API/manifest/diagnostic
changes or new acceptance policy are required.

## Validation And Completion

Confirm a clean non-`main` worktree, `HEAD == origin/main`, required ancestor,
and tracked handoff before edits. Read the Effigy skill and strict everyday
Rust-authoring route.

Run one coherent closeout round:

- `cargo fmt -p swallowtail-adapter-kimi-platform --check`
- `cargo test -p swallowtail-adapter-kimi-platform --test connection_lifecycle`
- `effigy validate:focused swallowtail-adapter-kimi-platform`
- `effigy package:verify-affected swallowtail-adapter-kimi-platform`
- `effigy --json scan god-files`
- `effigy qa:docs`
- `git diff --check`

Close the exact papercut with current before/after taxonomy and historical
correction. Commit, push, and open one PR. Report exact head/base, module
grouping, test count, moved-body equivalence, scan result, validation, changed
paths, and PR URL. Do not add a log while currentness owns logs/index. Do not
merge.

