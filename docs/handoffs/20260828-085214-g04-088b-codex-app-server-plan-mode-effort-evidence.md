---
title: g04.088b Codex app-server Plan-mode effort evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-085214-g04-088b-codex-app-server-plan-mode-effort-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, codex]
---

## What This Thread Was Doing

The orchestrator compiled g04.088 as four independent evidence-only lanes.
This lane owns Codex app-server Plan-mode effort evidence: card 249, Research
246, its reserved log, and optional new Codex-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

`plan_mode_reasoning_effort` is a specific app-server control. It must stay
distinct from ordinary turn reasoning, Codex exec configuration, Fast, and
model defaults.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7aa197abe19ecf360bafc40f301bdcd64df7a24f`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-088b-codex-app-server-plan-mode-effort-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-088b-codex-app-server-plan-mode-effort-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-088b-codex-app-server-plan-mode-effort-evidence /Users/tom/Dev/worktrees/swallowtail-g04-088b-codex-app-server-plan-mode-effort-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/249-codex-app-server-plan-mode-effort-evidence.md`
- **Research:** `docs/research/246-codex-app-server-plan-mode-effort-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-088b-codex-app-server-plan-mode-effort-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, C, D; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Codex package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 013-016, 020, 029, 034, 037, 040, 047, 052; Research 201, 229, 234, 238, and 242; `docs/guides/codex-app-server-prepared-integration.md`
- **Tool/runtime restrictions:** official docs and exact tagged source plus secret-free local inspection only; no install/update, login, credential, account/catalogue inspection, provider prompt, paid work, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-codex`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 249 exactly; exact maintained app-server versions,
  selected model, Plan selection, `plan_mode_reasoning_effort`, value
  membership, precedence, request bytes, confirmation, persistence,
  restoration, failure, omission, Research 246, and the assigned log.
- **Allowed changed files:** assigned card, Research 246, assigned log, and new
  Codex-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Codex exec, ordinary turn reasoning,
  Fast, personality, verbosity, multi-agent, search, shared milestone/
  inventory/programme/triage/matrices/indexes/Next Task, currentness, live
  provider work, release, merge, rollover, or g04 closure.
- Do not promote exec argv or ordinary reasoning behavior onto Plan-mode effort.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Exact maintained app-server evidence recently used tags `0.147.0`,
  `0.148.0`, `0.149.0`, and `0.149.1`; verify the exact relevant set rather
  than assuming inheritance.
- A non-empty row needs Plan selected first, closed model/value membership,
  pre-effect request agreement, exact confirmation, lifecycle persistence/
  restoration, failure, and omission.
- An honest empty set is a valid completion.

## Completion Protocol

### Before you start

1. Read this handoff first. Run `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree. If the launcher supplied
   `main` or dirty state, stop and report it. Never clean/reset/stash user work.
3. If current context is unusable, inspect the named worktree. Only if needed,
   use `.agents.local.env` and `AGENTS_WORKTREE_CONTAINER_DIR` for a unique fallback.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.088, card 249, Research 246, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 249 and edit only allowed lane files.
- Freeze sources with exact tag, path, retrieval date, digest, and decisive evidence.
- Separate configured, Plan-selected, dispatched, accepted, effective,
  returned, persisted, and restored truth.
- Stop on missing authority, shared-file need, scope expansion, or live-only facts.

### When complete

1. Run the listed validation.
2. Complete card 249, Research 246, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.088, card 249, Research 246, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Trace `plan_mode_reasoning_effort` through exact config schema, Plan selection,
model membership, thread/turn request construction, response state, and cold
restoration before constructing the closed table.
