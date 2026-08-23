---
title: g04.049 OpenAI Background service-tier worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-162630-g04-049-openai-background-service-tier.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed g04.048 after PR 47, resumed the sole roadmap Next
Task, reassessed the remaining promoted per-route feature inventory, and
compiled g04.049. OpenAI Background service-tier implementation has not
started. The ready runway begins with exact current official and repository
evidence and permits binding only for Research 196 deliver-now values and
lifecycle profiles.

Bedrock Runtime service-performance controls were assessed first but not
selected. Its resolved Cargo SDK and qualified public SDK identity disagree,
and the exact-pin currentness rule requires explicit operator authority before
reopening that family. This lane must not absorb that correction.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`openai.background` already fixes exact model `gpt-5.6`, exact Responses
facade, reasoning, output bounds, structured output, temporary retention, one
stream reattachment, cancellation, deletion, controlled detachment, and
exact-run reconciliation. It does not expose the Responses `service_tier`
request or returned processing tier.

Current official OpenAI documentation puts `service_tier` on the same create
request and returned/retrieved Response object. Omission behaves as project-
configured `auto`; explicit values include standard, Flex, Fast/Priority, and
access-controlled Ultrafast processing. The returned value may differ from the
request. This lane must qualify exact value, access, observation, and lifecycle
truth before adding a route-local control.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `e6960758999d45423518179773350589150f0fdb`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `e6960758999d45423518179773350589150f0fdb` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:** g04.049, cards 136-138, Research 196
  reservation, compilation log, closeout reservation, triage selection,
  PAPERCUTS baseline note, and updated sole Next Task
- **Worker branch:** `agent/g04-049-openai-background-service-tier-20260823-162630`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-049-openai-background-service-tier-20260823-162630`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-049-openai-background-service-tier-20260823-162630 -b agent/g04-049-openai-background-service-tier-20260823-162630 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator if the file or key is absent. Never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** per-route feature completion; no spec or contract edit
- **Roadmap milestone:** `docs/roadmaps/g04/049-openai-background-service-tier.md`
- **Ready cards, in order:**
  `136-openai-background-service-tier-evidence.md`, then conditional
  `137-openai-background-service-tier-binding.md`, then conditional
  `138-openai-background-service-tier-acceptance.md`
- **Allowed runway:** exact `openai.background` service-tier evidence, then
  only Research 196 deliver-now adapter-local binding
- **Remaining card budget:** three cards; cards 137-138 execute only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; every card shares OpenAI
  Background prepared input/plan/evidence, request encoding, response parsing,
  lifecycle fixtures, guide, research, and closeout. Do not use internal
  subagents; report through the operator.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  009, 011, 014, 021, 029, 037, 040, 048, 049, and 052
- **Route identity:** `openai.background`, driver
  `swallowtail.openai.background`, model route
  `openai.public.gpt-5.6.background`, model `gpt-5.6`, axis
  `openai.responses-background-facade`, current facade
  `openai-responses-background-2026-08-23`, behavior
  `openai.responses-background-v2`, claim
  `openai.responses-background-window-1`
- **Candidate mapping:** optional adapter-local typed selection to exact
  Responses `service_tier`; the complete current request and returned-response
  domains, aliases, access gates, and lifecycle profiles remain card 136
  decisions
- **Model capability profile:** exact-model, exact-facade, evidence-first;
  fail closed on value, alias, access, model, facade, plan/evidence, request,
  response, observation, reattachment, detachment, reconciliation, or control-
  composition ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account/project
  inspection, provider request, live OpenAI call, browser login, or paid work.
  Current official public-source inspection and secret-free deterministic
  repository fixtures are allowed by card 136.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-openai`, `effigy validate:focused
  swallowtail-adapter-openai`, `effigy package:verify-affected
  swallowtail-adapter-openai`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 374 inherited god-file findings: 329 warnings and
  45 errors, plus one generated-in-src warning. A graph-stale warning was also
  observed during planning. Keep inherited findings separate from lane-created
  findings.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-openai/**` for exact adapter-local
  prepared selection, evidence, driver/request binding, response parsing, and
  tests; `docs/guides/openai-background-prepared-integration.md`; Research
  196; g04.049; cards 136-138; the reserved g04.049 route-local closeout;
  applicable `swallowtail-adapter-openai` unreleased public-API baseline;
  current official OpenAI Responses create/retrieve/streaming/background/
  service-tier and exact-model sources; deterministic secret-free request,
  response, reasoning, structured-output, reattachment, cancellation,
  deletion, detachment, reconciliation, failure, and cleanup fixtures
- **Out of scope:** a portable Fast, speed, priority, service-tier, or quality
  capability; shared generation-control field or provider-settings map;
  shared checkpoint/contract changes; Codex Fast; Chat Completions; Batch;
  Realtime; another model or route; project-setting mutation; tier enrollment;
  quota/account inspection; capacity purchase; cost or latency guarantee;
  aliases; automatic tier choice; retry; fallback; hosted search; tools;
  prompt caching; verbosity; Pro mode; multi-agent; live work; Bedrock SDK
  correction; currentness; `CHANGELOG.md`; shared architecture; route/feature
  matrices; programme/front doors/indexes; matrix assertions; shared package
  lists; release, publication, or merge work
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract change,
  stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, persistence, security, billing, or compatibility decision.
- Do not silently rewrite the current opaque facade behavior. Research 196
  must decide the exact new facade point and private behavior revision for any
  admitted value/profile while retaining the current corrected reasoning proof.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Selection reason:** service tier is an explicit field on the exact
  Responses request and returned Response object already used by this route.
  Unlike hosted search, it does not require new tool-event sequencing or source
  projection merely to establish the request/response field.
- **Official evidence:** start with current [Responses create](https://developers.openai.com/api/reference/cli/resources/responses/methods/create),
  [Responses retrieve](https://developers.openai.com/api/reference/cli/resources/responses/methods/retrieve),
  [background mode](https://developers.openai.com/api/docs/guides/background),
  [streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events),
  and [GPT-5.6 Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol).
  Record retrieval dates and complete fetched-body digests. Use official
  OpenAI domains only. Do not widen from Chat Completions, Codex, another
  model, or a provider SDK type without independently classifying it.
- **Complete enum burden:** the visible prose list is not the complete schema.
  Freeze the exact current request and response enum values, then classify
  omission, `auto`, `default`, `flex`, Fast/Priority spellings, Ultrafast, any
  schema-only value, aliases, and unknown future values.
- **Default burden:** omission behaves as project-configured `auto`. Preserve
  the current omitted request bytes. Do not add explicit `auto` merely to mimic
  omission unless Research 196 admits it as a distinct useful selection.
- **Access burden:** the route's public API-key payg access profile proves no
  project setting, Flex/Fast/Priority/Ultrafast enrollment, quota, capacity, or
  entitlement. Public documentation may establish field/model support without
  establishing this caller's access.
- **Truth burden:** requested, planned, dispatched, accepted, returned,
  effective, billed, and observed tier are different states. Official docs say
  the returned tier may differ from the request. Do not call a request value
  effective or calculate cost/latency from it.
- **Current route truth:** `selection.rs` fixes exact model/facade/behavior.
  `prepared_profile/input.rs` carries reasoning, structured output, retention,
  reattachment, and detachment but no service tier. `request.rs` encodes no
  `service_tier`. `ResponseSnapshot` parses status, output, and usage but not
  the returned tier.
- **Profile burden:** ordinary runs, one in-process reattachment, active-run
  detachment, and restart reconciliation are separate dispositions. A selected
  value must not disappear from durable truth. Do not widen a shared checkpoint
  to make a conditional profile pass.
- **Observation burden:** determine whether current route-local surfaces can
  expose the returned tier without a breaking/shared API. Dispatch-only
  delivery is allowed only for values/profiles whose safe use does not require
  resolved-tier evidence and whose docs state the limitation exactly.
- **Composition burden:** omission and every admitted tier must compose with
  absent plus `none|low|medium|high|xhigh|max` reasoning and absent/selected
  provider-native structured output. Preserve output bound, background,
  stream, store, reattachment, cancellation, deletion, detachment, and
  reconciliation bytes and truth.
- **Version burden:** any delivered value/profile changes the exact opaque
  facade behavior. Research 196 must mint the required facade/private behavior
  and model-route revision while preserving the current 2026-08-23 reasoning
  point as superseded proof. Do not widen Contract 029 currentness.
- **Honest stop:** an empty Research 196 deliver-now set is a successful
  evidence result. Close cards 137-138 as blocked and open the evidence PR.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 136's exact enum/access/profile/observation decision.
  Continue only for a non-empty deliver-now set, then report after the complete
  cards 137-138 implementation and validation chunk.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick worktree-safety preflight in
`## Completion Protocol`. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately, record its actual path and
branch, and do not compare its generated identity with the fallback above. If
it is unusable, use the named worktree if it matches; only then read
`.agents.local.env` and follow its required container setting. Never fall back
to `/tmp` or `TMPDIR`.

Read `AGENTS.md`, g04.049, cards 136-138, Research 102, 191, and 196, the
OpenAI Background prepared guide, exact selection/preparation/request/response/
reattachment/detachment/reconciliation code and fixtures, and the canonical
contracts from the selected worker worktree.

Take card 136 as one coherent evidence chunk. Use current official OpenAI
sources plus deterministic repository evidence; do not send a live request.
If Research 196 has no deliver-now value/profile, close cards 137-138 as
blocked, finish the route-local stop record, validate, and open the evidence
PR. If an exact set survives, execute cards 137-138 in order and open one
implementation PR. At each natural pause, tell the operator what changed, what
validation ran, what remains, and whether a planning decision is needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch; do not create another worktree merely because they
   differ from the placeholders.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path.
   If the launcher supplied a dirty or `main` worktree, stop and report it
   instead of silently creating a second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor e6960758999d45423518179773350589150f0fdb HEAD`
   succeeds, and confirm this handoff exists in selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, guide, relevant research,
   implementation, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline separately from lane-created failures.

### While you work

- Execute ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named in Current State and card 138. If
   card 136 stops the lane, run its acceptance gates plus every applicable
   route-local/index gate and explain why binding-only gates did not run.
2. Update Research 196, cards, milestone, route-local closeout, applicable
   guide/examples/API baselines, and actual worktree/branch evidence. Keep the
   shared surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `e6960758999d45423518179773350589150f0fdb` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link g04.049, cards 136-138, Research 196, Contracts 021,
   029, 037, 040, 048, and 049, changed surfaces, exact official/repository
   evidence, validation, stop/delivery truth, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable, so the orchestrator posts the evidence-backed verdict as a PR
comment. If changes are requested, make only those changes on this branch,
push again, and report through the operator. The operator must explicitly
authorise any merge.

- **Closeout refs:** Research 196; cards 136-138; g04.049; reserved OpenAI
  Background service-tier closeout; `docs/roadmaps/README.md` sole Next Task
  after orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, milestone, research, log,
and next-task state honest. If the work is blocked, record the blocker and stop
rather than making the handoff look more complete than it is.
