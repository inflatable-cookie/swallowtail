---
title: Card 059 Unix tree attestation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260902-230154-card059-unix-tree-attestation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Card 057 separated root exit from owned-tree completion but proved the current
Unix process-group owner cannot observe the tree empty. The operator authorized
a narrowly contained unsafe implementation or dependency if that is what a
sound Unix mechanism requires.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

The Claude SDK route has a Node sidecar, a native Claude child, and possible
descendants. PR 188 must not report cleanup complete merely because Node or the
nearest child exited. A false positive is worse than an honest unsupported
platform.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `921039447ce3ec20c9ab3c5e439814b5b8e19a44`
- **Pushed main verification:** worker must start from pushed `origin/main`
  containing this handoff and prove the planning base is its ancestor
- **Planning checkout:** clean before this planning batch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g05.023, ready card 059, and this
  handoff in the worker's committed `HEAD`
- **Worker branch:** `worker/g05-card059-unix-tree-attestation`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/2ee7rnl8/g05-card059-unix-tree-attestation`
- **Worktree creation command:** Paseo `branch-off` from pushed `main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contracts 010 and 019 are canonical
- **Roadmap milestone:** `docs/roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md`
- **Ready cards, in order:** `docs/roadmaps/g05/batch-cards/059-unix-owned-tree-attestation.md`
- **Allowed runway:** card 059 only
- **Remaining card budget:** one card
- **Dispatch topology:** parallel with card 058 caller-bounded close
- **Parallel safety check:** this lane owns process-tree identity/observation;
  card 058 owns close/deadline traits and migrations. Same-repo merge order is
  serial and card 058 restacks if shared API evidence overlaps.
- **Surfaces this lane owns:** host-local Unix process ownership/supervision;
  runtime process-tree completion evidence only as required; native descendant
  fixtures; process-related Contract 010/019 clauses; card 059 and closeout log
- **Integration ownership:** orchestrator owns final front-door/index merge
  reconciliation with card 058 and PR 188
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/system-architecture.md`;
  `docs/contracts/010-execution-host-services-and-inputs.md` and
  `docs/contracts/019-embedded-sdk-and-cloud-client-boundary.md`
- **Review oracle:** card 059 `## Review Oracle`
- **Model capability profile:** frontier implementation, OS process lifecycle
- **Frontier-worker justification:** exceptional post-planning reasoning is
  required because ordinary Unix process groups, pid probes, and inherited
  descriptors each have adversarial false-positive cases; this is the
  highest-priority material blocker to safely exposing the subscription-backed
  SDK route on macOS and a false attestation can leave provider processes alive
- **Tool/runtime restrictions:** no provider contact, Claude execution, login,
  token access, release commands, broad unsafe, or best-effort positive claim
- **Required validation:** all card 059 validation, native claimed-platform
  counterexamples, API, docs, Northstar, god-files, and diff check
- **PR base/head:** current pushed `main` / `worker/g05-card059-unix-tree-attestation`
- **PR URL:** pending
- **Review state:** awaiting implementation and exact-head frontier review
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** card 059's smallest sound Unix owned-tree observation, including
  one narrowly contained unsafe/dependency boundary if required
- **Out of scope:** caller-bounded close; SDK adapter/PR 188 edits; provider
  execution; release work; weakening Contract 019
- **Outcome shape:** complete positive implementation or an evidence-backed card
  stop proving macOS cannot satisfy the oracle under the authorized boundary
- Do not infer safety from the permission to use unsafe. Descriptor EOF is not
  tree emptiness while a live descendant can close or fail to inherit it.
- This handoff represents one worker lane, and card 058 runs concurrently.
  Stop on an unexpected overlap rather than editing card 058's owned surfaces.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR.

## Important Context

- **Planning lineage:** PR 188 lifecycle review → g05.023 → card 057/PR 189
  root-only evidence → operator authorization for a narrow platform boundary
- **Why these cards are ready:** positive meaning and counterexamples are fixed;
  implementation mechanism remains bounded engineering judgment
- **Decisions and preferences:** prefer no unsafe; if unavoidable, isolate and
  document it; macOS support matters; unsupported is better than false success
- **Open tensions:** a liveness descriptor can be closed or dropped, process
  groups permit `setsid`, and released numeric identities can be reused
- **Report after:** one candidate mechanism has been attacked against all three
  counterexample classes, before broad integration
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight. Reproduce card 057's escaped-descendant
case, inventory native macOS/Linux facilities and dependency cost, then attack
the smallest candidate before integrating it.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`, `git branch
--show-current`, `git status --porcelain`, and `git worktree list --porcelain`.
Accept a clean launcher-provided non-main registered worktree. Otherwise follow
`.agents.local.env`; never guess or clean another worktree.

Fetch origin with a bounded non-interactive SSH command. Prove `HEAD` equals
`origin/main`, the planning base above is an ancestor, and this handoff's tracked
blob equals the dispatch file. Read `AGENTS.md`, g05.023, card 059, card 057's
log, and Contracts 010/019 before editing.

Work in meaningful batches. Preserve unrelated work. Use `apply_patch` for
edits. Run the card's exact validation after the coherent batch. Falsify every
universal/exact claim, especially descriptor-close/non-inheritance, `setsid`,
and identity reuse. Remove temporary instrumentation.

When complete or honestly stopped, reconcile card 059 and its closeout log.
Leave final shared front doors to the orchestrator where the ownership partition
says so. Commit, push, and open one PR against current pushed `main`. Do not
merge.

The orchestrator posts the canonical exact-head review. Requested changes use
the same branch and workspace. Blocking classes are `execution-miss`,
`oracle-gap`, `planning-change`, `validation-gap`, and `integration-drift`.

Closeout refs: card 059, g05.023, Contracts 010/019, architecture, API evidence,
and the lane's dated log.
