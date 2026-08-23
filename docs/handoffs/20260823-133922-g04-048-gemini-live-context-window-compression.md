---
title: g04.048 Gemini Live context-window compression worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-133922-g04-048-gemini-live-context-window-compression.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed g04.047 after PR 46, resumed the sole roadmap Next
Task, reassessed the remaining promoted per-route feature inventory, promoted
the Contract 027 boundary, and compiled g04.048. Context-window-compression
implementation has not started. The ready runway begins with exact public and
repository evidence and permits binding only for Research 195 deliver-now
configurations.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. The worker can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`gemini.live` already fixes exact model `gemini-3.1-flash-live-preview`, raw
WebSocket transport, manual asymmetric PCM, output transcription, caller
thinking/output maximum, and one provider-planned rollover. Current official
Google material explicitly exposes
`BidiGenerateContentSetup.contextWindowCompression` and shows default
sliding-window setup on that exact model.

The exact explicit token-pair domain and JSON integer form are not yet closed.
Setup completion also returns no configuration fields. This lane must qualify
the smallest exact setup shape before delivery, preserve omission and
rollover/restoration state, and publish dispatch truth only.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `5a7d951e621211caf1d188e5018f986b68a43fd1`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `5a7d951e621211caf1d188e5018f986b68a43fd1` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:** Contract 027 extension, g04.048, cards
  133-135, Research 195 reservation, compilation log, closeout reservation,
  triage selection, and updated sole Next Task
- **Worker branch:** `agent/g04-048-gemini-live-context-window-compression-20260823-133922`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-048-gemini-live-context-window-compression-20260823-133922`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-048-gemini-live-context-window-compression-20260823-133922 -b agent/g04-048-gemini-live-context-window-compression-20260823-133922 origin/main`
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
- **Roadmap milestone:** `docs/roadmaps/g04/048-gemini-live-context-window-compression.md`
- **Ready cards, in order:**
  `133-gemini-live-context-window-compression-evidence.md`, then conditional
  `134-gemini-live-context-window-compression-binding.md`, then conditional
  `135-gemini-live-context-window-compression-acceptance.md`
- **Allowed runway:** exact `gemini.live` compression evidence, then only
  Research 195 deliver-now adapter-local binding
- **Remaining card budget:** three cards; cards 134-135 execute only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane
- **Parallel safety check:** serial by design; every card shares Gemini
  prepared input/plan/evidence, driver setup encoding, rollover/restoration,
  thinking/output composition, fixtures, guide, research, and closeout. Do not
  use internal subagents; report through the operator.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 027, 029, 037, 040, 050, and 052
- **Route identity:** `gemini.live`, driver `swallowtail.gemini.live`, axis
  `gemini.live-facade`, current exact facade point
  `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-2026-08-23`,
  exact model `gemini-3.1-flash-live-preview`, behavior
  `gemini.live-preview-manual-pcm-rollover-thinking-output-max-v3`, claim
  `gemini.live-preview-window-3`, model-route revision `prepared-3`
- **Candidate mapping:** adapter-local typed selection to exact setup
  `contextWindowCompression.slidingWindow`; candidate default-only shape is
  `{ "slidingWindow": {} }`; explicit `triggerTokens` and nested
  `targetTokens` require Research 195 evidence
- **Model capability profile:** exact-model, exact-facade, evidence-first;
  fail closed on shape, encoding, domain, model, facade, plan/evidence, setup,
  handle, continuity, restoration, or control-composition ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account
  inspection, provider request, live Gemini call, browser login, or paid work.
  Current official public-source inspection and secret-free deterministic
  repository fixtures are allowed by card 133.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-gemini`, `effigy validate:focused
  swallowtail-adapter-gemini`, `effigy package:verify-affected
  swallowtail-adapter-gemini`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 371 inherited god-file findings: 326 warnings and
  45 errors, plus one generated-in-src warning. A graph-stale warning was also
  observed during planning. Keep inherited findings separate from lane-created
  findings.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; worker must not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-gemini/**` for the exact
  adapter-local prepared selection, driver, protocol binding, and tests;
  `docs/guides/realtime-prepared-integration.md`; Research 195; g04.048; cards
  133-135; the reserved g04.048 route-local closeout; applicable
  `swallowtail-adapter-gemini` unreleased public-API baseline; official public
  exact-model/Live/session-management/best-practices sources; deterministic
  secret-free setup, rollover, restoration, composition, rejection,
  lifecycle, and cleanup fixtures
- **Out of scope:** shared runtime API/carrier changes; new `Capability` or
  `CapabilityConstraint`; another realtime driver; generic context-window
  control; token counting; client-side truncation; summaries; effective
  compression, retained-history, session-duration, semantic-continuity, or
  token-saving guarantees; tools; automatic activity; Gemini CLI ACP/headless;
  Vertex AI; another Gemini model/API/route; browser access; aliases; clamping;
  fallback; live work; further contract edits; `CHANGELOG.md`; shared
  architecture; route/feature matrices; programme/front doors/indexes; matrix
  assertions; shared package lists; release, publication, or merge work
- Contract 027 at the planning base is the complete authorization boundary. Do
  not expand or rewrite it. If exact evidence contradicts it or another
  contract change is needed, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, persistence, security, or compatibility decision.
- Do not silently rewrite the current opaque facade behavior. Research 195
  must decide the exact new facade point and private behavior revision for any
  admitted compression shape while retaining prior proof.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Selection reason:** the promoted inventory names Gemini Live context-window
  compression as an official route-local feature gap. The exact retained route
  already has bounded setup, rollover, restoration, and deterministic fixtures.
- **Current route truth:** `live_selection.rs` fixes the output-maximum facade
  point and `-v3` behavior. `prepared_live_profile/input.rs` carries media,
  deadline, rollover, optional thinking, and optional output maximum but no
  compression. `live_protocol/client.rs` serializes setup without
  `contextWindowCompression`.
- **No shared carrier:** `OpenRealtimeMediaSessionRequest` has no compression
  field and must remain unchanged. Follow the adapter-local selection pattern
  used by Ollama `num_ctx`: typed public adapter input, immutable prepared
  evidence/driver state, no portable capability claim.
- **Official evidence:** start with the current
  [exact model page](https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview),
  [Live WebSocket reference](https://ai.google.dev/api/live),
  [session-management guide](https://ai.google.dev/gemini-api/docs/live-api/session-management),
  and [Live best practices](https://ai.google.dev/gemini-api/docs/live-api/best-practices).
  Record retrieval dates and complete specimen digests. Do not widen from a
  different model, API, SDK, or provider surface.
- **Shape burden:** classify omission, default sliding window, trigger-only,
  target-only, explicit trigger/target, unknown members, and alternate JSON
  forms. Default-only is a candidate, not automatic permission.
- **Domain burden:** close integer wire representation, valid ranges,
  target/trigger ordering, provider-default semantics, zero, negative,
  fractional, overflow, alias, and clamp behavior. Withhold explicit pairs if
  official evidence does not close them.
- **Omission burden:** current initial and resume setup frames contain no
  `contextWindowCompression`. Preserve those exact bytes and expose no
  compression evidence when omitted.
- **Continuity burden:** one selected config is immutable across initial setup,
  provider-planned rollover setup with private handle, and fresh realtime
  working-state restoration. The route must still wait for the latest
  resumable handle; compression does not make a non-resumable state safe.
- **Composition burden:** omission and all admitted
  `minimal|low|medium|high` thinking levels must compose with omitted and
  selected `1..=65_536` output maxima. Do not alter either control's casing,
  defaults, capability constraints, or qualified domain.
- **Claim burden:** `BidiGenerateContentSetupComplete` has no fields.
  Deterministic setup bytes prove dispatch only. Provider descriptions of
  eviction or longer sessions do not prove effective behavior in Swallowtail.
- **Version burden:** any delivered shape changes the exact opaque facade
  behavior. Research 195 must mint one new point/private behavior/claim/model-
  route revision and preserve the current output-maximum point as superseded
  proof. Do not widen Contract 029 currentness.
- **Honest stop:** an empty Research 195 deliver-now set is a successful
  evidence result. Close cards 134-135 as blocked and open the evidence PR.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 133's exact route/shape/domain decision. Continue only
  for a non-empty deliver-now set, then report after the complete cards 134-135
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

Read `AGENTS.md`, Contract 027, g04.048, cards 133-135, Research 021, Research
193-195, the realtime prepared guide, exact Gemini Live selection/preparation/
validation/setup/rollover/restoration code and fixtures, and the canonical
contracts from the selected worker worktree.

Take card 133 as one coherent evidence chunk. Use current official exact-model,
Live, session-management, and best-practices sources plus deterministic
repository evidence; do not send a live request. If Research 195 has no
deliver-now configuration, close cards 134-135 as blocked, finish the
route-local stop record, validate, and open the evidence PR. If an exact set
survives, execute cards 134-135 in order and open one implementation PR. At
each natural pause, tell the operator what changed, what validation ran, what
remains, and whether a planning decision is needed.

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
   `git merge-base --is-ancestor 5a7d951e621211caf1d188e5018f986b68a43fd1 HEAD`
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

1. Run the required final validation named in Current State and card 135. If
   card 133 stops the lane, run its acceptance gates plus every applicable
   route-local/index gate and explain why binding-only gates did not run.
2. Update Research 195, cards, milestone, route-local closeout, applicable
   guide/examples/API baselines, and actual worktree/branch evidence. Keep the
   shared surfaces listed above unchanged.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The handoff's
   `5a7d951e621211caf1d188e5018f986b68a43fd1` is the planning base before this
   handoff commit, not a self-referential hash for the commit containing it.
5. In the PR body, link g04.048, cards 133-135, Research 195, Contract 027,
   changed surfaces, exact official/repository evidence, validation,
   stop/delivery truth, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable, so the orchestrator posts the evidence-backed verdict as a PR
comment. If changes are requested, make only those changes on this branch,
push again, and report through the operator. The operator must explicitly
authorise any merge.

- **Closeout refs:** Research 195; cards 133-135; g04.048; reserved Gemini Live
  context-window-compression closeout; `docs/roadmaps/README.md` sole Next Task
  after orchestrator merge closeout

### Handoff closeout

Before calling the runway complete, leave the card, milestone, research, log,
and next-task state honest. If the work is blocked, record the blocker and stop
rather than making the handoff look more complete than it is.
