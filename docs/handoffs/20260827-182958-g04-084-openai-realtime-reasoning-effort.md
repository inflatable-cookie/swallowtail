---
title: g04.084 OpenAI Realtime reasoning effort worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-182958-g04-084-openai-realtime-reasoning-effort.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reviewed and fast-forwarded g04.083's four evidence lanes,
then promoted their shared result. Three candidates closed with honest empty
sets. Research 236 qualified five exact future session-scoped OpenAI Realtime
reasoning-effort rows.

The orchestrator closed g04.083 and compiled g04.084 cards 236-237 for the one
positive delivery set. This is the handoff to one bounded implementation
thread. Start from this file without a copied transcript or second prompt. Do
not spawn internal agents; the operator owns parallelism in their harness.

## Why It Matters

`openai.realtime` already fixes exact model `gpt-realtime-2.1`, public API-key
access, manual PCM media, two serial responses, native response cancellation,
and a positive output-token maximum. The shared realtime request already carries
portable reasoning, but the OpenAI route rejects it and sends no
`session.reasoning.effort`.

Research 236 closes the exact Realtime vocabulary and acknowledgement seam.
This lane can expose useful caller selection without importing Responses
semantics or claiming effective reasoning depth.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `ed1a2c296f7785deceaa08f9a014d76a8bd81361`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** g04.083 closeout; g04.084; cards
  236-237; Research 236 promotion; inventory, programme, generation, triage,
  research/log/card indexes; sole Next Task.
- **Worker branch:** `worker/g04-084-openai-realtime-reasoning-effort`
- **Worker worktree:** prefer the launcher worktree. Named fallback:
  `/Users/tom/Dev/worktrees/swallowtail-g04-084-openai-realtime-reasoning-effort`
- **Worktree creation command:** only if preflight permits:
  `git worktree add -b worker/g04-084-openai-realtime-reasoning-effort /Users/tom/Dev/worktrees/swallowtail-g04-084-openai-realtime-reasoning-effort origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree. If current context is unusable, use the named
  worktree when it matches; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique manual worktree/branch
  under that container from `origin/main`. Ask the operator if the key is
  absent; never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** per-route feature completion programme.
- **Roadmap milestone:**
  `docs/roadmaps/g04/084-openai-realtime-reasoning-effort.md`
- **Ready cards, in order:**
  `236-openai-realtime-reasoning-effort-binding.md`, then
  `237-openai-realtime-reasoning-effort-acceptance.md`.
- **Allowed runway:** bind and prove Research 236's five exact session-scoped
  values on `openai.realtime`; one PR.
- **Remaining card budget:** two cards.
- **Dispatch topology:** one serial worker lane; no subagents.
- **Parallel safety check:** safe alongside the papercuts wave-2 worker. This
  lane owns OpenAI Realtime code, route-local tests/docs/evidence, its cards,
  milestone, and log. It does not edit `PAPERCUTS.md`, OpenCode fixtures, or
  affected-package scripts.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 026, 029, 037, 040, 047, and 052; Research 236; the Realtime prepared
  integration guide.
- **Model capability profile:** exact route-local Rust implementation and
  deterministic protocol/conformance proof; no provider access.
- **Tool/runtime restrictions:** use Effigy selectors; no credential, account,
  endpoint, socket, provider request, media operation, paid work, install,
  update, or live probe.
- **Required validation:** `cargo fmt -p swallowtail-adapter-openai`;
  `effigy validate:focused swallowtail-adapter-openai`;
  `effigy package:verify-affected swallowtail-adapter-openai`;
  `effigy check:examples`; `effigy qa:routes`; `effigy qa:northstar`;
  `effigy package:api`; `git diff --check`.
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings (334
  warnings, 46 errors); `scan.generated-in-src` reports one warning. Do not
  widen or repair that unrelated baseline.
- **PR base/head:** current pushed `main` / selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting worker implementation and PR.
- **Merge authorisation:** absent; do not merge.

## Boundaries

- **In scope:** exact optional `ReasoningMode` prepared input; five-value gate;
  capability/constraint/plan/evidence/request agreement; new opaque facade and
  private behavior point; historical facade preservation; OpenAI
  `session.update` encoding; explicit matching `session.updated`
  acknowledgement; omission; output-maximum composition; fresh working-state
  restoration; lifecycle/failure/cleanup proof; route guide/matrices/example as
  needed; package API baseline; changelog; Research 236; cards 236-237;
  g04.084; reserved route-local log.
- **Out of scope:** per-response reasoning override, effective reasoning depth,
  reasoning-token inference, thought summaries, billing claims, live provider
  work, another model or OpenAI route, model selection, tools, text turns,
  WebRTC, SIP, browser use, rollover, generic settings, retry, fallback,
  contract changes, currentness, release, shared inventory/programme/index/Next
  Task closeout, generation rollover, g04 closure, or merge.
- Admit only `minimal|low|medium|high|xhigh`. Reject `none`, `max`, `off`,
  `default`, `on`, `auto`, aliases, casing variants, and numeric budgets before
  endpoint, credential, socket, or media work.
- Encode only `session.update.session.reasoning.effort`. Do not use
  `response.create.response.reasoning`.
- Explicit selection requires an exact matching acknowledgement before a
  usable session is returned. Missing, malformed, foreign, or mismatched values
  fail and join cleanup. Omission retains current no-`reasoning` bytes and the
  existing acknowledgement behavior without a default claim.
- Mint exact facade `openai-realtime-reasoning-2026-08-27` and private behavior
  `openai.realtime-manual-pcm-reasoning-v2`. Retain
  `openai-realtime-2026-07-22` and `openai.realtime-manual-pcm-v1` as
  superseded proof; do not backfill them.
- Preserve the selected request through fresh context-losing restoration.
  Planned rollover stays disabled.
- Keep requested, planned, dispatched, acknowledged, effective, returned,
  usage, and observed truth separate.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04's per-route feature programme; g04.083 card 235 and
  Research 236 provide the exact evidence gate; g04.084 owns delivery.
- **Current prepared seam:** `OpenAiRealtimeSessionProfileInput` carries media,
  deadline, disabled rollover, and optional output maximum. Preparation already
  builds `OpenRealtimeMediaSessionRequest`, whose shared carrier supports
  optional reasoning.
- **Current driver seam:** `OpenAiRealtimeDriver::validate` rejects any
  reasoning selection. Connection setup sends one `ClientEvent::SessionUpdate`
  with optional output maximum and accepts `SessionConfigured` after checking
  model/media fields.
- **Current restoration seam:** `OpenAiPreparedRealtimeSession` clones the same
  immutable plan and request into fresh context-losing replacement.
- **Exact value evidence:** official Realtime schema closes
  `minimal|low|medium|high|xhigh` for `gpt-realtime-2.1`. Responses values
  `none|max` are absent. `session.updated` returns the effective session
  configuration, including optional reasoning.
- **Acknowledgement tension:** explicit selection must compare exactly before
  session return. Omission must preserve the existing parser behavior rather
  than requiring the provider to omit or echo a default effort.
- **Facade tension:** the current constant and fixtures name the v1 no-reasoning
  behavior. Follow the established Gemini Live superseded-facade pattern; do
  not mutate historical proof in place.
- **Composition:** every selected value must compose independently with output
  maximum omission and positive 1..=4,096.
- **Decisions and preferences:** portable `ReasoningSelection` only; no OpenAI
  string escape hatch; no live proof; no per-response control; no inferred
  provider default or effectiveness.
- **Report after:** card 236 binding and its focused validation, then card 237
  acceptance and the PR. Report earlier only for a real stop condition.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Start with card 236. Trace reasoning from prepared input through the existing
shared request carrier, exact facade/capability plan, OpenAI validation,
`session.update`, `session.updated`, failure cleanup, and fresh restoration.
Keep omission byte-for-byte exact. Continue to card 237 only after the selected
value is confirmed before returning a usable session.

## Completion Protocol

### Before you start

1. Read this handoff. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain` before broad
   repository reads.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as launcher-provided. Record its actual
   root/branch and do not compare them with the placeholders or create another
   worktree merely because they differ.
3. Only if current context is `main`, dirty, unregistered, or unusable should
   you inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if absent. Create a unique worktree and branch there from
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path; never clean,
   reset, stash over, or discard another checkout. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, fetch origin, confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor ed1a2c296f7785deceaa08f9a014d76a8bd81361 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, g04.084, cards 236-237, Research 236, the named contracts,
   and the Realtime guide.
6. Run the repo's cheap orientation checks and record what you ran.

### While you work

- Execute cards 236-237 in order. Keep commits aligned with the binding and
  acceptance chunks.
- After each meaningful chunk, report changed files, validation actually run,
  remaining work, risks, and blockers through the operator.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  acknowledgement cannot be exact, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.

### When the assigned runway is complete

1. Run the final validation named in `Current State`.
2. Update Research 236, cards 236-237, g04.084, and the reserved route-local log
   with actual evidence. Record shared closeout deltas without editing the
   inventory, programme, indexes, or sole Next Task.
3. Push the selected worker branch.
4. Open one reviewable PR against current pushed `main`. The planning base above
   predates this handoff commit and is intentionally not self-referential.
5. Link the milestone, cards, Research 236, changed surfaces, validation, and
   unresolved items in the PR body.
6. Report the PR URL. Do not merge.

### Review and merge path

The orchestrator will review metadata, commits, diff, checks, and changed files
against g04.084, cards 236-237, Research 236, and the governing contracts. Shared
GitHub identity prevents formal self-approval, so the orchestrator's PR comment
is the canonical review record. Requested changes: none yet. The operator must
explicitly authorise merge.

- **Closeout refs:** Research 236; cards 236-237; g04.084; reserved route-local
  log; feature/route matrices; Realtime guide; package API baseline; changelog;
  shared inventory/programme/index/Next Task after merge.

### Handoff closeout

Leave assigned card, milestone, Research, and log state honest. If blocked,
record the exact blocker and stop. Do not make the lane look complete or widen
the feature to get a green check.
