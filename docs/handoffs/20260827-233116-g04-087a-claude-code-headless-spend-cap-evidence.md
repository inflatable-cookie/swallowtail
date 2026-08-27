---
title: g04.087a Claude Code headless spend-cap evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-233116-g04-087a-claude-code-headless-spend-cap-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, claude]
---

## What This Thread Was Doing

The orchestrator compiled g04.087 as four independent evidence-only lanes.
This lane owns Claude Code headless spend-cap evidence: card 244, Research 241,
its reserved log, and optional new Claude-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Claude advertises `--max-budget-usd`, but the selected Swallowtail headless
route is local-subscription, read-only, and explicitly not API-key billing.
Exact units, enforcement, result, billing source, and access compatibility must
close before a spend-cap claim can exist.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `d00cea2590f8926cb43bccfbad607719cd58d331`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.087, card 244, Research 241 reservation, lane log,
  indexes, and sole Next Task
- **Worker branch:** `worker/g04-087a-claude-code-headless-spend-cap-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-087a-claude-code-headless-spend-cap-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-087a-claude-code-headless-spend-cap-evidence /Users/tom/Dev/worktrees/swallowtail-g04-087a-claude-code-headless-spend-cap-evidence origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable,
  inspect the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.087 lane A
- **Roadmap:** `docs/roadmaps/g04/087-fourth-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/244-claude-code-headless-spend-cap-evidence.md`
- **Research:** `docs/research/241-claude-code-headless-spend-cap-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-087a-claude-code-headless-spend-cap-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes B, C, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Claude package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  006, 014, 016, 023, 029, 037, 040, 047, 052; Research 202, 226, 233, and
  237; `docs/guides/claude-agent-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and route-local audit
- **Tool/runtime restrictions:** official docs and exact package artifacts plus
  secret-free local inspection only; no install/update, login, credential,
  account inspection, provider prompt, paid work, or host mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-claude-agent`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 244 exactly; exact qualified versions, local-subscription
  access, positive values, units, parser, precedence, enforcement, cost source,
  terminal/exit, cleanup, omission, Research 241, and the assigned log.
- **Allowed changed files:** assigned card, Research 241, assigned log, and new
  Claude-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, response-only, ACP, API-key route
  creation, advisor, permission modes, Fast, autocompaction, max turns, shared
  milestone/inventory/programme/triage/matrices/indexes/Next Task, currentness,
  live provider work, release, merge, rollover, or g04 closure.
- Do not equate subscription allowance, local estimates, and provider-billed
  API USD. Do not use a paid prompt to prove enforcement.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The exact evidence set is every published point in qualified
  `2.1.220..=2.1.241`; Research 226 already freezes that census.
- The selected route uses local Claude subscription state and rejects API-key
  billing as a route choice. Evidence may therefore close an honest empty set.
- A non-empty row needs exact access compatibility, closed units/domain,
  operation-private precedence, native enforcement, limit terminal/exit, and
  unchanged omission.
- Report after Research 241 and card 244 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Trace `--max-budget-usd` from exact package parser
through cost accounting, loop guard, result schema, exit, and access/billing
branches before constructing the closed table.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree immediately. Record its
   actual path/branch; do not create a second one because placeholders differ.
3. If current context is unusable, inspect the named worktree. Only if needed,
   use `.agents.local.env` and `AGENTS_WORKTREE_CONTAINER_DIR` for a unique
   fallback from `origin/main`. Ask if absent. Never clean/reset/stash-over a
   dirty checkout or use `/tmp`. If the launcher supplied `main` or dirty
   state, stop and report it.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.087, card 244, Research 241, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 244 and edit only allowed lane files.
- Record primary sources with final URL/tag, retrieval date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, shared-file need, scope expansion, or validation
  that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 244, Research 241, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.087, card 244, Research 241, changed evidence, validation, and
   unresolved items. Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. Later lanes restack after
earlier evidence PRs land. An evidence-backed PR comment is canonical if
self-approval is unavailable. Merge requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.087 and Next Task
state belong to the orchestrator after the evidence PR lands.

