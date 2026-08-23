---
title: g04.047 Gemini Live output-token maximum worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-115830-g04-047-gemini-live-output-maximum.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed g04.046 after PR 45, resumed the sole roadmap Next
Task, reassessed the remaining promoted per-route feature inventory, and
compiled g04.047. Gemini Live output-token-maximum work has not started. The
ready runway begins with exact current model/facade evidence and permits
binding only for Research 194 deliver-now values.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`gemini.live` is the retained hosted API-key Gemini route. It already fixes
exact model `gemini-3.1-flash-live-preview`, raw WebSocket transport, manual
asymmetric PCM, output transcription, caller thinking levels, and one
provider-planned rollover. Its prepared facade does not expose an output-token
maximum even though `OpenRealtimeMediaSessionRequest` already has the typed
positive carrier and Contract 040 capability vocabulary.

Current official references say Live setup accepts a `GenerationConfig`,
define `GenerationConfig.maxOutputTokens`, and list 65,536 as this exact
model's output-token limit. They also warn that not every parameter is
configurable for every model. The lane must therefore prove exact composed
applicability and the numeric domain before delivery. Deterministic setup bytes
can support a dispatch claim; they cannot prove provider acceptance or an
effective generated length.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `c51e3e9898c6ea08e217d0d981d2b982e0a5590b`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `c51e3e9898c6ea08e217d0d981d2b982e0a5590b` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.047, cards 130-132,
  Research 194 reservation, compilation log, closeout reservation, triage
  selection, and updated sole Next Task
- **Worker branch:** `agent/g04-047-gemini-live-output-maximum-20260823-115830`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-047-gemini-live-output-maximum-20260823-115830`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-047-gemini-live-output-maximum-20260823-115830 -b agent/g04-047-gemini-live-output-maximum-20260823-115830 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches. Only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator if the file or key is absent. Never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** per-route feature completion; no spec edit
- **Roadmap milestone:** `docs/roadmaps/g04/047-gemini-live-output-token-maximum.md`
- **Ready cards, in order:**
  `130-gemini-live-output-token-maximum-evidence.md`, then conditional
  `131-gemini-live-output-token-maximum-binding.md`, then conditional
  `132-gemini-live-output-token-maximum-acceptance.md`
- **Allowed runway:** exact `gemini.live` output-token-maximum evidence, then
  only Research 194 deliver-now binding
- **Remaining card budget:** three cards; cards 131-132 execute only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; every card shares Gemini
  prepared input/plan/evidence, driver validation, initial/resume setup
  encoder, reasoning composition, fixtures, guide, research, and closeout. Do
  not use internal subagents; report through the operator.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 027, 029, 037, 040, 050, and 052
- **Route identity:** `gemini.live`, driver `swallowtail.gemini.live`, axis
  `gemini.live-facade`, exact current facade point
  `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23`,
  exact model `gemini-3.1-flash-live-preview`, current behavior
  `gemini.live-preview-manual-pcm-rollover-thinking-v2`, claim
  `gemini.live-preview-window-2`, model-route revision `prepared-2`
- **Candidate mapping:** positive portable `NonZeroU64` to exact setup
  `generationConfig.maxOutputTokens`; 65,536 is the official exact-model
  output-limit candidate, not yet a qualified setter bound
- **Model capability profile:** exact-model, exact-facade, evidence-first;
  fail closed on field applicability, numeric domain, model, facade, plan,
  request, setup, continuity, or reasoning-composition ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents
  or parallel worker lanes; no package install, login, credential/account
  inspection, provider request, live Gemini call, browser login, or paid work.
  Current official public source inspection and secret-free deterministic
  repository fixtures are allowed by card 130.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-gemini`, `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy package:verify-affected
  swallowtail-adapter-gemini`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research, logs, roadmaps, g04, batch-card and
  next-action index gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 371 inherited god-file findings: 326 warnings and
  45 errors, plus one generated-in-src warning. The graph-stale warning was
  observed during planning. Keep inherited findings separate from lane-created
  findings.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-gemini/**` for the exact prepared,
  driver, protocol binding and tests; `docs/guides/realtime-prepared-integration.md`;
  Research 194; g04.047; cards 130-132; the reserved g04.047 route-local
  closeout; applicable `swallowtail-adapter-gemini` unreleased public-API
  baseline; official public model/Live/generation-config docs; deterministic
  secret-free setup, rollover, restoration, reasoning-composition, rejection,
  lifecycle, and cleanup fixtures
- **Out of scope:** shared runtime API or carrier changes;
  `crates/swallowtail-adapter-openai/**`; another realtime driver; token
  counting; client-side truncation; generated-length guarantees; stop
  sequences; context-window controls; thinking vocabulary or omission changes;
  context compression; tools; automatic activity; Gemini CLI ACP/headless;
  Vertex AI; another Gemini model/API/route; consumer login; OAuth; ephemeral
  tokens; WebRTC; SIP; aliases; clamping; fallback; live work; contracts;
  `CHANGELOG.md`; shared architecture; route/feature matrices; programme/front
  doors/indexes; matrix assertions; shared package lists; release, publication,
  or merge work
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product, API, persistence, security, or compatibility decision.
- Do not silently rewrite the current opaque facade behavior. Research 194
  must decide the exact new facade point and private behavior revision for any
  admitted maximum, while retaining the current thinking-capable proof.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Selection reason:** the classification inventory names output-token limit
  on `gemini.live` as ready under the existing contract. The exact retained
  Gemini route and generic realtime request already supply the surrounding
  lifecycle and portable carrier. No new route or generic settings map is
  needed.
- **Current route truth:** `live_selection.rs` fixes model
  `gemini-3.1-flash-live-preview`, the thinking-qualified facade point, and
  private behavior `gemini.live-preview-manual-pcm-rollover-thinking-v2`.
  `prepared_live_profile/input.rs` has reasoning but no maximum. The generic
  request already has `maximum_output_tokens`. `live_protocol/client.rs`
  serializes `generationConfig` without `maxOutputTokens`.
- **Official evidence:** start with the current
  [exact model page](https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview),
  [Live WebSocket reference](https://ai.google.dev/api/live), and
  [generation-config reference](https://ai.google.dev/api/generate-content).
  Record retrieval dates and do not use a different model/API page to widen
  the exact route.
- **Applicability burden:** `BidiGenerateContentSetup.generationConfig` and the
  generic field definition are necessary but may not be sufficient. The
  reference warning that not every parameter is configurable for every model
  must be resolved for the exact composed route. An empty deliver-now set is
  valid if public evidence cannot close it without a live call.
- **Domain burden:** classify the candidate positive range through 65,536.
  The model catalogue's output limit does not independently prove arbitrary
  caller bounds. Reject zero, above-limit, negative, fractional, overflowing,
  aliased, or clamped values rather than guessing.
- **Omission burden:** current initial and resume setup frames contain no
  `maxOutputTokens`. Preserve those exact bytes and add no
  `OutputTokenLimit` capability when the caller omits the field.
- **Existing-carrier burden:** reuse
  `OpenRealtimeMediaSessionRequest::with_maximum_output_tokens` and exact
  `CapabilityConstraint::OutputTokenMaximum`. If delivery requires a shared
  runtime change, stop for orchestrator review.
- **Continuity burden:** the selected maximum is immutable across initial
  setup, the provider-planned rollover setup with private handle, and fresh
  realtime working-state restoration. Rollover remains an in-session
  connection replacement, not retry or public resume.
- **Reasoning burden:** omission and all admitted
  `minimal|low|medium|high` thinking levels must compose with the maximum. Do
  not alter thinking casing, default `MINIMAL` omission, capability truth, or
  the qualified value set.
- **Claim burden:** deterministic frame bytes prove dispatch. Do not claim
  provider acceptance, effective output length, token counting, or truncation
  without exact explicit evidence returned by this surface.
- **Absent sibling behavior:** no OpenAI Realtime or other driver change is
  needed or allowed. Their current maximum domains and wire behavior remain
  independent.
- **Honest stop:** an empty Research 194 deliver-now set is a successful
  evidence result. Close cards 131-132 as blocked and open the evidence PR.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 130's exact route/field/domain decision. Continue only
  for a non-empty deliver-now set, then report after the complete cards 131-132
  implementation and validation chunk.
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

Read `AGENTS.md`, g04.047, cards 130-132, Research 021, Research 193, Research
194, the realtime prepared guide, exact Gemini Live selection/preparation/
request/validation/setup/rollover/restoration code and fixtures, and the
canonical contracts from the selected worker worktree.

Take card 130 as one coherent evidence chunk. Use current official exact-model,
Live, and generation-config sources plus deterministic repository evidence; do
not send a live request. If Research 194 has no deliver-now value, close cards
131-132 as blocked, finish the route-local stop record, validate, and open the
evidence PR. If an exact positive domain survives, execute cards 131-132 in
order and open one implementation PR. At each natural pause, tell the operator
what changed, what validation ran, what remains, and whether a planning
decision is needed.

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
   `git merge-base --is-ancestor c51e3e9898c6ea08e217d0d981d2b982e0a5590b HEAD`
   succeeds, and confirm this handoff exists in the selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, guide, relevant research and
   implementation, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline separately from lane-created failures.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named in Current State and card 132. If
   card 130 stops the lane, run its acceptance gates plus every applicable
   route-local/index gate and explain why binding-only gates did not run.
2. Update Research 194, cards, milestone, route-local closeout, applicable
   guide/examples/API baselines, and actual worktree/branch evidence. Keep the
   shared surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `c51e3e9898c6ea08e217d0d981d2b982e0a5590b` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link g04.047, cards 130-132, Research 194, changed surfaces,
   exact official/repository evidence, validation, stop/delivery truth, and
   unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator records an evidence-backed verdict in the provider review
surface. When orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment. That comment is the canonical review record. If changes are
requested, make only those changes on this branch, push again, and report back
through the operator. Requested changes are: none. The operator must explicitly
authorise any merge.

- **Closeout refs:** Research 194; cards 130-132; g04.047; reserved Gemini Live
  output-token-maximum closeout; `docs/roadmaps/README.md` sole Next Task after
  orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, milestone, research, log,
and next-task state honest. If the work is blocked, record the blocker and stop
rather than making the handoff look more complete than it is.
