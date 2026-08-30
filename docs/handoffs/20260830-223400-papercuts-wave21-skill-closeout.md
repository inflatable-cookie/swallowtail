---
title: Papercuts wave 21 untracked Effigy skill closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-30
updated: 2026-08-30
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
- **Planning base commit:** `a346baa985f6b71d6bc2831ccff329fe15e9c3ed`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave21-skill-closeout`
- **Worker worktree:** launcher first.
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`.
- **Required sibling worktree links:** `none`
- **Ready work items, in order:**
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
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Close the untracked-skill copy. Do not implement g05.006. Do not merge.

## Important Context

- Open PR 126 owns watcher lifecycle/probe files. Stay off those paths.
- **Report to:** the operator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight. After
the committed `HEAD` handoff checks out, skip sibling links (`none`),
then prove the skill tree stays tracked and clean.

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

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave live-probe cleanup and scoped-task items open.
