---
title: Papercuts wave 21 untracked Effigy skill closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Tom / papercuts orchestrator
created: 2026-08-30
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260830-223400-papercuts-wave21-skill-closeout.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Card 011 docs/Northstar validation copied the Effigy skill into untracked
`.agents/skills/effigy/`. PR 125 then committed that tree. Effigy PR 58
completed project-local skill sync.

You are the Swallowtail implementation worker. Prove a cheap docs
validation no longer dirties an untracked skill tree, then close the
copy. Stay off the live g05.006 watcher-repair PR.

## Why It Matters

A read-only validation round still looks like it can dirty planning
checkouts.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit at dispatch:** `a346baa985f6b71d6bc2831ccff329fe15e9c3ed`
- **Merged main commit:** `811db499c2b59e42f1a290923b64ceac1468b237` (PR 132).
- **Pushed main verification:** local `main` and `origin/main` both resolve to
  the merged commit.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker completed; PR merged by the
  orchestrator after review.
- **Worker branch:** `worker/papercuts-wave21-skill-closeout`
- **Worker worktree:** launcher first.
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Required sibling worktree links:** `none`
- **Completed work item:**
  1. Effigy validation materializes an untracked repo skill — close if
     `.agents/skills/effigy/` is tracked (PR 125) and a cheap
     `effigy qa:docs` (or the docs/Northstar check that originally
     copied the tree) leaves `git status --porcelain` without untracked
     skill files. Cite PR 125 and Effigy
     `f3057b9bb554f1a54b4c2d4cab2df27d5f6da202` (PR 58) if that is the
     binary you ran. If validation still writes untracked files beside
     the managed tree, gitignore only those leftovers; do not untrack
     the committed skill. Do not re-implement Effigy init.
- **Out of scope:** live-probe workspace cleanup (PR 126 /
  `live_watcher_probe.rs`); scoped-task executor; god-file splits; SPA
  notes; g05.006 watcher proof repair.
- **Canonical refs:** `PAPERCUTS.md`; `.agents/skills/effigy/`;
  Swallowtail PR 125; Effigy PR 58.
- **Required validation:** cheap docs/Northstar validation does not add
  untracked `.agents/skills/effigy/` paths. Do not require green
  `effigy qa` against unrelated doctor noise.
- **PR URL:** https://github.com/inflatable-cookie/swallowtail/pull/132
- **Review state:** reviewed with no blocking findings; PR 132 merged as
  `811db499`.
- **Merge authorisation:** granted by the operator; orchestrator merged.

## Boundaries

- Close the untracked-skill copy. Do not implement g05.006. Do not relaunch
  this handoff.

## Important Context

- Open PR 126 owns watcher lifecycle/probe files. Stay off those paths.
- **Report to:** the operator.

## Suggested Next Move

Do not relaunch this handoff. The lane is complete; continue with the
existing roadmap `Next Task`. Live-probe cleanup and scoped-task items remain
open.

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
   `docs/handoffs/20260830-223400-papercuts-wave21-skill-closeout.md`.
   Confirm `HEAD == origin/main`, ancestor
   `a346baa985f6b71d6bc2831ccff329fe15e9c3ed`, and that relative path in
   `HEAD`. Load
   `git show HEAD:docs/handoffs/20260830-223400-papercuts-wave21-skill-closeout.md`.
   If the absolute dispatch file differs, stop. The `HEAD` copy is
   canonical.
5. Required sibling list is `none`. Skip link setup.
6. Read `AGENTS.md` and `PAPERCUTS.md`.

### Assigned runway complete

The worker updated `PAPERCUTS.md` and pushed PR 132. The orchestrator review
found no blocking issue, and the operator authorised the merge. PR 132 is
merged as `811db499`.

### Review and merge path

Review completed; PR 132 merged with operator authorisation.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave live-probe cleanup and scoped-task items open. No further worker run is
needed for this handoff.
