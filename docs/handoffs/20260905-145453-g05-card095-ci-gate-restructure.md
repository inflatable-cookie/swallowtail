---
title: g05.031 Card 095 CI gate restructure worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260905-145453-g05-card095-ci-gate-restructure.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Implement the manifest-defined post-tag CI gate restructure for g05.031 card
095. This is a workflow and nextest configuration lane only.

## Why It Matters

The release is complete; this lane reduces pull-request latency without
weakening the full pinned release floor or removing any existing validation.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `f1e6c4ed6f7ab2496e9611de7b78561f09dec597`
- **Pushed main verification:** `origin/main` is `f1e6c4ed6f7ab2496e9611de7b78561f09dec597`
- **Planning checkout:** clean and synchronized
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Planning artifacts included at the base:** card 095 and g05.031 manifest
- **Worker branch:** `worker/g05-card095-ci-gate-restructure`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card095-ci-gate-restructure`
- **Worktree creation command:** Paseo branch-off worktree from `main`
- **Required sibling worktree links:** none
- **Active spec lane:** `docs/roadmaps/g05/031-ci-latency.md`
- **Roadmap milestone:** `docs/roadmaps/g05/031-ci-latency.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/095-ci-gate-restructure.md`
- **Allowed runway:** card 095 only
- **Remaining card budget:** one card
- **Coordinator agent ID:** `af21d886-4053-4156-ae6a-e878dfb99985`
- **Delivery route:** coordinator-attached child with `notifyOnFinish: true`
- **Dispatch topology:** card 094's deferred PR 227 repair runs concurrently
- **Parallel safety check:** disjoint owned paths; no shared mutable implementation scope
- **Surfaces this lane owns:** `.github/workflows/ci.yml`, `.config/nextest.toml`, card 095 Result, and PAPERCUTS append only
- **Integration ownership:** coordinator owns merge, shared closeout surfaces, and any branch-protection decision
- **Canonical refs:** `docs/roadmaps/g05/031-ci-latency.md`; `docs/roadmaps/README.md`; `docs/contracts/036-crate-release-and-compatibility-boundary.md`
- **Review oracle:** every prior CI step remains present; full macOS pinned floor remains on main pushes and workflow dispatch; no branch protection change without coordinator agreement
- **Model capability profile:** Luna grind, full-access, xhigh
- **Worker provider/model identity:** `codex/gpt-5.6-luna`
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no production source, test content, Cargo, scripts, contracts, or release mutation
- **Required validation:** card 095 named workflow checks, `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`, before/after timing evidence
- **PR base/head:** current pushed `main` / worker head
- **PR URL:** pending
- **Review state:** awaiting independent review after PR creation
- **Merge path:** coordinator after accepted exact-head review and green required checks

## Boundaries

- **In scope:** the two manifest-owned CI configuration files and card evidence
- **Out of scope:** branch protection, production code, test content, release gates, tags, publication, and consumer mutation
- **Outcome shape:** bounded workflow implementation with timing evidence and reviewable PR
- Do not remove or weaken a validation step. If required-check names or branch protection need a policy decision, stop and return it to the coordinator.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `v0.4.1` is tagged; this is the first post-tag g05.031 lane.
- **Why the card is ready:** operator workflow authority for the named files was granted on 2026-09-05 and the manifest is on pushed `main`.
- **Decisions and preferences:** keep the full pinned floor on `main` pushes and workflow dispatch; isolate process-spawning suites; preserve all existing checks.
- **Open tensions:** any branch-protection change requires coordinator agreement in the card.
- **Report after:** one coherent workflow restructure, timing comparison, validation, and PR.
- **Report to:** the owning coordinator through the linked child result.

## Suggested Next Move

Run Completion Protocol preflight, inspect the existing workflow and nextest
shape, then implement one bounded restructure and validate it as a whole.

## Completion Protocol

Use the repository's standard Northstar worker Completion Protocol from the
orchestrator template: verify the tracked handoff in the selected worktree,
keep the diff inside the owned paths, push the branch, open one PR, and stop
for the retained independent reviewer. Do not create a review workspace; the
reviewer must be launched in this exact worker workspace.

## Handoff Closeout

The coordinator owns shared roadmap/index/log closeout after merge. The worker
must leave the card Result, timing evidence, and PR head exact and honest.
