---
title: g04.035 Cursor headless model parameters worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-085358-g04-035-cursor-headless-model-parameters.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, cursor, per-route-features]
---

## What This Thread Was Doing

The orchestrator resumed the sole Next Task after Gemini currentness landed. It
rechecked the per-route feature programme, Cursor's prepared headless path, all
four exact qualified CLI help surfaces, and current official model-parameter
documentation. It then compiled g04.035 and cards 095-097 for the first
route-local feature milestone.

This is one bounded implementation run: freeze exact evidence first, add typed
selection second, prove dispatch and close out third. It stands alone; no copied
transcript or second prompt is needed.

## Why It Matters

Cursor supports Fast, context, and effort as parameters inside `--model`, but
Swallowtail currently exposes only a plain model id. Leaving that gap forces a
consumer either to forgo useful controls or assemble provider grammar outside
the prepared boundary.

The safe fix is deliberately narrow. Cursor's CLI catalogue does not describe
parameter support, and official docs say availability varies by model. This run
must expose only exact evidence-backed tuples, bind them immutably, and keep
provider dispatch distinct from acceptance and effective application.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Repository posture:** strict-ready Northstar
- **Planning branch:** `main`
- **Planning base commit:** `2f6afaa6ee37d68dd873408370dce8e60d336c19`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled
  `2f6afaa6ee37d68dd873408370dce8e60d336c19` before this handoff was created.
  Fetch again at startup; the later main tip contains this handoff.
- **Planning checkout:** clean on `main` after the pushed planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.035; ready cards 095-097;
  the compiled planning log; active per-route programme and Next Task
- **Worker branch:** `g04-035-cursor-headless-model-parameters`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-035-cursor-headless-model-parameters`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-035-cursor-headless-model-parameters "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-035-cursor-headless-model-parameters" origin/main`
- **Worktree policy:** use a clean, dedicated, non-`main` registered worktree
  supplied by the launcher even if its generated path or branch differs from
  these placeholders. Record the actual values and do not create another
  worktree. If the current context is unusable, use the named worktree when it
  matches; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if it is absent. Never
  use `/tmp`, `TMPDIR`, or a guessed path for the worktree.
- **Active spec lane:** none. The promoted per-route programme and Contracts
  037 and 040 own the delivery boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g04/035-cursor-headless-model-parameters.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/095-cursor-headless-model-parameter-evidence.md`,
  `docs/roadmaps/g04/batch-cards/096-cursor-headless-model-parameter-binding.md`,
  then
  `docs/roadmaps/g04/batch-cards/097-cursor-headless-model-parameter-acceptance.md`
- **Allowed runway:** exact evidence, typed preparation, deterministic
  dispatch, and closeout for Cursor headless `fast`, `context`, and `effort`;
  one PR
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; card 095 defines the exact tuple allowlist
  consumed by 096 and 097
- **Parallel safety check:** all cards share Cursor fixtures, prepared
  selection, plan/driver validation, docs, matrices, and closeout state. No safe
  split.
- **Canonical refs:** `AGENTS.md`;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/triage/2026-08-21-advanced-route-features.md` (promoted);
  `docs/architecture/system-architecture.md`; Contracts 005, 020, 029, 037,
  040, and 052; Research 075, 077, 087, and 135;
  `docs/guides/cursor-prepared-integration.md`;
  `docs/guides/provider-route-matrix.md`;
  `docs/guides/provider-solution-feature-matrix.csv`
- **Qualified Cursor builds:** exact `2026.07.01-41b2de7`,
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and
  `2026.08.11-e8db854`; no inferred gap
- **Observed syntax:** every qualified binary currently gives the same quoted
  example:
  `claude-opus-4-8[context=1m,effort=high,fast=false]`
- **Observed official-doc boundary:** `fast`, `context`, and `effort` are model
  parameters; available ids and values vary by model. The SDK can return
  dynamic parameter descriptors, but it is a sibling surface, not the selected
  CLI catalogue.
- **Model capability profile:** capable coding model with medium or higher
  reasoning; frontier review for public API or Contract 040 ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; official artifacts may
  be downloaded only to `/tmp`; do not send a prompt, authenticate, inspect
  account state, invoke the live catalogue, install, or update the host
- **Known repository health:** `effigy doctor` has inherited god-file and stale
  graph findings already tracked in `PAPERCUTS.md`. Record the current output;
  do not duplicate an existing papercut.
- **Planning validation:** `effigy qa:northstar`; logs, roadmaps, g04, and
  batch-card index gates; roadmaps next-action gate; `git diff --check`
- **Required final validation:** `cargo fmt -p
  swallowtail-adapter-cursor`; focused and affected-package gates for
  `swallowtail-adapter-cursor`; `effigy check:examples`; `effigy qa:routes`;
  `effigy qa:northstar`; research, logs, roadmaps, g04, and batch-card index
  gates; roadmaps next-action gate; `effigy package:api`; `git diff --check`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate
  operator-authorised action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** exact secret-free CLI/documentation evidence and Research 183;
  a model-specific tuple allowlist; adapter-local typed Fast/context/effort
  input; canonical parameter rendering; immutable plan/request/argv binding;
  deterministic failure tests; current architecture, guide, route/feature
  matrices, changelog, logs, cards, programme, roadmap, and Next Task closeout.
- **Out of scope:** generic parameter maps; arbitrary names or values; inferred
  catalogue support; provider acceptance or effective-value claims; Cursor ACP
  or catalogue changes; sandbox, force, ask mode, session management; another
  route family; version-ceiling changes; live provider work; workspace `qa`,
  broad `qa:docs`, MSRV, consumer, release, or publication checks.
- Fast and context stay Cursor-local selected-model parameters. Do not create a
  portable speed or context-size control.
- Bind effort to portable `ReasoningSelection` only for exact tuples qualified
  by Research 183. Request policy, plan constraint, rendered model id, and argv
  must agree under Contract 040.
- Preserve `CursorHeadlessModelSelection::new` and valid plain-model argv.
  Reject caller-assembled bracket, comma, or equals grammar before host work.
- The selected CLI catalogue exposes base ids, not parameter descriptors. Do
  not treat the SDK's dynamic catalogue as evidence that the CLI route or an
  arbitrary account supports a tuple.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved public API or compatibility decision. Pause on a contract gap.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** Research 077 qualified the installed headless route;
  Research 087 and 135 extended its exact calendar-build range. The promoted
  advanced-feature inventory ranked Cursor parameters first, and the
  per-route programme requires one route/control family per milestone.
- **Why these cards are ready:** all qualified CLI builds expose the same
  bracket grammar, current official documentation names the three parameter
  families, the prepared path already carries one exact model route into one
  `--model` argv, and Contract 040 defines exact reasoning-control binding.
- **Artifact leads:** the planning pass observed archive SHA-256 values
  `48cbf291c2e28d81b79fa0dcbf18ab50bf4ac7772d0e9ab0948ecbd5f5a29158`
  for July 1,
  `f2eb25851f2079dcdf0558a816e06c402d187abfca93255d35167020439ebbf2`
  for July 23, and
  `46044d6d7bcbd7b49a0cf1cd01aa4ca79aaa2ea5f2c7a32965fc0ebe29841790`
  for August 11. Re-probe and freeze independently; do not trust temporary
  planning directories as corpus.
- **Decisions and preferences:** exact allowlist over permissive parsing;
  preserve the plain path; no generic composer aliases; qualified dispatch is
  useful even when the provider cannot confirm the effective value.
- **Open tensions:** official examples may qualify only a narrow model/value
  subset. That is acceptable. If no useful subset survives exact evidence,
  stop after card 095. Do not widen from a scalar type or dynamic SDK shape.
  If additive typed binding needs a new adapter-private behavior revision,
  explain and test it without changing the Contract 029 ceiling.
- **Report after:** Research 183 and the exact tuple disposition; then typed
  plan binding; then final dispatch/closeout and PR. Report sooner on a stop.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top. Before
broad repository reads, run the startup worktree-safety preflight in
`## Completion Protocol`. Accept a clean launcher-provided non-`main` worktree
as authoritative and do not create a second one.

Once the worktree is safe, read `AGENTS.md`, g04.035, cards 095-097, Contract
040, the Cursor research, prepared guide, and the current Cursor prepared and
driver source. Start with card 095. Freeze the exact allowlist before choosing
public types. Continue to card 096 only when the allowlist is useful and needs
no live provider evidence. Finish all three cards in one PR and stop.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run
   one read-only probe before broad reads: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare it with the placeholders or create
   another worktree because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable
   should you inspect the named worktree. If that cannot be used, read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path;
   never clean, reset, stash over, or discard the original checkout. If the
   launcher supplied a dirty or `main` worktree, stop and report it instead of
   silently creating a second worktree.
4. From the selected worktree, run `git fetch origin`, confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor 2f6afaa6ee37d68dd873408370dce8e60d336c19 HEAD`
   succeeds, and confirm this handoff file exists in `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, canonical refs, and current Cursor
   prepared, selection, validation, command, fixture, and test surfaces.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record what
   ran and distinguish inherited findings from new ones.

### While you work

- Execute card 095, then 096, then 097. Keep commits aligned with evidence,
  typed binding, and acceptance/closeout chunks.
- Use only official, prompt-free sources for the tuple allowlist. Freeze only
  secret-free evidence in the repository.
- Treat every unproved model/value combination as evidence-gated. Do not turn
  SDK descriptor shape or plain CLI catalogue ids into support claims.
- After card 095, report exact artifacts, Research 183, the tuple table, and
  validation. Stop if no useful subset survives.
- After card 096, report the public types, canonical rendering, plan/request
  binding, compatibility result, and validation.
- Stop on moved qualified identity, uncorroborated artifacts, required provider
  work, raw-map pressure, a Contract 040 gap, breaking API pressure, or scope
  expansion.

### When the assigned runway is complete

1. Run every final gate named by card 097 plus any card 095 or 096 gate not
   rerun on the final tree.
2. Complete Research 183, cards 095-097, g04.035, the programme progress,
   architecture, Cursor guide, route and feature matrices, changelog, closeout
   log, indexes, and sole Next Task honestly. Next Task must define the Ollama
   attached `num_ctx` milestone; do not implement it.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is not self-referential.
5. In the PR body, link Contracts 037/040, Research 075/077/087/135/183,
   g04.035, cards 095-097, changed surfaces, exact artifacts/digests, tuple
   dispositions, validation, and unresolved items.
6. Report the PR URL, exact head commit, evidence/claim boundary, and checks to
   the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, evidence,
and checks. Current review state: awaiting worker PR.

The orchestrator and worker share the GitHub identity `betterthanclay`, so the
orchestrator records its verdict as a PR comment rather than formal approval.
Requested changes are: none yet. The operator must explicitly authorise merge.

- **Closeout refs:** Research 183; cards 095-097; g04.035;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/roadmaps/README.md`; `docs/roadmaps/g04/README.md`;
  `docs/logs/README.md`; Cursor current architecture, guide, route matrix,
  feature matrix, and changelog surfaces

### Handoff closeout

If card 095 produces no useful exact tuple, record that stop and leave
production claims unchanged. Otherwise leave evidence, implementation, cards,
milestone, programme, logs, indexes, and Next Task honest. Do not present
provider acceptance or effective values as proved, and do not make the
milestone look merged before orchestrator review and operator authorization.
