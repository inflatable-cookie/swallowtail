---
title: Card 058 caller-bounded close worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-230154-card058-caller-bounded-close.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The Claude Agent SDK foundation on PR 188 exposed a shared lifecycle defect:
public close and post-expiry cleanup can wait forever. The operator accepted a
coordinated v0.4 breaking change rather than an unbounded compatibility shim.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

PR 188 cannot safely expose the SDK route until open, turn, and close cleanup
return under caller-owned host time. The same provider-neutral rule must hold
for every interactive-session implementation before the next release.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `921039447ce3ec20c9ab3c5e439814b5b8e19a44`
- **Pushed main verification:** worker must start from the pushed `origin/main`
  containing this handoff and prove the planning base is its ancestor
- **Planning checkout:** clean before this planning batch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.023, ready card 058, and this
  handoff in the worker's committed `HEAD`
- **Worker branch:** `worker/g05-card058-caller-bounded-close`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card058-caller-bounded-close`
- **Worktree creation command:** Paseo `branch-off` from pushed `main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 010 and 019 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/058-caller-bounded-interactive-session-cleanup.md`
- **Allowed runway:** card 058 only
- **Remaining card budget:** one card
- **Dispatch topology:** parallel with card 059 Unix owned-tree attestation
- **Parallel safety check:** card 058 owns shared close/deadline semantics and
  migrations; card 059 owns process-tree observation. Shared API/docs evidence
  is reconciled by restacking this lane after card 059 merges if necessary.
- **Surfaces this lane owns:** runtime interactive-session close/request/deadline
  types; every required implementation and fixture migration; close-related
  Contract 010/019 clauses; card 058 tests, API evidence, and closeout log
- **Integration ownership:** orchestrator owns final front-door/index merge
  reconciliation with card 059 and PR 188
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/contracts/010-execution-host-services-and-inputs.md` and
  `docs/contracts/019-embedded-sdk-and-cloud-client-boundary.md`
- **Review oracle:** card 058 `## Review Oracle`
- **Model capability profile:** frontier implementation, lifecycle/public API
- **Frontier-worker justification:** exceptional post-planning reasoning is
  required to migrate a cross-adapter object-safe async trait while proving a
  single caller deadline bounds every stalled cleanup stage; this is the
  highest-priority release-blocking shared API change and a false bound can
  leak credentials, resources, tasks, or processes
- **Tool/runtime restrictions:** no provider contact, live probes, release
  commands, guessed tick units, default timeout, or compatibility shim
- **Required validation:** all card 058 validation; adapters in explicit groups
  of at most four; semantic API; routes; docs; Northstar; god-files; diff check
- **PR base/head:** current pushed `main` / `worker/g05-card058-caller-bounded-close`
- **PR URL:** pending
- **Review state:** awaiting implementation and exact-head frontier review
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** card 058's breaking caller-bounded close and complete production
  migration
- **Out of scope:** process-tree attestation; SDK feature work or PR 188 edits;
  provider calls; release preparation; unrelated cleanup redesign
- **Outcome shape:** smallest complete contract-valid implementation, tests,
  cleanup, evidence, and one PR; diagnostics-only is valid only at a named card
  stop condition
- Do not invent architecture, change unrelated contracts, widen the roadmap, or
  choose another product/API/security decision.
- This handoff represents one worker lane, and card 059 runs concurrently.
  Stop on an unexpected overlap rather than editing card 059's owned surfaces.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g05.022 card 055 / PR 188 review → g05.023 → completed
  card 057 → operator acceptance of the v0.4 breaking close seam
- **Why these cards are ready:** the public break, absence of a compatibility
  shim, caller-selected host deadline, and failure semantics are settled
- **Decisions and preferences:** one hard caller boundary must cover close and
  every cleanup action after open or turn expiry; host time owns conversion
- **Open tensions:** choose the smallest provider-neutral request shape without
  silently widening unrelated operation semantics
- **Report after:** the common trait/request migration compiles and the first
  stalled-stage falsification passes
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight. Inventory every production and fixture
implementation of the affected interactive-session close trait, then implement
the provider-neutral shape and migrate one coherent batch before validation.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`, `git branch
--show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
Accept a clean launcher-provided non-main registered worktree. Otherwise follow
`.agents.local.env`; never guess or clean another worktree.

Fetch origin with a bounded non-interactive SSH command. Prove `HEAD` equals
`origin/main`, the planning base above is an ancestor, and this handoff's tracked
blob equals the dispatch file. Read `AGENTS.md`, the milestone, card 058, and
Contracts 010/019 before editing.

Work in meaningful batches. Preserve unrelated work. Use `apply_patch` for
edits. Run the card's exact validation after the coherent migration, not after
every file. Falsify every universal/deadline claim and remove temporary
instrumentation.

When complete, reconcile card 058 and its closeout log. Leave final shared
front doors to the orchestrator where the ownership partition says so. Commit,
push, and open one PR against current pushed `main`. If card 059 merged first,
restack, retain both lanes' changes, rerun validation, and report the new exact
head. Do not merge.

The orchestrator posts the canonical exact-head review. Requested changes use
the same branch and workspace. Blocking classes are `execution-miss`,
`oracle-gap`, `planning-change`, `validation-gap`, and `integration-drift`.

Closeout refs: card 058, g05.023, Contracts 010/019, semantic API evidence, and
the lane's dated log.
