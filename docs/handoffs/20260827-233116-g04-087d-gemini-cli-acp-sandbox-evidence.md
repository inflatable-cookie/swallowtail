---
title: g04.087d Gemini CLI ACP sandbox evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-233116-g04-087d-gemini-cli-acp-sandbox-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, gemini]
---

## What This Thread Was Doing

The orchestrator compiled g04.087 as four independent evidence-only lanes.
This lane owns Gemini CLI ACP sandbox evidence: card 247, Research 244, its
reserved log, and optional new Gemini-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Gemini has a native whole-process sandbox and the headless route already closed
an empty set because ambient precedence and unconfirmed backend activation
blocked binding. ACP re-exec and session readiness form a distinct route that
must be assessed without consumer OAuth or live backend/provider work.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `d00cea2590f8926cb43bccfbad607719cd58d331`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.087, card 247, Research 244 reservation, lane log,
  indexes, and sole Next Task
- **Worker branch:** `worker/g04-087d-gemini-cli-acp-sandbox-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-087d-gemini-cli-acp-sandbox-evidence`
- **Worktree creation command:** `git worktree add -b worker/g04-087d-gemini-cli-acp-sandbox-evidence /Users/tom/Dev/worktrees/swallowtail-g04-087d-gemini-cli-acp-sandbox-evidence origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable,
  inspect the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.087 lane D
- **Roadmap:** `docs/roadmaps/g04/087-fourth-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/247-gemini-cli-acp-sandbox-evidence.md`
- **Research:** `docs/research/244-gemini-cli-acp-sandbox-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-087d-gemini-cli-acp-sandbox-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, C; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Gemini package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  006, 013, 017, 023, 029, 033, 037, 047, 052; Research 182, 235, and 239;
  `docs/guides/gemini-cli-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and route-local audit
- **Tool/runtime restrictions:** official docs and exact tagged source plus
  secret-free local inspection only; no install/update, consumer OAuth/login,
  credential, provider prompt, paid work, backend start/image pull, or host mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 247 exactly; qualified published versions, ACP spawn,
  platform/backend/value membership, precedence, parent parsing, backend start,
  re-exec child, ACP connection/readiness, rejection, cleanup, omission,
  Research 244, and the assigned log.
- **Allowed changed files:** assigned card, Research 244, assigned log, and new
  Gemini-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Gemini headless/Live, thinking, output
  limits, consumer OAuth, portable containment, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not promote Research 239's headless conclusion without ACP proof. Keep
  sandbox request, activation, effectiveness, and containment distinct.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- ACP `0.51.0..=0.56.0` is qualified under enterprise Developer API-key
  access. Consumer account login no longer belongs to this route.
- Research 239 freezes cross-version sandbox parsing/backends and shows
  `GEMINI_SANDBOX` overriding argv/settings on headless. It is contrast only.
- A non-empty row needs process-private precedence, exact backend/platform
  membership, parent-to-child ownership, ACP-child activation before readiness,
  joined failure, and unchanged omission without claiming containment.
- Report after Research 244 and card 247 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Trace exact tagged `--sandbox`/environment parsing
through the ACP entry, backend re-exec, child connection, initialize/session
readiness, and cleanup before constructing the closed table.

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
5. Read `AGENTS.md`, g04.087, card 247, Research 244, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 247 and edit only allowed lane files.
- Record primary sources with final URL/tag, retrieval date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, shared-file need, scope expansion, or validation
  that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 247, Research 244, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.087, card 247, Research 244, changed evidence, validation, and
   unresolved items. Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current
`main` after lanes A-C land. An evidence-backed PR comment is canonical if
self-approval is unavailable. Merge requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.087 and Next Task
state belong to the orchestrator after the evidence PR lands.

