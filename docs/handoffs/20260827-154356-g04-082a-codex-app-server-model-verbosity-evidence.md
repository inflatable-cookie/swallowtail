---
title: g04.082a Codex app-server model verbosity evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-154356-g04-082a-codex-app-server-model-verbosity-evidence.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr, evidence, codex]
---

## What This Thread Was Doing

The orchestrator normalized the original 85-item feature inventory, selected
four route-distinct qualification questions, and compiled g04.082 as parallel
evidence only. This lane owns Codex app-server model verbosity: card 228,
Research 229, its reserved log, and optional new Codex-local frozen evidence.

This is one bounded manual worker thread. Start from this file without a copied
transcript or second prompt. Do not spawn internal agents. The operator owns
parallelism in their harness.

## Why It Matters

Codex exec already has exact adapter-local verbosity, but app-server is a
different transport and lifecycle. The remaining inventory must not promote
exec argv evidence into app-server sessions. Exact configuration, model
membership, precedence, attachment behavior, and confirmation decide whether
this route can expose the control at all.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e6f7258ca2a5532b7fe9fb92fb7dd03f83b27098`
- **Pushed main verification:** planning base was exact `origin/main` before
  the handoff commit
- **Planning checkout:** clean shared `main`; never use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts:** g04.082, card 228, Research 229 reservation, reserved
  lane log, indexes, and sole Next Task
- **Worker branch:** `worker/g04-082a-codex-app-server-verbosity-evidence`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-082a-codex-app-server-verbosity-evidence`
- **Worktree creation command:** `git worktree add -b
  worker/g04-082a-codex-app-server-verbosity-evidence
  /Users/tom/Dev/worktrees/swallowtail-g04-082a-codex-app-server-verbosity-evidence
  origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, even when its path or branch differs. If unusable,
  inspect the named worktree; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique fallback under it. Ask
  the operator if absent. Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active lane:** per-route feature completion, g04.082 lane A
- **Roadmap:** `docs/roadmaps/g04/082-parallel-per-route-feature-qualification.md`
- **Ready card:** `docs/roadmaps/g04/batch-cards/228-codex-app-server-model-verbosity-evidence.md`
- **Research:** `docs/research/229-codex-app-server-model-verbosity-evidence.md`
- **Lane log:** `docs/logs/2026-08-27-g04-082a-codex-app-server-model-verbosity-evidence.md`
- **Allowed runway:** evidence only; promote an exact non-empty table or honest
  empty set and open one reviewable PR
- **Card budget:** one card
- **Dispatch topology:** parallel with g04.082 lanes B-D; serial PR integration
  order A, B, C, D with restack after every earlier merge
- **Parallel safety:** unique card, Research, log, and Codex package evidence;
  no shared mutable planning file
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 020, 024, 029, 037, 040, 041, 047, 052; Research 037, 128, 160, 172,
  201, 213; `docs/guides/codex-prepared-integration.md`
- **Worker profile:** bounded exact-source research and route-local audit
- **Restrictions:** primary official/tagged sources and secret-free local
  inspection only; no install/update, login, credential, provider prompt,
  account inspection, paid work, or ambient configuration mutation
- **Required validation:** `effigy validate:focused
  swallowtail-adapter-codex`, `effigy qa:northstar`, `git diff --check`
- **Inherited doctor baseline:** 380 god-file findings (334 warnings, 46
  errors), one generated-in-src warning, stale graph index; record drift only
- **PR base/head:** current `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

- **In scope:** card 228 exactly; app-server `model_verbosity` configuration,
  version/model/value membership, defaults, precedence, new/import/load/resume/
  follow-up/restoration, dispatch and confirmation truth, omission, fixtures,
  Research 229, card state, and the lane log.
- **Allowed changed files:** the assigned card, Research 229, assigned log, and
  new Codex-local frozen evidence under a uniquely named fixture/evidence path.
- **Out of scope:** production code/API, exec changes, Fast tier, personality,
  Plan effort, multi-agent, shared milestone/inventory/programme/triage/matrix/
  index/Next Task files, currentness, release, merge, rollover, or g04 closure.
- Research 213 is a lead only. Do not reuse exec argv or silently widen its
  exact model table onto app-server.
- Requested, configured, dispatched, provider-accepted, effective, and observed
  verbosity remain separate. Output length is not effective-state proof.
- Do not invent architecture, alter contracts, or choose an unresolved product,
  API, persistence, or security decision.
- If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected worker worktree. Do not merge the PR.

## Important Context

- App-server is qualified from `0.80.0` through `0.149.1`; the maintained
  workspace-roots segment starts at `0.131.0`.
- Research 213 froze exec verbosity only at `0.147.0`, `0.148.0`, `0.149.0`,
  and `0.149.1`, including model metadata and unsupported-model ignore.
- The decisive question is whether app-server exposes a caller-bound session or
  turn configuration seam with precedence and confirmation across attachment
  lifecycles. Ambient/global config alone is an empty-set outcome.
- Omission must preserve the current app-server protocol/config bytes and must
  not claim the ambient or model default as caller-selected.
- Report after Research 229 and card 228 are complete, or earlier if a stop
  condition or shared-scope dependency fires.
- Report to the operator, who will relay the PR to the orchestrator.

## Suggested Next Move

Run the startup preflight below. Then read card 228 and its named exact Codex
sources. Freeze the app-server configuration and lifecycle seam before deciding
whether any row is deliverable. Promote an honest empty set rather than
borrowing exec behavior.

## Completion Protocol

### Before you start

1. Read this handoff first. Before broad reads run: `git rev-parse
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
5. Read `AGENTS.md`, g04.082, card 228, Research 229, the lane log, and named
   canonical refs. Then run the cheap repo orientation checks.

### While you work

- Execute only card 228. Edit only allowed lane files.
- Record retrieved primary sources with final URL/tag, date, digest, and the
  decisive bounded evidence. Respect source quotation limits.
- Report meaningful progress with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing authority, scope expansion, shared-file need, or validation
  that changes the plan.

### When complete

1. Run the required validation exactly as listed above.
2. Complete card 228, Research 229, and the assigned log honestly.
3. Push the worker branch and open a PR against current `main`.
4. Link g04.082, card 228, Research 229, changed evidence, validation, and
   unresolved items. Do not edit shared indexes to add links; they are reserved.
5. Report the PR URL. Do not merge or begin production binding.

### Review and merge path

The orchestrator reviews the exact diff and checks. If earlier evidence PRs
land first, restack this branch onto current `main` before fast-forward-only
merge. Formal self-approval may be unavailable; an evidence-backed PR comment
is the canonical verdict. Merge requires explicit operator authorisation.

### Handoff closeout

Leave the assigned card, Research, and log honest. Shared g04.082 and Next Task
state belong to the orchestrator after the evidence PR lands.
