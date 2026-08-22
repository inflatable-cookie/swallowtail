---
title: g04.037 Anthropic Messages effort worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-105500-g04-037-anthropic-messages-effort.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, anthropic, per-route-features]
---

## What This Thread Was Doing

The orchestrator assessed the per-route feature programme for safe concurrent
execution. It compiled g04.037 and cards 101-103, reserved Research 185 and the
route closeout log, and isolated all Anthropic-owned mutable surfaces from the
Ollama and DeepSeek lanes.

This is one bounded implementation run: freeze exact official effort evidence,
bind only the admitted subset, then prove route-local dispatch. It stands alone;
no copied transcript or second prompt is needed.

## Why It Matters

Anthropic Messages supports `output_config.effort`, but Swallowtail currently
exposes no reasoning selection on `anthropic.messages`. Consumers cannot request
an exact supported effort through the prepared facade.

The control is model- and profile-dependent. It is not Messages `thinking`,
Claude Code effort, Ultracode, Fast mode, or Managed Agents configuration. This
run must preserve those boundaries and claim no effective effort without an
exact confirmation surface.

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
- **Planning artifacts included at the base:** g04.037; cards 101-103;
  pre-indexed Research 185 and closeout log reservations; programme parallel
  boundary; compile log and sole Next Task
- **Worker branch:** `g04-037-anthropic-messages-effort`
- **Worker worktree:** launcher-provided dedicated worktree first. Named manual
  worktree: `/Users/tom/Dev/worktrees/swallowtail-g04-037-anthropic-messages-effort`
- **Worktree creation command:** only if startup requires the manual fallback:
  `git worktree add -b g04-037-anthropic-messages-effort /Users/tom/Dev/worktrees/swallowtail-g04-037-anthropic-messages-effort origin/main`
- **Worktree policy:** use a clean, dedicated, non-`main` registered worktree
  supplied by the launcher even if its generated path or branch differs. Record
  the actual values and do not create another worktree. If current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the
  operator if absent. Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. The promoted programme and Contracts 037 and 040
  own the delivery boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g04/037-anthropic-messages-effort.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/101-anthropic-messages-effort-evidence.md`,
  `docs/roadmaps/g04/batch-cards/102-anthropic-messages-effort-binding.md`, then
  `docs/roadmaps/g04/batch-cards/103-anthropic-messages-effort-acceptance.md`
- **Allowed runway:** exact official evidence and Research 185; typed portable
  reasoning binding for only deliver-now Messages model/value/profiles;
  deterministic dispatch; route-local closeout; one PR
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial inside this lane; card 101 defines the exact
  subset consumed by 102 and 103. This route runs concurrently with Ollama and
  DeepSeek.
- **Parallel safety check:** mutable scope is limited to the Anthropic adapter,
  fixtures, direct guide, milestone/cards, reserved Research 185, reserved
  Anthropic closeout log, and package-specific API baseline. Shared surfaces are
  explicitly forbidden and owned by orchestrator closeout.
- **Canonical refs:** `AGENTS.md`;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/triage/2026-08-21-advanced-route-features.md` (promoted);
  `docs/architecture/system-architecture.md`; Contracts 011, 020, 029, 037,
  040, and 052; Research 004, 067, and 169;
  `docs/guides/anthropic-direct-prepared-integration.md`;
  `docs/guides/provider-route-matrix.md`;
  `docs/guides/provider-solution-feature-matrix.csv`
- **Exact existing facade:** `anthropic-2023-06-01` on
  `anthropic.messages-facade`; current inference input binds model, content,
  maximum output tokens, optional PNG, and optional web search, but no reasoning
  selection
- **Current official leads:** Messages effort documentation and request field
  `output_config.effort`. Leads are not qualified findings; card 101 must freeze
  exact current model/value/profile evidence.
- **Model capability profile:** capable coding model with medium or higher
  reasoning; frontier review for public API, model-qualification, or Contract
  040 ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; use only official,
  prompt-free sources; temporary downloads only under `/tmp`; do not
  authenticate, inspect account state, send a prompt, or mutate provider state
- **Known repository health:** the last recorded `effigy doctor` had 42
  inherited god-file errors, stale graph, and one generated-in-src warning.
  Existing findings are in `PAPERCUTS.md`; record new friction only when it is
  distinct.
- **Planning validation:** `effigy qa:northstar`; research, logs, roadmaps, g04,
  batch-card index gates; roadmaps next-action gate; `git diff --check`
- **Required final validation:** `cargo fmt -p
  swallowtail-adapter-anthropic`; focused and affected-package gates for
  `swallowtail-adapter-anthropic`; `effigy check:examples`; `effigy qa:routes`;
  `effigy qa:northstar`; research, logs, roadmaps, g04, and batch-card index
  gates; roadmaps next-action gate; `effigy package:api`; `git diff --check`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** official secret-free effort evidence; Research 185; exact
  model/value/profile dispositions; additive prepared reasoning input and
  capability constraint; prepared evidence and driver binding; exact Messages
  request encoding; deterministic failures; Anthropic direct guide; g04.037,
  cards 101-103, reserved closeout log, package-specific API baseline, and PR.
- **Out of scope:** Messages `thinking`; Claude Code effort, Ultracode, Fast
  mode, Managed Agents; newer web-search tool; another Anthropic route or
  model; live provider/account work; facade/currentness expansion; generic maps;
  release or publication.
- Do not edit shared parallel-closeout surfaces: `CHANGELOG.md`,
  `docs/architecture/system-architecture.md`, provider route/feature matrices,
  `docs/roadmaps/g04/per-route-feature-completion.md`, roadmap front doors,
  shared indexes, or `release-baselines/public-api-0.3.3/packages.txt`.
  Record their exact required delta in the reserved closeout log and PR body.
- Replace only the pre-indexed Research 185 and Anthropic closeout reservations;
  do not edit their indexes.
- Map effort to portable `ReasoningSelection` only for exact Research 185 rows.
  Request, plan constraint, evidence, configured driver, and wire must agree.
- Preserve current constructors and byte-identical absent-effort request bodies.
- If sessions are admitted, one preparation-time value applies to every request
  attempt and fresh restoration. No per-turn raw override.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved public API or compatibility decision. Pause on a contract gap.
- Work only in the selected clean worker worktree. Do not merge the PR.

## Important Context

- **Planning lineage:** the advanced-feature inventory ranked Anthropic Messages
  effort after Ollama `num_ctx`; the programme promotes one route/control family
  at a time. The operator approved parallel route-family execution.
- **Why these cards are ready:** the prepared route already binds one exact
  model route and immutable output controls; Contract 040 defines portable
  reasoning application states; current official material names the candidate
  field. Exact model/value/profile evidence is deliberately card 101's gate.
- **Decisions and preferences:** exact allowlist over provider-family inference;
  portable reasoning only where semantics match; no thinking synthesis; preserve
  absent behavior; dispatch truth before effectiveness claims.
- **Open tensions:** official effort support may be narrow or may interact with
  thinking. A structured-only or small-value outcome is acceptable. Stop after
  card 101 if no useful subset survives or a new facade revision is needed.
- **Report after:** Research 185 dispositions; then typed plan/evidence/driver
  binding; then deterministic acceptance, shared-delta report, and PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, run the
startup preflight below, and accept a clean launcher-provided non-`main`
worktree as authoritative. Once safe, read `AGENTS.md`, g04.037, cards 101-103,
Contracts 037/040, the named research and guide, and current Anthropic prepared,
driver, protocol, fixture, and test surfaces. Start with card 101. Continue only
when Research 185 admits a useful exact subset. Finish the route-local runway in
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
   `git merge-base --is-ancestor ad03d7d1371342bc1610479a843b4821a7824e24 HEAD`; confirm this handoff exists in
   `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, canonical refs, and Anthropic source.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; distinguish
   inherited findings from new ones.

### While you work

- Execute card 101, then 102, then 103 with meaningful evidence, binding, and
  acceptance commits.
- Use official sources only. Freeze secret-free evidence and no provider output.
- After card 101, report exact sources/specimens/digests, model/value/profile
  table, Research 185, and validation. Stop if no subset survives.
- After card 102, report public input, plan/evidence/driver binding, exact JSON,
  absent path, and validation.
- Stay inside the parallel mutable-file boundary. Stop on a shared-file need,
  thinking dependency, new facade segment, breaking API pressure, live work, or
  scope expansion.

### When the assigned runway is complete

1. Run every final gate named by card 103 plus any earlier gate not rerun.
2. Complete Research 185, cards 101-103, g04.037, Anthropic guide, package API
   baseline, and reserved closeout log honestly. Leave shared surfaces unchanged
   and list their exact delta in the closeout log and PR body.
3. Push the selected worker branch and open a reviewable PR against current
   pushed `main`. The planning base predates this handoff commit.
4. Link Contracts 037/040, Research 185, g04.037, cards 101-103, exact evidence,
   changed surfaces, validation, shared closeout delta, and unresolved items.
5. Report PR URL, exact head, evidence/claim boundary, and checks. Do not merge.

### Review and merge path

The orchestrator will review independently. Shared GitHub identity is
`betterthanclay`, so the verdict is a PR comment rather than formal self-approve.
Requested changes: none yet. The operator must explicitly authorise merge.
Integration order is Ollama, Anthropic, DeepSeek; the orchestrator owns any
restack and the deferred shared-surface closeout.

- **Closeout refs:** Research 185; cards 101-103; g04.037; reserved Anthropic
  closeout log; Anthropic guide and package API baseline

### Handoff closeout

If card 101 produces no useful exact subset, record the stop in Research 185 and
the closeout log, leave production claims unchanged, and open no speculative
implementation. Otherwise leave the route-owned surfaces and PR evidence honest
without claiming merge or shared-surface completion.
