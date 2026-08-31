---
title: g05.008 consumer projection contract worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-31
updated: 2026-08-31
handoff_path: /home/box/Dev/projects/swallowtail/docs/handoffs/20260831-005345-g05-008-consumer-projection-contract.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed g05 after two honest Claude watcher evidence stops.
The operator selected the already reviewed consumer route-feature/control
census as the next planning lane, chose one dedicated composing contract, and
deferred a closed availability-reason taxonomy. Spec 012 and g05.008 now bound
one docs-only contract-promotion card.

This dispatches card 021 only. No transcript or second prompt is part of the
authority chain.

## Why It Matters

Consumers need cohesive route, model, feature, and control truth without
adapter downcasts or unsafe inference. Contract 061 must compose existing
evidence without turning descriptive projection into execution or mutation
authority.

## Current State

- **Repository:** `/home/box/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `a12dc5f695dc8ad68e6ec92e89df907b6786c253`
- **Pushed main verification:** base verified equal to `origin/main`; this
  handoff and its planning artifacts must be published by Helm before dispatch
- **Planning checkout:** clean at the recorded base; orchestrator docs edits
  require Helm publication because this session mounts `.git` read-only
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the published handoff head:** Spec 012,
  g05.008, ready card 021, compilation log, and reconciled front doors
- **Worker branch:** `worker/g05-008-consumer-projection-contract`
- **Worker worktree:** launcher-provided dedicated worktree
- **Worktree creation command:** none; Helm's launcher owns initial worktree
  selection, with `.agents.local.env` only as the worker's manual fallback
- **Required sibling worktree links:** none
- **Active spec lane:** `docs/specs/012-consumer-route-feature-and-control-projection.md`
- **Roadmap milestone:** `docs/roadmaps/g05/008-consumer-route-feature-and-control-projection.md`
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/021-consumer-route-feature-and-control-projection-contract.md`
- **Allowed runway:** Contract 061 creation, contract index/summaries/front
  door updates, Spec 012 archive, and exact card/milestone/log/Next Task
  closeout
- **Remaining card budget:** one card; one reviewable PR
- **Dispatch topology:** serial single-card lane
- **Parallel safety check:** no other worker is authorized; do not touch
  watcher, skill-visibility, currentness, PR 127, or parked work
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/037-prepared-consumer-integration.md`,
  `docs/contracts/047-configured-provider-instance-catalogue.md`, and
  `docs/contracts/057-route-readiness-and-connection-admission.md`
- **Review oracle:** Spec 012 and card 021
- **Model capability profile:** frontier worker with high reasoning; this is a
  public-boundary and exact negative-claim contract
- **Tool/runtime restrictions:** documentation only; no provider contact, live
  probes, installs, external research, code, manifests, or public API baselines
- **Required validation:** `effigy qa:docs`, `effigy qa:northstar`,
  `git diff --check`, and documentation-only changed-path proof
- **PR base/head:** published `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** withheld until orchestrator review, required checks,
  and a merge-authorized GitHub verdict; the worker never merges

## Boundaries

Please keep this run inside card 021:

- **In scope:** the one docs-only Contract 061 promotion and exact closeout
  surfaces named by card 021
- **Out of scope:** Rust or manifests; amendments to Contracts 037, 047, or
  057; architecture realization; implementation roadmaps; exhaustive
  availability taxonomy; provider work; PR 127; watcher retry, Darwin,
  fallback, or another Claude turn
- **Outcome shape:** one reviewable contract-promotion PR, then stop for
  orchestrator reassessment
- Do not invent architecture, widen the roadmap, or choose a new product/API,
  persistence, security, or lifecycle decision.
- This handoff represents one worker lane. Stop if another lane creates shared
  mutable scope.
- Work only in the clean worker worktree selected by the Completion Protocol.
- Do not merge the PR.

## Important Context

- **Planning lineage:** the reviewed 767-row census, post-card-020
  reassessment, operator decisions, Spec 012, g05.008, and card 021
- **Why the card is ready:** every product fork is settled; the spec names the
  exact sources, three views, authority boundary, acceptance, stop conditions,
  and adversarial review oracle
- **Decisions and preferences:** one dedicated Contract 061; no amendments to
  037/047/057; source dimensions plus bounded safe reasons; no closed reason
  enum; implementation remains unplanned
- **Open tensions:** Rust naming and placement are intentionally later. If
  normative contract wording requires them or another product choice, stop and
  return to planning.
- **Report after:** the contract and index/spec promotion surfaces form one
  coherent draft and again after final validation/PR creation
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the Completion Protocol preflight before broad reads. Then read `AGENTS.md`,
Spec 012, g05.008, card 021, and Contracts 037/047/057. Draft Contract 061 from
the spec, map each review-oracle counterexample to a normative failure point,
and keep every implementation question out of the diff.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as launcher-provided. Record its actual
   root/branch; do not create another because its names differ from this
   handoff.
3. If the current context is `main`, dirty, unregistered, or unusable, inspect
   the named context. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique fallback there from pushed `origin/main`. Never use `/tmp` or a
   guessed path; never clean, reset, or discard dirty state. Report a
   launcher-supplied dirty or `main` worktree instead of creating another.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor a12dc5f695dc8ad68e6ec92e89df907b6786c253 HEAD`,
   and confirm the handoff exists in `HEAD`. Load it with `git show
   HEAD:docs/handoffs/20260831-005345-g05-008-consumer-projection-contract.md`.
   If the absolute file differs, stop. The tracked copy is canonical.
5. Required sibling links are `none`.
6. Read the active milestone, card, spec, `AGENTS.md`, and canonical refs.
7. Run the repo's cheap orientation checks and record what you ran.

### While you work

- Execute card 021 only and keep the commit aligned with that one coherent
  docs batch.
- Stop if a contract is missing, intent becomes ambiguous, scope expands, or
  validation changes the plan.
- Do not quietly turn a naming or implementation question into architecture.
- Report the coherent draft and the final validated PR state through the
  operator.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`, and prove
   every changed path is documentation.
2. Try to falsify the diff against card 021. Exercise every review-oracle
   counterexample and map it to a normative Contract 061 clause and acceptance
   row. Reconcile card, roadmap, log, handoff, spec, contract indexes, and sole
   Next Task. Return any new product threshold or acceptance rule to planning.
3. Mark card 021 and g05.008 honestly, archive Spec 012 only after Contract 061
   is active, and write the contract-promotion closeout log.
4. Push the selected worker branch and open one reviewable PR against current
   pushed `main`.
5. Link the spec, milestone, card, changed surfaces, census evidence,
   validation, and unresolved items in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR against Spec 012, card 021, the census, and
Contracts 037/047/057. Current review state: awaiting worker PR.

If the orchestrator and worker share a GitHub identity, the orchestrator posts
the verdict as a PR comment. A `planning-change` returns here before revision.
The operator must explicitly authorise merge, and Helm merges only after a
merge-authorized GitHub verdict following checks and review.

- **Requested changes:** none
- **Closeout refs:** card 021, g05.008, contract-promotion log, specs/contracts
  indexes, g05/generation indexes, and the roadmaps Next Task

### Handoff closeout

Leave card, milestone, log, and Next Task honest. If the contract cannot stay
inside Spec 012, record the blocker and stop rather than widening the lane.
