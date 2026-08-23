---
title: g04.050 DeepSeek structured-run thinking-mode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-191009-g04-050-deepseek-thinking-mode.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed g04.049 after PR 48, resumed the sole roadmap Next
Task, reassessed the remaining promoted per-route feature inventory, and
compiled g04.050 as the final numbered roadmap in generation g04. DeepSeek
thinking-mode implementation has not started. The ready runway begins with
exact current official and repository evidence and permits binding only for a
Research 197 deliver-now one-request structured-run subset.

Ollama attached `think=max` was assessed first but not selected. The attached
catalogue advertises only generic thinking support, while exact Ollama 0.32.15
maps `max` to `high` for Harmony/GPT-OSS. That is not enough to promise a
distinct exact mode for an arbitrary selected model.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. You can start from this file without a copied
transcript or a second prompt.

## Why It Matters

`deepseek.continuation` already gives consumers exact V4 Pro
`low|high|max` reasoning on one-request runs and bounded direct tool
continuation, but it always sends `thinking.type=enabled`. Current official
DeepSeek material names explicit `enabled|disabled` thinking mode and lists V4
Pro as supporting both.

Non-thinking mode is a real missing route feature, not another reasoning-effort
label. A safe implementation must omit any false portable reasoning selection,
keep existing enabled calls byte-stable, and leave direct continuation
enabled-only because that proof depends on private `reasoning_content` replay.
This closes one concrete failure from the per-route feature inventory without
flattening provider semantics.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `61f18efe86474c55fdb69fc8e04e40ec250cebbd`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `61f18efe86474c55fdb69fc8e04e40ec250cebbd` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Done:** g04.050, cards 139-141, Research 197 reservation, compilation log,
  route-local closeout reservation, triage selection, and the sole Next Task
  are published on `main`
- **Still open:** card 139 exact evidence; conditional cards 140-141; worker
  PR, review, merge, orchestrator shared closeout, and g04 boundary closeout
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-050-deepseek-thinking-mode-20260823-191009`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-050-deepseek-thinking-mode-20260823-191009`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-050-deepseek-thinking-mode-20260823-191009 -b agent/g04-050-deepseek-thinking-mode-20260823-191009 origin/main`
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
- **Roadmap milestone:** `docs/roadmaps/g04/050-deepseek-structured-run-thinking-mode.md`
- **Current batch card:**
  `docs/roadmaps/g04/batch-cards/139-deepseek-structured-run-thinking-mode-evidence.md`
- **Ready cards, in order:** card 139, then conditional card 140, then
  conditional card 141
- **Allowed runway:** exact DeepSeek V4 Pro thinking-mode evidence, then only
  Research 197 deliver-now adapter-local one-request binding
- **Remaining continuation envelope:** three serial cards; cards 140-141 run
  only after their named evidence and implementation gates
- **Lane budget / pause signal:** one PR. Stop after card 139 if Research 197
  has no deliver-now row or if a shared contract/currentness change is needed.
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  009, 011, 014, 024, 029, 030, 037, 040, 041, and 052
- **Route identity:** `deepseek.continuation`, driver
  `swallowtail.deepseek.direct`, model route/model `deepseek-v4-pro`, axis
  `deepseek.openai-chat-facade`, current facade
  `deepseek-openai-chat-2026-07-22`, private behavior
  `deepseek.v4-thinking-tools-v1`
- **Candidate mapping:** explicit `thinking.type=disabled` for one-request
  structured runs, normally with `reasoning_effort` and portable
  `ReasoningSelection` absent; card 139 owns the exact field and revision
  decision
- **Current enabled mapping:** exact `low|high|max` portable reasoning sends
  the same `reasoning_effort` plus `thinking.type=enabled`
- **Continuation boundary:** enabled-only initial request, tool-result attempt,
  later user turn, and fresh restoration with bounded adapter-private
  `reasoning_content` replay
- **Model capability profile:** exact-model, exact-facade, evidence-first;
  fail closed on mode, effort, model, facade, plan/evidence, driver, request,
  response, cache, profile, continuation, or lifecycle ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account/balance
  inspection, provider request, live DeepSeek call, browser login, or paid
  work. Current official public-source inspection and secret-free deterministic
  repository fixtures are allowed by card 139.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-deepseek`, `effigy validate:focused
  swallowtail-adapter-deepseek`, `effigy package:verify-affected
  swallowtail-adapter-deepseek`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 374 inherited god-file findings: 329 warnings and
  45 errors, plus one generated-in-src warning. Keep inherited findings
  separate from lane-created findings.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/050-deepseek-structured-run-thinking-mode.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/research/197-deepseek-structured-run-thinking-mode-evidence.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/guides/deepseek-prepared-integration.md`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-deepseek/src/protocol/request.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-deepseek/src/prepared_profile/input.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-deepseek/src/prepared_profile/plan.rs`

## Boundaries

Please keep this pass inside the named runway:

- **In scope:** `crates/swallowtail-adapter-deepseek/**` for exact
  adapter-local one-request mode selection, prepared input, plan/evidence,
  driver/request agreement, response parsing, and deterministic tests;
  `docs/guides/deepseek-prepared-integration.md`; Research 197; g04.050; cards
  139-141; the reserved g04.050 route-local closeout; applicable
  `swallowtail-adapter-deepseek` examples and unreleased public-API baseline;
  current official DeepSeek Chat Completions, Thinking Mode, Tool Calls, and
  Models/Pricing sources; secret-free request, response, enabled-effort, cache,
  cancellation, deadline, failure, cleanup, session, tool-result, later-turn,
  and restoration fixtures
- **Out of scope:** disabled direct continuation; portable
  `ReasoningMode("none")`; a shared thinking capability; generic provider
  settings; changes to `low|high|max`; accepting `medium|xhigh`; V4 Flash,
  vision, retired aliases, Responses API, Anthropic facade, `/v1`, another
  model or route; consumer-visible private reasoning; durable private replay;
  automatic tool execution; output-bound or cache-policy changes; retries or
  fallback; live work; Bedrock exact-pin correction; Ollama `max`; currentness;
  `CHANGELOG.md`; shared architecture; route/feature matrices; programme/front
  doors/indexes; shared package lists; release, publication, or merge work
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract change,
  stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, privacy, cache, security, billing, or compatibility decision.
- Do not silently rewrite the current opaque facade behavior. Research 197
  must decide the exact facade/private behavior/claim/model-route revision for
  any admitted state while retaining the current enabled proof.
- Do not represent disabled Chat Completions mode as a portable reasoning
  effort merely because DeepSeek's Responses API uses `reasoning.effort=none`.
- This handoff represents one worker lane. Do not edit another lane's scope. If
  shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- **Repo constraints:** follow `/Users/tom/Dev/projects/swallowtail/AGENTS.md`
  and the canonical architecture/contracts named above. Work in one meaningful
  batch, use Effigy selectors, and keep glue-light reporting.

## Important Context

- **Planning lineage:** the promoted advanced-route inventory feeds the
  per-route feature-completion programme. g04.038 delivered exact DeepSeek
  effort but deliberately fixed thinking enabled. Research 186 explicitly
  left a later structured-only typed control open. g04.050 takes that seam and
  is the final numbered roadmap in g04.
- **How the plan fits the system:** Contract 040 requires exact reasoning
  selection when claimed. Disabled mode is an independent provider field, so a
  disabled run must not claim an exact effort it does not dispatch. Contract
  030 owns direct continuation and requires the enabled private-replay shape;
  this lane leaves it unchanged.
- **Official evidence:** start with current [Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/),
  [Thinking Mode](https://api-docs.deepseek.com/guides/thinking_mode/),
  [Tool Calls](https://api-docs.deepseek.com/guides/tool_calls/), and
  [Models And Pricing](https://api-docs.deepseek.com/quick_start/pricing/).
  Record retrieval dates and complete fetched-body digests. Do not send a
  provider request.
- **Current source truth:** `protocol/request.rs` always encodes
  `reasoning_effort=<selected>` and `thinking.type=enabled`.
  `prepared_profile/input.rs` requires one `ReasoningMode` for both run and
  session inputs. `prepared_profile/plan.rs` stores reasoning as an optional
  plan field. Selection admits only exact `low|high|max`.
- **Field burden:** settle the exact disabled request shape, including whether
  `reasoning_effort` must be absent, what omission means, and how explicit
  enabled differs from the current fixed behavior. Do not borrow the Responses
  API `none` spelling onto Chat Completions.
- **Plan/evidence burden:** the exact route-local mode must survive preparation
  and driver validation. A disabled run cannot reuse `ReasoningSelection` as a
  convenient carrier. If existing adapter-private evidence cannot retain mode
  without shared contract work, stop.
- **Response burden:** DeepSeek documents `reasoning_content` for thinking
  mode. Determine the exact disabled-mode parser disposition. Absence does not
  prove effective non-thinking; unexpected private reasoning must not become
  consumer output.
- **Compatibility burden:** Research 186 said the current facade was sufficient
  for the field's documented existence, but delivery may still need a new
  opaque behavior/facade point. Card 139 owns that decision. Do not widen the
  compatibility window or Contract 029 currentness.
- **Cache burden:** both admitted profiles currently require explicit
  `ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority`. Confirm
  that non-thinking mode changes no claim about cache read, deletion,
  retention, billing, or retry.
- **Preservation burden:** every existing enabled public constructor,
  `low|high|max` plan constraint, request body, response parser, cancellation,
  deadline, cleanup, and continuation/restoration test remains authoritative.
- **Honest stop:** an empty Research 197 deliver-now set is a successful
  evidence result. Close cards 140-141 as blocked, finish the route-local stop
  record, validate, and open the evidence PR.
- **Generation boundary:** do not compile or plan g04.051. The orchestrator
  will reassess and close g04 only after this PR is merged and reconciled.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. Keep parked or unrelated route families out of routine
  reporting.
- **Open tensions:** official docs establish the public field and model support
  but not yet the exact local plan/evidence representation, disabled response
  drift policy, or compatibility revision. Those are card 139 decisions, not
  assumptions for card 140.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 139's exact field/profile/response/facade decision.
  Continue only for a non-empty deliver-now set, then report after the complete
  cards 140-141 implementation and validation chunk.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Start by reading this handoff from the top. Before broad repository reads, run
the quick worktree-safety preflight in `## Completion Protocol`. If the current
context is a clean, dedicated, non-`main` registered worktree, use it
immediately, record its actual path and branch, and do not compare its generated
identity with the fallback above. If it is unusable, use the named worktree if
it matches; only then read `.agents.local.env` and follow its required container
setting. Never fall back to `/tmp` or `TMPDIR`.

Then read `AGENTS.md`, g04.050, cards 139-141, Research 023, 186, and 197, the
DeepSeek prepared guide, exact selection/preparation/request/response/session
code and fixtures, and the canonical contracts from the selected worker
worktree.

Take card 139 as one coherent evidence chunk. Use current official DeepSeek
sources plus deterministic repository evidence; do not send a live request. If
Research 197 has no deliver-now structured-run state, close cards 140-141 as
blocked, finish the route-local stop record, validate, and open the evidence
PR. If an exact set survives, execute cards 140-141 in order and open one
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
   actual path and branch. Do not create another worktree because its name
   differs from this handoff.
3. If the current checkout is `main`, dirty, shared, or otherwise unusable,
   first use the named worker worktree if it already exists and matches. Only
   when it does not, read `.agents.local.env`, require the named container key,
   fetch `origin/main`, and create one unique branch/worktree from the planning
   base. Do not guess a path.
4. Confirm the selected worktree contains planning base
   `61f18efe86474c55fdb69fc8e04e40ec250cebbd` and is clean before editing.
   If `origin/main` moved, use the current pushed main only when it contains
   that planning base; otherwise stop and report the divergence.

### Work the cards

1. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan` once at
   startup. Keep inherited doctor findings separate.
2. Read card 139 and its named refs completely. Freeze official evidence and
   promote Research 197. Do not edit production code during the evidence card.
3. Report the exact deliver-now or stop table to the operator. Continue to card
   140 only if Research 197 has a non-empty structured-run set and no stop
   condition fired.
4. If continuing, implement cards 140 and 141 as one meaningful code/test/docs
   batch. Keep session/continuation behavior enabled-only.
5. Update only route-local worker surfaces. In the reserved closeout, list the
   shared architecture, Contract 029, route/feature matrix, programme, indexes,
   changelog, generation-boundary, and Next Task changes the orchestrator must
   apply after merge.
6. Run the complete card-specific validation once after the coherent batch.
   Record exact pass/fail counts and any inherited baseline.

### PR loop

1. Review `git diff`, `git diff --check`, branch name, and worktree state.
   Commit the worker batch with a concise message. Push the worker branch.
2. Open one PR against current `main`. The PR body must name g04.050, cards
   completed or blocked, Research 197 disposition, exact route/model/facade,
   validation, shared-closeout delta, and all explicit withholds.
3. Do not merge. Report the PR URL and exact head SHA to the operator for the
   orchestrator's review loop.
4. If review requests changes, keep the same branch, worktree, PR, and lane.
   Fix only in-scope issues, rerun proportionate validation, push, and report
   the new exact head.
5. Do not restack or merge unless the operator explicitly asks in a later
   message. The orchestrator owns exact-head review, CI state, fast-forward
   merge, shared closeout, and generation-boundary reconciliation.

### Before you finish

1. Confirm card 139 and Research 197 record the exact evidence disposition.
2. Confirm cards 140-141 and g04.050 reflect completed work or an honest stop.
3. Confirm the reserved route-local closeout records actual validation, PR,
   head, unresolved risks, and the shared closeout delta without claiming
   merge.
4. State whether the continuation envelope ended after evidence or after all
   three cards. There is no later worker card in this handoff.
5. Leave the operator one clear next task: review the exact PR head. After
   merge, the orchestrator—not this worker—closes g04 and selects the next
   generation boundary.
