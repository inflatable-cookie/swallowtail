---
title: g04.083b Codex exec Fast service-tier evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-172043-g04-083b-codex-exec-fast-service-tier-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, codex]
---

## What This Thread Was Doing

The orchestrator compiled g04.083 as four independent evidence-only lanes.
This lane owns Codex exec Fast/service tier: card 233, Research 234, its
reserved log, and optional new Codex-local frozen evidence.

This is one bounded manual worker thread. Start from this file without copied
context or a second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Codex exposes `/fast`, a `fast_mode` feature gate, and `service_tier`, but their
composition, model membership, access/billing profile, request truth, and
returned state must be exact before Swallowtail can offer a caller control.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `59c8238623dfdda61a87c7147b5240d87d611ebb`
- **Pushed main verification:** planning base equalled `origin/main` before the
  handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.083, card 233, Research 234 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-083b-codex-exec-fast-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-083b-codex-exec-fast-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-083b-codex-exec-fast-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-083b-codex-exec-fast-evidence
  origin/main`
- **Worker worktree policy:** prefer a clean launcher-provided non-`main`
  registered worktree regardless of generated path/branch. If unusable, inspect
  the named worktree; only then use `.agents.local.env` with required
  `AGENTS_WORKTREE_CONTAINER_DIR`. Ask if absent. Never guess `/tmp`.
- **Active lane:** per-route feature completion, g04.083 lane B
- **Roadmap:** `docs/roadmaps/g04/083-second-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/233-codex-exec-fast-service-tier-evidence.md`
- **Research:** `docs/research/234-codex-exec-fast-service-tier-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-083b-codex-exec-fast-service-tier-evidence.md`
- **Allowed runway:** evidence only; exact non-empty table or honest empty set;
  one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with lanes A, C, D; serial integration order
  A, B, C, D with restack after earlier merges
- **Parallel safety:** unique card, Research, log, and Codex package evidence
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 201, 213, and 229;
  `docs/guides/codex-prepared-integration.md`
- **Model capability profile:** bounded exact-source research and route-local audit
- **Tool/runtime restrictions:** official/tagged sources and secret-free local
  inspection only; no install/update, login, credential, provider request,
  account/catalogue inspection, paid work, or ambient configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-codex`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised

## Boundaries

- **In scope:** card 233 exactly; `codex.exec` Fast/service-tier introduction,
  feature gate, config precedence, model/access membership, request/returned
  state, billing, lifecycle, omission, Research 234, and assigned log.
- **Allowed changed files:** assigned card, Research 234, assigned log, and new
  Codex-local frozen evidence under a unique fixture/evidence path.
- **Out of scope:** production code/API, app-server, generic Fast vocabulary,
  search, personality, Plan effort, multi-agent, shared milestone/inventory/
  programme/triage/matrices/indexes/Next Task, currentness, live provider work,
  release, merge, rollover, or g04 closure.
- Do not assume `/fast`, `features.fast_mode`, and `service_tier = "fast"` are
  interchangeable. Prove the exact relationship and precedence.
- Keep ChatGPT-credit and API-key billing profiles separate. Catalogue
  advertisement is not provider acceptance or returned-tier truth.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- The maintained exec segments are `0.80.0..=0.81.0`, `0.84.0..=0.107.0`,
  and `0.110.0..=0.149.1`. Locate Fast's exact introduction point rather than
  assuming Research 213's four verbosity points own it.
- Current exec is structured JSONL with read-only sandbox, approval `never`,
  explicit model, and no Fast config. Model verbosity is already delivered and
  must remain byte-for-byte unchanged under omission.
- A non-empty row needs static selected-model membership, closed config
  composition and precedence, pre-effect rejection, plus honest request or
  returned-state evidence.
- An empty set is expected if membership or access depends on a live account
  catalogue, or if the tier silently substitutes without confirmation.
- Report after Research 234 and card 233 are complete, or earlier on a stop.
- Report to the operator, who relays the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight. Locate the exact tagged introduction of the feature
gate and service tier, then trace their composition through exec config and
request construction before assessing model or billing rows.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` current worktree immediately. Record its
   actual path/branch; do not create a second one for placeholder differences.
3. If unusable, inspect the named worktree; only then use `.agents.local.env`
   and required `AGENTS_WORKTREE_CONTAINER_DIR` for a unique fallback from
   `origin/main`. Ask if absent. Never clean/reset/stash-over or use `/tmp`.
   Stop if the launcher supplied `main` or dirty state.
4. Fetch origin. Confirm `HEAD == origin/main`, the planning base is an
   ancestor, and this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.083, card 233, Research 234, lane log, and named refs.
   Then run cheap repo orientation checks.

### While you work

- Execute only card 233 and edit only allowed lane files.
- Use primary official/tagged sources. Record exact version, final URL/tag,
  retrieval date, digest, and decisive evidence. Respect quotation limits.
- Report meaningful progress. Stop on missing authority, shared-file need,
  scope expansion, or validation that changes the plan.

### When complete

1. Run the listed validation.
2. Complete card 233, Research 234, and the assigned log honestly.
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
