---
title: Papercuts wave 23 OpenAI test target names worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Tom / local Swallowtail orchestrator
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260831-153924-papercuts-wave23-openai-test-targets.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Card 023 exposed a pre-existing OpenAI adapter test-routing papercut. The three
explicit Cargo integration-test target names do not match their suite-root
filenames. The card 023 lane is merged and closed; this repair is a separate
papercuts wave 23 lane.

This handoff reserves one bounded implementation lane. No worker, workspace,
or branch has been launched. Do not launch it concurrently with another lane
that edits `PAPERCUTS.md` or the papercuts closeout log.

## Why It Matters

An agent adding a module to `tests/direct_suite.rs` naturally runs
`cargo test -p swallowtail-adapter-openai --test direct_suite`. Cargo rejects
that command and exposes the unrelated target name `prepared_facade`. The same
mismatch exists for the catalogue and Realtime suite roots.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `0f19306147d5d3d5b0221db0128639fa6d6b78b7`
- **Pushed main verification:** local `main` and `origin/main` matched that
  exact commit before this handoff was created.
- **Planning checkout:** clean before this handoff batch.
- **Queue state:** complete; PR 136 merged.
- **Worker mode:** implementation worker completed; orchestrator review and
  merge completed.
- **Planning artifacts included at the base:** merged card 023 closeout and the
  unchanged open papercut entry.
- **Worker branch:** `worker/papercuts-wave23-openai-test-target-names`
- **Worker worktree:** launcher first; manual fallback
  `/Users/tom/Dev/worktrees/papercuts-wave23-openai-test-target-names`.
- **Worktree creation command:** launcher-managed; manual fallback only through
  the worker preflight and configured worktree container.
- **Required sibling worktree links:** none.
- **Active spec lane:** none; this is repository-maintenance routing only.
- **Roadmap milestone:** none; keep the existing g05 Next Task unchanged.
- **Ready cards:** none; the assigned issue and this handoff are the complete
  authority surface.
- **Allowed runway:** reproduce the mismatch; rename the three explicit
  `[[test]]` targets in `crates/swallowtail-adapter-openai/Cargo.toml` to
  `catalogue_suite`, `direct_suite`, and `realtime_suite`; close only this
  `PAPERCUTS.md` entry; validate; push one PR.
- **Remaining budget:** one papercut, one PR.
- **Dispatch topology:** serial. The route-matrix Python-bytecode papercut
  follows only after this lane closes.
- **Parallel safety check:** production and card 023 test code are out of
  scope. Shared `PAPERCUTS.md` and closeout-log ownership makes papercut lanes
  serial.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`;
  `crates/swallowtail-adapter-openai/Cargo.toml`; the three suite-root files.
- **Review oracle:** Cargo metadata exposes exactly the three suite-root target
  names; each `cargo test --test <suite-root>` command resolves and runs the
  same test tree; no source module, test body, feature, dependency, public Rust
  API, or historical log is rewritten.
- **Model capability profile:** bounded mechanical Rust/Cargo maintenance.
- **Tool/runtime restrictions:** no provider contact, live probe, roadmap
  movement, card 023 reopening, route-matrix cleanup, or workflow edit.
- **Required validation:** the three exact suite-root test commands;
  `effigy validate:focused swallowtail-adapter-openai`;
  `effigy package:verify-affected swallowtail-adapter-openai`;
  `effigy qa:docs`; `effigy qa:northstar`; `git diff --check`.
- **PR base/head:** current pushed `main` / worker branch above.
- **PR URL:** https://github.com/inflatable-cookie/swallowtail/pull/136
- **Reviewed head:** `274bd2ac58ed5ca856a44300bb7edaf5b85467e4`.
- **Merged main commit:** `af339fb65d5a249bddcf0f58abae95953d4d465a`.
- **Review state:** accepted exact-head verdict recorded on PR 136; all five CI
  checks passed; merged by the orchestrator.
- **Merge authorisation:** exercised under the operator's standing authority.

## Boundaries

- **In scope:** align the three explicit Cargo test target names with their
  suite-root filenames and close the exact papercut.
- **Out of scope:** module/file renames, test rewrites, package API changes,
  route-matrix bytecode cleanup, card 023, Contract 061, PR 130, CI workflow
  edits, and unrelated papercuts.
- **Outcome shape:** smallest complete fix, validation, `PAPERCUTS.md` closure,
  and one reviewable PR.
- If repository-owned automation or a non-historical current instruction uses
  an old target name, stop and report instead of adding compatibility aliases.
- Work only in the clean worker worktree selected by the completion protocol.
- Do not merge the PR. Review and merge belong to this orchestrator.

## Important Context

- `cargo test -p swallowtail-adapter-openai --test direct_suite --no-run`
  currently fails with no target named `direct_suite`.
- `cargo test -p swallowtail-adapter-openai --test prepared_facade --no-run`
  builds `tests/direct_suite.rs`, proving the routing mismatch.
- Repository-wide search found no live script or selector using the three old
  names. One 2026-08-27 log records the then-valid Realtime target command; it
  is historical evidence and stays unchanged.
- Card 023 merged through PR 133 and its closeout is pushed at `0f193061`; do
  not reopen or restack that lane.
- Report after reproduction plus manifest repair, then after final validation
  and PR creation.
- Report to the local Swallowtail orchestrator.

## Suggested Next Move

Do not relaunch this handoff. Continue the serial queue through
`/Users/tom/Dev/projects/swallowtail/docs/handoffs/20260831-195246-papercuts-wave23-route-matrix-bytecode.md`.

## Completion Protocol

### Before you start

1. Worker mode activates only when the orchestrator dispatches this committed
   handoff. Before broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree. Record its actual
   root and branch; do not create another because names differ.
3. If the current context is `main`, dirty, unregistered, or unusable, inspect
   the named fallback. If needed, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`, and create a
   unique worktree there from pushed `origin/main`. Never use `/tmp`; never
   clean, reset, stash over, or discard dirty state.
4. Fetch origin with non-interactive SSH. Confirm selected `HEAD ==
   origin/main`, confirm planning base `0f19306147d5d3d5b0221db0128639fa6d6b78b7`
   is an ancestor, and load this tracked handoff from `HEAD`. Stop if the
   absolute and tracked files differ.
5. Required sibling list is none.
6. Read `AGENTS.md`, `PAPERCUTS.md`, the OpenAI manifest, and the three suite
   roots. Use Effigy for repository-owned validation.

### While you work

- Reproduce before editing. Rename only the three manifest targets unless the
  exact search exposes a current consumer that changes the plan.
- Do not rename suite files or modules, change tests, or add aliases.
- Mark the OpenAI target-name entry closed only after the new commands pass.
- Leave the route-matrix Python-bytecode entry open and queued next.
- Stop on a public, workflow, external-consumer, or planning dependency.

### When the assigned runway is complete

1. Run the three exact suite-root test commands, focused OpenAI validation,
   affected-package verification, docs QA, Northstar QA, and diff check.
2. Falsify the repair: each new target resolves to its matching suite root;
   all three suites retain their test counts and module trees; no old target
   declaration or new alias remains.
3. Update only the assigned `PAPERCUTS.md` entry. Do not create the merge
   closeout log; the orchestrator owns post-merge log reconciliation.
4. Push the worker branch and open one PR against current pushed `main`.
5. Report the exact head, changed files, test counts, checks, and PR URL. Do not
   merge.

### Review and merge path

The local orchestrator accepted exact head `274bd2ac`, recorded the verdict on
GitHub, waited for all five checks, and squash-merged PR 136 as `af339fb6`.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; post-merge papercuts log.

### Handoff closeout

This handoff is merged. The orchestrator wrote the serial closeout log and
prepared the route-matrix bytecode candidate next without moving the g05
roadmap pointer.
