---
title: Papercuts wave 23 route-matrix Python bytecode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / local Swallowtail orchestrator
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260831-195246-papercuts-wave23-route-matrix-bytecode.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Papercuts wave 23 closed the OpenAI test-target mismatch through PR 136. The
next serial item is route-matrix validation writing Python bytecode beneath
`scripts/provider_route_matrix/` on hosts without an external cache prefix.

This is one bounded repository-maintenance lane. It may start only after the
OpenAI lane's merge closeout is committed and pushed. Do not overlap another
worker that edits `PAPERCUTS.md` or the papercuts closeout log.

## Why It Matters

Documentation and route validation should leave a clean source tree on every
supported host. Host-local Python cache redirection currently hides the defect
on this Mac; repository selectors must not depend on that ambient setting.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `af339fb65d5a249bddcf0f58abae95953d4d465a`
- **Pushed main verification:** the orchestrator will commit this handoff and
  closeout batch, push it, and verify local `HEAD == origin/main` before launch.
- **Planning checkout:** clean before the closeout/handoff batch.
- **Queue state:** reserved; no worker, workspace, worktree, or branch exists.
- **Worker mode:** implementation worker only after orchestrator dispatch.
- **Worker branch:** `worker/papercuts-wave23-route-matrix-bytecode`
- **Worker worktree:** launcher first; manual fallback
  `/Users/tom/Dev/worktrees/papercuts-wave23-route-matrix-bytecode`.
- **Required sibling worktree links:** none.
- **Roadmap milestone:** none; keep the g05 Next Task on card 024.
- **Allowed runway:** reproduce source-tree bytecode under a deliberately empty
  Python cache-prefix environment; diagnose which repository selectors import
  `provider_route_matrix`; make those selectors prevent source-tree bytecode;
  close only this papercut; validate; push one PR.
- **Remaining budget:** one papercut, one PR.
- **Dispatch topology:** serial. No later papercut is reserved by this handoff.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`; `effigy.toml`;
  `scripts/check-consumer-front-door.py`;
  `scripts/check-integration-guide-coverage.py`;
  `scripts/check-provider-route-matrix.sh`; `scripts/provider_route_matrix/`.
- **Review oracle:** with ambient Python cache redirection neutralized, each
  affected Effigy selector passes with identical validation meaning and leaves
  no `__pycache__` or `.pyc` beneath `scripts/provider_route_matrix/`. The fix
  prevents generation; a new ignore rule alone is not acceptance.
- **Model capability profile:** bounded mechanical Python/task maintenance.
- **Tool/runtime restrictions:** no provider contact, live probe, roadmap
  movement, route-data change, workflow edit, global Python configuration, or
  unrelated papercut repair.
- **Required validation:** controlled reproduction; `effigy qa:consumer-docs`;
  `effigy qa:guides`; `effigy qa:routes`; `effigy qa:docs`;
  `effigy qa:northstar`; `git diff --check`; clean generated-file audit after
  every affected selector under a neutral cache-prefix environment.
- **PR base/head:** current pushed `main` / worker branch above.
- **PR URL:** pending.
- **Review state:** awaiting dispatch, then exact-head orchestrator review.
- **Merge authorisation:** the local orchestrator may merge after a clean
  exact-head review and settled checks under standing operator authority.

## Boundaries

- **In scope:** prevent affected repository selectors from materializing Python
  bytecode under `scripts/provider_route_matrix/`; close the exact papercut.
- **Out of scope:** route inventory or matrix content, Python refactors,
  repository-wide ignore policy, GitHub workflows, g05 cards, release evidence,
  other generated files, and unrelated papercuts.
- Prefer the smallest repository-owned execution guard. Do not rely on this
  Mac's `sys.pycache_prefix`, a user environment variable, or manual cleanup.
- Do not weaken a selector, remove an import, skip validation, or redirect
  bytecode into another tracked source directory.
- Stop if preventing bytecode requires a workflow change, broad task-system
  policy, route-data change, or external Effigy modification.
- Work only in the clean worker worktree selected by the completion protocol.
- Do not merge. Review and merge belong to the orchestrator.

## Important Context

- This Mac reports `sys.pycache_prefix` under `~/Library/Caches`, so an ordinary
  run may look clean. Neutralize that ambient benefit when reproducing and
  validating.
- `qa:docs` reaches the route inventory through `qa:consumer-docs` and
  `qa:guides`; `qa:routes` invokes the route-matrix Python entry points through
  a shell wrapper.
- The intended invariant covers each affected selector directly, not only the
  aggregate `qa:docs` path.
- Report after reproduction and diagnosis, then after final validation and PR
  creation. Report to the local Swallowtail orchestrator.

## Suggested Next Move

Do not launch from this reservation alone. After orchestrator dispatch, run the
worker preflight, reproduce under a neutral cache-prefix environment, implement
the smallest prevention guard, and validate the complete bounded tranche.

## Completion Protocol

### Before you start

1. Worker mode activates only when the orchestrator dispatches this committed
   handoff. Before broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree. Record its actual
   root and branch; do not create another because names differ.
3. If the current context is `main`, dirty, unregistered, or unusable, inspect
   the named fallback. If needed, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/Dev/worktrees`, and create a unique
   worktree there from pushed `origin/main`. Never use `/tmp`; never clean,
   reset, stash over, or discard dirty state.
4. Fetch origin with non-interactive SSH. Confirm selected `HEAD ==
   origin/main`, confirm planning base `af339fb65d5a249bddcf0f58abae95953d4d465a`
   is an ancestor, and load this tracked handoff from `HEAD`. Stop if the
   absolute and tracked files differ.
5. Required sibling list is none.
6. Read `AGENTS.md`, `PAPERCUTS.md`, `effigy.toml`, the three named validation
   entry points, and the route-matrix package. Use Effigy for owned validation.

### While you work

- Reproduce before editing with ambient cache redirection neutralized. Remove
  only the generated reproduction cache inside the worker worktree before the
  repair proof.
- Diagnose the exact selector boundary. The handoff does not preselect whether
  the narrow guard belongs in `effigy.toml` or a named entry-point wrapper.
- Preserve route counts, messages, failure behavior, and all imported checks.
- Mark the route-matrix bytecode entry closed only after each selector proves
  the no-bytecode invariant.
- Stop on scope expansion or a validation result that changes the plan.

### When the assigned runway is complete

1. Run every required selector with host cache redirection neutralized and
   assert after each that no `__pycache__` directory or `.pyc` exists beneath
   `scripts/provider_route_matrix/`.
2. Falsify the repair: show the pre-edit reproduction was capable of writing
   local bytecode; show the repository-owned selector guard, not ambient host
   state or ignore policy, prevents it; confirm route validation still fails on
   a bounded temporary invalid input or equivalent existing negative proof.
3. Update only the assigned `PAPERCUTS.md` entry. Do not write the merge
   closeout log; the orchestrator owns post-merge reconciliation.
4. Push the worker branch and open one PR against current pushed `main`.
5. Report exact head, changed files, reproduction, selector outputs, generated
   file audits, checks, and PR URL. Do not merge.

### Review and merge path

The local orchestrator independently reviews the exact head, records its
verdict on GitHub, waits for settled checks, and may merge under standing
operator authority. Requested changes return to the same worker branch.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; post-merge papercuts log.

### Handoff closeout

After merge, the orchestrator marks this handoff merged, writes the serial
closeout log, and leaves the g05 roadmap pointer unchanged.
