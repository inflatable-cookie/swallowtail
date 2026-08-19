---
title: g04.005 connection-lifecycle kernel worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-000805-g04-005-connection-lifecycle-kernel.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

g04 is building a portable library surface so apps can list addable routes,
admit connections, collect credentials, and present the models those
connections can actually run. Contract 057 now owns that lifecycle. `v0.3.3`
is tagged. Nothing in core, runtime, or host-local yet holds the records or
the store.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Consumers cannot start catalog, admission, or sign-in work from types that
do not exist. This kernel is the persistence port and the optional simple
adapter: records, a store trait, and in-memory plus JSON-file implementations.
It is still a library. It is not a connection server.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `2ca191252a275dee177da54b4a88454c39facf61`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057, archived Spec
  011, milestone g04.005, ready cards 013-015, planned g04.006-007
- **Worker branch:** `g04-005-connection-lifecycle-kernel`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-005-connection-lifecycle-kernel`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-005-connection-lifecycle-kernel "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-005-connection-lifecycle-kernel" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Spec 011 is archived at
  `docs/specs/archive/011-route-readiness-and-connection-admission.md`.
  Contract 057 is the authority.
- **Roadmap milestone:** `docs/roadmaps/g04/005-connection-lifecycle-kernel.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/013-lifecycle-core-records.md`,
  then `docs/roadmaps/g04/batch-cards/014-lifecycle-store-port.md`,
  then `docs/roadmaps/g04/batch-cards/015-host-local-simple-store-adapters.md`
- **Allowed runway:** g04.005 cards 013 → 014 → 015. Stop after the simple
  adapters. Do not start cards 016-021.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** core records, runtime store trait, and
  host-local adapters share one type graph. 006-007 depend on these types.
  No parallel lane.
- **Canonical refs:** Contract 057;
  `docs/architecture/system-architecture.md` planned connection-lifecycle
  section; Contracts 006, 008, 014, 047; `docs/contracts/001-working-rules.md`;
  `docs/contracts/036-crate-release-and-compatibility-boundary.md`;
  `scripts/check-public-api.sh`; `release-baselines/public-api-0.3.3/` and
  `release-baselines/public-api-unreleased/`
- **Model capability profile:** capable coding model, medium reasoning.
  Persistence is in scope but bounded: references only, no secret bytes.
- **Tool/runtime restrictions:** no live provider, install, login, or
  billing work. No production adapter crates. No GitHub Release, crates.io,
  or tag mutation. Do not rewrite `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 013:
  `effigy validate:focused swallowtail-core`, `git diff --check`. Card 014:
  `effigy validate:focused swallowtail-runtime`, `git diff --check`. Card
  015: `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`,
  `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local`,
  `effigy package:api`, `git diff --check`.
- **PR base/head:** `main` / selected worker branch
  (`g04-005-connection-lifecycle-kernel` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** portable 057 records in `swallowtail-core`; store trait and
  lifecycle roles in `swallowtail-runtime`; optional in-memory and JSON-file
  adapters in `swallowtail-host-local`; additive unreleased public-API
  snapshots for packages this kernel changes.
- **Out of scope:** addable-route catalog assembly; production adapter
  descriptors; admission API; sign-in loop; new host ports; readiness
  refresh; overlay projection; first-proof Anthropic, Codex, or Ollama
  wiring; a Swallowtail server, keychain, or raw-secret store; cards
  016-021; rewriting `public-api-0.3.3`; GitHub Release; crates.io; tag
  mutation; consumer repository edits.
- Do not invent architecture, change Contract 057, or reopen authenticated
  subject, library-max sign-in, persistence-port, or overlay policy.
- Do not reuse `PlannedConnectionRolloverPolicy`. That is realtime
  connection replacement, not this store.
- Topology grouping is hosted / installed / local-runtime. It is not
  `ExecutionLayer`.
- The store never requires raw secrets. JSON on disk carries references
  only. Enablement does not change 047 `Ready` / `NotReady`. Subject
  records default to redacted and do not enter 047.
- Overlay marker records may exist in the store. Do not implement overlay
  projection or make `NotReady` selectable.
- Additive public API belongs in `release-baselines/public-api-unreleased/`.
  Leave the tagged `v0.3.3` snapshots alone.
- This handoff represents one worker lane. Do not edit another lane's
  assigned scope; if shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  the named fallback created by the startup preflight. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04 inventoried Poodle/T3 surfaces, folded them into
  Spec 011, tagged `v0.3.3` at `51d18620`, and promoted Contract 057. The
  generation runway now realizes the store port first, then catalog and
  admission, then sign-in.
- **Why these cards are ready:** Contract 057 names crate placement and the
  store contents. Operator decisions for subject, sign-in, persistence, and
  overlay are already in the contract. 013-015 do not need a new product
  choice.
- **Decisions and preferences:** Swallowtail is a library. Persistence is a
  port plus an optional simple adapter. Consumers may supply SQLite or
  keychain stores later. Authenticated subject is redacted by default and
  lives on this facade, not in 047.
- **Open tensions:** overlay projection and 047 presentation metadata stay
  later. Do not smuggle them into the kernel beyond stored marker records.
  There is no `.agents.local.env` on the planning machine; if the launcher
  does not supply a worktree, ask the operator for
  `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 013 records compiling; card 014 store trait with
  enablement independence; card 015 adapters plus unreleased API snapshots
  and the PR
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

Once that checks out, take card 013 first. Keep topology off
`ExecutionLayer`, keep secrets and paths out of records, and leave
`PlannedConnectionRolloverPolicy` alone. When 013 is green, continue into
014, then 015. When the simple adapters and unreleased API snapshots are
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
   `git merge-base --is-ancestor 2ca191252a275dee177da54b4a88454c39facf61 HEAD`
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
- Do not start cards 016-021.

### When the assigned runway is complete

1. Run the required final validation for card 015.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `2ca191252a275dee177da54b4a88454c39facf61` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 013-015, Contract 057, changed
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

- **Closeout refs:** cards 013-015, g04.005, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge, then compile or dispatch
g04.006.
