---
title: g04.082c Bedrock Runtime service-tier evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-154358-g04-082c-bedrock-runtime-service-tier-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, bedrock]
---

## What This Thread Was Doing

The orchestrator compiled g04.082 as four independent evidence-only lanes.
This lane owns Bedrock Runtime latency/service tier: card 230, Research 231,
its reserved log, and optional new Bedrock-local frozen evidence.

This is one bounded manual worker thread. Start from this file without copied
context or a second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Bedrock `ConverseStream` exposes performance and tier fields, but eligibility
may depend on model, region, account, capacity, or inference profile. The route
also has an existing public SDK-version constant versus locked dependency
mismatch. Evidence must settle both without turning a product label into a
generic Fast claim or silently repairing currentness.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e6f7258ca2a5532b7fe9fb92fb7dd03f83b27098`
- **Pushed main verification:** planning base was exact `origin/main` before
  the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.082, card 230, Research 231 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-082c-bedrock-service-tier-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-082c-bedrock-service-tier-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-082c-bedrock-service-tier-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-082c-bedrock-service-tier-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` and required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.082 lane C
- **Roadmap:** `docs/roadmaps/g04/082-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/230-bedrock-runtime-service-tier-evidence.md`
- **Research:** `docs/research/231-bedrock-runtime-service-tier-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-082c-bedrock-runtime-service-tier-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, B, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Bedrock package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 019, 020, 024, 029, 037, 040, 041, 047, 052; Research 013, 024, 127,
  159; `docs/guides/bedrock-sdk-prepared-integration.md`
- **Worker profile:** bounded exact SDK/service-model research and route audit
- **Restrictions:** primary AWS/API/SDK sources and secret-free inspection
  only; no dependency update, AWS credential, account/region inspection,
  provider call, paid work, or ambient configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-bedrock`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 230 exactly; `performanceConfig.latency`, `serviceTier`,
  exact SDK/service model, request/response/error truth, model/region/account/
  inference-profile dependencies, omission, Research 231, card state, log.
- **Allowed changed files:** assigned card, Research 231, assigned log, and new
  Bedrock-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, SDK pin/constant repair, thinking,
  tools, guardrails, catalogue route, shared milestone/inventory/programme/
  triage/matrices/indexes/Next Task, currentness, live AWS work, release, merge,
  rollover, or g04 closure.
- Record `SDK_VERSION = 1.136.0` versus Cargo `=1.139.0` exactly. Do not choose
  or change the canonical version in this lane.
- Account, region, entitlement, capacity, or billing facts must not become
  static capability claims.
- Requested, SDK-built, accepted, effective, returned, billed, and observed
  latency/tier truth remain separate.
- If shared mutable scope or hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The route uses delegated cloud identity and exact `ConverseStream`. Current
  SDK invocation sends model, messages, and `inferenceConfig.maxTokens` only.
- Public constants claim Runtime SDK `1.136.0`; Cargo locks `1.139.0`.
  Research 127 and 159 already record that mismatch.
- A non-empty row needs an honest exact evidence point and a preparation-time
  bounded selection that does not depend on remote account facts. Dispatch-only
  truth may still be useful, but must not imply acceptance, returned tier, cost,
  or measured latency.
- Omission must retain the current builder call and route behavior.
- Report after Research 231 and card 230 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Resolve the exact generated SDK/service-model shapes
at both recorded versions, then decide whether the mismatch itself forces an
empty set. Only after that assess static eligibility and returned-state truth.

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
5. Read `AGENTS.md`, g04.082, card 230, Research 231, lane log, and named refs.
   Then run cheap orientation checks.

### While you work

- Execute only card 230 and edit only allowed lane files.
- Use primary AWS API/service-model/SDK sources. Record exact version, URL,
  date, digest, and decisive bounded evidence. Respect quotation limits.
- Report meaningful progress. Stop on missing authority, shared-file need,
  scope expansion, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 230, Research 231, and the assigned log honestly.
3. Push the branch and open a PR against current `main`.
4. Link g04.082, card, Research, evidence, validation, and unresolved items.
   Shared index links are reserved; do not edit them.
5. Report the PR URL. Do not merge or begin production binding/currentness.

### Review and merge path

The orchestrator reviews the exact diff and checks. Restack onto current `main`
after earlier lane merges before fast-forward-only integration. An evidence-
backed PR comment is canonical if self-approval is unavailable. Merge requires
explicit operator authorisation.

### Handoff closeout

Leave assigned card, Research, and log honest. Shared promotion belongs to the
orchestrator after merge.
