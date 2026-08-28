---
title: g04.088c Grok Build ACP web-search disable evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-085214-g04-088c-grok-build-acp-web-search-disable-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, grok]
---

## What This Thread Was Doing

The orchestrator compiled g04.088 as four independent evidence-only lanes.
This lane owns Grok Build ACP web-search-disable evidence: card 250, Research
247, its reserved log, and optional new Grok-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Grok advertises `--disable-web-search`, but parser acceptance does not prove
the ACP child cannot invoke provider search. The control also must not be
misstated as host network containment.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `7aa197abe19ecf360bafc40f301bdcd64df7a24f`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-088c-grok-build-acp-web-search-disable-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-088c-grok-build-acp-web-search-disable-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-088c-grok-build-acp-web-search-disable-evidence /Users/tom/Dev/worktrees/swallowtail-g04-088c-grok-build-acp-web-search-disable-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/250-grok-build-acp-web-search-disable-evidence.md`
- **Research:** `docs/research/247-grok-build-acp-web-search-disable-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-088c-grok-build-acp-web-search-disable-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Grok package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 023, 029, 033, 037, 040, 045, 047, 052; Research 130, 163, 204, and 219; `docs/guides/grok-build-prepared-integration.md`
- **Tool/runtime restrictions:** official docs and exact package/binary artifacts plus isolated secret-free local probes only; no install/update, login, credential, authenticate, account inspection, provider prompt, search execution, paid work, or host config mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-grok`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 250 exactly; exact `1.0.4..=1.0.5`, flag placement,
  parser, precedence, config resolution, provider-search registry/application,
  prompt-free confirmation, all owned ACP lifecycles, replacement, cleanup,
  omission, Research 247, and the assigned log.
- **Allowed changed files:** assigned card, Research 247, assigned log, and new
  Grok-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, host networking, sandboxing,
  reasoning, model selection, subagents, generic search enablement, shared
  milestone/inventory/programme/triage/matrices/indexes/Next Task,
  currentness, live provider work, release, merge, rollover, or g04 closure.
- Keep requested restriction, argv, parsing, configuration application,
  search-tool absence, provider behavior, host networking, and containment distinct.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Current exact spawn is `grok --no-auto-update agent stdio`; root-flag
  placement must be proved rather than assumed.
- Research 219 proved `--no-subagents` parser acceptance but stopped because
  effective suppression and spawn-path coverage did not close. Avoid repeating
  that inference failure for search.
- The current search matrix is `No`; omission must not create a search claim.
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
5. Read `AGENTS.md`, g04.088, card 250, Research 247, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 250 and edit only allowed lane files.
- Freeze official and exact artifact evidence with identity and digest.
- Use isolated homes for any exact binary parser/initialize probe.
- Stop on missing exact source/application, shared-file need, or provider work.

### When complete

1. Run the listed validation.
2. Complete card 250, Research 247, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.088, card 250, Research 247, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Freeze exact root-flag placement and precedence first, then trace it through
configuration resolution and every provider web-search construction path.
