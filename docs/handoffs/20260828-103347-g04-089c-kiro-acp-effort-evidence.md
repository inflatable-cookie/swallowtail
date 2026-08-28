---
title: g04.089c Kiro ACP effort evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260828-103347-g04-089c-kiro-acp-effort-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, kiro]
---

## What This Thread Was Doing

The orchestrator closed g04.088 and compiled g04.089 as four independent
evidence-only lanes. This lane owns Kiro ACP effort evidence: card 254,
Research 251, its reserved log, and optional new Kiro-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Kiro documents `--effort`, but the production route is ACP and advertises no
model-setting method. Exact evidence must locate where the selection applies
and how it is confirmed before any feature can ship.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `3491683087bbd5c0670aba7fe28355d70a89ce9b`
- **Pushed main verification:** planning base equalled `origin/main` before the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:** `worker/g04-089c-kiro-acp-effort-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-089c-kiro-acp-effort-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-089c-kiro-acp-effort-evidence /Users/tom/Dev/worktrees/swallowtail-g04-089c-kiro-acp-effort-evidence origin/main`
- **Ready card:** `docs/roadmaps/g04/batch-cards/254-kiro-acp-effort-evidence.md`
- **Research:** `docs/research/251-kiro-acp-effort-evidence.md`
- **Lane log:** `docs/logs/2026-08-28-g04-089c-kiro-acp-effort-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set; one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration A, B, C, D
- **Parallel safety:** unique card, Research, log, and Kiro package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts 006, 014-016, 020, 023, 029, 034, 037, 040, 047, 052; Research 153, 156, and 251; `docs/guides/kiro-acp-prepared-integration.md`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings: 334 warnings and 46 errors; graph index is stale; one generated-in-src warning
- **Tool/runtime restrictions:** official docs and exact package/source artifacts plus secret-free local inspection only; no install/update, login, credentials, account/catalogue inspection, provider prompt, paid work, or host mutation
- **Required validation:** `effigy validate:focused swallowtail-adapter-kiro`, `effigy qa:northstar`, `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 254 exactly; exact `kiro.acp` `2.18.1`; official effort
  values; parser, model dependence, ACP spawn/session application,
  confirmation, failure, lifecycle, cleanup, omission, Research 251, and the
  assigned log.
- **Allowed changed files:** assigned card, Research 251, assigned log, and new
  Kiro-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Kiro headless, agent profile, cloud
  sessions, trust-all tools, model routing, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not infer ACP support from headless/interactive docs or from unsupported
  `session/set_model`.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Production argv is `kiro-cli acp` with no effort argument.
- The deferred Kiro headless route does not own this ACP feature row.
- A non-empty row must close model/value membership, exact application,
  confirmation, failure, lifecycle, and omission before provider effects.
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
5. Read `AGENTS.md`, g04.089, card 254, Research 251, the lane log, and named refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`.

### While you work

- Execute only card 254 and edit only allowed lane files.
- Freeze sources with final URL/tag, retrieval date, digest, and decisive evidence.
- Separate requested, parsed, configured, dispatched, accepted, effective,
  returned, observed, and persisted truth.
- Stop on missing authority, shared-file need, scope expansion, or a result
  requiring provider work.

### When complete

1. Run the listed validation.
2. Complete card 254, Research 251, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.089, card 254, Research 251, evidence, validation, and unresolved items.
5. Report the PR URL. Do not merge or begin production binding.

## Suggested Next Move

Trace `--effort` through exact CLI parsing and ACP startup/session code before
deciding whether any value reaches a confirmable production-route seam.
