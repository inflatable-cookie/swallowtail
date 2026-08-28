---
title: g04.089d Mistral Vibe headless agent-profile evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-103347-g04-089d-mistral-vibe-headless-agent-profile-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, mistral-vibe]
---

## What This Thread Was Doing

The orchestrator closed g04.088 and compiled g04.089 as four independent
evidence-only lanes. This lane owns Mistral Vibe headless agent-profile
evidence: card 255, Research 252, its reserved log, and optional new
Mistral-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Mistral Vibe exposes profiles beyond Swallowtail's fixed Plan profile. Those
profiles can alter tool, write, and approval authority, so only exact
non-bypass rows with closed application truth can be considered.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `3491683087bbd5c0670aba7fe28355d70a89ce9b`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-089d-mistral-vibe-headless-agent-profile-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-089d-mistral-vibe-headless-agent-profile-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-089d-mistral-vibe-headless-agent-profile-evidence /Users/tom/Dev/worktrees/swallowtail-g04-089d-mistral-vibe-headless-agent-profile-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/255-mistral-vibe-headless-agent-profile-evidence.md`
- **Research:** `docs/research/252-mistral-vibe-headless-agent-profile-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-089d-mistral-vibe-headless-agent-profile-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, C; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Mistral Vibe package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 013-014, 016, 023, 029, 037, 040, 047, 052; Research 150, 199, and 252; `docs/guides/mistral-vibe-headless-prepared-integration.md`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings: 334 warnings and 46 errors; graph index is stale; one generated-in-src warning
- **Tool/runtime restrictions:** official docs and exact tagged artifacts plus secret-free local inspection only; no install/update, login, credentials, provider prompts, paid work, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-mistral-vibe`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 255 exactly; exact `mistral-vibe.headless` `2.24.2`;
  `ask` and `accept-edits` against current fixed Plan; parser, precedence,
  resource/tool authority, prompt-mode application, terminal, lifecycle,
  cleanup, omission, Research 252, and the assigned log.
- **Allowed changed files:** assigned card, Research 252, assigned log, and new
  Mistral Vibe-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Mistral Vibe ACP, model selection,
  `auto-approve`, `--auto-approve`, `--yolo`, maximum-turn changes, shared
  milestone/inventory/programme/triage/matrices/indexes/Next Task,
  currentness, live provider work, release, merge, rollover, or g04 closure.
- Never infer host containment or portable resource access from an agent label.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Current argv fixes `--agent plan`, `--trust`, and bounded `--max-turns`.
- Programmatic default `auto-approve` remains refused and must not enter a row.
- A non-empty row must close profile membership, exact authority, application,
  terminal behavior, lifecycle, and omission before provider effects.
- An honest empty set is a valid completion.

## Completion Protocol

### Before you start

1. Read this handoff first. Run `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree. If the launcher supplied
   `main` or dirty state, stop and report it. Never clean/reset/stash user work.
3. If current context is unusable, inspect the named worktree. Only if needed,
   use `.agents.local.env` and `AGENTS_WORKTREE_CONTAINER_DIR` for a unique
   fallback. Never guess `/tmp`.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.089, card 255, Research 252, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 255 and edit only allowed lane files.
- Freeze sources with final URL/tag, retrieval date, digest, and decisive evidence.
- Separate requested, parsed, configured, dispatched, accepted, effective,
  returned, observed, and persisted truth.
- Stop on missing authority, shared-file need, scope expansion, or a result
  requiring provider work.

### When complete

1. Run the listed validation.
2. Complete card 255, Research 252, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.089, card 255, Research 252, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Trace `ask` and `accept-edits` from exact parser membership through authority,
prompt-mode application, and terminal behavior before constructing the closed
table.
