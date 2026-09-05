---
title: g05.030 Card 093 Claude SDK sidecar fixture determinism worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260905-100102-g05-card093-claude-sdk-sidecar-fixture-determinism.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release-lane]
---

## What This Thread Was Doing

This lane repairs the Claude SDK sidecar test fixture that made Card 091's
frozen release floor depend on host load. The worker owns the smallest
test-only ordering repair and its proof, then opens one PR for exact-head
review.

## Why It Matters

The v0.4.1 candidate cannot proceed while a passing sidecar-asset test can
time out because another project is busy. The fixture must publish
observations before the wire event that guarantees them, so the release floor
proves ordering rather than a wall-clock bound.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7d9371c1e34f863a7d985d195e7c7ccbd27753ff`
- **Pushed main verification:** `HEAD == origin/main == 7d9371c1e34f863a7d985d195e7c7ccbd27753ff`
- **Planning checkout:** clean and aligned with pushed `origin/main`
- **Worker mode:** implementation worker dispatched by the orchestrator; this handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Card 093 manifest in `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`, including the two failing floor tests and the freeze boundary.
- **Worker branch:** `worker/g05-card093-claude-sdk-sidecar-fixture-determinism`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card093-claude-sdk-fixture-determinism`
- **Worktree creation command:** launcher-created dedicated worktree from pushed `origin/main`
- **Worker worktree policy:** follow Completion Protocol; do not use the planning checkout.
- **Required sibling worktree links:** none
- **Active spec lane:** `g05.030 v0.4.1 release readiness`
- **Roadmap milestone:** `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/093-claude-sdk-sidecar-fixture-determinism.md`
- **Allowed runway:** Card 093 only; Card 091 re-prepares after this PR merges.
- **Remaining card budget:** one bounded repair PR
- **Coordinator agent ID:** `af21d886-4053-4156-ae6a-e878dfb99985`
- **Delivery route:** coordinator-attached child with `notifyOnFinish: true`; preserve parentage and the same workspace for review.
- **Dispatch topology:** Card 093 alone; feature freeze remains active.
- **Parallel safety check:** no approved concurrent siblings; Card 091 is retained but paused and writes a separate workspace.
- **Surfaces this lane owns:** `crates/swallowtail-adapter-claude-agent/tests/**`, including `sidecar_asset_support/fake-sdk.mjs` and `mod.rs`; this card's `## Result`; append-only `PAPERCUTS.md` entries if a new solvable execution hurdle appears.
- **Integration ownership:** coordinator owns the PR merge, Card 093 closeout, and the later Card 091 rebase/reprepare.
- **Merge ordering:** Card 093 must merge before Card 091's fourth prepare; rebase/revalidate Card 091 after this merge.
- **Canonical refs:** `docs/roadmaps/README.md`; `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`; `docs/contracts/036-release-and-distribution.md`; `docs/roadmaps/g05/batch-cards/093-claude-sdk-sidecar-fixture-determinism.md`
- **Review oracle:** a passing sidecar-asset test never waits on a timing bound; the fake SDK writes and flushes observations before the wire event the Rust fixture observes next.
- **Model capability profile:** economical Rust/Node test-fixture implementation worker with concurrency discipline
- **Worker provider/model identity:** selected and materialized by the coordinator's launch profile
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no provider credentials; no live calls; do not change production sidecar or adapter source; do not touch Cargo files, CHANGELOG, release baselines, contracts, or other crates.
- **Required validation:** `cargo fmt -p swallowtail-adapter-claude-agent -- --check`; `effigy validate:focused swallowtail-adapter-claude-agent`; `effigy package:verify-affected swallowtail-adapter-claude-agent`; `rustup run 1.95.0 cargo test -p swallowtail-adapter-claude-agent --all-features --locked`; `effigy qa:northstar`; `git diff --check`; loop-under-load sidecar-asset binary proof with at least 20 runs and zero failures.
- **PR base/head:** current pushed `main` / worker branch HEAD
- **PR URL:** pending
- **Review state:** stop for retained independent reviewer in this exact worker workspace after PR creation
- **Merge path:** coordinator after accepted exact-head review and all required checks

## Boundaries

- **In scope:** ordering and flush guarantees in `crates/swallowtail-adapter-claude-agent/tests/sidecar_asset_support/fake-sdk.mjs`, Rust fixture reads and wire ordering in the corresponding test support, card result evidence, and a narrowly justified papercut append.
- **Out of scope:** `crates/swallowtail-adapter-claude-agent/src/**`, `sidecar/**`, every other crate, Cargo files, `CHANGELOG.md`, `release-baselines/**`, contracts, production behavior, release candidate preparation, tags, publication, and consumer mutation.
- **Outcome shape:** smallest complete test-fixture repair with deterministic evidence and one reviewable PR. If ordering cannot be guaranteed without a production change, stop and return the decision to Chatterbox.
- Do not raise the timeout as the fix, add a poll loop, invent architecture, change contracts, or widen the feature-freeze scope.
- Do not merge; merge belongs to the coordinator.

## Important Context

- **Planning lineage:** Card 091's frozen-tree floor failed only because two sidecar-asset tests timed out waiting for fake-SDK observations; Chatterbox ruled that load-sensitive fixture behavior is the defect. Card 093 is the v0.4.0 fixture-race precedent under the v0.4.1 feature freeze.
- **Why this card is ready:** the failing tests, exact fixture path, ordering rule, forbidden production surfaces, and zero-failure load proof are all specified in the published Card 093 manifest.
- **Decisions and preferences:** fix ordering, not the bound; preserve provider-free tests; keep Card 091 paused until this PR merges.
- **Open tensions:** if the required ordering crosses into production code, stop and send a complete pre-PR decision request to Chatterbox.
- **Report after:** one coherent fixture repair and its full validation/evidence batch.
- **Report to:** the owning coordinator through the linked child result; do not open a placeholder PR.

## Suggested Next Move

Run the Completion Protocol preflight, read Card 093 and the relevant test fixture, reproduce the two failures/ordering path, implement the smallest test-only fix, and run the named stable and pinned validation plus the loop-under-load proof.

## Completion Protocol

Use the standard Northstar orchestrator worker protocol from the template: verify the committed handoff and clean launcher worktree, preserve exact owned paths, push one reviewable PR, and stop for independent exact-head review in this same workspace. Do not merge, create tags, publish, or modify Card 091's retained workspace.

