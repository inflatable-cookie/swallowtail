---
title: g04.039 xAI Responses controls worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-133218-g04-039-xai-responses-controls.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, xai, per-route-features]
---

## What This Thread Was Doing

The orchestrator merged the Anthropic and DeepSeek feature lanes in fixed order,
reconciled their deferred shared surfaces, refreshed current official xAI
evidence, and compiled g04.039 with cards 107-109. Research 187 and the route
closeout log are reserved and pre-indexed.

This is one bounded implementation run: qualify the exact current xAI
WebSocket/model/control subset, bind only surviving reasoning and output rows,
then prove connection-local dispatch. It stands alone; no copied transcript or
second prompt is needed.

## Why It Matters

`xai.responses-websocket` already provides one-response runs and serial
connection-local continuation, but its driver rejects reasoning and maximum
output-token policy. Consumers cannot request either official control through
the prepared facade.

The upstream surface has moved since the original inventory. Grok 4.5 and 4.6
have different reasoning sets, Grok 4.6 documents no intrinsic text output
limit, and multi-agent effort controls agent count rather than reasoning depth.
Exact evidence must lead the implementation.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Repository posture:** strict-ready Northstar
- **Planning branch:** `main`
- **Planning base commit:** `834476f4fa504badad9d6fce8920a315cde98f19`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled
  `834476f4fa504badad9d6fce8920a315cde98f19` before this handoff was created.
  Fetch again at startup; the later `main` tip contains this handoff.
- **Planning checkout:** clean on `main` after the pushed planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** merged g04.037-038 shared
  closeout; g04.039; cards 107-109; pre-indexed Research 187 and closeout
  reservation; compile log; sole Next Task
- **Worker branch:** `g04-039-xai-responses-controls`
- **Worker worktree:** launcher-provided dedicated worktree first. Named manual
  worktree: `/Users/tom/Dev/worktrees/swallowtail-g04-039-xai-responses-controls`
- **Worktree creation command:** only if startup requires the manual fallback:
  `git worktree add -b g04-039-xai-responses-controls /Users/tom/Dev/worktrees/swallowtail-g04-039-xai-responses-controls origin/main`
- **Worktree policy:** use a clean, dedicated, non-`main` registered worktree
  supplied by the launcher even if its generated path or branch differs. Record
  the actual values and do not create another worktree. If current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the
  operator if absent. Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. The promoted per-route programme and Contracts
  037 and 040 own the delivery boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g04/039-xai-responses-reasoning-output-bounds.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/107-xai-responses-control-evidence.md`,
  `docs/roadmaps/g04/batch-cards/108-xai-responses-control-binding.md`, then
  `docs/roadmaps/g04/batch-cards/109-xai-responses-control-acceptance.md`
- **Allowed runway:** official secret-free evidence and Research 187; exact
  reasoning and output-bound binding for only deliver-now WebSocket
  model/value/profiles; deterministic continuation; route-local closeout; one PR
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial. Card 107 defines the exact subset consumed by
  cards 108 and 109. No parallel worker lane is approved.
- **Parallel safety check:** mutable scope is limited to the xAI adapter,
  fixtures, prepared guide, milestone/cards, reserved Research 187, reserved
  xAI closeout log, and package-specific API baseline. Shared surfaces are
  explicitly reserved for orchestrator closeout.
- **Canonical refs:** `AGENTS.md`;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/triage/2026-08-21-advanced-route-features.md` (promoted);
  `docs/architecture/system-architecture.md`; Contracts 011, 020, 024, 029,
  037, 040, 041, and 052; Research 004, 067, and 169;
  `docs/guides/xai-prepared-integration.md`;
  `docs/guides/provider-route-matrix.md`;
  `docs/guides/provider-solution-feature-matrix.csv`
- **Exact existing facade:** `xai-responses-websocket-2026-04-23` on
  `xai.responses-websocket-facade`; current prepared run input binds model,
  content, and deadline, while session input binds model and deadline. The
  low-level driver rejects reasoning and maximum-output-token policy.
- **Current official leads:** WebSocket Mode says `response.create` uses the
  Responses create body minus transport-only fields. Current reasoning docs name
  Grok 4.5 `low|medium|high` and Grok 4.6
  `low|medium|high|xhigh`; Grok 4.6 documents no intrinsic text output limit;
  Responses examples still expose optional `max_output_tokens`. These are leads,
  not qualified findings. Card 107 must freeze exact current evidence.
- **Official lead URLs:**
  `https://docs.x.ai/developers/advanced-api-usage/websocket-mode`;
  `https://docs.x.ai/developers/model-capabilities/text/reasoning`;
  `https://docs.x.ai/developers/grok-4-6`;
  `https://docs.x.ai/developers/models`; official release notes and the exact
  Responses reference/specimens selected by card 107
- **Model capability profile:** capable coding model with medium or higher
  reasoning; frontier review for public API, model-qualification, facade, or
  Contract 040 ambiguity
- **Tool/runtime restrictions:** use Effigy selectors and official prompt-free
  sources; temporary downloads only under `/tmp`; do not authenticate, inspect
  account state, send a prompt, or mutate provider state
- **Known repository health:** the last recorded `effigy doctor` had 42
  inherited god-file errors, stale graph, and one generated-in-src warning.
  Existing findings are in `PAPERCUTS.md`; record new friction only when distinct.
- **Planning validation:** `effigy qa:docs`; `effigy qa:routes`;
  `effigy qa:northstar`; `effigy test --plan`; `git diff --check`
- **Required final validation:** `cargo fmt -p swallowtail-adapter-xai`;
  focused and affected-package gates for `swallowtail-adapter-xai`;
  `effigy check:examples`; `effigy qa:routes`; `effigy qa:northstar`; research,
  logs, roadmaps, g04, and batch-card index gates; roadmaps next-action gate;
  `effigy package:api`; `git diff --check`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** official secret-free WebSocket, Responses-body, model,
  reasoning, and output-bound evidence; Research 187; exact dispositions;
  additive prepared reasoning and maximum-output inputs; capability constraints;
  plan/evidence/driver/protocol binding; deterministic failures; xAI prepared
  guide; g04.039; cards 107-109; reserved closeout log; package-specific API
  baseline; one PR.
- **Out of scope:** web/X search, tools, code execution, files, citations,
  multi-agent, Grok Bot, Grok Build, prompt caching, encrypted reasoning export,
  durable storage, warmup, background, reattachment, another xAI route, live
  provider/account work, release, or publication.
- Do not edit shared closeout surfaces: `CHANGELOG.md`,
  `docs/architecture/system-architecture.md`, provider route/feature matrices,
  `docs/roadmaps/g04/per-route-feature-completion.md`, roadmap front doors,
  shared indexes, matrix validation snapshots, or
  `release-baselines/public-api-0.3.3/packages.txt`. Record their exact required
  delta in the reserved closeout log and PR body.
- Replace only the pre-indexed Research 187 and xAI closeout reservations; do
  not edit their indexes.
- Map reasoning to portable `ReasoningSelection` only for exact Research 187
  rows. Multi-agent effort controls agent count and is not portable reasoning.
- Use the existing portable maximum-output-token authority only where Research
  187 admits it. Do not truncate output locally or infer effective length.
- Request, plan constraints, evidence, configured driver, and every wire request
  must agree. Preserve byte-identical absent-control bodies where compatible.
- For sessions, one preparation-time selection applies to first response, later
  turns, and fresh replacement. No per-turn raw override.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved public API or compatibility decision. Pause on a contract gap.
- Work only in the selected clean worker worktree. Do not merge the PR.

## Important Context

- **Planning lineage:** the promoted advanced-feature inventory ranked xAI
  Responses controls after Cursor, Ollama, Anthropic, and DeepSeek. Those four
  families are merged; the programme advances one route/control family at a
  time.
- **Why these cards are ready:** the route already has an exact dated WebSocket
  facade, immutable prepared operations, serial continuation, and fail-closed
  policy rejection. Official material names the candidate fields. Card 107 is
  deliberately the exact model/value/profile gate.
- **Decisions and preferences:** exact allowlists over model-family inference;
  portable reasoning only where semantics match; no multi-agent flattening;
  dispatch truth before acceptance/effectiveness claims; no live work.
- **Open tensions:** current model drift may require a new facade segment;
  `max_output_tokens` may remain a valid caller bound even when a model has no
  intrinsic text output limit; the exact positive domain may differ by model.
  A reasoning-only, output-only, or small-model outcome is acceptable. Stop
  after card 107 if no useful subset survives.
- **Report after:** Research 187 dispositions; then typed
  plan/evidence/driver/protocol binding; then deterministic acceptance,
  shared-delta report, and PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, run the
startup preflight below, and accept a clean launcher-provided non-`main`
worktree as authoritative. Once safe, read `AGENTS.md`, g04.039, cards 107-109,
Contracts 037/040, the named research and guide, and current xAI prepared,
driver, protocol, fixture, and test surfaces. Start with card 107. Continue only
when Research 187 admits a useful exact subset. Finish the route-local runway in
one PR and stop.

## Completion Protocol

### Before you start

1. Read this handoff path. Its worker metadata activates worker mode. Before
   broad reads, run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Use a clean registered non-`main` launcher worktree immediately, regardless
   of generated path/branch differences. Record it and do not create another.
3. If current context is unusable, inspect the named worktree. Only if needed,
   read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create
   a unique worktree there from `origin/main`. Never clean/reset another tree or
   use `/tmp`. If the launcher supplied dirty or `main`, stop and report it.
4. Run `git fetch origin`; confirm `HEAD == origin/main`; confirm
   `git merge-base --is-ancestor 834476f4fa504badad9d6fce8920a315cde98f19 HEAD`;
   confirm this handoff exists in `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, canonical refs, and xAI source.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; distinguish
   inherited findings from new ones.

### While you work

- Execute card 107, then 108, then 109 with meaningful evidence, binding, and
  acceptance commits.
- Use official sources only. Freeze secret-free evidence and no provider output.
- After card 107, report exact sources/specimens/digests,
  model/value/profile/control table, Research 187, and validation. Stop if no
  subset survives.
- After card 108, report public input, capability constraints,
  plan/evidence/driver binding, exact JSON, absent path, and validation.
- Stay inside the route-local mutable-file boundary. Stop on a shared-file need,
  new facade segment, contract change, breaking API pressure, live work, or
  scope expansion.

### When the assigned runway is complete

1. Run every final gate named by card 109 plus any earlier gate not rerun.
2. Complete Research 187, cards 107-109, g04.039, xAI guide, package API
   baseline, and reserved closeout log honestly. Leave shared surfaces unchanged
   and list their exact delta in the closeout log and PR body.
3. Push the selected worker branch and open a reviewable PR against current
   pushed `main`. The planning base predates this handoff commit.
4. Link Contracts 037/040, Research 187, g04.039, cards 107-109, exact evidence,
   changed surfaces, validation, shared closeout delta, and unresolved items.
5. Report PR URL, exact head, evidence/claim boundary, and checks. Do not merge.

### Review and merge path

The orchestrator will review independently. Shared GitHub identity is
`betterthanclay`, so the verdict is a PR comment rather than formal self-approve.
Requested changes: none yet. The operator must explicitly authorise merge.

- **Closeout refs:** Research 187; cards 107-109; g04.039; reserved xAI
  closeout log; xAI guide and package API baseline

### Handoff closeout

If card 107 produces no useful exact subset, record the stop in Research 187
and the closeout log, leave production claims unchanged, and open no speculative
implementation. Otherwise leave the route-owned surfaces and PR evidence honest
without claiming merge or shared-surface completion.
