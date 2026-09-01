---
title: Host-local watcher god-file papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-125509-papercuts-host-local-watcher-god-files.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, rust, mechanical]
---

## Objective

Close the open host-local watcher registry god-file papercut by splitting the
remaining oversized implementation files into focused internal modules without
changing behavior, public API, lifecycle, or proof strength.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base:** `51ed44f57a2a878628f0f30b251dd454f1907a9f`
- **Pushed-main check:** local `main` and `origin/main` matched at that merge
  before this handoff was compiled.
- **Worker branch:** `worker/papercuts-host-local-watcher-god-files`
- **Worker worktree:** Paseo-managed worktree branched from pushed
  `origin/main`, titled with the capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a long, source-preserving Rust module
  split, not frontier implementation reasoning.
- **Current god-file baseline:** 387 findings: 7 critical, 42 high, 338
  warning.
- **Current exact residual:** only two of the four paths named by the 2026-08-29
  entry remain findings: `src/watcher/accept.rs` at 288 code lines and
  `src/process.rs` at 284. `src/watcher.rs` and
  `tests/watcher_service/policy.rs` are already below the warning threshold on
  current `main`; do not split them merely to match stale prose.
- **Ready-frontier shape:** independent mechanical lane. It may run beside the
  Contract 029 research worker; code and documentation surfaces do not overlap.
- **Serial edge:** all later Swallowtail papercuts that edit `PAPERCUTS.md` or
  `docs/logs/README.md` wait for this lane to merge or stop.
- **Merge authority:** orchestrator exact-head review; worker must not merge.

## Scope

In scope:

- re-measure the exact four paths and reconcile the stale four-file claim;
- split watcher acceptance/lookup and local process construction/validation
  into focused private modules;
- preserve module visibility, diagnostic codes/messages, ordering, ownership,
  cleanup, process-group, watcher registry, and failure behavior exactly;
- keep each changed and new file below the configured warning threshold;
- close only the matching papercut with the exact before/after finding counts;
- add one bounded closeout log and log-index entry when existing papercut
  practice warrants it.

Out of scope:

- semantic refactors, performance changes, API changes, compatibility aliases,
  new abstractions, or lint suppression;
- changing watcher lifecycle, the PR 149 executor repair, process policy,
  limits, diagnostics, platform behavior, or test meaning;
- splitting unrelated god files, including `tests/local_process.rs`;
- roadmap, contract, architecture, research, feature-matrix, or route changes;
- any other papercut.

## Acceptance And Review Oracle

The PR is acceptable when:

1. The two current findings disappear and no new finding appears; expected
   total is 385 if the baseline is otherwise unchanged.
2. `watcher.rs` and `watcher_service/policy.rs` remain behaviorally untouched
   unless a minimal module declaration/import adjustment is strictly required.
3. Public and semantic API baselines are unchanged.
4. All existing host-local process, watcher service, watcher race, wakeup,
   cleanup, and scoped-task tests retain their assertions and pass.
5. A source/diff audit shows functions and branches moved, not rewritten:
   diagnostic codes/messages, ordering, limits, cfg gates, and ownership
   remain exact.
6. Recombining either split into its original file locally recreates the
   warning finding without changing test outcomes; restoring the split removes
   it. This makes the structural result load-bearing.

Do not accept line-count gaming through compressed statements, removed docs,
ignored findings, generated files, `#[allow]`, or reduced tests.

## Stop Conditions

Stop and report evidence if the split requires a public visibility change,
cyclic module dependency, semantic rewrite, new shared abstraction, test
weakening, or any change outside the bounded host-local/Papercuts surfaces.
If current Effigy measurement differs from the numbers above, record the exact
new baseline before editing and reconcile it rather than forcing 385.

## Validation

After the complete mechanical split, run:

- `cargo fmt --check`
- `effigy validate:focused swallowtail-host-local`
- `effigy package:verify-affected swallowtail-host-local`
- `effigy package:api swallowtail-host-local`
- warranted docs checks for the papercut/log edit
- `effigy --json scan god-files`
- `git diff --check`

No provider command, live probe, install, authentication, or broad workspace
QA is authorized.

## Completion Protocol

Before broad reads, confirm a clean registered non-`main` worktree, exact
branch, `HEAD == origin/main`, and that this handoff is loaded from `HEAD`.
Read `AGENTS.md`, the exact papercut, current Effigy god-file output,
`process.rs`, `watcher.rs`, `watcher/accept.rs`, `watcher_service/policy.rs`,
their module roots, and focused host-local test surfaces.

Move coherent implementation units in one batch. Do not opportunistically
refactor. Compare the moved functions against the base, run the accepting
checks, commit, push, and open one PR against current pushed `main`. Report
exact head/base, before/after per-file and total findings, changed files,
semantic-diff audit, validation, residuals, and PR URL. Do not merge.
