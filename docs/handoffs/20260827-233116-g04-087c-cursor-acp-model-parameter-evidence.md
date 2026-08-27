---
title: g04.087c Cursor ACP model-parameter evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-233116-g04-087c-cursor-acp-model-parameter-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, cursor]
---

## What This Thread Was Doing

The orchestrator compiled g04.087 as four independent evidence-only lanes.
This lane owns Cursor ACP model-parameter evidence: card 246, Research 243, its
reserved log, and optional new Cursor-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Cursor headless accepts Fast, effort, and context parameters inside its model
string, but ACP is a separate ambient interactive route with no prepared model
option. Exact ACP-local membership, selection, and confirmation must exist
before any parameter can cross that route boundary.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `d00cea2590f8926cb43bccfbad607719cd58d331`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.087, card 246, Research 243 reservation, lane log,
  indexes, and sole Next Task
- **Worker branch:** `worker/g04-087c-cursor-acp-model-parameter-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-087c-cursor-acp-model-parameter-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-087c-cursor-acp-model-parameter-evidence /Users/tom/Dev/worktrees/swallowtail-g04-087c-cursor-acp-model-parameter-evidence origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable,
  inspect the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.087 lane C
- **Roadmap:** `docs/roadmaps/g04/087-fourth-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/246-cursor-acp-model-parameter-evidence.md`
- **Research:** `docs/research/243-cursor-acp-model-parameter-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-087c-cursor-acp-model-parameter-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Cursor package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  006, 015, 020, 029, 034, 037, 040, 047, 052; Research 135, 183, and 224;
  `docs/guides/cursor-prepared-integration.md`
- **Model capability profile:** bounded exact-source/binary research and route-local audit
- **Tool/runtime restrictions:** official docs, exact qualified builds, and
  prompt-free local inspection only; no install/update, login, credential,
  account/catalogue inspection, provider prompt, paid work, or host mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-cursor`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 246 exactly; four qualified builds, ACP-local model and
  Fast/effort/context membership, selection seams, application, confirmation,
  new/turn/replacement scope, omission, Research 243, and the assigned log.
- **Allowed changed files:** assigned card, Research 243, assigned log, and new
  Cursor-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Cursor catalogue/headless, Ask/Plan/
  Agent modes, sandbox, force/Yolo, shared milestone/inventory/programme/triage/
  matrices/indexes/Next Task, currentness, live provider work, release, merge,
  rollover, or g04 closure.
- Do not promote headless model-string syntax or catalogue observations onto
  ACP. Keep Fast, effort, and context as independent exact parameters.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Qualified identities are exactly `2026.07.01-41b2de7`,
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and
  `2026.08.11-e8db854`; date alone is insufficient.
- Current ACP preparation accepts no model, reasoning, parameter, tool,
  permission, or plan-mode option. It is ambient read-write.
- A non-empty row needs exact ACP selection, model/parameter membership,
  pre-effect rejection, returned selected value, lifecycle scope, and omission.
- Report after Research 243 and card 246 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Inspect exact ACP initialize/session frames,
configuration options, commands, and build-local help/source before comparing
them with the headless parameter grammar.

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
5. Read `AGENTS.md`, g04.087, card 246, Research 243, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 246 and edit only allowed lane files.
- Record primary sources with final URL/build, retrieval date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, shared-file need, scope expansion, or validation
  that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 246, Research 243, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.087, card 246, Research 243, changed evidence, validation, and
   unresolved items. Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current
`main` after lanes A-B land. An evidence-backed PR comment is canonical if
self-approval is unavailable. Merge requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.087 and Next Task
state belong to the orchestrator after the evidence PR lands.

