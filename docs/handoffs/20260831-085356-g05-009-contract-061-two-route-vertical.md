---
title: g05.009 Contract 061 two-route vertical worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: worker-in-flight
owner: Tom
created: 2026-08-31
updated: 2026-08-31
handoff_path: /home/box/Dev/projects/swallowtail/docs/handoffs/20260831-085356-g05-009-contract-061-two-route-vertical.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator compiled Contract 061 realization planning from the reviewed
767-row census. The operator selected runtime-owned composition, immutable
adapter contributions, fixed library bounds, and a Codex plus OpenAI Realtime
first proof. Batch 9.1 then closed the shared baseline and one final Realtime
API fork: an additive adapter-owned prepared-open result preserves exact
`session.updated` truth while the existing `open_session` remains unchanged.

This dispatches card 022 only. No transcript or second prompt is part of the
authority chain.

## Why It Matters

Consumers need one bounded projection of exact route-feature and lifecycle
control truth without adapter downcasts or inferred provider state. The first
implementation must prove both a four-lifecycle route and an exact
acknowledgement route; the common kernel or either route alone is insufficient.

## Current State

- **Repository:** `/home/box/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `be98c30d682bea9ab01c5fa5e9af46e7180d4fbc`
- **Pushed main verification:** Helm published the planning batch at
  `693e75352701eaae13e3642ffd369936a591c682`; `HEAD == origin/main` was
  verified before this in-flight status update
- **Planning checkout:** the tracked handoff at the published head is canonical;
  this session can write docs but not `.git`
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the published handoff head:** completed
  Batch 9.1 gate, ready g05.009/card 022, reconciled front doors, and this
  handoff
- **Worker branch:** `worker/g05-009-contract-061-two-route-vertical`
- **Worker worktree:** launcher-provided dedicated worktree
- **Worktree creation command:** none; Helm's launcher owns initial worktree
  selection, with `.agents.local.env` only as the worker's manual fallback
- **Required sibling worktree links:** none
- **Active spec lane:** none; Contract 061 is active
- **Roadmap milestone:**
  `/home/box/Dev/projects/swallowtail/docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`
- **Ready cards, in order:**
  `/home/box/Dev/projects/swallowtail/docs/roadmaps/g05/batch-cards/022-contract-061-composer-and-two-route-vertical.md`
- **Allowed runway:** card 022's four-package runtime/testkit plus exact
  `codex.app-server` and `openai.realtime` 51-row tranche
- **Remaining card budget:** one card; one reviewable PR
- **Dispatch topology:** serial single-card lane
- **Parallel safety check:** no other worker is authorized; do not touch
  watcher, skill visibility, currentness, package expansion, PR 127, or parked
  work
- **Canonical refs:**
  `/home/box/Dev/projects/swallowtail/docs/contracts/001-working-rules.md`,
  `/home/box/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/home/box/Dev/projects/swallowtail/docs/contracts/047-configured-provider-instance-catalogue.md`,
  `/home/box/Dev/projects/swallowtail/docs/contracts/057-route-readiness-and-connection-admission.md`,
  and
  `/home/box/Dev/projects/swallowtail/docs/contracts/061-consumer-route-feature-and-control-projection.md`
- **Review oracle:** card 022, the Batch 9.1 gate, and the exact census
- **Model capability profile:** Rust public-API and lifecycle-refactor capable;
  concrete worker selection remains operator-owned
- **Tool/runtime restrictions:** no provider contact, live probes, external
  research, new route, compatibility work, or runtime package outside the four
  named by card 022
- **Required validation:** the eight exact commands in card 022; no live probe
- **PR base/head:** published `main` / selected worker branch
- **PR URL:** pending worker push
- **Review state:** worker in flight; awaiting its PR, checks, and orchestrator
  review
- **Merge authorisation:** withheld until exact-head checks, orchestrator
  review, and a merge-authorized GitHub verdict; the worker never merges

## Boundaries

Please keep this run inside card 022:

- **In scope:** the runtime-owned Contract 061 composer, testkit conformance,
  nine named Codex prepared-facade contribution methods, the preserved and
  additive OpenAI Realtime open paths, and exact 51-row provider-free
  disposition proof; matrix-only rows remain withheld
- **Out of scope:** core or contract changes; another public Realtime route;
  the remaining 716 rows; package expansion; provider contact or live probes;
  watcher or skill-visibility work; generation closeout; PR 127
- **Outcome shape:** one reviewable implementation PR, then stop for the
  orchestrator's two-route checkpoint
- Do not invent architecture, change a fixed maximum or public signature, add
  a registry/callback/downcast/provider payload, or turn projection into
  execution or mutation authority.
- This handoff represents one worker lane. Stop if another lane creates shared
  mutable scope.
- Work only in the clean worker worktree selected by the Completion Protocol.
- Do not merge the PR.

## Important Context

- **Planning lineage:** Contract 061, the realization-readiness inventory, all
  four accepted option 1 decisions, g05.009, the Batch 9.1 gate, and card 022
- **Why the card is ready:** package direction, public names/signatures, source
  identity, fixed maxima, failures, replacement, 51-row coverage, fixtures,
  validation, and stops are all closed; the operator accepted the last
  Realtime seam
- **Decisions and preferences:** preserve existing `open_session`; share one
  private low-level lifecycle; expose only a normalized contribution; rejected
  state needs exact well-formed differing `session.updated` evidence; unknown
  failures carry none
- **Open tensions:** the implementation may discover that a selected public
  signature cannot satisfy a named contract invariant. That is a planning
  stop, not permission to revise the API or narrow the 51-row tranche.
- **Report after:** the runtime/testkit kernel compiles with portable fixtures,
  after both route proofs are coherent, and after final validation/PR creation
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the Completion Protocol preflight before broad reads. Then read `AGENTS.md`,
Contract 061, g05.009, the Batch 9.1 gate, card 022, and the exact census from
the selected worktree. Start with the runtime contribution/composer and
testkit contract surface, then keep the two route proofs in the same card and
PR.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as launcher-provided. Record its actual
   root/branch; do not create another because its generated names differ from
   this handoff.
3. If the current context is `main`, dirty, unregistered, or unusable, inspect
   the named context. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique fallback there from pushed `origin/main`. Never use `/tmp` or a
   guessed path; never clean, reset, stash over, or discard dirty state.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor be98c30d682bea9ab01c5fa5e9af46e7180d4fbc HEAD`,
   and confirm the handoff exists in `HEAD`. Load it with `git show
   HEAD:docs/handoffs/20260831-085356-g05-009-contract-061-two-route-vertical.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling links are `none`.
6. Read the active milestone, card, `AGENTS.md`, and canonical refs.
7. Run the repository's cheap orientation checks and record what you ran.

### While you work

- Execute card 022 only and keep commits aligned with coherent code and proof
  chunks.
- Preserve the existing public Realtime open method and make both methods
  delegate to one private lifecycle.
- Report the named meaningful chunks through the operator with changed files,
  validation run, remaining work, risks, and blockers.
- Stop if a contract is missing, intent becomes ambiguous, scope expands, a
  selected signature or bound cannot hold, or validation changes the plan.
- Do not quietly turn an implementation problem into new architecture.

### When the assigned runway is complete

1. Run every validation command named by card 022. Do not run a live probe.
2. Try to falsify the diff against the card: map every fixed bound and failure
   kind to portable proof, every exact census row to one adapter fixture, and
   every Realtime state to exact acknowledgement evidence. Prove there is no
   claim for the remaining 716 rows.
3. Reconcile card, milestone, batch-card index, g05/generation indexes, and the
   sole Next Task. Stop and return any new product threshold, contract choice,
   or acceptance rule to planning.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. Link Contract 061, g05.009, card 022, the Batch 9.1 gate, census evidence,
   changed surfaces, validation, and unresolved items in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the exact PR head against Contract 061, card 022, the
Batch 9.1 gate, census, diff, and checks. If the shared GitHub identity prevents
formal approval, the orchestrator posts the verdict as a PR comment. A
`planning-change` returns to planning before revision. Helm merges only after a
merge-authorized GitHub verdict following checks and review.

- **Requested changes:** none
- **Closeout refs:** card 022, g05.009, the batch-card and generation indexes,
  and the sole roadmap Next Task

### Handoff closeout

Leave card, milestone, and Next Task honest. If the 51-row tranche cannot stay
inside the selected signatures and Contract 061, record the blocker and stop
rather than narrowing or widening the card.
