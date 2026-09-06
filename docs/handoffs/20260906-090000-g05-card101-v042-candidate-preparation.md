---
title: g05.032 Card 101 v0.4.2 candidate preparation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-06
updated: 2026-09-06
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260906-090000-g05-card101-v042-candidate-preparation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, release]
---

## What This Thread Was Doing

Dispatch g05.032 Card 101 from Card 103's merged fixture-only verdict at
`d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`. Prepare the source-only `0.4.2`
candidate under the recorded standing-grant authorization.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning base:** `d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`
- **Roadmap:** `docs/roadmaps/g05/032-v0-4-2-release-readiness.md`
- **Card:** `docs/roadmaps/g05/batch-cards/101-v0-4-2-candidate-preparation.md`
- **Worker branch:** `worker/g05-card101-v042-candidate-preparation`
- **Worker mode:** implementation worker dispatched by the coordinator; this
  handoff activates the worker-only worktree preflight.
- **Dispatch topology:** one serial release-preparation lane; Card 082 remains
  branch-only and must not merge before the tag.
- **Card 103 verdict:** fixture-only; no production OpenCode change; merged at
  `d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`.
- **Worker profile:** Luna grind, `codex/gpt-5.6-luna`, `full-access`, xhigh.

## Required Outcome

Read the Card 100 Result, merged Contract 061 tranches B/K/L and the 767/767
completion, and cards 081, 093, 094, and 095. Author `docs/releases/0.4.2.md`
and its index entry. State the additive patch class: Card 100 adds five
baseline lines and the merged tranches add contribution methods. Preserve the
live `claude-agent.sdk` editing turn as unresolved typed `ProviderFailed`
evidence; Card 102 proves it in the consumer run.

Run the read-only release status and verify it infers `0.4.2` with the
three-mutation plan. Bring the lock into sync before the first locked gate.
Run exactly once:

`effigy --json release prepare --yes --check-gates --version 0.4.2`

Keep the per-gate logs as evidence. Do not run a separate frozen-tree rerun.
Stop on any real Effigy gate defect and report the JSON plus per-gate logs;
only a captured transient may be renewed by Chatterbox under the standing
grant.

Repoint the four gate scripts and consumer front-door to `0.4.2` exactly as
Card 091 did. Candidate release note must say `Status: candidate; not tagged`.
Do not create or push a tag. Do not touch credentials, auth, permissions,
Desktop, consumer pins, feature/runtime code, or `.github/workflows/`.

Open one candidate PR from the worker branch. Launch independent review and
workflow-dispatch exact-SHA CI in parallel. Merge only after both are green.
Report the exact PR/head and later merged candidate SHA to the coordinator for
Chatterbox/Bovine packet completion. Do not merge Card 082 before the tag.

## Owned Paths

Only the Card 101 manifest-owned release surfaces: workspace versions and lock
entries created by the prepare transaction, `CHANGELOG.md`, `docs/releases/0.4.2.md`,
`docs/releases/README.md`, `release-baselines/public-api-0.4.2/**`,
`production-routes-0.4.2.txt`, `internal-dependencies-0.4.2.tsv`,
`.release-prepared.json`, the four gate/front-door scripts and their README,
root release-posture lines, Card 101 Result, and append-only `PAPERCUTS.md`.
No crate source or tests, no prior baselines, no contracts, architecture,
guides, matrices, tags, credentials, consumer mutation, or workflow edits.

## Validation and Stop Rules

Use Effigy selectors and the manifest's exact release command. Record the
read-only status, prepare JSON, every gate log, lock sync, semantic baseline,
route inventory, dependency graph, PR head, review verdict, workflow-dispatch
run id, and merged SHA. Stop and report if status is not `0.4.2`, a gate has a
real defect, an open feature PR exists, or the scope expands.

Work only in the launcher-provided clean non-main worktree. Do not create a
second worktree, reset, stash, or merge from the worker. The coordinator owns
the exact-head review gate and merge.
