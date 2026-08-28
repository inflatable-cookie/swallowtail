---
title: Papercuts wave 4 QA flakes worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-for-review
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-235310-papercuts-wave4-qa-flakes.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 2 closed `/var` patch paths, OpenCode BrokenPipe, isolated HOME, and
rustfmt edition. Remaining Swallowtail papercuts still let docs QA miss
roadmap-status drift and let timing/cancellation tests redden unrelated
PRs.

You are the Swallowtail implementation worker. Leave SPA HTML research
notes and T3 launcher cleanup alone.

## Why It Matters

PR 73 passed every docs-index selector while generation-index still
called a finished card ready. Timing-sensitive deadline fixtures fail
unrelated heads. DeepSeek cancellation flakes as ProviderFailed.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `437af7f6ed8680f4e10e8d12705120ec48a4d040`
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave4-qa-flakes`
- **Worker worktree:** `/Users/tom/.t3/worktrees/swallowtail/t3code-3c36e1a0`
- **Ready work items, in order:**
  1. Docs index QA misses roadmap-status drift — done
  2. Timing-sensitive deadline fixtures make unrelated PR heads red — done
  3. DeepSeek stream-cancellation test flakes as ProviderFailed — done
  4. Pi replay-during-resume fixture can hang MSRV CI — done (replay before
     switch response)
- **Out of scope:** unchanged.
- **Required validation:** focused adapter tests + `qa:docs:roadmaps:status`.
- **PR URL:** pending push
- **Merge authorisation:** absent; do not merge

## Boundaries

- Do not widen god-file baselines to hide proof files. Do not merge.

## Important Context

- Touched papercuts now carry concrete Fix/Closed notes in `PAPERCUTS.md`.
- **Report to:** the operator.

## Suggested Next Move

Push the PR and await orchestrator review.

## Completion Protocol

### When the assigned runway is complete

1. Close finished papercuts. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.
