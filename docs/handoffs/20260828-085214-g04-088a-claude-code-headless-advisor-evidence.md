---
title: g04.088a Claude Code headless advisor evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-085214-g04-088a-claude-code-headless-advisor-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, claude]
---

## What This Thread Was Doing

The orchestrator closed g04.087 and compiled g04.088 as four independent
evidence-only lanes. This lane owns Claude Code headless advisor evidence:
card 248, Research 245, its reserved log, and optional new Claude-local frozen
evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Claude advertises `--advisor`, but an extra model or request can change model
authority, entitlement, spend, and terminal behavior. Swallowtail cannot bind
it from a flag name alone.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7aa197abe19ecf360bafc40f301bdcd64df7a24f`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-088a-claude-code-headless-advisor-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-088a-claude-code-headless-advisor-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-088a-claude-code-headless-advisor-evidence /Users/tom/Dev/worktrees/swallowtail-g04-088a-claude-code-headless-advisor-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/248-claude-code-headless-advisor-evidence.md`
- **Research:** `docs/research/245-claude-code-headless-advisor-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-088a-claude-code-headless-advisor-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes B, C, D; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Claude package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 014, 016, 023, 029, 037, 040, 047, 052; Research 202, 226, 233, 237, and 241; `docs/guides/claude-agent-prepared-integration.md`
- **Tool/runtime restrictions:** official docs and exact package artifacts plus secret-free local inspection only; no install/update, login, credential, account inspection, provider prompt, paid work, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-claude-agent`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 248 exactly; qualified versions, local-subscription
  access, advisor model/value, parser, precedence, entitlement, model
  resolution, request/spend, application, result, lifecycle, cleanup,
  omission, Research 245, and the assigned log.
- **Allowed changed files:** assigned card, Research 245, assigned log, and new
  Claude-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, response-only, ACP, API-key route
  creation, permission modes, Fast, compaction, spend cap, maximum turns,
  shared milestone/inventory/programme/triage/matrices/indexes/Next Task,
  currentness, live provider work, release, merge, rollover, or g04 closure.
- Do not flatten advisor into main-model selection, subagents, or portable
  vocabulary. Keep subscription allowance, local estimates, and provider-
  billed work distinct.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The exact evidence set is every published point in qualified
  `2.1.220..=2.1.241`.
- The selected route uses local Claude subscription state, fixed Plan tools,
  and no session persistence.
- A non-empty row needs exact advisor membership, access/entitlement,
  operation-private precedence, application, any extra request/spend,
  terminal/cleanup, and unchanged omission.
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
5. Read `AGENTS.md`, g04.088, card 248, Research 245, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 248 and edit only allowed lane files.
- Freeze sources with final URL/tag, retrieval date, digest, and decisive evidence.
- Separate requested, parsed, resolved, dispatched, accepted, effective,
  returned, observed, and billed truth.
- Stop on missing authority, shared-file need, scope expansion, or a result
  requiring provider work.

### When complete

1. Run the listed validation.
2. Complete card 248, Research 245, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.088, card 248, Research 245, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Trace `--advisor` from exact package declaration through model resolution,
access/entitlement, request construction, accounting, and terminal behavior
before constructing the closed table.
