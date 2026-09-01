---
title: g05 Card 033 Kimi reassessment worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-103625-g05-card033-kimi-reassessment.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g05, contract-061]
---

## What This Thread Was Doing

The orchestrator completed and reviewed Contract 061 candidate G, merged its
48-row implementation through PR 144, and reconciled Card 032 on `main`.
Planning-only Card 033 is now the sole Next Task. It closes coverage at
249/767 proved rows, then reassesses candidate F's complete 89-row Kimi package
remainder without granting Kimi implementation or public-baseline authority.

This dispatches one bounded worker lane. No transcript or second prompt is part
of the authority chain.

## Why It Matters

Contract 061 must expand package by package without turning documentation,
prepared success, or discarded provider confirmations into active-session
truth. Candidate F is the last lifecycle-priority candidate and couples three
post-open observation families, so its next state must be evidence-led.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `63463d7b0d42f97e58efb58788db648b4e7f3a79`
- **Pushed main verification:** local `main` and `origin/main` both resolved to
  `63463d7b0d42f97e58efb58788db648b4e7f3a79` before this handoff commit
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** completed Card 032, ready Card
  033, reconciled 249/518 front-door state, Batch 9.4 checkpoint, and reviewed
  census
- **Worker branch:** `worker/g05-card033-kimi-reassessment`
- **Worker worktree:** launcher-generated Paseo worktree
- **Worktree creation command:** Paseo `create_workspace`, worktree branch-off
  from pushed `origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** Contract 061 package-coherent realization
- **Roadmap milestone:**
  `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`
- **Ready cards, in order:**
  `docs/roadmaps/g05/batch-cards/033-contract-061-card-032-closeout-and-kimi-reassessment.md`
- **Allowed runway:** Card 033 only
- **Remaining card budget:** one planning card and one reviewable planning PR
- **Dispatch topology:** serial; no papercut or other g05 worker may edit the
  shared roadmap/log/front-door surfaces during this lane
- **Parallel safety check:** shared `docs/roadmaps`, `docs/logs/README.md`, and
  Batch 9.4 checkpoint are reserved to this worker
- **Canonical refs:** Contract 061; Batch 9.4 package expansion; card 030;
  completed cards 031-032; reviewed 767-row census
- **Review oracle:** Card 033 `## Review Oracle`; exact 249/518 reconciliation,
  complete 89-row F remainder, distinct acknowledgement/model/catalogue
  lifecycles, and stop-before-promotion on any unresolved baseline decision
- **Model capability profile:** frontier/high-reasoning worker selected from
  current Paseo profile notes because public API and lifecycle evidence are
  coupled
- **Tool/runtime restrictions:** docs-only planning; no Rust, manifest,
  baseline, contract, architecture, census, provider, live-probe, watcher,
  skill-inventory, currentness, papercut, or Batch 9.5 work
- **Required validation:** `effigy qa:docs`; `effigy qa:northstar`;
  `git diff --check`
- **PR base/head:** current pushed `main` / worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator exact-head review
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** execute every Card 033 scope item and acceptance criterion;
  inspect current Rust read-only to trace Kimi evidence; update only the
  planning, checkpoint, log, index, and front-door surfaces Card 033 names.
- **Out of scope:** modifying Kimi or shared production code; changing public
  baselines; making the Kimi operator decisions; promoting another candidate;
  provider contact; live probes; Batch 9.5.
- **Outcome shape:** one docs-only planning PR. Promote F only if current main
  already passes the fixed rubric without a new decision. Otherwise record an
  evidence stop and name the narrow Kimi-only gate and its unresolved operator
  decisions.
- Do not invent architecture, contract rules, retention semantics, public API,
  bounds, or product policy.
- Do not narrow F around a blocker or omit one of its two packages/four routes.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge. Merge belongs to the orchestrator.

## Important Context

- **Planning lineage:** cards 022-024, 031, and 032 prove candidates A, H, D,
  and G. Card 030 found candidate F blocked on current-main active observation.
  Card 031 changed only Claude Agent; Card 032 changed only Cline, Command
  Code, Copilot CLI, and Goose. Neither grants Kimi authority.
- **Why this card is ready:** its scope, exact row counts, promotion rubric,
  adversarial counterexamples, validation, stops, and one-PR boundary are
  fixed. It may stop; it may not answer a new gate.
- **Decisions and preferences:** keep reasoning acknowledgement, Plan
  acknowledgement, negotiated model options, and provider-session catalogue
  observation distinct. One candidate at a time. Documentation is not runtime
  evidence.
- **Open tensions:** `EffectiveReasoningSetup` cannot currently represent an
  exact rejected value; Kimi may also discard Plan confirmation, model-option
  evidence, and post-open catalogue state. Determine exact current-main truth
  rather than copying Card 030's conclusion.
- **Report after:** one complete candidate disposition and reconciled planning
  PR, or an authority stop that prevents an honest PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then execute Card
033 in order: bind Card 032's merge, re-partition the census, trace candidate
F's current prepared and active evidence, apply the rubric, and reconcile the
single next state.

## Completion Protocol

### Before you start

1. This handoff's worker metadata activates worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree. Record its actual
   root and branch; do not create another because generated names differ.
3. If the current context is unusable, inspect `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask when absent. Never use `/tmp` or
   discard dirty state.
4. Fetch origin with
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm selected `HEAD == origin/main`, confirm planning base
   `63463d7b0d42f97e58efb58788db648b4e7f3a79` is an ancestor, confirm this
   repository-relative handoff exists in `HEAD`, and load it with `git show`.
   Stop if the absolute file differs from the tracked blob.
5. Required sibling links: none.
6. Read `AGENTS.md`, Contract 061, milestone g05.009, Card 033, Batch 9.4,
   Card 030, Cards 031-032, and the reviewed census.
7. Run the repo's cheap orientation checks and record what you ran.

### While you work

- Use `rg` and read-only source inspection to trace Kimi evidence. Do not edit
  production source to discover whether the planning card passes.
- Reconcile exact census sets independently of the checkpoint tables. Name
  every route tuple once and use no exception list.
- Treat every missing active-observation facade or discarded exact value as a
  blocker, not as permission to infer from docs or prepared success.
- If F needs a new public-baseline or operator decision, stop promotion and
  describe the smallest gate precisely. Do not compile its implementation.
- Keep commits aligned with the one coherent planning outcome.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`.
2. Falsify every universal, exact, and negative claim in Card 033. Repartition
   the census, test the 249/518 arithmetic, and challenge every claimed Kimi
   observation against the actual retained source.
3. Check every Card 033 acceptance item honestly. Update Card 033, Batch 9.4,
   g05.009, g05 and generation front doors, batch-card index, one closeout log,
   logs index, and the sole Next Task. Do not edit unrelated triage notes.
4. Push the worker branch and open one reviewable docs-only PR against current
   pushed `main`.
5. Report exact head/base, candidate F disposition, counts, changed files,
   evidence, validation, unresolved decisions, and PR URL. Do not merge.

### Review and merge path

The orchestrator will review the exact head against Card 033 and current main.
If changes are requested, repair only the posted in-bounds findings on this
branch. Requested changes are: none.

- **Closeout refs:** Card 033; Card 032 closeout; Batch 9.4 checkpoint;
  g05.009; roadmaps/g05/generation/batch-card front doors; one log and index

### Handoff closeout

Leave one honest next state. An unresolved Kimi baseline means strict pause and
an operator gate, not a speculative implementation card. A passing candidate
means one exact implementation card and no authority for any other candidate.
