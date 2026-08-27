---
title: g04.082d Ollama attached think max evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-154359-g04-082d-ollama-attached-think-max-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, ollama]
---

## What This Thread Was Doing

The orchestrator compiled g04.082 as four independent evidence-only lanes.
This lane owns Ollama attached think `max`: card 231, Research 232, its reserved
log, and optional new Ollama-local frozen evidence.

This is one bounded manual worker thread. Start from this file without copied
context or a second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Ollama already exposes exact `off|low|medium|high` reasoning when selected-model
detail advertises generic thinking. Official `max` exists, but generic thinking
support may not prove exact level membership. This lane decides whether any
qualified version/model/operation can admit it without silent clamp, default,
template substitution, or a live model run.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e6f7258ca2a5532b7fe9fb92fb7dd03f83b27098`
- **Pushed main verification:** planning base was exact `origin/main` before
  the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.082, card 231, Research 232 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-082d-ollama-think-max-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-082d-ollama-think-max-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-082d-ollama-think-max-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-082d-ollama-think-max-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` and required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.082 lane D
- **Roadmap:** `docs/roadmaps/g04/082-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/231-ollama-attached-think-max-evidence.md`
- **Research:** `docs/research/232-ollama-attached-think-max-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-082d-ollama-attached-think-max-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A-C; serial integration order A,
  B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Ollama package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 049, 067, 138, 174,
  184; `docs/guides/ollama-attached-prepared-integration.md`
- **Worker profile:** bounded exact-source research and route-local audit
- **Restrictions:** official/tagged sources and secret-free inspection only;
  no runtime install/update, model pull, prompt, live inference, credential,
  remote account, paid work, or ambient configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-ollama`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 231 exactly; native `think: "max"`, exact version/model/
  template membership, parser/validation/fallback, structured run and
  interactive replay lifecycle, response truth, omission, Research 232, card
  state, and assigned log.
- **Allowed changed files:** assigned card, Research 232, assigned log, and new
  Ollama-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, context size, new generic reasoning
  vocabulary, model install/pull, owned runtime, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live inference,
  release, merge, rollover, or g04 closure.
- Generic `thinking` capability is not exact `max` membership. Model-family
  names and reasoning output are also insufficient.
- Requested, encoded, accepted, template-applied, effective, and observed
  reasoning remain separate.
- Existing `off|low|medium|high` and omission must remain exact.
- If shared mutable scope or hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- Route `ollama.attached` is qualified across maintained
  `0.14.0..=0.32.15`, excluding exact `0.32.2` and `0.32.10`, with permitted
  unverified-newer stable points.
- Production selected-model detail records generic `thinking`; validation
  currently admits only `off|low|medium|high`, and the native encoder maps
  `off` to boolean `false` and other modes to strings.
- A non-empty `max` row needs static exact membership available through the
  already-bound detail/preparation surface and no silent server substitution.
- Structured runs and transcript-replay interactive turns need independent
  lifecycle dispositions. Omission and existing modes cannot change.
- Report after Research 232 and card 231 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Freeze when and how `max` entered the tagged native
request/parser/template path, then inspect whether the selected-model detail
exposes exact level membership. If it only exposes generic thinking, promote an
honest empty set unless another already-bound static fact closes the table.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree immediately and record
   actual path/branch. Do not create a second one for placeholder differences.
3. If unusable, inspect the named worktree; only then use `.agents.local.env`
   and required `AGENTS_WORKTREE_CONTAINER_DIR` for a unique fallback from
   `origin/main`. Ask if absent. Never clean/reset/stash-over or use `/tmp`.
   Stop if the launcher supplied `main` or dirty state.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.082, card 231, Research 232, lane log, and named refs.
   Then run cheap orientation checks.

### While you work

- Execute only card 231 and edit only allowed lane files.
- Use primary official/tagged sources. Record exact version/tag, URL, date,
  digest, and decisive bounded evidence. Respect quotation limits.
- Report meaningful progress. Stop on missing authority, shared-file need,
  scope expansion, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 231, Research 232, and the assigned log honestly.
3. Push the branch and open a PR against current `main`.
4. Link g04.082, card, Research, evidence, validation, and unresolved items.
   Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current `main`
after earlier lane merges before fast-forward-only integration. An evidence-
backed PR comment is canonical if self-approval is unavailable. Merge requires
explicit operator authorisation.

### Handoff closeout

Leave assigned card, Research, and log honest. Shared promotion belongs to the
orchestrator after merge.
