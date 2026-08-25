---
title: g04.062 Anthropic Messages adaptive-thinking worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
planning_base: 19c3250556f17daed5679b2034204f0363e7b449
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260825-152529-g04-062-anthropic-adaptive-thinking.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reconciled g04.061, reassessed the remaining per-route feature
inventory, and selected adaptive thinking on `anthropic.messages`. g04.062 is
compiled. Implementation has not started. The runway begins with exact current
official request/response/tool-continuation evidence; cards 174-175 are
conditional on a non-empty Research 209 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md` before task work. This handoff's metadata selects
the implementation worker loop. Follow the ready cards and the completion
protocol below.

## Why It Matters

`anthropic.messages` already binds exact `claude-opus-4-7` effort on bounded
structured attempts and resource-free consumer-tool continuation. Current
official Anthropic documentation treats adaptive thinking as a separate
Messages control and requires the complete signed thinking-block sequence to
be returned unmodified with tool results.

The route cannot safely do that today. Its SSE grammar accepts text, tool use,
and web search but rejects thinking blocks and signature deltas. Its private
session history reconstructs the assistant tool-use block without any signed
thinking blocks. Enabling `thinking` only in the request would therefore turn
the existing continuation into a provider 400 or, worse, incomplete private
state handling.

Contracts 030 and 044 already provide the boundary: bounded, route-bound,
zeroized provider-private continuation; no hidden-reasoning disclosure. The
candidate first tranche uses omitted display. It enables adaptive provider
reasoning but exposes no thought text, summary, signature, redacted data, or
raw block through portable output, activity, callbacks, evidence, formatting,
or diagnostics.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `19c3250556f17daed5679b2034204f0363e7b449`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `19c3250556f17daed5679b2034204f0363e7b449` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.062, cards 173-175,
  Research 209 reservation, compilation log, route-local closeout reservation,
  Anthropic effort-inventory correction, programme boundary, and sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-062-anthropic-adaptive-thinking-20260825-152529`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-062-anthropic-adaptive-thinking-20260825-152529`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-062-anthropic-adaptive-thinking-20260825-152529
  -b agent/g04-062-anthropic-adaptive-thinking-20260825-152529 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion; Contracts 030, 040, and
  044 are already authoritative; no contract edit is planned
- **Roadmap milestone:**
  `docs/roadmaps/g04/062-anthropic-messages-adaptive-thinking.md`
- **Ready cards, in order:** card 173, then conditional card 174, then
  conditional card 175
- **Allowed runway:** exact Anthropic adaptive-thinking evidence, then only
  Research 209-admitted adapter-local binding and route-local acceptance
- **Remaining card budget:** three serial cards; cards 174-175 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share request/profile API, SSE grammar,
  private continuation state, fixtures, guide, research, and closeout; they are
  not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 030, 037, 040, 041, 044, and 052
- **Route identity:** `anthropic.messages`, driver
  `swallowtail.anthropic.direct`, facade header `anthropic-2023-06-01`, hosted
  API-key access
- **Candidate model:** exact `claude-opus-4-7`; Research 209 owns final
  model/profile qualification
- **Current model-route control:** exact effort
  `low|medium|high|xhigh|max` through portable `ReasoningSelection` on
  structured attempts and fixed direct-continuation sessions
- **Candidate new control:** opaque adapter-local
  `AnthropicThinkingMode::adaptive()` mapping only to the exact
  adaptive/omitted-display request qualified by Research 209
- **Current structured behavior:** one streamed request, no continuation,
  thinking events currently rejected, private response state discarded at
  terminal
- **Current continuation behavior:** bounded two-turn consumer-tool profile;
  assistant tool-use history is rebuilt in zeroizing memory; one exact
  correlated result authorizes the next provider attempt; fresh restoration
  returns `SessionReplaced`
- **Current activity truth:** assistant, consumer-tool, and qualified search
  activity only. No reasoning-summary claim. This lane must not add one.
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no account, credential, catalogue, endpoint, live
  Messages, paid inference, or browser-login work. Current official public
  Anthropic documentation, local source/fixtures, and deterministic specimens
  are allowed by card 173.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-anthropic`, `effigy validate:focused
  swallowtail-adapter-anthropic`, `effigy package:verify-affected
  swallowtail-adapter-anthropic`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, relevant research/logs/roadmaps/g04/batch-card/next-
  action index gates, `effigy package:api`, `effigy doctor`, and
  `git diff --check`
- **Known doctor baseline:** inherited 378 god-file findings: 332 warnings and
  46 errors; stale graph index; one generated-in-src warning. New parser and
  fixture tests must be focused and must not increase those counts.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, all docs index and next-action gates, and `git diff --check`
  passed before the planning commit. A later attempted `effigy qa:indexes`
  failed only because that aggregate task does not exist; it changed nothing.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-anthropic/**` for exact prepared
  adaptive-thinking input/evidence, request encoding, SSE grammar, private
  block capture/replay, bounds, zeroization, failures, fixtures, examples, and
  package API baseline; `docs/guides/anthropic-direct-prepared-integration.md`;
  feature matrix only when warranted; Research 209; g04.062; cards 173-175;
  reserved closeout; triage, programme, and sole Next Task; current official
  Anthropic thinking/Messages/tool/stream/model docs; existing secret-free
  repository evidence
- **Out of scope:** portable generic thinking capability; raw provider JSON or
  generation map; manual `thinking.type=enabled`; `budget_tokens`; readable
  thinking summaries; hidden reasoning; `ReasoningSummary` activity; another
  model/facade/access route; newer web-search type; Managed Agents; Claude Code;
  UltraCode; Fast mode; durable private state; live provider work; currentness;
  `CHANGELOG.md`; shared architecture/contracts/runtime; release, publication,
  merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a shared contract,
  shared runtime change, or provider-neutral capability, stop for orchestrator
  review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, access, privacy, persistence, or compatibility decision.
- Current official documentation is a lead. Research 209 must freeze retrieved
  source bodies/dates/hashes and the exact supported model/profile/display row.
- Only omitted-display adaptive thinking is a candidate. Summarized display is
  outside this lane even if official documentation supports it.
- `adaptive` is not a `ReasoningMode` value. Existing effort remains its own
  optional exact portable control. Omission and all admitted effort values must
  compose without defaults, clamps, inference, or shared confirmation.
- Research 209 must freeze every private response shape that the selected
  request can produce: thinking, redacted thinking, block start/delta/stop,
  thinking delta, signature delta, opaque signature/data, ordering,
  multiplicity, and the valid no-thinking case.
- Structured attempts may validate and discard qualified private blocks. They
  must retain none after terminal and emit no thought content or activity.
- Direct continuation must replay the complete required private block sequence
  before the correlated tool-use block, in exact order and unmodified. Never
  reconstruct or omit a block based on consumer-visible transcript state.
- Private thinking text, signature, redacted data, and raw blocks must remain
  bounded, zeroizing, route/session-bound, non-serializable, redacted from
  formatting/diagnostics/debug evidence, and destroyed on close/invalidation.
- Missing, duplicate, reordered, altered, malformed, oversized, foreign,
  stale, contradictory, or post-terminal private state fails closed. It grants
  no retry, fallback, another request, or weaker mode.
- A consumer tool result remains the only authority for a continuation
  attempt. Thinking blocks never authorize tool execution or network work.
- Fresh restoration remains replacement with no thinking-state recovery,
  export, import, resume, reconstruction, or durable transcript claim.
- Default QA must not resolve credentials or call Anthropic.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 004 and 067 established the Anthropic direct
  route and protocol corpus; Research 169 established its addable/API-key
  boundary; Research 185 and g04.037 delivered exact Opus 4.7 effort. PR 37
  landed that route-local control at `56a7b87b`.
- **Official lead:** current Anthropic docs describe
  `thinking: {type: adaptive}`, omitted versus summarized display, streamed
  `thinking_delta` and `signature_delta`, and complete unmodified block replay
  with tool results. Re-fetch and digest the decisive official bodies rather
  than copying this planning observation into promoted evidence.
- **Current request path:** `src/protocol.rs` builds structured and direct
  request JSON. Effort is additive through `output_config.effort`; no
  `thinking` object exists.
- **Current stream path:** `src/protocol/events.rs` recognizes only text,
  client tool, and qualified search blocks. Any thinking content start or delta
  currently becomes a protocol failure.
- **Current continuation path:** `src/driver/session/history.rs` stores user,
  tool id/name/arguments, result, and final answer. Its assistant continuation
  message contains only `tool_use`; it has no private thinking record.
- **Current parser path:** `src/driver/session/attempt.rs` assumes one active
  text or tool block and finalizes a tool outcome only when assistant text is
  empty. Adaptive/interleaved block order therefore needs exact state-machine
  qualification, not an additive ignored event.
- **Current privacy posture:** Contract 030 already demands bounded zeroizing
  private continuation. Contract 044 excludes hidden reasoning and private
  continuation from activity. Do not create a new public activity merely
  because the API can return display text.
- **Prepared API precedent:** `DeepSeekThinkingMode` is an opaque adapter-local
  typed dispatch control, not proof that the Anthropic shape is identical.
  Research 209 owns the Anthropic type, profile, and claim boundary.
- **Model qualification:** exact `claude-opus-4-7` is a candidate because its
  effort row is already admitted. Catalogue presence, model-family names,
  aliases, or later docs do not qualify another model.
- **Claim strength:** deterministic request fixtures prove planned/dispatched
  adaptive mode. A successful response fixture proves parser acceptance only.
  Do not claim effective thinking depth from blocks, output, tokens, or prose.
- **Research outcome:** an honest evidence stop after card 173 is complete work
  if model/display support, private grammar, replay, effort composition, or
  privacy cannot be frozen. Mark cards 174-175 blocked and open the evidence PR.
- **Generation boundary:** do not close or roll over g04. After merge the
  orchestrator reconciles g04.062 and follows the sole Next Task.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. New-route research does not pre-empt per-route feature
  work.
- **Report after:** card 173 and Research 209 are complete, then after the
  binding/acceptance batch if the deliver-now set is non-empty
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight in `## Completion Protocol`. If the current context is a clean,
dedicated, non-`main` registered worktree, use it immediately, record its actual
path/branch, and do not create another worktree because its generated name
differs from this file.

Read `AGENTS.md`, g04.062, cards 173-175, Research 004/067/169/185/209, the
Anthropic direct guide, system architecture, and Contracts
011/030/037/040/041/044/052. Execute card 173 first. Promote Research 209 with
a non-empty exact table or an honest empty set. Continue automatically only
when its gate is satisfied.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad reads: `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare it with the placeholders or create a
   second worktree merely because names differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if absent. Create a unique worktree/branch there from pushed
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path. Never clean,
   reset, stash-over, or discard another checkout. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm `git merge-base --is-ancestor
   19c3250556f17daed5679b2034204f0363e7b449 HEAD` succeeds, and confirm this
   handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline and do not run the full planned workspace suite.

### While you work

- Execute cards 173-175 in order. Stop after card 173 when Research 209 is
  empty or a named gate fails. An evidence stop is a complete worker outcome.
- Keep commits aligned with meaningful chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the card-specific final validation. If code executes, run
   `cargo fmt -p swallowtail-adapter-anthropic`, `effigy validate:focused
   swallowtail-adapter-anthropic`, `effigy package:verify-affected
   swallowtail-adapter-anthropic`, `effigy check:examples`, `effigy qa:routes`,
   `effigy qa:northstar`, the relevant docs index gates, `effigy package:api`,
   `effigy doctor`, and `git diff --check`. If card 173 stops with docs only,
   run its named docs/index/diff gates and record why code-only gates did not
   apply.
2. Update Research 209, milestone/cards, Anthropic guide/matrix only as
   warranted, reserved closeout, programme, triage, and sole Next Task. Keep
   g04 open.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The planning
   base above predates this handoff commit and is intentionally not
   self-referential.
5. In the PR body, link the milestone, cards, Research 209, changed surfaces,
   exact evidence, validation, and unresolved items.
6. Report the PR URL and exact head SHA to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator and worker may share one GitHub identity. Formal self-approval
is then unavailable; the orchestrator posts the evidence-backed verdict as a
PR comment. If changes are requested, make only those changes on this branch,
push again, and report back through the operator. Requested changes: none yet.
The operator must explicitly authorise any merge.

- **Closeout refs:** Research 209; g04.062; cards 173-175; reserved g04.062
  closeout; Anthropic guide/matrix only as warranted; triage; programme; sole
  Next Task
- **Merge conditions:** exact Research 209 deliver-now truth; all executed
  cards complete; required gates green; PR head reviewed; no unresolved drift,
  privacy, bounds, replay, compatibility, composition, or lifecycle issue;
  explicit operator merge command
- **After merge:** fast-forward only from the exact reviewed green head, then
  complete the post-merge closeout on `main`. Keep g04 open and follow the sole
  Next Task unless the operator supplies a different direction.
