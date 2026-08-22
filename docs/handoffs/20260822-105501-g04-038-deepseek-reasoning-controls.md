---
title: g04.038 DeepSeek reasoning controls worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-105501-g04-038-deepseek-reasoning-controls.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, deepseek, per-route-features]
---

## What This Thread Was Doing

The orchestrator assessed the per-route feature programme for safe concurrent
execution. It compiled g04.038 and cards 104-106, reserved Research 186 and the
route closeout log, and isolated all DeepSeek-owned mutable surfaces from the
Ollama and Anthropic lanes.

This is one bounded implementation run: freeze current V4 reasoning evidence,
bind only the continuation-safe subset, then prove every admitted request shape.
It stands alone; no copied transcript or second prompt is needed.

## Why It Matters

DeepSeek currently hard-codes `reasoning_effort=high` and
`thinking.type=enabled`. Official V4 material exposes more reasoning choices,
but consumers cannot select them through Swallowtail.

Effort and thinking mode are independent. Disabling thinking may invalidate the
private `reasoning_content` replay that makes the continuation route correct.
This run must preserve that private lifecycle and reject provider aliases rather
than silently normalizing them to `high`.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Repository posture:** strict-ready Northstar
- **Planning branch:** `main`
- **Planning base commit:** `ad03d7d1371342bc1610479a843b4821a7824e24`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  `ad03d7d1371342bc1610479a843b4821a7824e24` before this handoff was created. Fetch
  again at startup; the later main tip contains this handoff.
- **Planning checkout:** clean on `main` after the pushed planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.038; cards 104-106;
  pre-indexed Research 186 and closeout log reservations; programme parallel
  boundary; compile log and sole Next Task
- **Worker branch:** `g04-038-deepseek-reasoning-controls`
- **Worker worktree:** launcher-provided dedicated worktree first. Named manual
  worktree: `/Users/tom/Dev/worktrees/swallowtail-g04-038-deepseek-reasoning-controls`
- **Worktree creation command:** only if startup requires the manual fallback:
  `git worktree add -b g04-038-deepseek-reasoning-controls /Users/tom/Dev/worktrees/swallowtail-g04-038-deepseek-reasoning-controls origin/main`
- **Worktree policy:** use a clean, dedicated, non-`main` registered worktree
  supplied by the launcher even if its generated path or branch differs. Record
  the actual values and do not create another worktree. If current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the
  operator if absent. Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. The promoted programme and Contracts 030, 037,
  and 040 own the delivery boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g04/038-deepseek-continuation-reasoning-controls.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/104-deepseek-reasoning-evidence.md`,
  `docs/roadmaps/g04/batch-cards/105-deepseek-reasoning-binding.md`, then
  `docs/roadmaps/g04/batch-cards/106-deepseek-reasoning-acceptance.md`
- **Allowed runway:** exact official evidence and Research 186; typed reasoning
  binding for only deliver-now V4 Pro profiles; deterministic structured and
  continuation request proof; route-local closeout; one PR
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial inside this lane; card 104 defines the exact
  subset consumed by 105 and 106. This route runs concurrently with Ollama and
  Anthropic.
- **Parallel safety check:** mutable scope is limited to the DeepSeek adapter,
  fixtures, prepared guide, milestone/cards, reserved Research 186, reserved
  DeepSeek closeout log, and package-specific API baseline. Shared surfaces are
  explicitly forbidden and owned by orchestrator closeout.
- **Canonical refs:** `AGENTS.md`;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/triage/2026-08-21-advanced-route-features.md` (promoted);
  `docs/architecture/system-architecture.md`; Contracts 011, 020, 024, 029,
  030, 037, 040, 041, and 052; Research 023, 067, and 169;
  `docs/guides/deepseek-prepared-integration.md`;
  `docs/guides/provider-route-matrix.md`;
  `docs/guides/provider-solution-feature-matrix.csv`
- **Exact existing facade/model:** `deepseek-openai-chat-2026-07-22` and
  `deepseek-v4-pro`; `reasoning_effort=high` and `thinking.type=enabled` are
  hard-coded on structured and continuation requests
- **Existing continuation boundary:** at most two user turns and three provider
  attempts; private `reasoning_content` is bounded, replayed, never disclosed,
  and invalidated on failure/close; unmanaged provider cache acceptance is
  explicit
- **Model capability profile:** capable coding model with medium or higher
  reasoning; frontier review for public API, private continuation, facade
  revision, or Contract 030/040 ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; use only official,
  prompt-free sources; temporary downloads only under `/tmp`; do not
  authenticate, inspect account/balance state, send a prompt, or mutate provider
  state
- **Known repository health:** the last recorded `effigy doctor` had 42
  inherited god-file errors, stale graph, and one generated-in-src warning.
  Existing findings are in `PAPERCUTS.md`; record new friction only when distinct.
- **Planning validation:** `effigy qa:northstar`; research, logs, roadmaps, g04,
  batch-card index gates; roadmaps next-action gate; `git diff --check`
- **Required final validation:** `cargo fmt -p
  swallowtail-adapter-deepseek`; focused and affected-package gates for
  `swallowtail-adapter-deepseek`; `effigy check:examples`; `effigy qa:routes`;
  `effigy qa:northstar`; research, logs, roadmaps, g04, and batch-card index
  gates; roadmaps next-action gate; `effigy package:api`; `git diff --check`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** official secret-free V4 effort/thinking/continuation evidence;
  Research 186; exact per-control/profile dispositions; additive prepared
  reasoning binding; exact capability constraints, evidence, driver, and every
  request attempt; deterministic failures and replay; DeepSeek guide; g04.038,
  cards 104-106, reserved closeout log, package-specific API baseline, and PR.
- **Out of scope:** V4 Flash, retired aliases, Anthropic facade, `/v1`, beta
  strict tools, streamed tool-call assembly, another model or route, tool-loop
  or output-limit expansion, live provider/account work, facade/currentness
  expansion, generic maps, release, or publication.
- Do not edit shared parallel-closeout surfaces: `CHANGELOG.md`,
  `docs/architecture/system-architecture.md`, provider route/feature matrices,
  `docs/roadmaps/g04/per-route-feature-completion.md`, roadmap front doors,
  shared indexes, or `release-baselines/public-api-0.3.3/packages.txt`.
  Record their exact required delta in the reserved closeout log and PR body.
- Replace only the pre-indexed Research 186 and DeepSeek closeout reservations;
  do not edit their indexes.
- Do not accept `medium` or `xhigh` because upstream maps them to `high`.
- Map exact effort to portable `ReasoningSelection`. Add a thinking-mode input
  only if Research 186 finds an exact typed representation that does not
  falsify Contract 030 or portable capabilities.
- For sessions, one preparation-time selection applies to initial, tool-result,
  final, later-turn, failure, and fresh-restoration paths. Private reasoning
  remains adapter-held and undisclosed.
- Preserve the current high/enabled path byte-for-byte when the additive path is
  absent.
- Stop before code if a new facade revision, behavior segment, or Contract 030
  amendment is required.
- Do not invent architecture, widen the roadmap, or choose an unresolved public
  API. Work only in the selected worktree. Do not merge the PR.

## Important Context

- **Planning lineage:** Research 023 selected exact V4 Pro continuation and
  froze its private replay. The advanced-feature inventory ranked DeepSeek
  reasoning after Anthropic effort; the operator approved concurrent route
  families.
- **Why these cards are ready:** prepared inputs already require one
  `ReasoningMode`; plans already advertise high reasoning; protocol encoding
  already emits the two exact fields on every request. Card 104 is the evidence
  gate for widening that existing path without weakening it.
- **Decisions and preferences:** exact values over provider aliasing; effort and
  thinking remain independent; structured and session profiles are classified
  independently; continuation privacy outranks feature breadth.
- **Open tensions:** thinking disable may be structured-only or entirely
  withheld. The useful result may be only a low/high/max effort ladder. Stop if
  official currentness needs a new facade revision; do not disguise it as a
  field addition.
- **Report after:** Research 186 dispositions and facade/contract verdict; then
  typed binding and replay result; then acceptance, shared-delta report, and PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, run the
startup preflight below, and accept a clean launcher-provided non-`main`
worktree as authoritative. Once safe, read `AGENTS.md`, g04.038, cards 104-106,
Contracts 030/037/040, Research 023, the DeepSeek guide, and current prepared,
selection, protocol, fixture, driver, and restoration surfaces. Start with card
104. Continue only when Research 186 admits a useful subset without a new
facade or contract. Finish the route-local runway in one PR and stop.

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
   `git merge-base --is-ancestor ad03d7d1371342bc1610479a843b4821a7824e24 HEAD`; confirm this handoff exists in
   `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, canonical refs, and DeepSeek source.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; distinguish
   inherited findings from new ones.

### While you work

- Execute card 104, then 105, then 106 with meaningful evidence, binding, and
  acceptance commits.
- Use official sources only. Freeze secret-free evidence and no provider output.
- After card 104, report exact sources/specimens/digests, value/profile table,
  thinking disposition, facade/contract verdict, Research 186, and validation.
- After card 105, report public input, plan/evidence/driver binding, every
  request shape, absent path, and validation.
- Stay inside the parallel mutable-file boundary. Stop on a shared-file need,
  new facade/contract segment, replay drift, breaking API pressure, live work,
  or scope expansion.

### When the assigned runway is complete

1. Run every final gate named by card 106 plus any earlier gate not rerun.
2. Complete Research 186, cards 104-106, g04.038, DeepSeek guide, package API
   baseline, and reserved closeout log honestly. Leave shared surfaces unchanged
   and list their exact delta in the closeout log and PR body.
3. Push the selected worker branch and open a reviewable PR against current
   pushed `main`. The planning base predates this handoff commit.
4. Link Contracts 030/037/040, Research 023/186, g04.038, cards 104-106, exact
   evidence, changed surfaces, validation, shared closeout delta, and unresolved
   items.
5. Report PR URL, exact head, evidence/claim boundary, and checks. Do not merge.

### Review and merge path

The orchestrator will review independently. Shared GitHub identity is
`betterthanclay`, so the verdict is a PR comment rather than formal self-approve.
Requested changes: none yet. The operator must explicitly authorise merge.
Integration order is Ollama, Anthropic, DeepSeek; the orchestrator owns any
restack and the deferred shared-surface closeout.

- **Closeout refs:** Research 186; cards 104-106; g04.038; reserved DeepSeek
  closeout log; DeepSeek guide and package API baseline

### Handoff closeout

If card 104 produces no useful exact subset or requires a new facade/contract,
record the stop in Research 186 and the closeout log, leave production claims
unchanged, and open no speculative implementation. Otherwise leave route-owned
surfaces and PR evidence honest without claiming merge or shared-surface
completion.
