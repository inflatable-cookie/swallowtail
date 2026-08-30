---
title: Papercuts wave 19 census grammar and gated status worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-194600-papercuts-wave19-census-gated.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

`qa:docs:roadmaps:status` rejected truthful g05 census wording until it
used the exact phrases `N completed milestones`, `honest evidence
stops`, and `ready milestones` with numeric counts. That grammar lives
only in `scripts/check-roadmap-status-drift.py`.

Separately, card 010 once used `Status: gated`, which is not an
accepted bucket. Card 010 is now `Status: complete`. Document the
allowed buckets so the next card does not repeat `gated` as a status.

You are the Swallowtail implementation worker. Document the census
grammar and close the gated copy. Do not touch the live card 011 lane.

## Why It Matters

Ordinary generation-index edits fail through trial and error, and a
future card will reuse `gated` as a status.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `ac6997146efc4e4ab34068ba4e1eac522052a1ed`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave19-census-gated`
- **Worker worktree:** launcher first.
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Required sibling worktree links:** `none`
- **Ready work items, in order:**
  1. Roadmap status census requires undocumented exact prose — document
     the accepted census grammar next to
     `scripts/check-roadmap-status-drift.py` and in
     `docs/roadmaps/README.md` (or a short sibling note it already
     links). Name the live regexes: `N completed milestones`, `honest
     evidence stops at …`, `ready milestones at 003` / `one ready
     milestone at`. Prefer document over rewriting the parser. Do
     **not** edit `docs/roadmaps/generation-index.md` (card 011 closeout
     may change those counts).
  2. Batch cards use `gated` as a status — card 010 is `Status:
     complete`. Close that copy. Document allowed buckets
     (`planned`, `ready`, `blocked`, `stopped`, `complete` and the
     complete aliases the checker already admits) and that a gate is
     `Status: planned; gated behind …` or `ready; …`, not `Status:
     gated`. Swallowtail-local docs only. Do not edit the Northstar
     batch-card template. Do not edit card 011.
- **Out of scope:** card 011 live acceptance; scoped-task watcher host;
  god-file splits; SPA research notes; launcher worktree cleanup;
  GitHub workflows; release mutations.
- **Canonical refs:** `PAPERCUTS.md`;
  `scripts/check-roadmap-status-drift.py`; `docs/roadmaps/README.md`;
  `docs/roadmaps/g05/batch-cards/010-claude-code-watcher-bridge.md`.
- **Required validation:** the documented phrases match the live
  regexes. `effigy qa:docs:roadmaps:status` still passes without
  rewriting generation-index. Card 011 files are unchanged.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Document census grammar and close gated. Do not implement card 011.
  Do not merge.

## Important Context

- Live worker handoff:
  `docs/handoffs/20260830-193131-g05-003-card-011-watcher-acceptance.md`.
  Stay off that runway.
- **Report to:** the operator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight. After
the committed `HEAD` handoff checks out, skip sibling links (`none`),
then document the grammar and close gated.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it. Record the actual path/branch.
3. If the launcher supplied a dirty or `main` worktree, stop and report
   it. Fallback container is
   `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`. Never use
   `/tmp`.
4. From the selected worktree, record the repository-relative path
   `docs/handoffs/20260830-194600-papercuts-wave19-census-gated.md`.
   Confirm `HEAD == origin/main`, ancestor
   `ac6997146efc4e4ab34068ba4e1eac522052a1ed`, and that relative path in
   `HEAD`. Load
   `git show HEAD:docs/handoffs/20260830-194600-papercuts-wave19-census-gated.md`.
   If the absolute dispatch file differs, stop. The `HEAD` copy is
   canonical.
5. Required sibling list is `none`. Skip link setup.
6. Read `AGENTS.md` and `PAPERCUTS.md`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave scoped-task, god-file, and SPA notes open.
