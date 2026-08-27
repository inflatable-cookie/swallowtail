---
title: g04.085c Gemini CLI headless sandbox evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-202539-g04-085c-gemini-cli-headless-sandbox-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, gemini]
---

## What This Thread Was Doing

The orchestrator compiled g04.085 as four independent evidence-only lanes.
This lane owns Gemini CLI headless sandboxing: card 240, Research 239, its
reserved log, and optional new Gemini-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Gemini CLI remains available through enterprise API-key access and documents a
native sandbox surface. Swallowtail must distinguish selecting that surface
from proving backend activation or making a portable containment claim.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `4861bbe07a1aaa39dbb243fbbc300f3133496475`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.085, card 240, Research 239 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-085c-gemini-cli-headless-sandbox-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-085c-gemini-cli-headless-sandbox-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-085c-gemini-cli-headless-sandbox-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-085c-gemini-cli-headless-sandbox-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.085 lane C
- **Roadmap:** `docs/roadmaps/g04/085-third-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/240-gemini-cli-headless-sandbox-evidence.md`
- **Research:** `docs/research/239-gemini-cli-headless-sandbox-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-085c-gemini-cli-headless-sandbox-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Gemini package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 033, 037, 040, 041, 047, 052; Research 182 and 230;
  `docs/guides/gemini-cli-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and route-local audit
- **Tool/runtime restrictions:** official docs and exact tagged package source
  plus secret-free local inspection only; no install/update, login, credential,
  provider prompt, sandbox backend start, paid work, or host mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 240 exactly; `gemini-cli.headless` sandbox versions,
  platforms, backends, values, precedence, activation, authority, lifecycle,
  omission, Research 239, and the assigned log.
- **Allowed changed files:** assigned card, Research 239, assigned log, and new
  Gemini-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Gemini ACP or Live, thinking, output
  limits, consumer login, portable containment, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not infer filesystem, process, credential, or network containment from a
  parsed flag or environment value.
- Requested, encoded, backend-started, accepted, effective, contained, and
  observed truth remain distinct.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The exact headless route is qualified across `0.51.0..=0.56.0`, uses
  enterprise Developer API-key access, runs Plan approval, and disables
  extensions and MCP.
- Research 230 freezes current settings/process behavior for the separate
  thinking lane but does not qualify sandboxing.
- A non-empty row needs closed platform/backend membership, process-private
  precedence, pre-effect rejection, and prompt-free activation confirmation.
- An empty set is correct if a backend must be installed/run or activation
  cannot be distinguished from a parsed request without provider work.
- Report after Research 239 and card 240 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze exact `--sandbox` and `GEMINI_SANDBOX`
precedence, backend selection, platform gates, and prompt-free activation
evidence before building the closed table.

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
   dirty checkout or use `/tmp`. If the launcher supplied `main` or dirty state,
   stop and report it.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.085, card 240, Research 239, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 240 and edit only allowed lane files.
- Record primary sources with final URL/tag, retrieval date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, shared-file need, scope expansion, or validation
  that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 240, Research 239, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.085, card 240, Research 239, changed evidence, validation, and
   unresolved items. Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. If lanes A or B land first,
restack this branch onto current `main` before fast-forward-only merge. An
evidence-backed PR comment is canonical if self-approval is unavailable. Merge
requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.085 and Next Task
state belong to the orchestrator after the evidence PR lands.
