---
title: g05.030 Card 094 deadline fixture determinism sweep worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260905-110707-g05-card094-deadline-fixture-determinism-sweep.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release-lane, frontier]
---

## What This Thread Was Doing

This lane removes scheduler-dependent deadline and cleanup assertions from
test fixtures under the v0.4.1 feature freeze. It fixes the Pi lifecycle test
that stopped Card 091, sweeps the named workspace candidates, records the
classification ledger, and opens one reviewable test-only PR.

## Why It Matters

Card 091's fourth prepare found a Pi test whose result depended on whether a
fake sidecar child exited before the immediate fixture deadline was observed.
The runtime is correct; the fixture must control process lifecycle explicitly
so release-floor evidence is independent of host load.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `fcd3211a6454a21aaece7afffeb39670faec7651`
- **Pushed main verification:** `HEAD == origin/main == fcd3211a6454a21aaece7afffeb39670faec7651`
- **Planning checkout:** clean and aligned with pushed `origin/main`
- **Worker mode:** implementation worker dispatched by the orchestrator; this handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Card 094 manifest in `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`, including the Card 091 floor excerpt and release-freeze boundary.
- **Worker branch:** `worker/g05-card094-deadline-fixture-determinism-sweep`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card094-deadline-fixture-determinism-sweep`
- **Worktree creation command:** launcher-created dedicated worktree from pushed `origin/main`
- **Worker worktree policy:** follow Completion Protocol; never use the planning checkout.
- **Required sibling worktree links:** none
- **Active spec lane:** `g05.030 v0.4.1 release readiness`
- **Roadmap milestone:** `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/094-deadline-fixture-determinism-sweep.md`
- **Allowed runway:** Card 094 only; Card 091 resumes after this PR merges.
- **Remaining card budget:** one bounded repair PR
- **Coordinator agent ID:** `af21d886-4053-4156-ae6a-e878dfb99985`
- **Delivery route:** coordinator-attached child with `notifyOnFinish: true`; preserve parentage and the same workspace for review.
- **Dispatch topology:** Card 094 alone; Card 091 is paused in its retained workspace; feature freeze remains active.
- **Parallel safety check:** no approved concurrent siblings; Card 094 writes only test and card-result surfaces.
- **Surfaces this lane owns:** `crates/*/tests/**`; this card's `## Result`; append-only `PAPERCUTS.md` entries for new solvable execution hurdles.
- **Integration ownership:** coordinator owns the PR merge, Card 094 closeout, and Card 091's later rebase/reprepare.
- **Merge ordering:** Card 094 must merge before Card 091's fifth prepare; rebase/revalidate Card 091 after this merge.
- **Canonical refs:** `docs/roadmaps/README.md`; `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`; `docs/contracts/036-release-and-distribution.md`; `docs/roadmaps/g05/batch-cards/094-deadline-fixture-determinism-sweep.md`
- **Review oracle:** no passing test depends on which of two independently scheduled events wins; a widened bound is not a fix.
- **Model capability profile:** frontier-tier Rust test-fixture worker with concurrency discipline and no provider credentials
- **Worker provider/model identity:** selected and materialized by the coordinator's launch profile
- **Frontier-worker justification:** the sweep spans workspace-wide timing-sensitive test fixtures and the release floor; it requires exceptional reasoning plus material release consequence.
- **Tool/runtime restrictions:** no provider credentials; no live calls; no production source, sidecar scripts, Cargo files, CHANGELOG, release baselines, contracts, or non-test Rust paths.
- **Required validation:** `cargo fmt --all -- --check`; `effigy validate:focused` for each touched adapter package, at most four per run; `rustup run 1.95.0 cargo test --workspace --all-features --locked`; `effigy qa:northstar`; `git diff --check`; 20+ CPU-load runs with zero failures for every fixed test binary; sweep ledger recorded in Card 094 Result.
- **PR base/head:** current pushed `main` / worker branch HEAD
- **PR URL:** pending
- **Review state:** stop for retained independent reviewer in this exact worker workspace after PR creation
- **Merge path:** coordinator after accepted exact-head review and all required checks

## Boundaries

- **In scope:** Pi lifecycle fixture process control, the named workspace test candidates from the card's grep and known Claude Agent structured-run/SDK driver areas, test-only ordering repairs, the sweep ledger, load proofs, and Card 094 Result.
- **Out of scope:** every `crates/**/src` path, production sidecar scripts, Cargo files, CHANGELOG, release baselines, contracts, release candidate preparation, tags, publication, consumer mutation, and Card 091's workspace.
- **Outcome shape:** smallest complete test-fixture repair and evidence batch with one reviewable PR. If determinism requires a production change, stop and return the decision to Chatterbox.
- Do not raise bounds as the fix, invent architecture, or widen the feature-freeze scope.
- Do not merge; merge belongs to the coordinator.

## Important Context

- **Planning lineage:** Card 091 attempt 4 failed `floor` on `swallowtail-adapter-pi`'s `host_deadline_uses_native_abort_and_resolves_timed_out`, which observed `None` instead of `swallowtail.session_cleanup.deadline_expired` because a held fake child's exit raced the immediate deadline. Card 093 is the accepted test-ordering precedent.
- **Why this card is ready:** the exact failure, fixture mechanism, sweep starting points, load-proof requirement, forbidden surfaces, and serial edge to Card 091 are promoted in the Card 094 manifest.
- **Decisions and preferences:** explicit process control and ordering first; no larger timeout; preserve production behavior; keep Card 091 paused until this PR merges.
- **Open tensions:** if any candidate cannot be made deterministic without production changes, stop and send a complete pre-PR decision request to Chatterbox.
- **Report after:** one coherent fixture sweep and its full validation/evidence batch.
- **Report to:** the owning coordinator through the linked child result; do not open a placeholder PR.

## Suggested Next Move

Run Completion Protocol preflight, reproduce the Pi failure, fix the Hold process control, sweep and classify every named candidate, then run the pinned workspace test and load proofs before opening the PR.

## Completion Protocol

Use the standard Northstar orchestrator worker protocol: verify this committed handoff and clean launcher worktree, preserve exact owned paths, push one reviewable PR, and stop for independent exact-head review in this same workspace. Do not merge, create tags, publish, or modify Card 091's retained workspace.

