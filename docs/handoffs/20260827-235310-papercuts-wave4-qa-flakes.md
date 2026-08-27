---
title: Papercuts wave 4 QA flakes worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
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
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave4-qa-flakes`
- **Worker worktree:** launcher first.
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Ready work items, in order:**
  1. Docs index QA misses roadmap-status drift
  2. Timing-sensitive deadline fixtures make unrelated PR heads red
  3. DeepSeek stream-cancellation test flakes as ProviderFailed
  4. Pi replay-during-resume fixture can hang MSRV CI — only if a
     bounded timeout/fail-closed fix is obvious; otherwise report and
     leave open
- **Out of scope:** xAI/Copilot/Anthropic/Codex SPA corpus notes;
  god-file baseline raises; Antigravity probes; host `agy` auto-update;
  zsh special vars; evidence-download cwd; parallel duplicate cards;
  launcher stale worktree registrations.
- **Canonical refs:** `PAPERCUTS.md`; Northstar roadmap/batch-card index
  QA; Ollama/Codex deadline tests; DeepSeek driver cancellation test.
- **Required validation:** docs-index QA fails when generation-index or
  batch-card Status disagrees with card frontmatter; deadline tests are
  no longer wall-clock flakes on unrelated heads (fake clock or
  looser-but-honest bound); DeepSeek cancellation does not flake
  ProviderFailed on the same SHA that already passed. Focused tests.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Do not widen god-file baselines to hide proof files. Do not merge.

## Important Context

- `PAPERCUTS.md` is missing `Possible fix` on most entries; complete
  only the ones you touch.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then add the roadmap-status
docs check.

## Completion Protocol

### Before you start

1. Read this handoff. Run the four git identity commands.
2. Accept a clean dedicated non-`main` registered worktree.
3. Confirm `HEAD == origin/main` and ancestor
   `437af7f6ed8680f4e10e8d12705120ec48a4d040`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Close finished papercuts. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If a flake cannot be made deterministic without a product change, leave
it open and say so.
