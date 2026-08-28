---
title: g04.090a Goose ACP mode evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-180615-g04-090a-goose-acp-mode-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, goose]
---

## What This Thread Was Doing

The orchestrator closed g04.089, audited the full feature remainder, and
compiled g04.090 as the final two bounded qualification questions. This lane
owns Goose ACP mode evidence: card 256, Research 253, its reserved log, and
optional new Goose-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Goose `1.46.0` advertises exact ACP modes and a current mode, but Swallowtail
does not select one. Evidence must decide whether any safe adapter-local mode
has closed request, application, confirmation, failure, and authority truth.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `3d5481590d9c4c7eb087b283856892aedb6ac406`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Planning artifacts included at the base:** g04.089 closeout, remainder audit, g04.090, card 256, Research 253, and reserved lane log
- **Worker branch:** `worker/g04-090a-goose-acp-mode-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-090a-goose-acp-mode-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-090a-goose-acp-mode-evidence /Users/tom/Dev/worktrees/swallowtail-g04-090a-goose-acp-mode-evidence origin/main`
- **Active programme:** `docs/roadmaps/g04/per-route-feature-completion.md`
- **Roadmap milestone:** `docs/roadmaps/g04/090-residual-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/256-goose-acp-mode-evidence.md`
- **Research:** `docs/research/253-goose-acp-mode-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-090a-goose-acp-mode-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lane B; serial integration Goose then Kiro
- **Parallel safety:** unique card, Research, log, and Goose package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 013-016, 020, 023, 029, 034, 037, 040, 047, 052; Research 148, 250, and 253; `docs/guides/goose-acp-prepared-integration.md`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings: 334 warnings and 46 errors; graph index is stale; one generated-in-src warning
- **Model capability profile:** bounded evidence worker, medium reasoning
- **Tool/runtime restrictions:** official docs and exact tagged source/artifacts plus secret-free local inspection only; no install/update, login, credentials, provider prompts, paid work, auto-approval widening, extension setup, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-goose`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 256 exactly; exact `goose.acp` `1.46.0`; mode membership,
  standard or provider-specific selection method, application, selected-value
  confirmation, failure, permission/resource authority, lifecycle, cleanup,
  omission, Research 253, and the assigned log.
- **Allowed changed files:** assigned card, Research 253, assigned log, and new
  Goose-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Goose builtins, MCP management,
  extension installation, provider/model setup, `goose serve`, shared
  milestone/inventory/programme/triage/matrices/indexes/Next Task, currentness,
  live provider work, release, merge, rollover, or g04 closure.
- Do not map `chat`, `approve`, or another Goose label to portable
  `HarnessMode::Plan` without exact semantic equivalence.
- Do not admit `auto`, durable approval, or any row that silently widens
  permission authority.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Current argv is exactly `goose acp`; the host-owned mode remains unselected.
- `session/new` evidence names `auto|approve|smart_approve|chat` and returns
  `currentModeId`, but advertisement alone does not prove a selectable feature.
- Research 250 is builtin evidence only; it explicitly leaves Goose mode out of
  scope and must not be reused as a mode disposition.
- A non-empty row needs exact membership, pre-prompt selection, application,
  confirmation, safe authority, fail-closed unknown handling, and omission.
- An honest empty set is a valid completion.
- Report after the complete evidence table and source freeze, or immediately on
  a stop condition.

## Suggested Next Move

Start from exact Goose `1.46.0` ACP session source. Trace mode advertisement and
the supported selection request through stored state, permission behavior,
updates, invalid values, and the first prompt boundary.

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
   `git merge-base --is-ancestor 3d5481590d9c4c7eb087b283856892aedb6ac406 HEAD`
   succeeds, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.090, card 256, Research 253, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 256 and edit only allowed lane files.
- Freeze sources with final URL/tag, retrieval date, digest, and decisive evidence.
- Separate requested, parsed, configured, dispatched, accepted, effective,
  returned, observed, and persisted truth.
- Stop on missing authority, shared-file need, scope expansion, or a result
  requiring provider work.
- Report meaningful progress through the operator; do not start a nested worker.

### When the assigned runway is complete

1. Run the listed validation.
2. Complete card 256, Research 253, and the assigned log honestly.
3. Push the worker branch and open a reviewable PR against current pushed `main`.
4. Link g04.090, card 256, Research 253, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the PR independently and records its verdict on the PR.
If changes are requested, make only those changes on this branch, push, and
report through the operator. The operator must explicitly authorise merge.

- **Closeout refs:** card 256, Research 253, assigned log, g04.090, inventory,
  programme, indexes, and sole Next Task remain orchestrator-owned.

### Handoff closeout

Leave the assigned card, Research record, and lane log honest. If blocked,
record the named blocker and stop rather than making the lane look complete.
