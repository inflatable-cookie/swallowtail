---
title: g05.030 Card 091 v0.4.1 candidate preparation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260905-081403-g05-card091-v0-4-1-candidate-preparation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release-readiness]
---

## What This Thread Was Doing

This dispatches g05.030 Card 091 after Card 090's accepted compatibility audit.
It prepares exactly one frozen `0.4.1` source candidate, reruns the named
gates, opens one candidate PR, and stops for exact-head review and merged-SHA
CI. The operator's one-shot prepare authorization is consumed by one Effigy
prepare transaction only.

## Why It Matters

Card 091 turns Research 286's patch-compatible census into the reviewed source
candidate that Card 092 can consume. It is the last preparation step before
the separately gated consumer proof and operator tag decision.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `860a91a078f68c7b58f289168abe61c72d8d12dd`
- **Pushed main verification:** `HEAD == origin/main == 860a91a078f68c7b58f289168abe61c72d8d12dd`
- **Planning checkout:** clean at the Card 090 closeout
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** accepted Card 090, Research
  286, g05.030, and the one-shot prepare authorization.
- **Worker branch:** `worker/g05-card091-v0-4-1-candidate-preparation`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card091-v0-4-1-candidate-preparation`
- **Worktree creation command:** `paseo workspace create --isolation worktree --mode branch-off --new-branch worker/g05-card091-v0-4-1-candidate-preparation --base main`
- **Worker worktree policy:** follow Completion Protocol; the reviewer uses
  this same workspace and never receives a separate review worktree.
- **Required sibling worktree links:** none
- **Active spec lane:** g05.030 v0.4.1 Release Readiness
- **Roadmap milestone:** `docs/roadmaps/g05/030-v0-4-1-release-readiness.md`
- **Ready cards, in order:** Card 091 only
- **Allowed runway:** one candidate-preparation PR and its exact-SHA CI gate
- **Remaining card budget:** one authorized prepare transaction; one card
- **Dispatch topology:** one worker and one same-workspace independent reviewer
- **Parallel safety check:** feature and currentness merges are frozen through
  Card 092; no sibling mutable scope is approved.
- **Surfaces this lane owns:** `docs/releases/0.4.1.md` and its release index
  entry; the promoted `CHANGELOG.md` section; coordinated package versions in
  `Cargo.toml`; the workspace-only `Cargo.lock` sync; regenerated `0.4.1`
  public API baselines and production route inventory; Card 091's Result; and
  the named preparation evidence.
- **Integration ownership:** coordinator owns shared roadmap, batch-card,
  generation-index, logs, tag, publication, and post-merge closeout surfaces.
- **Merge ordering:** same-repository PRs merge one at a time; refresh against
  current `main` and re-review if the base advances.
- **Canonical refs:** Contract 036; immutable v0.4.0 at `56f3913a`; Research
  286; Card 050's prepare precedent.
- **Review oracle:** exactly one successful prepare transaction, frozen
  candidate evidence, all local gates green, and CI green at the merged SHA;
  no mutation to v0.4.0 baselines.
- **Model capability profile:** release-preparation worker with Effigy,
  Contract 036, semantic API, route, and exact-SHA CI discipline; no provider
  credentials or consumer mutation.
- **Worker provider/model identity:** Codex `gpt-5.6-luna`, full-access,
  xhigh reasoning.
- **Frontier-worker justification:** none; this is a bounded serial release
  preparation lane.
- **Tool/runtime restrictions:** run exactly one prepare transaction; if it
  fails or rolls back, stop and report that the authorization is consumed; no
  tag, publication, provider call, or consumer-repo mutation.
- **Required validation:** the Card 091 named Effigy release status,
  prepare, local package/API/route/docs/Northstar gates, exact-SHA candidate
  CI, and `git diff --check`.
- **PR base/head:** current pushed `main`; worker reports the exact head.
- **PR URL:** pending
- **Review state:** awaiting worker PR and same-workspace independent review
- **Merge path:** orchestrator after accepted exact-head review, green checks,
  and the required merged-SHA CI.

## Boundaries

- **In scope:** the Card 091 release note/index, promoted changelog,
  coordinated `0.4.1` package versions, workspace-only lock sync, new `0.4.1`
  API/route baselines, Card 091 Result, and the named candidate evidence.
- **Out of scope:** tag creation or push; crates.io/GitHub publication; GitHub
  Release objects; binaries, sidecars, installers; consumer edits; providers;
  feature/currentness changes; any v0.4.0 baseline mutation; Card 092 smoke.
- **Outcome shape:** one candidate PR, or a precise stop if the single prepare
  transaction fails, rolls back, or reveals a planning/compatibility problem.
- Do not invent release policy, change contracts, widen the roadmap, or spend
  the authorization twice. Any new decision returns to Chatterbox.
- Do not merge the PR; merge belongs to the coordinator after its gate.

## Important Context

- **Planning lineage:** Card 090 was accepted and merged as `3dcf4f12`; its
  Research 286 census is the frozen input for this lane.
- **Decisions and preferences:** the operator granted exactly one prepare
  transaction on 2026-09-05; failure or rollback consumes it. Feature and
  currentness merges remain frozen until Card 092 stops.
- **Open tensions:** Card 092's Bovine Desktop smoke packet lacks its exact
  checkout, command/test, retry budget, and consumer mutation permission; do
  not enter Card 092.
- **Report after:** the candidate PR exact head, prepare evidence, all local
  gates, and the merged-SHA CI requirement or a complete stop capsule.
- **Report to:** the operator, who relays progress to the orchestrator.

## Suggested Next Move

Run the Completion Protocol preflight and confirm the clean canonical base.
Read Card 091, Research 286, Contract 036, and the release tooling. Perform
the one authorized Effigy prepare transaction, rerun every named gate, open one
candidate PR, and stop for exact-head review. Never create a tag.

## Completion Protocol

Use the standard Northstar orchestrator Completion Protocol: verify this
committed handoff and canonical base before broad reads, work only in the
selected clean worker worktree, push one PR, and leave merge/review and
reserved closeout surfaces to the coordinator. The reviewer must use a
different underlying provider/model identity in this exact worker workspace.
