---
title: g04.012 installed Codex app-server worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-152526-g04-012-installed-codex-app-server.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Hosted API-key Anthropic Messages is on `main`. Research 169 mapped Codex
app-server as the installed first-proof: a prepared facade, discovery, and
029/032 classification already exist. A consumer still cannot list or admit
that route through Contract 057. ChatGPT access is cached local login, not
hosted URL-open OAuth.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without an adapter-local installed descriptor, the hosted Anthropic proof
is the only 057 addable row. Codex is the installed Poodle group. The
prepared facade must stay after admission. Tokens must not enter portable
records.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `b5d6a0766bf99853fa485e028f789c32f8001076`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057; Research 169;
  realized Anthropic Messages first-proof; completed g04.011; milestone
  g04.012; ready cards 033-035
- **Worker branch:** `g04-012-installed-codex-app-server`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-012-installed-codex-app-server`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-012-installed-codex-app-server "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-012-installed-codex-app-server" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contract 057 is the authority. Research 169 is
  evidence, not a contract.
- **Roadmap milestone:** `docs/roadmaps/g04/012-installed-codex-app-server.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/033-codex-app-server-addable-descriptor.md`,
  then `docs/roadmaps/g04/batch-cards/034-codex-app-server-admission-and-prepare.md`,
  then `docs/roadmaps/g04/batch-cards/035-codex-app-server-refresh-update-and-subject.md`
- **Allowed runway:** g04.012 cards 033 → 034 → 035. Stop after refresh,
  update observation, and subject. Do not start Ollama or hosted OAuth.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** admission and refresh write the same admitted
  Codex record. No parallel lane.
- **Canonical refs:** Contract 057; Contracts 011, 014, 029, 032, 037, 047;
  Research 169; `codex_app_server_descriptor`; `codex_app_server_claim`;
  `codex_chatgpt_subscription_access_profile`; `prepare_codex`;
  `AddableRouteDescriptor`; `admit_instance`; `refresh_readiness`;
  `observe_instance_update`; `observe_authenticated_subject`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** no live provider, install, login, or
  billing work. No hosted OAuth. No Anthropic Messages edits. No Ollama
  descriptors. No OpenHands production route. Do not invent a catalogue
  `provider_id`. Do not extract ChatGPT tokens. No 047 snapshot field
  additions. No GitHub Release, crates.io, or tag mutation. Do not rewrite
  `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 033:
  `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime`,
  `git diff --check`. Card 034:
  `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime swallowtail-host-local`,
  `git diff --check`. Card 035:
  `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime swallowtail-testkit`,
  `git diff --check`. If public types are added, update
  `release-baselines/public-api-unreleased/` and run `effigy package:api`
  before opening the PR.
- **PR base/head:** `main` / selected worker branch
  (`g04-012-installed-codex-app-server` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** adapter-local installed addable descriptor for
  `codex.app-server`; opaque binary-path and env config fields; 057
  admission of the ChatGPT subscription profile with no `CredentialRef`;
  reuse of `prepare_codex(AppServer)`; host-supplied refresh; 029/032
  update observation; subject Absent.
- **Out of scope:** hosted interactive OAuth; Anthropic Messages; Ollama;
  OpenHands production wiring; live login or install probes; extracting
  ChatGPT tokens; inventing a catalogue `provider_id`; adding overlay
  metadata to 047; rewriting `public-api-0.3.3`; GitHub Release; crates.io;
  tag mutation.
- Do not invent architecture or change Contract 057.
- Topology is installed. Do not fold it into `ExecutionLayer`.
- Discovery of the executable stays Contract 008. The addable row does not
  run discovery.
- ChatGPT admission does not register URL-open, loopback, or device-code
  ports. Missing those ports must not fail this path.
- Catalogue rows without `provider_id` stay unmarked. Do not invent a
  provider id so overlay can key.
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

- **Planning lineage:** g04 tagged `v0.3.3`, facade through overlay (PRs
  4-8), Research 169, hosted Anthropic Messages (PR 9). Installed Codex is
  the next representative shape.
- **Why these cards are ready:** Research 169 mapped the prepared facade,
  ChatGPT profile, and 029/032 claims onto 057. Hosted OAuth stays gated.
- **Decisions and preferences:** do not extract secrets. Do not treat this
  ChatGPT path as hosted URL-open OAuth.
- **Open tensions:** `start_sign_in` still requires immediately-ready host
  futures; do not change that here. Codex catalogue rows omit `provider_id`;
  leave them unmarked. There is no `.agents.local.env` on the planning
  machine; if the launcher does not supply a worktree, ask the operator for
  `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 033 descriptor; card 034 admission and prepare;
  card 035 refresh/update/subject and the PR
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

Once that checks out, take card 033 first. Ship the installed Codex
app-server addable descriptor. When 033 is green, continue into 034, then
035. When refresh, update observation, and subject are green, open the PR
and stop.

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
   `git merge-base --is-ancestor b5d6a0766bf99853fa485e028f789c32f8001076 HEAD`
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
- Do not start Ollama or hosted OAuth work.

### When the assigned runway is complete

1. Run the required final validation for card 035, plus `effigy package:api`
   if public types were added.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `b5d6a0766bf99853fa485e028f789c32f8001076` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 033-035, Contract 057, Research
   169, changed surfaces, evidence, validation, and unresolved items.
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

- **Closeout refs:** cards 033-035, g04.012, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge, then compile the next
first-proof route.
