---
title: g04.057 Grok Build ACP reasoning-selection worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-24
updated: 2026-08-24
planning_base: a40cefd510b6131ec867637ff80157544a4d6e7e
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260824-210042-g04-057-grok-build-acp-reasoning-selection.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining promoted per-route feature inventory
after g04.056 and selected exact `grok-build.acp` reasoning selection. g04.057
is compiled. Implementation has not started. The ready runway begins with exact
current official, package/source, and existing no-prompt handshake evidence;
cards 159-160 are conditional on a non-empty Research 204 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

`grok-build.acp` fixes one exact model per executable behavior and already owns
ACP initialization, cached-token activation, `session/new`, an operation-
private session for structured runs, a reusable interactive session, provider-
state retention, and joined attachment cleanup. Existing exact handshakes expose
bounded effort sets, but the public prepared facade rejects reasoning selection.

Contract 034 already defines the correct seam: receive one bounded option
snapshot, map one adapter-private option to an exact portable value, send one
selection request, and require effective confirmation before readiness. The
target is that exact sequence for new sessions only. Advertisement, changelog
text, CLI `--effort`, or an observed label is not enough.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `a40cefd510b6131ec867637ff80157544a4d6e7e`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `a40cefd510b6131ec867637ff80157544a4d6e7e` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.057, cards 158-160,
  Research 204 reservation, compilation log, route-local closeout reservation,
  reassessment disposition, corrected PR55 doctor evidence, and the sole Next
  Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-057-grok-acp-reasoning-20260824-210042`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-057-grok-acp-reasoning-20260824-210042`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-057-grok-acp-reasoning-20260824-210042 -b agent/g04-057-grok-acp-reasoning-20260824-210042 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion; Contracts 034 and 040 are
  already authoritative; no contract edit is planned
- **Roadmap milestone:**
  `docs/roadmaps/g04/057-grok-build-acp-reasoning-selection.md`
- **Ready cards, in order:** card 158, then conditional card 159, then
  conditional card 160
- **Allowed runway:** exact ACP reasoning-option evidence, then only Research
  204 deliver-now prepared/driver negotiation on new run/session shapes
- **Remaining card budget:** three serial cards; cards 159-160 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share prepared inputs, capability/plan
  assembly, `session/new`, connection dispatch, fixtures, guide, research, and
  closeout; they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 017, 023, 029, 034, 037, 040, 041, and 052
- **Route identity:** `grok-build.acp`, driver `swallowtail.grok-build.acp`,
  axis `grok-build.executable`; deprecated `0.2.114..=0.2.117` binds
  `grok-4.5`; maintained `1.0.4..=1.0.5` binds `grok-4.6`
- **Candidate mapping:** `low|medium|high` on `grok-4.5`; those plus `xhigh`
  on `grok-4.6`; no value is prequalified
- **Current mapping:** interactive `SessionOptions` must be empty; structured
  runs expose no reasoning input; `session/new` receives only cwd and an empty
  MCP server list; no `session/set_config_option` request is sent
- **Model capability profile:** exact route-fixed model, evidence-first, fail
  closed on version/model/value/snapshot/request/confirmation/lifecycle drift;
  no fallback
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no installation, browser login, account inspection,
  credential capture, authenticated provider prompt, external inference
  request, or paid work. Current official public documentation, exact public
  package/source artifacts, existing secret-free handshake corpus, and
  deterministic fixtures are allowed by card 158.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-grok`, `effigy validate:focused
  swallowtail-adapter-grok`, `effigy package:verify-affected
  swallowtail-adapter-grok`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, `effigy doctor`, and `git diff --check`
- **Known doctor baseline:** 378 god-file findings: 332 warnings and 46 errors;
  stale graph index; one generated-in-src warning. New Grok tests must be split
  into focused modules and must not increase the finding/error counts.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index gates,
  and `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-grok/**` for exact prepared
  run/session reasoning inputs, capability and plan binding, request agreement,
  bounded ACP option parsing/selection/confirmation, failure cleanup, fixtures,
  examples, and package-specific API baseline; `docs/guides/grok-build-prepared-integration.md`;
  Research 204; g04.057; cards 158-160; the reserved g04.057 route-local
  closeout; current official Grok Build docs; exact public package/source;
  existing secret-free no-prompt handshake evidence
- **Out of scope:** raw provider configuration maps or snapshots; model
  switching/catalogue widening; `off`, `minimal`, `max`, aliases, clamps,
  defaults, or values not admitted by Research 204; per-turn mutation;
  attachment-recovery/load/resume mutation; child `--effort`; max turns, web
  search, plan, subagents, sandbox, permission, allow/deny, or approval
  controls; hosted xAI routes; usage/cost/output/schema/attachment/tool/callback
  expansion; provider-session lifecycle; login/account/provider-prompt work;
  another version family; currentness; `CHANGELOG.md`; shared architecture,
  contracts, route/feature matrices, programme, front doors, or indexes;
  release, publication, merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, process, access, security, persistence, or compatibility
  decision.
- Do not translate CLI `--effort`, display labels, array order, or hosted xAI
  values into ACP protocol truth. Research 204 owns the exact private mapping.
- Do not treat advertisement as selectability or request success as effective
  confirmation. Preserve advertised, selectable, requested, accepted,
  effective, and observed states separately.
- Caller omission must keep current wire and behavior. Do not infer a selected
  provider default from omission.
- A selected new session does not authorize mutation of an existing binding.
  Attachment recovery must remain on its qualified unchanged path.
- Failure after `session/new` returns no ready session or run, joins every owned
  surface, and preserves provider-owned durable-session truth. Do not claim
  rollback or deletion.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, the canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 130 records exact `1.0.4` model `grok-4.6`
  and efforts `xhigh|high|medium|low`; Research 163 extends the same behavior
  through `1.0.5`. Deprecated compatibility fixtures record
  `high|medium|low` with `grok-4.5`.
- **Official lead:** current xAI docs name `--effort` and ACP session effort,
  but card 158 must freeze the actual ACP session configuration surface. Do not
  copy child argv onto `grok agent stdio`.
- **Current source truth:** `GrokSessionProfileInput` already carries
  `SessionOptions` but `validate_options` rejects every non-empty value.
  `GrokRunProfileInput` has no reasoning member. Structured runs internally
  call `start_session` before their first prompt.
- **Negotiation truth:** current `start_session` validates initialize, activates
  `cached_token`, sends `session/new`, and immediately builds the handle.
  Research 204 must identify the bounded option snapshot and exact confirmation
  point inserted before that handle becomes ready.
- **Existing pattern:** Claude Agent and Kimi adapters contain contract-aligned
  private `session/set_config_option` parsing and confirmation examples. They
  are implementation references, not Grok evidence or values.
- **Version truth:** exact deprecated and maintained segments are distinct;
  mid-gap `0.2.118..=0.2.121` and `1.0.0..=1.0.3` stay incompatible. Later
  stable `UnverifiedNewer` may use only the latest qualified private mapping
  and must fail closed on drift.
- **Application truth:** require exact effective enum confirmation before
  readiness or first prompt. That confirms the selected enum, not reasoning
  depth, quality, output, usage, cost, or billing effect.
- **Restoration truth:** `prepare_working_state_restoration` attaches an exact
  durable provider session. Selected new-session reasoning must not silently
  reconfigure that session.
- **Honest stop:** an empty Research 204 deliver-now set is a successful
  evidence result. Mark cards 159-160 blocked, finish the route-local stop
  record, validate, and open the evidence PR.
- **Generation boundary:** do not close or roll over g04. After merge the
  orchestrator reconciles g04.057 and reassesses remaining inventory.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. New-route research does not pre-empt per-route feature
  work.
- **Known baseline:** PR55 added two god-file warnings and the corrected 378
  baseline is now recorded in `PAPERCUTS.md`. Do not repair that separate
  package here; avoid adding Grok warnings.
- **Report after:** card 158's exact version/model/value and confirmation
  decision. Continue automatically only for a non-empty deliver-now set and no
  stop condition, then report after the complete cards 159-160 batch.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Start by reading this handoff from the top. Before broad repository reads, run
the quick worktree-safety preflight in `## Completion Protocol`. If the current
context is a clean, dedicated, non-`main` registered worktree, use it
immediately, record its actual path and branch, and do not compare its generated
identity with the fallback above. If it is unusable, use the named worktree if
it matches; only then read `.agents.local.env` and follow its required
container setting. Never fall back to `/tmp` or `TMPDIR`.

Then read `AGENTS.md`, g04.057, cards 158-160, Research 130, 163, and 204,
Contract 034, the Grok prepared guide, exact preparation/driver/connection
source and fixtures, and the other named canonical refs from the selected
worker worktree.

Take card 158 as one coherent evidence chunk. Use public official docs, exact
public package/source, existing no-prompt corpus, and deterministic repository
evidence; do not install, authenticate, inspect account state, or send a prompt.
If Research 204 has no deliver-now set, close cards 159-160 as blocked, finish
the route-local stop record, validate, and open the evidence PR. If an exact set
survives, execute cards 159-160 in order and open one implementation PR. At each
natural pause, tell the operator what changed, what validation ran, what
remains, and whether a planning decision is needed.

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
   `a40cefd510b6131ec867637ff80157544a4d6e7e` and is clean before editing. If
   `origin/main` moved, use current pushed main only when it contains that
   planning base; otherwise stop and report divergence.
5. Confirm this handoff file exists in selected `HEAD`, then read the milestone,
   assigned cards, `AGENTS.md`, and canonical refs completely.
6. Run the repo's cheap orientation checks and record what you actually ran.

### Work the cards

1. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan` once at
   startup. Keep inherited doctor findings separate.
2. Read card 158 and its named refs completely. Freeze official/exact package
   and existing no-prompt evidence and promote Research 204. Do not edit
   production code during the evidence card.
3. Report the exact deliver-now or stop table to the operator. Continue to card
   159 only if Research 204 has a non-empty exact set and no stop condition.
4. If continuing, implement cards 159 and 160 as one meaningful code/test/docs
   batch. Preserve every fixed access, model, provider-state, and lifecycle
   boundary.
5. Update only route-local worker surfaces. In the reserved closeout, list the
   shared architecture, Contract 029, route/feature matrix, programme, indexes,
   changelog, milestone, and Next Task changes the orchestrator must apply
   after merge.
6. Keep one clean worker branch. Commit meaningful batches, push the branch,
   and open one PR against `main`. Record the PR URL and exact head.

### Before you report completion

1. Run every named card gate. For an evidence stop, run card 158's focused and
   docs gates. For delivery, run the full card 160 set, including `effigy
   doctor` and public API validation.
2. Confirm default validation used no installation, login, account inspection,
   provider prompt, credential capture, external inference request, or paid
   work.
3. Confirm `git diff --check`, worker status, pushed head, PR base/head, and PR
   CI state. Do not claim a gate you did not run.
4. Ensure Research 204, cards, milestone, guide, and route-local closeout agree
   on deliver-now/stop, application state, version/model/value rows, lifecycle,
   and validation.
5. Report: result, exact evidence table, cards completed/blocked, material files,
   validation, doctor delta, PR URL, exact head, branch/worktree, and shared
   orchestrator closeout needed. Do not merge.

### Stop and ask instead of guessing when

- the exact ACP option id, values, selection request, or effective confirmation
  remains ambiguous
- evidence requires authentication, account inspection, a provider prompt,
  credential capture, installation, or paid work
- a generic provider configuration API, shared contract/runtime change, model
  switch, load/resume mutation, currentness change, or breaking public API is
  required
- a selected run can prompt before confirmation, or post-allocation failure
  cannot preserve honest cleanup/provider-state truth
- the worker checkout is unsafe, the planning base diverges, or shared mutable
  scope overlaps another lane
- generation rollover, g04 closure, release, publication, or merge would be
  required

If card 158 admits an empty deliver-now set for exact evidence reasons, that is
not an operator blocker: record the honest stop, block cards 159-160, validate,
and open the evidence PR.
