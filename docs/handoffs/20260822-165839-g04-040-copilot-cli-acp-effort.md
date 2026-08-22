---
title: g04.040 Copilot CLI ACP effort worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260822-165839-g04-040-copilot-cli-acp-effort.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, copilot-cli, per-route-features]
---

## What This Thread Was Doing

The orchestrator merged and closed g04.039, reassessed the remaining promoted
per-route feature inventory, and selected Copilot CLI ACP startup/session effort
as the next bounded family. It compiled g04.040 with cards 110-112 and reserved
Research 188 plus the route-local closeout log.

This is one bounded implementation run: prove the exact `1.0.80` effort surface
and Contract 040 fit, bind only surviving values to the prepared ACP session,
then prove process-fixed dispatch. It stands alone; no copied transcript or
second prompt is needed.

## Why It Matters

`copilot-cli.acp` already owns one ACP stdio child for one bounded interactive
session, but it starts only `copilot --acp --stdio` and exposes no reasoning
selection. Current official documentation names server-start effort flags and
says sessions inherit the selected value. If exact package `1.0.80` proves that
surface, Swallowtail can expose a useful route-local control with a lifetime
that matches the existing prepared operation.

The route does not select a model and current docs are not exact package
evidence. Research must lead. No implementation is allowed if the interface
clamps values, requires unknown model capability, or needs a new contract or
facade decision.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Repository posture:** strict-ready Northstar
- **Planning branch:** `main`
- **Planning base commit:** `cb6810e8df6ab4642a8a1338c2f8c424bfb7cb04`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled
  `cb6810e8df6ab4642a8a1338c2f8c424bfb7cb04` before this handoff was created.
  Fetch again at startup; the later `main` tip contains this handoff.
- **Planning checkout:** clean on `main` after the pushed planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.040; cards 110-112;
  pre-indexed Research 188 and closeout reservation; compile log; advanced-
  feature selection disposition; sole Next Task
- **Worker branch:** `g04-040-copilot-cli-acp-effort`
- **Worker worktree:** launcher-provided dedicated worktree first. Named manual
  worktree: `/Users/tom/Dev/worktrees/swallowtail-g04-040-copilot-cli-acp-effort`
- **Worktree creation command:** only if startup requires the manual fallback:
  `git worktree add -b g04-040-copilot-cli-acp-effort /Users/tom/Dev/worktrees/swallowtail-g04-040-copilot-cli-acp-effort origin/main`
- **Worktree policy:** use a clean, dedicated, non-`main` registered worktree
  supplied by the launcher even if its generated path or branch differs. Record
  the actual values and do not create another worktree. If current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the
  operator if absent. Never use `/tmp`, `TMPDIR`, or a guessed path for a
  worktree.
- **Active spec lane:** none. The promoted per-route programme and Contracts
  037 and 040 own the delivery boundary.
- **Roadmap milestone:**
  `docs/roadmaps/g04/040-copilot-cli-acp-session-effort.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/110-copilot-cli-acp-effort-evidence.md`,
  `docs/roadmaps/g04/batch-cards/111-copilot-cli-acp-effort-binding.md`, then
  `docs/roadmaps/g04/batch-cards/112-copilot-cli-acp-effort-acceptance.md`
- **Allowed runway:** official and exact-package evidence; Research 188; exact
  startup/session effort binding for only deliver-now `1.0.80` values;
  deterministic process/session inheritance; route-local closeout; one PR
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial. Card 110 defines the exact subset consumed by
  cards 111 and 112. No parallel or nested worker lane is approved.
- **Parallel safety check:** mutable scope is limited to the Copilot CLI adapter,
  fixtures, prepared guide, milestone/cards, reserved Research 188, reserved
  closeout log, and package-specific unreleased API baseline. Shared surfaces
  are reserved for orchestrator closeout.
- **Canonical refs:** `AGENTS.md`;
  `docs/roadmaps/g04/per-route-feature-completion.md`;
  `docs/triage/2026-08-21-advanced-route-features.md` (promoted);
  `docs/architecture/system-architecture.md`; Contracts 011, 020, 029, 037,
  040, 041, and 052; Research 049, 149, and 159;
  `docs/guides/copilot-cli-acp-prepared-integration.md`;
  `docs/guides/provider-route-matrix.md`;
  `docs/guides/provider-solution-feature-matrix.csv`
- **Exact existing route:** `copilot-cli.acp`, driver
  `swallowtail.copilot-cli.acp`, exact `copilot-cli.package` `1.0.80`, behavior
  `copilot-cli.acp.stdio-v1`, qualified-only, public-preview maturity. One
  prepared child owns initialize, `session/new`, prompts, cancellation, and
  joined cleanup. There is no selected model route.
- **Current official lead:** the GitHub Copilot CLI ACP-server reference names
  `--effort=LEVEL` and `--reasoning-effort=LEVEL` with
  `low|medium|high|xhigh|max`, says the setting is fixed when the server starts,
  and says every session opened against that server inherits it. This is a
  lead, not an exact `1.0.80` finding.
- **Official lead URL:**
  `https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server`
- **Current local boundary:** `command::arguments()` returns only `--acp` and
  `--stdio`; its test explicitly forbids effort, tool-filter, TCP, permission,
  and login flags. The prepared input binds request identity and read-only
  working resource only. Fresh restoration reuses the prepared plan/request
  with a new child.
- **Model capability profile:** capable coding model with medium or higher
  reasoning; frontier review for public API, no-model-route, behavior-revision,
  or Contract 040 ambiguity
- **Tool/runtime restrictions:** use Effigy selectors and official secret-free
  sources; temporary evidence downloads may use a disposable temp directory,
  never a worktree; do not install, authenticate, inspect account state, send a
  prompt, or mutate provider state; do not spawn subagents or nested workers
- **Known repository health:** `effigy doctor` reports the inherited 371
  god-file findings (326 warnings, 45 errors), a stale graph, and one
  generated-in-src warning. Do not attribute that baseline to this lane.
  Record only distinct new friction in `PAPERCUTS.md`.
- **Planning validation:** `effigy qa:docs`; `effigy qa:routes`;
  `effigy qa:northstar`; `effigy test --plan`; `git diff --check`
- **Required final validation:** `cargo fmt -p
  swallowtail-adapter-copilot-cli`; focused and affected-package gates for
  `swallowtail-adapter-copilot-cli`; `effigy check:examples`;
  `effigy qa:routes`; `effigy qa:northstar`; research, logs, roadmaps, g04, and
  batch-card index gates; roadmaps next-action gate; `effigy package:api`;
  `git diff --check`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** official ACP-server and exact `1.0.80` package/help/source
  evidence; Research 188 dispositions; optional typed prepared reasoning input;
  capability and plan constraints; request/evidence/driver binding; canonical
  child argv; first/later prompt and fresh-replacement inheritance; deterministic
  zero-process failures; Copilot CLI ACP guide; g04.040; cards 110-112; reserved
  closeout; `release-baselines/public-api-unreleased/swallowtail-adapter-copilot-cli.txt`;
  one PR.
- **Out of scope:** tool filters, `--yolo`, `--allow-all`, permissions, TCP,
  Copilot IDE/API, login, account inspection, BYOK, model selection, usage,
  output limits, structured output, attachments, callbacks, session management,
  currentness, live provider work, release, or publication.
- Do not edit shared closeout surfaces: `CHANGELOG.md`,
  `docs/architecture/system-architecture.md`, provider route/feature matrices,
  `docs/roadmaps/g04/per-route-feature-completion.md`, roadmap front doors,
  shared indexes, matrix assertions, or either `packages.txt`. Record their
  exact required delta in the reserved closeout log and PR body.
- Replace only the pre-indexed Research 188 and Copilot closeout reservations;
  do not edit their indexes.
- Treat `--reasoning-effort` as an upstream alias unless Research 188 proves a
  different required disposition. Expose no raw flag or arbitrary argv map.
- Map to portable `ReasoningSelection` only when exact `1.0.80` and Contract 040
  allow it without model inference, clamping, or default substitution.
- One preparation-time value applies to the owned child/session and fresh
  replacement. No per-turn mutation.
- Request, plan constraints, evidence, configured driver, and spawn argv must
  agree before task or process effects. Preserve exact absent-control argv.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved public API or compatibility decision. Pause on a contract gap.
- Work only in the selected clean worker worktree. Do not merge the PR.

## Important Context

- **Planning lineage:** g04.035-039 completed the initial five per-route feature
  families. The remaining inventory was reassessed rather than bulk-promoted.
  Copilot startup effort was selected as the cleanest next exact-transport
  candidate.
- **Why these cards are ready:** the route has an exact package pin, one owned
  process/session lifetime, immutable prepared operations, and fail-closed
  version selection. Official material names the candidate flags and values.
  Card 110 deliberately owns the package and contract gate.
- **Decisions and preferences:** one route/control family; exact allowlists;
  canonical typed inputs; dispatch truth before acceptance/effectiveness;
  dangerous permissions remain withheld; harness worker handoff, not internal
  subagents.
- **Open tensions:** current official docs may postdate `1.0.80`; the package
  may accept only one argv syntax; the server may normalize or clamp effort;
  the route has no model identity; adding a capability may require a behavior-
  revision decision. Any of these can stop after card 110.
- **Deferred candidates:** Qwen effort remains promoted but needs config/clamp
  qualification. Cline thinking is a boolean control, not this effort ladder.
  Do not switch to either inside this run.
- **Report after:** Research 188 dispositions; then typed
  plan/evidence/driver/argv binding; then deterministic acceptance, shared-delta
  report, and PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, run the
startup preflight below, and accept a clean launcher-provided non-`main`
worktree as authoritative. Once safe, read `AGENTS.md`, g04.040, cards 110-112,
Contracts 037/040, Research 149/159, the Copilot guide, and current adapter,
command, prepared-session, driver, fixture, and test surfaces. Start with card
110. Continue only when Research 188 admits a useful exact subset. Finish the
route-local runway in one PR and stop.

## Completion Protocol

### Before you start

1. Read this handoff path. Its worker metadata activates worker mode. Before
   broad reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Use a clean registered non-`main` launcher worktree immediately, regardless
   of generated path/branch differences. Record it and do not create another.
3. If current context is unusable, inspect the named worktree. Only if needed,
   read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create
   a unique worktree there from `origin/main`. Never clean/reset another tree or
   use `/tmp`. If the launcher supplied dirty or `main`, stop and report it.
4. Run `git fetch origin`; confirm `HEAD == origin/main`; confirm
   `git merge-base --is-ancestor cb6810e8df6ab4642a8a1338c2f8c424bfb7cb04 HEAD`;
   confirm this handoff exists in `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, canonical refs, and Copilot source.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; distinguish
   inherited findings from new ones.

### While you work

- Execute card 110, then 111, then 112 with meaningful evidence, binding, and
  acceptance commits.
- Use official sources only. Freeze secret-free evidence and no provider output.
- After card 110, report exact sources/specimens/digests, package/syntax/value/
  lifetime table, no-model-route decision, Research 188, and validation. Stop if
  no subset survives.
- After card 111, report public input, capability constraints,
  request/plan/evidence/driver agreement, exact argv, absent path, zero-process
  failures, and validation.
- Stay inside the route-local mutable-file boundary. Stop on a shared-file need,
  new contract, unresolved facade/behavior revision, breaking API pressure,
  live work, or scope expansion.

### When the assigned runway is complete

1. Run every final gate named by card 112 plus any earlier gate not rerun.
2. Complete Research 188, cards 110-112, g04.040, Copilot CLI ACP guide,
   package API baseline, and reserved closeout log honestly. Leave shared
   surfaces unchanged and list their exact delta in the closeout log and PR
   body.
3. Push the selected worker branch and open a reviewable PR against current
   pushed `main`. The planning base predates this handoff commit.
4. Link Contracts 037/040, Research 188, g04.040, cards 110-112, exact evidence,
   changed surfaces, validation, shared closeout delta, and unresolved items.
5. Report PR URL, exact head, evidence/claim boundary, and checks. Do not merge.

### Review and merge path

The orchestrator will review independently. Shared GitHub identity is
`betterthanclay`, so the verdict is a PR comment rather than formal self-approve.
Requested changes: none yet. The operator must explicitly authorise merge.

- **Closeout refs:** Research 188; cards 110-112; g04.040; reserved Copilot CLI
  ACP closeout log; Copilot guide and package-specific unreleased API baseline

### Handoff closeout

If card 110 produces no useful exact subset, record the stop in Research 188
and the closeout log, leave production claims unchanged, and open no speculative
implementation. Otherwise leave the route-owned surfaces and PR evidence honest
without claiming merge or shared-surface completion.
