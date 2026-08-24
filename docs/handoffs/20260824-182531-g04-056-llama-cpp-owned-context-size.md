---
title: g04.056 llama.cpp owned context-size worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-24
updated: 2026-08-24
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260824-182531-g04-056-llama-cpp-owned-context-size.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining promoted per-route feature inventory
after g04.053 and selected exact `llama-cpp.owned` `b10069` context size.
g04.056 is compiled. Implementation has not started. The ready runway begins
with exact current official and tagged-source evidence; cards 156-157 are
conditional on a non-empty Research 203 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

`llama-cpp.owned` launches exact `llama-server` `b10069` without
`--ctx-size`, leaving context allocation at the model-derived default. The
route already owns the server's artifact, startup, readiness, endpoint, stop,
and release lifecycle, so explicit context size can fit as route-local serving
configuration.

The target is narrow: an exact safe positive selection, unchanged omission,
and honest requested/dispatched/accepted/effective/observed truth. It is not a
portable context capability, message-composer field, attached-inference
control, or guarantee of model/host feasibility.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `522c15816618efe04fb3c50280c651280fb621f0`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `522c15816618efe04fb3c50280c651280fb621f0` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.056, cards 155-157,
  Research 203 reservation, compilation log, route-local closeout reservation,
  corrected triage row, and the sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-056-llama-cpp-context-size-20260824-182531`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-056-llama-cpp-context-size-20260824-182531`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-056-llama-cpp-context-size-20260824-182531 -b agent/g04-056-llama-cpp-context-size-20260824-182531 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion; no spec or contract edit
- **Roadmap milestone:**
  `docs/roadmaps/g04/056-llama-cpp-owned-context-size.md`
- **Ready cards, in order:** card 155, then conditional card 156, then
  conditional card 157
- **Allowed runway:** exact `b10069` owned context-size evidence, then only
  Research 203 deliver-now adapter-local serving binding
- **Remaining card budget:** three serial cards; cards 156-157 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share one prepared input, driver launch path,
  fixture set, guide, research record, and closeout; they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  007, 008, 011, 018, 029, 037, 040, and 052
- **Route identity:** `llama-cpp.owned`, driver
  `swallowtail.llama-cpp.owned-b10069-openai-chat`, axis
  `llama.cpp.owned-runtime`, exact point `b10069-178a6c449`, behavior
  `llama-cpp.owned-openai-chat-b10069`
- **Candidate mapping:** explicit positive `--ctx-size N`; no numeric range is
  prequalified
- **Current mapping:** eleven launch arguments: exact artifact and alias,
  loopback host, port zero, offline, no UI, and no agent; no context flag
- **Model capability profile:** exact-build, evidence-first, fail closed on
  value, build, plan/evidence/specification, driver, launch, readiness,
  observation, or lifecycle ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no model or executable download, installation, server
  launch, model load, credential/account inspection, external inference
  request, browser login, or paid work. Current official public-source and
  exact-tag source inspection are allowed by card 155.
- **Required validation:** card-specific gates plus final `cargo fmt -p
  swallowtail-adapter-llama-cpp`, `effigy validate:focused
  swallowtail-adapter-llama-cpp`, `effigy package:verify-affected
  swallowtail-adapter-llama-cpp`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action
  index gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 376 inherited god-file findings: 330 warnings and
  46 errors; stale graph index; one generated-in-src warning. Keep inherited
  findings separate from lane-created findings.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, and `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-llama-cpp/**` for exact adapter-
  local owned-serving selection, prepared input, immutable safe start evidence
  or specification, configured driver/launch agreement, readiness and
  deterministic lifecycle tests; `docs/guides/llama-cpp-prepared-integration.md`;
  Research 203; g04.056; cards 155-157; the reserved g04.056 route-local
  closeout; applicable package examples, fixtures, and unreleased public-API
  baseline; current official llama.cpp documentation; exact official `b10069`
  source; secret-free parser, parameter, model-clamp, allocation, property,
  readiness, error, and shutdown evidence
- **Out of scope:** explicit zero, negatives, fractions, overflow, or any
  positive domain not admitted by Research 203; portable context or generation
  controls; generic settings maps; `llama-cpp.attached`; reasoning,
  prediction, batching, GPU, threads, parallelism, cache, rope, or other server
  flags; artifact acquisition or mutation; router/persistent/Monkey lifecycle;
  live model work; another build; currentness; `CHANGELOG.md`; shared
  architecture; contracts; route/feature matrices; programme/front doors/
  indexes; release, publication, merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, process, resource, security, or compatibility decision.
- Do not represent context size as Contract 040 `OutputTokenLimit`, a generic
  composer control, or proof of effective allocation or model fit.
- Do not normalize upstream parser breadth into the public API. Research 203
  owns the admitted subset. Caller omission stays no-flag; explicit zero is
  not an omission alias by inference.
- Do not infer effective context from successful startup. Preserve requested,
  dispatched, accepted, effective, and observed states separately.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, the canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 008 and Contract 018 realize exact `b10069`
  owned ephemeral serving. g04.056 adds no runtime, route, artifact, topology,
  or currentness movement. It assesses one native server-start flag.
- **Official evidence:** exact tagged documentation says `-c, --ctx-size N`
  controls prompt context and defaults to `0`, meaning loaded from model.
  Freeze current and tagged bodies plus decisive source paths and digests.
- **Current source truth:** `driver/owned.rs::launch_arguments` emits eleven
  fixed arguments and no context member. `LlamaCppOwnedServingSelection`
  currently carries only artifact and model; prepared evidence carries the
  artifact and plan. The selected value needs an inspectable immutable home
  without creating a generic settings map.
- **Observation burden:** current `/props` decoding retains build, alias,
  template, and modalities but not context size. Exact source may expose more;
  card 155 decides whether to decode it, withhold effective truth, or stop.
- **Domain burden:** classify caller omission, explicit zero, positives,
  negatives, fractions, representation overflow, above-training values, and
  resource-infeasible values separately. Model and host feasibility cannot
  become a universal numeric guarantee.
- **Lifecycle burden:** preserve artifact acquisition before process start,
  bounded endpoint observation, health/properties/catalogue readiness, early-
  failure cleanup, stop/join, endpoint invalidation, and artifact release.
- **Revision burden:** decide whether the exact runtime behavior, driver,
  claim, or configured-instance revision must move. Do not alter Contract 029
  or shared closeout surfaces on the worker branch.
- **Honest stop:** an empty Research 203 deliver-now set is a successful
  evidence result. Mark cards 156-157 blocked, finish the route-local stop
  record, validate, and open the evidence PR.
- **Generation boundary:** do not close or roll over g04. After merge the
  orchestrator reconciles g04.056 and reassesses remaining inventory.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. New-route research and parked families do not pre-empt
  per-route feature work.
- **Known baseline:** do not repair inherited doctor findings unless this lane
  creates distinct friction. Record new recurring Northstar friction in
  `PAPERCUTS.md`.
- **Report after:** card 155's exact domain/application-state decision.
  Continue automatically only for a non-empty deliver-now set and no stop
  condition, then report after the complete cards 156-157 batch.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Start by reading this handoff from the top. Before broad repository reads, run
the quick worktree-safety preflight in `## Completion Protocol`. If the current
context is a clean, dedicated, non-`main` registered worktree, use it
immediately, record its actual path and branch, and do not compare its generated
identity with the fallback above. If it is unusable, use the named worktree if
it matches; only then read `.agents.local.env` and follow its required
container setting. Never fall back to `/tmp` or `TMPDIR`.

Then read `AGENTS.md`, g04.056, cards 155-157, Research 008 and 203, the
llama.cpp prepared guide, exact owned preparation/driver/startup/protocol source
and fixtures, and the canonical contracts from the selected worker worktree.

Take card 155 as one coherent evidence chunk. Use current official docs, exact
official tag source, and deterministic repository evidence; do not launch a
server or load a model. If Research 203 has no deliver-now set, close cards
156-157 as blocked, finish the route-local stop record, validate, and open the
evidence PR. If an exact set survives, execute cards 156-157 in order and open
one implementation PR. At each natural pause, tell the operator what changed,
what validation ran, what remains, and whether a planning decision is needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe: `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list
   --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual path and branch. Do not create another worktree because its name
   differs from this handoff.
3. If the current checkout is `main`, dirty, shared, or otherwise unusable,
   first use the named worker worktree if it already exists and matches. Only
   when it does not, read `.agents.local.env`, require the named container key,
   fetch `origin/main`, and create one unique branch/worktree from the planning
   base. Do not guess a path.
4. Confirm the selected worktree contains planning base
   `522c15816618efe04fb3c50280c651280fb621f0` and is clean before editing. If
   `origin/main` moved, use current pushed main only when it contains that
   planning base; otherwise stop and report divergence.
5. Confirm this handoff file exists in selected `HEAD`, then read the milestone,
   assigned cards, `AGENTS.md`, and canonical refs completely.
6. Run the repo's cheap orientation checks and record what you actually ran.

### Work the cards

1. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan` once at
   startup. Keep inherited doctor findings separate.
2. Read card 155 and its named refs completely. Freeze official/exact-tag
   evidence and promote Research 203. Do not edit production code during the
   evidence card.
3. Report the exact deliver-now or stop table to the operator. Continue to card
   156 only if Research 203 has a non-empty exact set and no stop condition.
4. If continuing, implement cards 156 and 157 as one meaningful code/test/docs
   batch. Preserve every fixed owned-serving boundary.
5. Update only route-local worker surfaces. In the reserved closeout, list the
   shared architecture, Contract 029, route/feature matrix, programme, indexes,
   changelog, milestone, and Next Task changes the orchestrator must apply
   after merge. Do not propose g04 closure.
6. Run the complete card-specific validation once after the coherent batch.
   Record exact pass/fail counts and any inherited baseline.

### PR loop

1. Review `git diff`, `git diff --check`, branch name, and worktree state.
   Commit the worker batch with a concise message. Push the worker branch.
2. Open one PR against current `main`. The PR body must name g04.056, cards
   completed or blocked, Research 203 disposition, exact route/build, selected
   and omitted values, application-state truth, validation, shared-closeout
   delta, and every explicit withhold.
3. Do not merge. Report the PR URL and exact head SHA to the operator for the
   orchestrator's review loop.
4. If review requests changes, keep the same branch, worktree, PR, and lane.
   Fix only in-scope issues, rerun proportionate validation, push, and report
   the new exact head.
5. Do not restack or merge unless the operator explicitly asks in a later
   message. The orchestrator owns exact-head review, CI state, fast-forward
   restacking, merge, and shared closeout.

### Completion report

Return the PR URL, exact head SHA, actual branch/worktree, Research 203
deliver-now or stop table, cards completed/blocked, exact validation, inherited
baseline, shared-closeout delta, and any unresolved decision. Keep the report
glue-light. Do not claim merge, release, currentness movement, generation
rollover, or g04 closure.
