---
title: g04.007 sign-in loop and host ports worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-095258-g04-007-sign-in-loop-and-host-ports.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Catalog and admission are on `main`. Apps can list addable routes and write
an `AdmittedInstanceRecord`, but they still cannot collect an API key or
run interactive OAuth through host ports. Contract 008 `SignInAction` is
only an advertisement. ACP `authenticate` is not login.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without a library-owned sign-in loop, every consumer has to invent browser
placement, device-code display, and API-key collection. Swallowtail should
own start, poll, complete, cancel, and timeout. The host still owns the
browser, keychain, and secret bytes.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `5cdffebbbb66c9f7247d2343b56a9008874be956`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057, realized kernel,
  catalog, and admission; milestone g04.007; ready cards 019-021
- **Worker branch:** `g04-007-sign-in-loop-and-host-ports`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-007-sign-in-loop-and-host-ports`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-007-sign-in-loop-and-host-ports "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-007-sign-in-loop-and-host-ports" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contract 057 is the authority. Contract 010
  already names the interactive sign-in ports.
- **Roadmap milestone:** `docs/roadmaps/g04/007-sign-in-loop-and-host-ports.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/019-interactive-sign-in-host-ports.md`,
  then `docs/roadmaps/g04/batch-cards/020-sign-in-loop.md`,
  then `docs/roadmaps/g04/batch-cards/021-sign-in-fail-closed-and-api-key-collection.md`
- **Allowed runway:** g04.007 cards 019 → 020 → 021. Stop after fail-closed
  API-key collection. Do not compile or start refresh, subject, overlay, or
  first-proof cards.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** host ports, the sign-in loop, and API-key
  collection share `HostServiceKind`, host traits, and credential
  materialization. Refresh depends on this loop. No parallel lane.
- **Canonical refs:** Contract 057 Host Ports and Sign-In Loop;
  Contract 010 Interactive Sign-In Ports; Contracts 006, 014, 015, 017, 047;
  `docs/architecture/system-architecture.md` connection-lifecycle section;
  `swallowtail-core::HostServiceKind`; `swallowtail-runtime` host traits;
  `admit_instance` and `ConnectionLifecycleStore`
- **Model capability profile:** capable coding model, medium reasoning.
  Persistence and credentials are in scope but bounded: references and
  leases only, no secret bytes.
- **Tool/runtime restrictions:** no live provider OAuth, install, login, or
  billing work. No embedding a browser, keychain, or OAuth client secret.
  No ACP `authenticate`. No Contract 017 delegated login as this loop. No
  production adapter crates. No GitHub Release, crates.io, or tag mutation.
  Do not rewrite `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 019:
  `effigy validate:focused swallowtail-runtime swallowtail-host-local`,
  `git diff --check`. Card 020:
  `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit`,
  `git diff --check`. Card 021:
  `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`,
  `git diff --check`. If public types are added, update
  `release-baselines/public-api-unreleased/` and run `effigy package:api`
  before opening the PR.
- **PR base/head:** `main` / selected worker branch
  (`g04-007-sign-in-loop-and-host-ports` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** optional Contract 010 ports for URL open, loopback callback,
  and device-code display; host-local test doubles; library-owned start,
  poll, complete, cancel, and timeout for interactive OAuth, device OAuth,
  and delegated CLI login through those ports; fail-closed when a required
  port is missing; API-key collection through credential-field descriptors
  into an opaque `CredentialRef` stored by the 057 store.
- **Out of scope:** embedding a browser, keychain, or OAuth client secret;
  live provider OAuth; extracting harness secrets; first-proof Anthropic
  subscription wiring; ACP `authenticate`; Contract 017 delegated login as
  this facade; readiness refresh; overlay projection; cards after 021;
  rewriting `public-api-0.3.3`; GitHub Release; crates.io; tag mutation;
  consumer repository edits; putting emails or tokens into 047.
- Do not invent architecture or change Contract 057.
- New host service kinds must not collapse into Credential, Process, or
  Network. Spawning an approved login helper stays process authority.
  Registering a port must not start sign-in. Ports never return secret
  bytes to portable records.
- `SignInAction` remains an advertisement, not permission to execute.
- Success only materializes a `CredentialRef` for the same route and
  audience that started the loop. Contract 014 still owns acquire, audience
  binding, redaction, and awaited release. The 057 store holds the
  reference, not the secret.
- A loop that would change mechanism, account, endpoint audience, or
  billing authority fails closed.
- Tests are deterministic with mock ports. Do not require a live provider
  to pass.
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

- **Planning lineage:** g04 promoted Contract 057, tagged `v0.3.3` at
  `51d18620`, merged the kernel in PR 4, and merged catalog/admission in
  PR 5. Sign-in is the next generation-runway step. Refresh, subject, and
  overlay compile after this milestone closes.
- **Why these cards are ready:** Contract 010 already names the ports.
  Contract 057 already names the loop. Catalog and admission exist, so
  API-key collection has a store to write a `CredentialRef` into. No new
  product choice is required.
- **Decisions and preferences:** library-max, host-executed. Swallowtail
  owns the loop. The host opens the URL, binds loopback, displays a device
  code, or spawns an approved helper.
- **Open tensions:** re-admitting the same instance id currently overwrites;
  do not change that here unless a sign-in complete path must fail closed
  on collision — if so, stop and report. `SubjectDisclosure::Absent` is
  still unused; do not expand subject observation. There is no
  `.agents.local.env` on the planning machine; if the launcher does not
  supply a worktree, ask the operator for `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 019 ports and test doubles; card 020 loop with
  mock ports; card 021 missing-port fail-closed, API-key collection, and
  the PR
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

Once that checks out, take card 019 first. Add URL, loopback, and device-code
ports that do not collapse into Credential, Process, or Network. Prove that
registering them does not start sign-in. When 019 is green, continue into
020, then 021. When fail-closed missing ports and API-key collection are
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
   `git merge-base --is-ancestor 5cdffebbbb66c9f7247d2343b56a9008874be956 HEAD`
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
- Do not start refresh, subject, overlay, or first-proof work.

### When the assigned runway is complete

1. Run the required final validation for card 021, plus `effigy package:api`
   if public types were added.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `5cdffebbbb66c9f7247d2343b56a9008874be956` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 019-021, Contract 057, changed
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

- **Closeout refs:** cards 019-021, g04.007, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge, then compile readiness
refresh, subject observation, and overlay projection.
