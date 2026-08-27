---
title: g04.082b Gemini CLI headless thinking evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-154357-g04-082b-gemini-cli-headless-thinking-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, gemini]
---

## What This Thread Was Doing

The orchestrator compiled g04.082 as four independent evidence-only lanes.
This lane owns Gemini CLI headless thinking configuration: card 229, Research
230, its reserved log, and optional new Gemini-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Gemini CLI is current through exact `0.56.0` for enterprise Developer API-key
access. The headless route still rejects reasoning. Official settings mention
thinking configuration, but ambient settings, model entitlement, and lack of
prompt-free confirmation may make it unsafe. This lane settles that precisely
without consumer login or live provider work.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e6f7258ca2a5532b7fe9fb92fb7dd03f83b27098`
- **Pushed main verification:** planning base was exact `origin/main` before
  the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.082, card 229, Research 230 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-082b-gemini-headless-thinking-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-082b-gemini-headless-thinking-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-082b-gemini-headless-thinking-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-082b-gemini-headless-thinking-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.082 lane B
- **Roadmap:** `docs/roadmaps/g04/082-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/229-gemini-cli-headless-thinking-evidence.md`
- **Research:** `docs/research/230-gemini-cli-headless-thinking-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-082b-gemini-cli-headless-thinking-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, C, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Gemini package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 045, 182;
  `docs/guides/gemini-cli-prepared-integration.md`
- **Worker profile:** bounded exact-source research and route-local audit
- **Restrictions:** official/tagged sources and secret-free inspection only;
  no install/update, consumer login, credential, provider prompt, account
  inspection, paid work, or host configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 229 exactly; `gemini-cli.headless` thinking settings,
  exact versions/models/values, configuration precedence and child isolation,
  prompt dispatch and stream truth, retention/cleanup effects, omission,
  Research 230, card state, and assigned log.
- **Allowed changed files:** assigned card, Research 230, assigned log, and new
  Gemini-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, Gemini ACP, Gemini Live, sandbox,
  output bounds, consumer login, shared milestone/inventory/programme/triage/
  matrices/indexes/Next Task, currentness, release, merge, rollover, g04 closure.
- Keep enterprise Developer API-key access exact. Do not restore or inspect
  consumer-account authentication.
- Gemini Live thinking is sibling-route evidence only and cannot promote here.
- Requested, configured, dispatched, effective, and observed thinking remain
  distinct. Reasoning output is not selected-value confirmation.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- ACP and headless are separate exact axes qualified at published points
  `0.51.0..=0.56.0`; headless behavior is
  `gemini-cli.headless.stream-json.v1`.
- Current headless argv selects explicit model, stream JSON, Plan approval,
  disabled extensions/MCP, trust handling, and a session id. It passes no
  thinking setting and the prepared guide rejects reasoning.
- The decisive gate is a caller-bound process-private seam that overrides
  ambient settings without reading or mutating them, plus static model/value
  membership and prompt-free dispatch or effective confirmation.
- An empty set is expected if settings are ambient/durable, membership is
  authenticated, or confirmation requires a model invocation.
- Report after Research 230 and card 229 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze exact `0.56.0` settings schema/source and the
full child-process configuration precedence path. Decide isolation and
confirmation before building any deliver-now table.

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
5. Read `AGENTS.md`, g04.082, card 229, Research 230, the lane log, and named
   refs. Then run cheap orientation checks.

### While you work

- Execute only card 229 and edit only allowed lane files.
- Use primary official/tagged sources. Record URL/tag, date, digest, and
  decisive bounded evidence. Respect quotation limits.
- Report meaningful progress and stop on missing authority, shared-file need,
  scope expansion, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 229, Research 230, and the assigned log honestly.
3. Push the branch and open a PR against current `main`.
4. Link g04.082, card, Research, evidence, validation, and unresolved items.
   Shared index links are already reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the diff and checks. Restack onto current `main` after
earlier lane merges before fast-forward-only integration. An evidence-backed PR
comment is the canonical review if self-approval is unavailable. Merge requires
explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared promotion belongs to
the orchestrator after merge.
