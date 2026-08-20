---
title: g04.008 readiness refresh subject and updates worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-102815-g04-008-readiness-refresh-subject-updates.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Sign-in is on `main`. An admitted instance can hold enablement and
credential references, but consumers still cannot refresh access
dimensions, observe a provider-disclosed subject, or project an update
affordance from existing 029/032 evidence.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without refresh, enablement is the only instance preference and 047 stays
stale until the consumer rebuilds it from nowhere. Subject observation is
the T3 blur/unblur surface. Update observation should reuse currentness
claims, not invent a second system.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `91e14e3d69cf7697b06b5de3dc73b773e7171a7b`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057; realized kernel,
  catalog, admission, and sign-in; milestone g04.008; ready cards 022-024;
  planned g04.009
- **Worker branch:** `g04-008-readiness-refresh-subject-updates`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-008-readiness-refresh-subject-updates`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-008-readiness-refresh-subject-updates "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-008-readiness-refresh-subject-updates" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contract 057 is the authority.
- **Roadmap milestone:** `docs/roadmaps/g04/008-readiness-refresh-subject-and-updates.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/022-readiness-refresh.md`,
  then `docs/roadmaps/g04/batch-cards/023-authenticated-subject-observation.md`,
  then `docs/roadmaps/g04/batch-cards/024-instance-update-observation.md`
- **Allowed runway:** g04.008 cards 022 → 023 → 024. Stop after update
  observation. Do not start cards 025-026 or first-proof work.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** refresh writes access status on the same
  admitted record subject observation will use. Overlay depends on this
  refresh existing. No parallel lane.
- **Canonical refs:** Contract 057 Readiness Refresh, Authenticated Subject,
  and Update Observation; Contracts 006, 008, 029, 032, 047;
  `AdmittedInstanceRecord`; `AccessStatus`; `AuthenticatedSubjectObservation`;
  `InstalledExecutableObservation`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** no live provider probes, install, login, or
  billing work. No overlay projection. No 047 snapshot mutation. No second
  currentness system. No GitHub Release, crates.io, or tag mutation. Do not
  rewrite `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 022:
  `effigy validate:focused swallowtail-runtime swallowtail-host-local`,
  `git diff --check`. Card 023:
  `effigy validate:focused swallowtail-core swallowtail-runtime`,
  `git diff --check`. Card 024:
  `effigy validate:focused swallowtail-core swallowtail-runtime`,
  `git diff --check`. If public types are added, update
  `release-baselines/public-api-unreleased/` and run `effigy package:api`
  before opening the PR.
- **PR base/head:** `main` / selected worker branch
  (`g04-008-readiness-refresh-subject-updates` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** refresh of credential, entitlement, endpoint, runtime, and
  support dimensions for one admitted instance; constructible
  `SubjectDisclosure::Absent`; optional redacted-by-default subject
  observation; update observation derived from 029 claims and 032
  observations.
- **Out of scope:** overlay projection; 047 watcher or in-place mutation;
  emails as instance ids or 047 fields; install/upgrade/authenticate; a
  second currentness system; live provider probes; first-proof adapters;
  cards 025-026; rewriting `public-api-0.3.3`; GitHub Release; crates.io;
  tag mutation.
- Do not invent architecture or change Contract 057.
- Refresh writes `AccessStatus`. It does not write enablement or invent an
  aggregate ready boolean. 047 stays an immutable snapshot the consumer
  replaces after refresh.
- Subject is never a configured-instance id, never a 047 selection field,
  never a default diagnostic, and never a routing key. `Debug` redacts
  revealed values. Adapters must be able to report a field as not disclosed.
- Update observation reuses 029/032. It cannot create a configured instance
  or start sign-in.
- Additive public API belongs in `release-baselines/public-api-unreleased/`.
- This handoff represents one worker lane. Do not edit another lane's
  assigned scope; if shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  the named fallback created by the startup preflight. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04 tagged `v0.3.3`, merged kernel (PR 4), catalog
  (PR 5), and sign-in (PR 6). Refresh, subject, and 029 updates are the next
  generation-runway step. Overlay is g04.009.
- **Why these cards are ready:** Contract 057 already names refresh, subject,
  and update observation. Kernel records exist. `SubjectDisclosure::Absent`
  is currently unused; card 023 exists to make it representable.
- **Decisions and preferences:** enablement stays independent of readiness.
  Subject is blur/unblur presentation, redacted by default.
- **Open tensions:** `start_sign_in` still requires immediately-ready host
  futures; do not change that here. Re-admitting the same instance id still
  overwrites; do not change that here. There is no `.agents.local.env` on
  the planning machine; if the launcher does not supply a worktree, ask the
  operator for `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 022 refresh with enablement unchanged; card 023
  Absent/Redacted/Revealed; card 024 update observation and the PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the
current context is a clean, dedicated, non-`main` registered worktree, it is the
launcher-provided worktree: use it immediately, record its actual path/branch,
and do not compare its generated path/branch with this handoff or create another
worktree. If it is `main`, dirty, unregistered, or otherwise unusable, use the
named worktree if it matches; only then read `.agents.local.env`, require a valid
`AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create a
unique manual worktree and branch under that container from pushed `origin/main`.
Never fall back to `/tmp` or `TMPDIR`. Do not run broad repo orientation before
this decision. Read `AGENTS.md`, the active milestone, each assigned card, and
the canonical architecture/contracts from the selected worker worktree.

Once that checks out, take card 022 first. Refresh access dimensions on one
admitted instance and leave enablement alone. When 022 is green, continue
into 023, then 024. When subject observation and update observation are
green, open the PR and stop.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run one
   quick read-only safety probe before
   broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the placeholders above
   or create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; if it is
   absent, ask the operator before creating the file or worktree. Then create a
   unique worktree and branch under that container from pushed `origin/main`,
   record the actual path and branch, and run all subsequent commands there.
   Never use `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash-over,
   or discard the original checkout's dirty state. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 91e14e3d69cf7697b06b5de3dc73b773e7171a7b HEAD`
   succeeds, and confirm this handoff file exists in the selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run the repo's cheap orientation checks and record what you actually ran.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.
- Do not start overlay projection or first-proof work.

### When the assigned runway is complete

1. Run the required final validation for card 024, plus `effigy package:api`
   if public types were added.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `91e14e3d69cf7697b06b5de3dc73b773e7171a7b` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 022-024, Contract 057, changed
   surfaces, evidence, validation, and unresolved items.
6. Report the PR URL and the evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and checks.
Current review state: awaiting worker PR.

The orchestrator records an evidence-backed verdict in the provider's review
surface. When the orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment; that comment is the canonical review record. If changes are requested,
make only those changes on this branch, push again, and report back through the
operator. Requested changes are: none yet. The PR should
link the card, milestone, spec, changed surfaces, evidence, validation, and
unresolved items. The operator must explicitly authorise any merge.

- **Closeout refs:** cards 022-024, g04.008, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge, then dispatch g04.009
overlay projection.
