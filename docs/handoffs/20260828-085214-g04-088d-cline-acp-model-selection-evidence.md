---
title: g04.088d Cline ACP model-selection evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-085214-g04-088d-cline-acp-model-selection-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, cline]
---

## What This Thread Was Doing

The orchestrator compiled g04.088 as four independent evidence-only lanes.
This lane owns Cline ACP model-selection evidence: card 251, Research 248, its
reserved log, and optional new Cline-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Cline headless parses `-m/--model`, but its provider identity, membership, and
durable writes did not close. ACP needs an independent selection and
confirmation audit; sibling-route evidence cannot be promoted.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7aa197abe19ecf360bafc40f301bdcd64df7a24f`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-088d-cline-acp-model-selection-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-088d-cline-acp-model-selection-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-088d-cline-acp-model-selection-evidence /Users/tom/Dev/worktrees/swallowtail-g04-088d-cline-acp-model-selection-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/251-cline-acp-model-selection-evidence.md`
- **Research:** `docs/research/248-cline-acp-model-selection-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-088d-cline-acp-model-selection-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, C; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Cline package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 008, 020, 023, 029, 033, 034, 037, 040, 047, 052; Research 147, 190, 220, 221, and 240; `docs/guides/cline-prepared-integration.md`
- **Tool/runtime restrictions:** official docs and exact tagged source/schemas plus secret-free local protocol inspection only; no install/update, login, credential, account/catalogue inspection, provider prompt, paid work, or ambient settings mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-cline`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 251 exactly; exact `cline.acp` `3.0.55`, provider/model
  membership, route agreement, picker/config option, pre-prompt selection,
  response/update confirmation, settings reads/writes, lifecycle, replacement,
  failure, omission, Research 248, and the assigned log.
- **Allowed changed files:** assigned card, Research 248, assigned log, and new
  Cline-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Cline headless, caller provider
  selection, thinking, Plan delivery, Act/Yolo/Zen, auto-approve, shared
  milestone/inventory/programme/triage/matrices/indexes/Next Task,
  currentness, live provider work, release, merge, rollover, or g04 closure.
- Do not promote root `-m/--model`, headless request echoes, or headless durable-
  write conclusions onto ACP without exact ACP evidence.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Exact Cline ACP `3.0.55` already supports new-session Plan through
  `session/set_config_option` with exact selected-value confirmation. That
  proves the protocol pattern, not model support.
- Research 221 closed headless model selection on ambient provider identity,
  open membership, and unavoidable shared settings writes.
- A non-empty ACP row needs independent membership, route agreement, pre-
  prompt selection, exact confirmation, session-private authority, lifecycle,
  failure, and omission.
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
5. Read `AGENTS.md`, g04.088, card 251, Research 248, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 251 and edit only allowed lane files.
- Freeze exact source/schema evidence with tag, path, identity, and digest.
- Separate advertisement, selection, application, confirmation, persistence,
  restoration, and observation.
- Stop on missing route authority, shared-file need, durable mutation, or live-only facts.

### When complete

1. Run the listed validation.
2. Complete card 251, Research 248, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.088, card 251, Research 248, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Trace exact ACP initialize and `session/new` model surfaces first. Then follow
any picker/config method through selected-value confirmation and durable state.
