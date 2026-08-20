---
title: g04.014 connection-lifecycle consumer path worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-163734-g04-014-connection-lifecycle-consumer-path.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Hosted Anthropic Messages, installed Codex app-server, and local Ollama
attach are on `main`. Contract 057 is realized. A consumer still cannot
follow a Contract 052 path from addable catalog to the existing prepared
facade. Hosted interactive OAuth stays a remaining gate.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without a feature guide, first-proof route amendments, and compiling
examples, the three proofs stay test-only. Contract 052 owns consumer
documentation. The remaining 44 production routes must not be documented
as addable.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e23e795f9039991212f8249a25d3f307cd293fd3`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057; Contract 052;
  Research 169; realized Anthropic, Codex, and Ollama first-proofs;
  completed g04.013; milestone g04.014; ready cards 039-041
- **Worker branch:** `g04-014-connection-lifecycle-consumer-path`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-014-connection-lifecycle-consumer-path`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-014-connection-lifecycle-consumer-path "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-014-connection-lifecycle-consumer-path" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contracts 052 and 057 are the authority.
  Research 169 is evidence, not a contract.
- **Roadmap milestone:** `docs/roadmaps/g04/014-connection-lifecycle-consumer-path.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/039-connection-lifecycle-feature-guide.md`,
  then `docs/roadmaps/g04/batch-cards/040-first-proof-route-guide-amendments.md`,
  then `docs/roadmaps/g04/batch-cards/041-connection-lifecycle-examples-and-guide-map.md`
- **Allowed runway:** g04.014 cards 039 → 040 → 041. Stop after examples
  and the complete guide-map family. Do not start hosted OAuth.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** the feature guide, route guides, examples,
  and checker token share the same documentation surface. No parallel
  lane.
- **Canonical refs:** Contract 052; Contract 057; Contracts 011, 037, 047;
  Research 169; `docs/guides/integration-guide-map.md`;
  `docs/guides/anthropic-direct-prepared-integration.md`;
  `docs/guides/codex-prepared-integration.md`;
  `docs/guides/ollama-attached-prepared-integration.md`;
  `scripts/check-integration-guide-coverage.py`;
  `anthropic_messages_addable_route_descriptor`;
  `codex_app_server_addable_route_descriptor`;
  `ollama_attached_addable_route_descriptor`;
  `AddableRouteCatalog`; `admit_instance`; `refresh_readiness`;
  `observe_authenticated_subject`; `observe_instance_update`;
  `apply_stored_model_presentation_overlay`;
  `MemoryConnectionLifecycleStore`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** no live provider, install, start, pull,
  login, or billing work. No hosted OAuth. No new addable descriptors.
  No OpenHands production route. Do not invent a catalogue
  `provider_id`. No 047 snapshot field additions. No GitHub Release,
  crates.io, or tag mutation. Do not rewrite
  `release-baselines/public-api-0.3.3/`. Do not add a feature-matrix
  column.
- **Required validation:** card 039:
  `effigy qa:docs`, `effigy qa:guides`, `git diff --check`. Card 040:
  `effigy qa:docs`, `effigy qa:guides`, `git diff --check`. Card 041:
  `effigy check:examples`, `effigy qa:guides`, `effigy qa:docs`,
  `git diff --check`.
- **PR base/head:** `main` / selected worker branch
  (`g04-014-connection-lifecycle-consumer-path` unless the launcher
  supplied a different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** Contract 052 feature guide for 057; Key Concepts and
  guides-index links; Anthropic Messages, Codex app-server, and Ollama
  attach route-guide 057 amendments; compiling examples for those three;
  portable feature token `connection_lifecycle`; complete guide-map
  family row; architecture note that the consumer path is realized for
  those three routes only.
- **Out of scope:** hosted interactive OAuth; addable descriptors for
  any other route; replacing prepared-facade examples in the route-map
  example column; a feature-matrix column; OpenHands production wiring;
  inventing a catalogue `provider_id`; adding overlay metadata to 047;
  rewriting `public-api-0.3.3`; GitHub Release; crates.io; tag
  mutation; fixing `start_sign_in` pending-future panic; feeding stored
  `ConfigFieldRef` values into `prepare_*`.
- Do not invent architecture or change Contracts 052 or 057.
- `qa:guides` requires every feature family to be `complete` and every
  machine-readable surface token to match the portable-feature
  inventory. Do not add the family row in cards 039 or 040. Add the
  checker token and the complete family row together in card 041.
- Only `anthropic.messages`, `codex.app-server`, and `ollama.attached`
  export addable descriptors. Do not document `codex.exec` or any other
  production route as addable.
- Topology is hosted / installed / local-runtime. Do not fold it into
  `ExecutionLayer`.
- Examples follow `prepared_direct.rs`: compile-only helpers and
  `fn main() {}`. Use `MemoryConnectionLifecycleStore`. No secret
  bytes. Do not call `start_sign_in` with a pending host future.
- Catalogue rows without `provider_id` stay unmarked.
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
  4-8), Research 169, hosted Anthropic Messages (PR 9), installed Codex
  (PR 10), local Ollama attach (PR 11). The consumer path is the last
  piece of the g04.010 first-proof goal.
- **Why these cards are ready:** the three representative shapes exist
  on `main`. Contract 052 already owns feature guides, route-guide
  amendments, compiling examples, and guide-map coverage. Hosted OAuth
  stays gated.
- **Decisions and preferences:** do not mark all 47 routes as addable.
  Do not replace prepared-facade examples as the canonical route-map
  examples. Do not add a matrix column.
- **Open tensions:** `start_sign_in` still requires immediately-ready
  host futures; do not change that here. `prepare_*` still takes host
  target refs, not stored `ConfigFieldRef` values; examples must not
  pretend otherwise. Codex and Ollama catalogue rows omit
  `provider_id`; leave them unmarked. There is no `.agents.local.env` on
  the planning machine; if the launcher does not supply a worktree, ask the
  operator for `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 039 feature guide; card 040 route amendments;
  card 041 examples, checker token, guide map, and the PR
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

Once that checks out, take card 039 first. Write the connection-lifecycle
feature guide. When 039 is green, continue into 040, then 041.
When examples, the checker token, and the complete family row are green,
open the PR and stop.

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
   `git merge-base --is-ancestor e23e795f9039991212f8249a25d3f307cd293fd3 HEAD`
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
- Do not start hosted OAuth work.

### When the assigned runway is complete

1. Run the required final validation for card 041.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `e23e795f9039991212f8249a25d3f307cd293fd3` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 039-041, Contracts 052 and 057,
   Research 169, changed surfaces, evidence, validation, and unresolved items.
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

- **Closeout refs:** cards 039-041, g04.014, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge. Hosted OAuth stays a
remaining gate.
