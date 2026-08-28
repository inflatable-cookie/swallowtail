---
title: g05.001 harness skill and watcher surface census worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-190722-g05-001-harness-skill-watcher-census.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, research, harness, watchers]
---

## What This Thread Was Doing

The orchestrator closed g04 after the full per-route feature inventory reached
83 closed dispositions, no active row, and two parked Bedrock rows. g05 now
starts with the only promoted open product family: truthful harness skill
visibility and dependable operation-scoped process observation.

This bounded worker owns the first evidence census only. Start from this file
without a copied transcript or second prompt. Do not spawn subagents; the
operator owns thread parallelism in their harness.

## Why It Matters

Consumer applications should not guess which skills a model can see or infer
background-process state from product labels. Before Swallowtail can shape any
portable surface, it needs exact route/version evidence that separates bundled
distribution facts, ambient configuration, session visibility, native task
control, process ownership, and turn-completion enforcement.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `dc9f8df5797e19338ece25c15a0a1ab731d64b6c`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Planning artifacts included at the base:** g05 generation, g05.001, cards 001-003, reserved Research 255, and the reserved lane log
- **Worker branch:** `worker/g05-001-harness-skill-watcher-census`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g05-001-harness-skill-watcher-census`
- **Worktree creation command:** `git worktree add -b worker/g05-001-harness-skill-watcher-census /Users/tom/Dev/worktrees/swallowtail-g05-001-harness-skill-watcher-census origin/main`
- **Active spec lane:** none; evidence precedes any spec or contract selection
- **Roadmap milestone:** `docs/roadmaps/g05/001-harness-skill-and-watcher-surface-inventory.md`
- **Ready card:** `docs/roadmaps/g05/batch-cards/001-production-harness-skill-and-watcher-surface-census.md`
- **Research:** `docs/research/255-production-harness-skill-and-watcher-surface-census.md`
- **Lane log:** `docs/logs/2026-08-28-g05-001-harness-skill-watcher-surface-census.md`
- **Allowed runway:** card 001 evidence only; one closed route matrix and one reviewable PR
- **Remaining card budget:** one card
- **Dispatch topology:** serial single lane; cards 002-003 are not dispatched
- **Parallel safety check:** no sibling worker; all shared planning and later decisions remain orchestrator-owned
- **Canonical refs:** `docs/architecture/system-architecture.md`, `docs/architecture/product-guardrails.md`, `docs/architecture/release-and-package-topology.md`, `docs/guides/provider-route-matrix.md`, `docs/guides/provider-solution-feature-matrix.csv`, `docs/guides/provider-solution-activity-matrix.csv`, `docs/triage/2026-08-27-harness-skill-discovery-and-process-watchers.md`; Contracts 013, 017, 023, 029, 034, 041, 044, 047, and 052
- **Inherited doctor baseline:** `scan.god-files` reports 381 findings: 335 warnings and 46 errors; graph index is stale; one generated-in-src warning
- **Model capability profile:** bounded research worker, medium reasoning
- **Tool/runtime restrictions:** official documentation, exact source/distribution manifests, existing fixtures, prompt-free help/list surfaces, and secret-free local inspection only; no model prompt, credentials, paid work, install/update, login, account inspection, recursive home/project scan, skill injection, process mutation, or host configuration mutation
- **Required validation:** `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / `worker/g05-001-harness-skill-watcher-census`
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 001 exactly; enumerate every production harness route;
  freeze exact route/version official or repository evidence for skill listing,
  provenance, model/session visibility, native background task identity,
  status, wait, output, stop, terminal, join, cancellation, deadline, and
  turn-completion truth; promote Research 255.
- **Allowed changed files:** assigned card, Research 255, assigned lane log, and
  the card 001 section move in `docs/roadmaps/g05/batch-cards/README.md`.
- **Out of scope:** production code or public API, architecture/contracts/specs,
  skill injection, watcher tools, process registries, process start/stop,
  arbitrary PID authority, consumer UI, raw log streaming, route currentness,
  new routes, parked Bedrock work, deferred routes, cards 002-003, shared
  milestone/front doors/triage, release, merge, or g05 closeout.
- Do not infer model visibility from distribution membership, files on disk,
  model prose, tool advertisement, or a product name.
- Do not infer a controllable watcher from provider-observed command activity or
  a native task identifier.
- Do not inspect arbitrary personal, project, plugin, or enterprise state to
  manufacture a complete row. Record the evidence gate instead.
- Work only in the selected clean worker worktree. Do not merge the PR.

## Important Context

- `docs/guides/provider-route-matrix.md` is the production route inventory.
  Include harness-owned child processes and attached harness servers; explain
  any exclusion rather than silently narrowing the set.
- The exact qualified route/version is authoritative. Current stable or host
  binaries may be contrast evidence only and must not widen a claim.
- Skill truth needs separate columns for distribution membership, ambient
  configuration, session/model visibility, provenance, mutation, auth, and
  freshness.
- Watcher truth needs separate columns for identity, start, status, wait,
  output, stop, terminal, descendants/join, cancellation/deadline, and whether
  successful turn completion is actually gated.
- Claude/T3 Code is a research lead only. Separate Claude-emitted truth from
  consumer-owned T3 Code projection.
- An unavailable, unsafe, or honest empty surface is a valid disposition.
- Report once the complete matrix and decisive sources are ready, or
  immediately on a stop condition. Avoid route-by-route chat churn.

## Suggested Next Move

Start from the provider route matrix and derive the exact production harness
set. Then reuse existing fixtures and qualified-version evidence before looking
for official prompt-free listing or task-control surfaces. Build the matrix
schema first so absence, unsafe evidence, and ownership differences stay
comparable across routes.

## Completion Protocol

### Before you start

1. Read this handoff first. Run `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree supplied by the launcher,
   even if its path or branch differs from the placeholders above. Record the
   actual path/branch and do not create a second worktree for that reason.
3. If the launcher supplied `main` or dirty state, stop and report it. Never
   clean, reset, stash over, or discard user work. Only when the current context
   is otherwise unusable should you inspect the named worktree. If that is also
   unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if it is absent. Never
   use `/tmp`, `TMPDIR`, or a guessed path.
4. From the selected worktree, fetch origin. Confirm `HEAD == origin/main`,
   `git merge-base --is-ancestor dc9f8df5797e19338ece25c15a0a1ab731d64b6c HEAD`
   succeeds, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g05.001, card 001, Research 255, the lane log, the promoted
   triage note, provider matrices, and named architecture/contracts.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline; do not repair it in this lane.

### While you work

- Execute only card 001 and edit only the allowed lane files.
- Freeze each decisive external source with exact URL/tag/version, retrieval
  date, digest or immutable identity, and the claim it supports.
- Separate requested, present, configured, dispatched, visible, controllable,
  observed, persisted, and consumer-projected truth where applicable.
- Stop on missing authority, shared-file need, scope expansion, privacy risk, or
  evidence requiring provider work or ambient mutation.
- Report one meaningful completed census through the operator; do not start a
  nested worker or continue into card 002.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`.
2. Complete card 001, Research 255, the assigned log, and move only card 001
   from Ready to Completed in the g05 batch-card index.
3. Push the worker branch and open a reviewable PR against current pushed
   `main`.
4. Link g05.001, card 001, Research 255, the lane log, decisive sources,
   validation, and unresolved evidence gates in the PR body.
5. Report the PR URL. Do not merge, edit cards 002-003, or choose a watcher
   architecture.

### Review and merge path

The orchestrator reviews the PR independently and records its verdict on the
PR. If changes are requested, make only those changes on this branch, push, and
report through the operator. The operator must explicitly authorise merge.

- **Closeout refs:** card 001, Research 255, assigned log, and its batch-card
  index move are worker-owned. g05.001, cards 002-003, triage disposition,
  generation/front-door state, and sole Next Task remain orchestrator-owned.

### Handoff closeout

Leave the assigned card, Research record, lane log, and card index honest. If
blocked, record the named blocker and stop rather than making the census look
complete.
