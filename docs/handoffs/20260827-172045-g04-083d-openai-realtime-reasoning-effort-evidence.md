---
title: g04.083d OpenAI Realtime reasoning-effort evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-172045-g04-083d-openai-realtime-reasoning-effort-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, openai, realtime]
---

## What This Thread Was Doing

The orchestrator compiled g04.083 as four independent evidence-only lanes.
This lane owns OpenAI Realtime reasoning effort: card 235, Research 236, its
reserved log, and optional new OpenAI-local frozen evidence.

This is one bounded manual worker thread. Start from this file without copied
context or a second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Realtime documentation exposes reasoning effort on reasoning-capable models,
but Swallowtail's dated media facade fixes one exact route. Evidence must prove
that exact model and lifecycle rather than importing Responses semantics.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `59c8238623dfdda61a87c7147b5240d87d611ebb`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.083, card 235, Research 236 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-083d-openai-realtime-reasoning-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-083d-openai-realtime-reasoning-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-083d-openai-realtime-reasoning-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-083d-openai-realtime-reasoning-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.083 lane D
- **Roadmap:** `docs/roadmaps/g04/083-second-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/235-openai-realtime-reasoning-effort-evidence.md`
- **Research:** `docs/research/236-openai-realtime-reasoning-effort-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-083d-openai-realtime-reasoning-effort-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, C; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and OpenAI package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 026, 029, 037, 040, 041, 047, 052; Research 020, 049, and
  127; `docs/guides/realtime-prepared-integration.md`
- **Model capability profile:** bounded exact-schema research and Realtime lifecycle audit
- **Tool/runtime restrictions:** official primary sources and secret-free local
  inspection only; no credential, provider connection, live media, paid work,
  dependency/currentness update, or fixture rebaseline
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-openai`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 235 exactly; `openai.realtime` exact model, effort values,
  session/response operation, encoding, acknowledgement, output/usage,
  cancellation/disconnect/restoration truth, omission, Research 236, and log.
- **Allowed changed files:** assigned card, Research 236, assigned log, and new
  OpenAI-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, OpenAI background, Responses
  reasoning, tools, images, search, output-limit changes, planned rollover,
  shared milestone/inventory/programme/triage/matrices/indexes/Next Task,
  currentness, live provider work, release, merge, rollover, or g04 closure.
- Do not use shared OpenAI catalogue or Responses support as Realtime model or
  transport proof.
- Requested, session-encoded, response-encoded, accepted, effective, returned,
  token-usage, and observed reasoning remain distinct.
- If shared mutable scope, a facade rebaseline, or hidden dependency appears,
  stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The selected route uses exact opaque revision `openai-realtime-2026-07-22`,
  public API-key access, a fixed manual PCM profile, exact output-token maximum,
  response cancellation, and no planned rollover.
- Current low-level validation rejects any reasoning selection before access or
  connection. The dated fixture README records reasoning as unsupported.
- A non-empty row needs exact selected-model membership, exact session versus
  response update timing, pre-connection rejection, and a bounded confirmation
  story. Reasoning-token usage alone is not selected-effort confirmation.
- An empty set is expected if the fixed model is not reasoning-capable or the
  only decisive confirmation requires a live provider session.
- Report after Research 236 and card 235 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze the exact dated model and current official
Realtime schemas, then reconcile effort support and update timing before
building any lifecycle table.

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
5. Read `AGENTS.md`, g04.083, card 235, Research 236, lane log, and named refs.
   Then run cheap repo orientation checks.

### While you work

- Execute only card 235 and edit only allowed lane files.
- Use primary official schemas/docs and the dated route corpus. Record final
  URL or exact fixture identity, retrieval date, digest, and decisive evidence.
- Report meaningful progress. Stop on missing authority, shared-file need,
  scope expansion, facade rebaseline, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 235, Research 236, and the assigned log honestly.
3. Push the worker branch and open a PR against current pushed `main`.
4. Link g04.083, card, Research, evidence, validation, and unresolved items.
   Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding/currentness.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current `main`
after earlier lane merges before fast-forward-only integration. An evidence-
backed PR comment is canonical if self-approval is unavailable. Merge requires
explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared promotion belongs to
the orchestrator after merge.
