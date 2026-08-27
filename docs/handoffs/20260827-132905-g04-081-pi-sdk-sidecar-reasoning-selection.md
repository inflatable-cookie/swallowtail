---
title: g04.081 Pi SDK sidecar reasoning selection worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-132905-g04-081-pi-sdk-sidecar-reasoning-selection.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator resumed the sole roadmap continuation after g04.080 and
reassessed the current production feature matrix against the historical
advanced-feature inventory. It selected reasoning selection on
`pi.sdk-sidecar` as the next bounded route-local evidence candidate.

The orchestrator compiled g04.081, cards 225-227, Research 228, programme and
front-door updates, triage disposition, and the compilation log. The planning
base was validated and pushed to `main` at
`5cbb4d6ff4726364c7fe3bde6313fc248211f625`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`pi.sdk-sidecar` already owns exact provider/model selection, persistent new,
load with bounded typed replay, replay-free resume, reasoning activity,
attachments, cancellation, and joined cleanup. Its feature matrix still says
`reasoning_selection = No`.

The source-tagged sidecar already accepts optional bootstrap `thinkingLevel`,
passes it to `createAgentSessionFromServices`, and reports
`session.thinkingLevel` in bootstrap and state snapshots. Rust preparation and
startup omit and ignore the field. This is a direct SDK seam, but exact Pi
0.84.2 clamps unsupported values to model capability. A string accepted by the
sidecar is not a qualified selection. Exact static model/value membership,
pre-effect rejection, lifecycle semantics, and effective-state agreement are
the gate.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:**
  `5cbb4d6ff4726364c7fe3bde6313fc248211f625`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** shared main checkout; do not use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.081, cards 225-227, Research 228 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-081-pi-sdk-sidecar-reasoning-selection`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-081-pi-sdk-sidecar-reasoning-selection`
- **Worktree creation command:** `git worktree add -b
  worker/g04-081-pi-sdk-sidecar-reasoning-selection
  /Users/tom/Dev/worktrees/swallowtail-g04-081-pi-sdk-sidecar-reasoning-selection
  origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent; never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:**
  `docs/roadmaps/g04/081-pi-sdk-sidecar-reasoning-selection.md`
- **Ready cards, in order:**
  `225-pi-sdk-sidecar-reasoning-selection-evidence.md`, then conditional
  `226-pi-sdk-sidecar-reasoning-selection-binding.md`, then conditional
  `227-pi-sdk-sidecar-reasoning-selection-acceptance.md`
- **Allowed runway:** execute card 225 and promote Research 228; continue to
  cards 226-227 only for a non-empty exact provider/model/value/lifecycle set
  with static pre-effect rejection and effective-state confirmation
- **Remaining card budget:** three cards; stop after card 225 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and all three cards touch the same Pi preparation,
  sidecar, wire, persistence, fixtures, and docs
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  008, 012, 017, 019, 029, 034, 040, and 052
- **Model capability profile:** exact official tagged SDK/source research plus
  route-local Rust/sidecar implementation and deterministic conformance
- **Tool/runtime restrictions:** secret-free exact tagged source and local
  fixture work only; no install, update, provider prompt, model run, credential
  use, account/login inspection, authenticated catalogue, paid work, ambient
  configuration mutation, or sibling-route work
- **Required validation:** card 225 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-pi`,
  `effigy validate:focused swallowtail-adapter-pi`,
  `effigy package:verify-affected swallowtail-adapter-pi`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings (334
  warnings, 46 errors); `scan.generated-in-src` reports one warning; graph
  index is stale. Existing papercut records cover the structural baseline;
  record drift and do not add duplicates or repair unrelated findings.
- **PR base:** `main`
- **PR head:** worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact Pi 0.84.2 `ThinkingLevel` vocabulary, model capability
  representation, clamp behavior, provider/model membership, explicit-option
  precedence, stored/default state, runtime replacement, new/load/resume/fresh
  restoration semantics, bootstrap/state confirmation, setup/rebind events,
  current wire and fixture audit, conditional portable selection binding,
  deterministic acceptance, route-local docs/matrices/API truth, Research 228,
  closeout, and sole Next Task
- **Out of scope:** changing mode after readiness, level cycling, model
  switching, raw Pi settings/options, generic provider configuration,
  `pi.rpc`, newer Pi SDK currentness, provider execution, sibling routes,
  release, generation rollover, g04 closure, or merge
- Existing omission must remain exact: no `thinkingLevel` in Rust bootstrap,
  no `ReasoningSelection` capability, and current Pi default/stored behavior.
  Do not label the omitted effective mode as caller-selected.
- A deliver-now row requires an exact static provider/model/value gate before
  launch recipe, environment, credential, resource, process, or provider work.
  Catalogue reasoning booleans, examples, family names, accepted strings, and
  emitted thought are insufficient.
- Requested, planned, dispatched, accepted, effective, and observed truth stay
  separate. Pi's `session.thinkingLevel` may confirm the effective setup value;
  reasoning activity and token/output shape cannot.
- New, load, resume, runtime replacement, and fresh context-losing restoration
  must be classified independently. Attachment must re-declare, reapply, and
  confirm the caller-selected mode under Contracts 012 and 017, or reject that
  lifecycle.
- Preserve the exact provider/model/resource/session gates, bounded replay,
  attachments, steer/follow-up, cancellation, durable provider state,
  close/join, and credential-last cleanup.
- Advance private wire, behavior, or source-tag truth only when exact evidence
  requires it. Do not hide a semantic change behind the current axis claim.
- Do not invent architecture, change contracts, widen the roadmap, or choose
  an unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact route point:** route `pi.sdk-sidecar`, driver
  `swallowtail.pi.sdk-sidecar`, package
  `@earendil-works/pi-coding-agent@0.84.2`, Node `22.23.2`, wire
  `swallowtail-pi-sdk-jsonl-v1`, behavior `pi.sdk-sidecar-v1`, and source tag
  `swallowtail-pi-sdk-sidecar@0.3.3`.
- **Current prepared seam:** `PiSdkSidecarSessionPreparation` carries provider,
  model, resource, request, and optional image posture but no reasoning mode.
  `validate_open` rejects any non-empty `SessionOptions`; attachment validation
  requires options empty.
- **Current startup seam:** Rust sends bootstrap `cwd`, `provider`, and `model`
  only. Bootstrap and later state checks validate provider/model/resource/tool
  and session identity but ignore returned `thinkingLevel`.
- **Current sidecar seam:** bootstrap accepts optional string `thinkingLevel`,
  forwards it to `createAgentSessionFromServices`, and snapshots
  `session.thinkingLevel`. The v1 compatibility command/response fixture already
  contains `medium`; that freezes permissive wire shape, not a public claim.
- **Exact upstream lead:** Pi 0.84.2 `CreateAgentSessionOptions` types
  `thinkingLevel` as `ThinkingLevel`; the SDK guide lists
  `off|minimal|low|medium|high|xhigh|max`. SDK construction restores or defaults
  a level when absent and calls `clampThinkingLevel(model, thinkingLevel)`
  before constructing the Agent. Freeze the nested exact tagged source behind
  the type and clamp function; do not rely on guide prose alone.
- **Replacement tension:** the sidecar runtime factory closes over bootstrap
  `thinkingLevel` and is reused by `AgentSessionRuntime` during session switch.
  Prove whether explicit selection overrides stored state on every claimed
  attachment and whether state confirmation happens before readiness.
- **Event tension:** `thinking_level_changed` is currently classified as a
  disabled feature event. Determine whether construction or rebind emits it
  after subscription and whether the present unexpected-event terminal rule
  must remain, move, or gain a bounded setup-only disposition.
- **Membership tension:** sidecar catalogue currently returns provider/id only.
  Determine whether a closed source-frozen provider/model/value table is small
  and durable enough for preparation-time admission without remote catalogue
  or account facts. If not, Research 228 is empty.
- **Related evidence:** Research 181 freezes the SDK-sidecar boundary and exact
  package/runtime axes. g04.033 freezes durable new/load/resume, resource
  binding, replay, and cleanup. Contract 040 forbids clamping or substituting a
  portable reasoning value.
- **Primary exact sources:**
  `https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/docs/sdk.md`,
  `https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/sdk.ts`,
  and
  `https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session.ts`.
  Follow exact imports to the tagged `ThinkingLevel`, model, and clamp sources.
- **Decisions and preferences:** portable `ReasoningSelection` only for exact
  rows; no adapter string escape hatch; no live proof; an empty Research 228
  set is valid when static membership or effective attachment truth cannot be
  frozen.
- **Report after:** Research 228 and card 225 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  226-227 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 225 as one coherent evidence chunk. Begin with exact tagged Pi
0.84.2 type/model/clamp sources, then trace explicit and omitted thinking level
through SDK construction, stored sessions, runtime replacement, subscription,
sidecar snapshots, Rust startup/attachment validation, fixtures, and prepared
surfaces. Promote an exact empty or non-empty Research 228 set before touching
production binding.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare it with the placeholders or create
   another worktree merely because they differ.
3. Otherwise stop before edits. Use the named registered worktree if it is
   clean and correctly based. If no usable worker worktree exists, follow the
   `.agents.local.env` fallback policy in `## Current State`. Never edit
   `main`, a dirty checkout, or another worker's branch.
4. Fetch `origin/main`. Require the worker base to contain planning commit
   `5cbb4d6ff4726364c7fe3bde6313fc248211f625`. Fast-forward or recreate the
   clean worker branch if needed; do not merge main into it.
5. Read `AGENTS.md`, the `northstar` and `effigy` skills, the g04.081 roadmap,
   cards 225-227, Research 228, Research 181, g04.033, the Pi SDK-sidecar
   prepared guide, relevant contracts, and the advanced-feature triage tail
   before edits.

### Execute and stop correctly

6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Treat the
   inherited doctor baseline above as known; report only drift.
7. Execute card 225. Use exact tagged public source and deterministic,
   prompt-free local evidence. Do not install the package, send a provider
   request, use credentials, inspect account state, or mutate ambient
   configuration.
8. Promote Research 228 and update card/milestone state. If the deliver-now set
   is empty, mark cards 226-227 blocked, complete the honest evidence-stop
   closeout, update indexes and the sole Next Task, validate, and stop.
9. If and only if Research 228 admits a non-empty exact set, execute cards 226
   and 227 serially. Bind only exact admitted rows. Do not use fixtures to
   invent model membership, clamp behavior, persistence, replacement, or
   effective state the production route cannot guarantee.
10. Work in meaningful batches. Run focused validation after the evidence
    chunk and the complete named acceptance round once after implementation.
    Do not repair inherited doctor findings or unrelated papercuts.

### Prepare the review handoff

11. Update Research 228, the roadmap/cards, Pi prepared guide, matrices,
    programme, triage, logs, indexes, changelog, API baseline when changed, and
    sole Next Task so they agree on complete delivery or honest stop.
12. Run every applicable card command. At minimum run the exact package-focused
    selectors and all named docs/index checks. Run `git diff --check` and
    `effigy doctor`; record exact failures or baseline drift.
13. Review `git diff --stat`, `git diff --check`, `git status --short`, and the
    full changed-file list. Ensure the branch contains no credentials, fetched
    source caches outside authorized evidence, runtime caches, ambient config,
    generated probe debris, or unrelated changes.
14. Commit coherent worker changes, push the worker branch, and open one PR to
    `main`. Do not merge it. Confirm the PR head SHA equals the pushed branch
    head and report required CI state.
15. Return a compact operator report containing: outcome and evidence tier;
    exact Research 228 deliver-now or empty set; cards executed/blocked; files
    and public API changed; validation and doctor drift; PR URL, number, base,
    head SHA, mergeability, and CI; unresolved risks; and the precise next move.
    Keep g04 open.
