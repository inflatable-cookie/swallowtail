---
title: g04.021 unmarked overlay rows worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260821-075348-g04-021-unmarked-overlay-rows.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

g04.020 config-ref prepare handoff is on `main`. Overlay markers still
key to configured-instance, provider, and model ids. Catalogue rows that
omit `provider_id` stay unmarked. Codex, Claude Agent ACP, Ollama, and
llama.cpp attached hit that gap. Inventing a catalogue `provider_id` is
forbidden.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without a named rule, overlay silently cannot mark the installed and
local-runtime catalogues the second-proof just admitted. The allowed
change is overlay keying, not catalogue identity invention.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `3d7616555a94233b8d03a5f3f20382b6a62a084c`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057 overlay
  section; Contract 047 provider id on catalogue entries; completed
  g04.020; compiled g04.021-023; ready cards 059-061
- **Worker branch:** `g04-021-unmarked-overlay-rows`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-021-unmarked-overlay-rows`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-021-unmarked-overlay-rows "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-021-unmarked-overlay-rows" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contracts 020, 047, and 057 are the
  authority.
- **Roadmap milestone:** `docs/roadmaps/g04/021-unmarked-overlay-rows.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/059-unmarked-overlay-classification.md`,
  then `docs/roadmaps/g04/batch-cards/060-unmarked-overlay-rule.md`,
  then `docs/roadmaps/g04/batch-cards/061-unmarked-addable-overlay-proof.md`
- **Allowed runway:** g04.021 cards 059 → 060 → 061. Stop after the
  unmarked addable catalogues follow the chosen rule. Do not start 022
  or 023.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** overlay kernel and the four unmarked
  adapters share the same marker key. No parallel lane.
- **Canonical refs:** Contracts 020, 047, 057; `OverlayMarker`;
  `apply_stored_model_presentation_overlay`; Codex, Claude Agent, Ollama,
  and llama.cpp attached catalogues; Anthropic and DeepSeek `provider_id`
  rows
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** no live provider, install, login, or
  catalogue probes. No hosted OAuth. No OpenHands production route. Do
  not invent a catalogue `provider_id`. Do not flatten mixed gateway
  rows. Do not change 047 `Ready` / `NotReady`. Do not start g04.022
  addable inventory or g04.023 047 fields. No GitHub Release, crates.io,
  or tag mutation. Do not rewrite `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 059: `effigy qa:docs:index:logs`,
  `git diff --check`. Card 060:
  `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`,
  `git diff --check`, and `effigy package:api` if public types are added.
  Card 061:
  `effigy validate:focused swallowtail-adapter-codex swallowtail-adapter-claude-agent swallowtail-adapter-ollama swallowtail-adapter-llama-cpp`,
  `git diff --check`. If public types are added, update
  `release-baselines/public-api-unreleased/` before opening the PR.
- **PR base/head:** `main` / selected worker branch
  (`g04-021-unmarked-overlay-rows` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** classify unmarked-row overlay; realize the chosen rule;
  prove it on Codex, Claude Agent, Ollama, and llama.cpp attached;
  leave Anthropic and DeepSeek provider-id keying unchanged.
- **Out of scope:** inventing a catalogue `provider_id`; 047 presentation
  metadata; new addable routes; hosted OAuth; OpenHands production
  wiring; rewriting `public-api-0.3.3`; GitHub Release; crates.io; tag
  mutation.
- Do not invent architecture. Preferred direction: overlay keys instance
  plus model when `provider_id` is absent. Stop and ask if that would
  flatten mixed gateway rows.
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

- **Planning lineage:** six addable routes, config-ref prepare handoff on
  `main`, OAuth parked, currentness standing, g04 continues toward 30-50.
- **Why these cards are ready:** 057 overlay already keys exact instance,
  provider, and model ids. 047 forbids inventing a provider id. Card 059
  classifies before any overlay edit.
- **Decisions and preferences:** do not invent `provider_id`. Preferred
  direction is instance-plus-model keying when provider id is absent.
  Mixed gateway rows stay consumer assembly. Ready/NotReady unchanged.
- **Open tensions:** if classification is still forked after card 059,
  stop and ask before card 060. There is no `.agents.local.env` on the
  planning machine; if the launcher does not supply a worktree, ask the
  operator for `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 059 classification; card 060 rule; card 061
  proof and the PR
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

Once that checks out, take card 059 first. Classify unmarked overlay.
When 059 is green and the choice is not forked, continue into 060, then
061. When the unmarked addable catalogues follow the chosen rule, open
the PR and stop.

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
   `git merge-base --is-ancestor 3d7616555a94233b8d03a5f3f20382b6a62a084c HEAD`
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
- Do not start g04.022, g04.023, or hosted OAuth work.

### When the assigned runway is complete

1. Run the required final validation for card 061, plus `effigy package:api`
   if public types were added.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `3d7616555a94233b8d03a5f3f20382b6a62a084c` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 059-061, Contracts 020/047/057,
   changed surfaces, evidence, validation, and unresolved items.
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

- **Closeout refs:** cards 059-061, g04.021, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge. Do not start 022-023.
Hosted OAuth stays parked.
