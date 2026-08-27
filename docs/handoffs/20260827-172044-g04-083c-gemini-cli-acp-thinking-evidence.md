---
title: g04.083c Gemini CLI ACP thinking evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-172044-g04-083c-gemini-cli-acp-thinking-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, gemini]
---

## What This Thread Was Doing

The orchestrator compiled g04.083 as four independent evidence-only lanes.
This lane owns Gemini CLI ACP thinking configuration: card 234, Research 235,
its reserved log, and optional new Gemini-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Gemini CLI remains useful through enterprise Developer API-key access. Its ACP
route negotiates session options that headless does not, so the stopped headless
thinking result cannot settle whether ACP has a bounded, confirmable control.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `59c8238623dfdda61a87c7147b5240d87d611ebb`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.083, card 234, Research 235 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-083c-gemini-acp-thinking-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-083c-gemini-acp-thinking-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-083c-gemini-acp-thinking-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-083c-gemini-acp-thinking-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.083 lane C
- **Roadmap:** `docs/roadmaps/g04/083-second-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/234-gemini-cli-acp-thinking-evidence.md`
- **Research:** `docs/research/235-gemini-cli-acp-thinking-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-083c-gemini-cli-acp-thinking-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Gemini package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 182 and 230;
  `docs/guides/gemini-cli-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and ACP lifecycle audit
- **Tool/runtime restrictions:** official/tagged sources and secret-free local
  inspection only; no install/update, consumer login, credential, provider
  prompt, account inspection, paid work, or host configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 234 exactly; `gemini-cli.acp` thinking settings and ACP
  options, exact versions/models/values, configuration precedence and child
  isolation, new/follow-up/load/resume/replacement truth, omission, Research
  235, card state, and assigned log.
- **Allowed changed files:** assigned card, Research 235, assigned log, and new
  Gemini-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Gemini headless or Live, sandbox,
  output bounds, consumer login, shared milestone/inventory/programme/triage/
  matrices/indexes/Next Task, currentness, release, merge, rollover, g04 closure.
- Keep enterprise Developer API-key access exact. Do not restore or inspect
  consumer-account authentication.
- Research 230 is a settings-loader lead only. Do not promote its headless
  conclusion or Gemini Live thinking to ACP.
- Requested, negotiated, configured, dispatched, effective, and observed
  thinking remain distinct. Thought output is not selected-value confirmation.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- ACP is qualified at published points `0.51.0..=0.56.0` under
  `gemini-cli.acp-agent` and enterprise Developer API-key access.
- Current ACP prepares negotiated model options and optional Plan mode but
  rejects reasoning. Headless Research 230 found a redirectable settings loader
  but no adapter binding or stream confirmation.
- The decisive question is whether ACP initialize/session configuration exposes
  a typed selection plus effective confirmation before prompt effects. If not,
  a settings-only row still needs process-private precedence over every ambient
  layer.
- An empty set is expected if membership is authenticated, settings remain
  ambient, or confirmation requires a provider prompt.
- Report after Research 235 and card 234 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze exact ACP initialize, `session/new`, and
configuration-option frames before revisiting settings. Decide whether ACP has
its own selection and confirmation seam.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree immediately and record
   its actual path/branch. Do not create a second worktree for name differences.
3. If unusable, inspect the named worktree; only then use `.agents.local.env`
   and required `AGENTS_WORKTREE_CONTAINER_DIR` for a unique fallback from
   `origin/main`. Ask if absent. Never clean/reset/stash-over dirty state or use
   `/tmp`. Stop if the launcher supplied `main` or dirty state.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.083, card 234, Research 235, the lane log, and named
   refs. Then run cheap orientation checks.

### While you work

- Execute only card 234 and edit only allowed lane files.
- Use primary official/tagged sources. Record URL/tag, retrieval date, digest,
  and decisive bounded evidence. Respect quotation limits.
- Report meaningful progress and stop on missing authority, shared-file need,
  scope expansion, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 234, Research 235, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.083, card, Research, evidence, validation, and unresolved items.
   Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current `main`
after earlier lane merges before fast-forward-only integration. An evidence-
backed PR comment is canonical if self-approval is unavailable. Merge requires
explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared promotion belongs to
the orchestrator after merge.
