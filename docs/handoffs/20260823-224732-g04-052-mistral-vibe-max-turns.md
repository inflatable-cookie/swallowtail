---
title: g04.052 Mistral Vibe headless maximum-turn worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-224732-g04-052-mistral-vibe-max-turns.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining promoted per-route feature inventory
after g04.051 and selected exact Mistral Vibe `2.24.2` caller-decreasing
maximum turns. g04.052 is compiled. Implementation has not started. The ready
runway begins with exact current official and tagged-source evidence; cards
146-147 are conditional on a non-empty Research 199 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

`mistral-vibe.headless` already sends `--max-turns 8` on its one structured-run
child. Consumers cannot select a smaller conversation envelope. Exact Vibe
`2.24.2` implements a native turn limiter, but upstream parser breadth is not a
safe public API: zero stops before the first assistant turn, negative values
appear parser-valid, and flag omission is unbounded.

The target is narrow: exact positive caller-decreasing selection, omission
compatibility, and truthful limit-terminal behavior. It is not a portable
output-token or generic budget control and does not change output, agent,
trust, workdir, access, deadlines, cancellation, failure, or cleanup.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `ae0c300b0c4be4d6cc90a553fc15ca68bd783ed3`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `ae0c300b0c4be4d6cc90a553fc15ca68bd783ed3` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Done:** g04.052, cards 145-147, Research 199 reservation, compilation log,
  route-local closeout reservation, triage selection, and the sole Next Task
  are published on `main`
- **Still open:** card 145 exact evidence; conditional cards 146-147; worker PR,
  review, merge, and orchestrator shared closeout. g04 remains active.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-052-mistral-vibe-max-turns-20260823-224732`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-052-mistral-vibe-max-turns-20260823-224732`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-052-mistral-vibe-max-turns-20260823-224732 -b agent/g04-052-mistral-vibe-max-turns-20260823-224732 origin/main`
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
- **Roadmap milestone:**
  `docs/roadmaps/g04/052-mistral-vibe-headless-max-turns.md`
- **Current batch card:**
  `docs/roadmaps/g04/batch-cards/145-mistral-vibe-headless-max-turns-evidence.md`
- **Ready cards, in order:** card 145, then conditional card 146, then
  conditional card 147
- **Allowed runway:** exact Vibe `2.24.2` maximum-turn evidence, then only
  Research 199 deliver-now adapter-local caller-decreasing binding
- **Remaining continuation envelope:** three serial cards; cards 146-147 run
  only after their named evidence and implementation gates
- **Lane budget / pause signal:** one PR. Stop after card 145 if Research 199
  has no deliver-now row or if shared contract/currentness work is needed.
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  008, 011, 029, 033, 037, 040, and 052
- **Route identity:** `mistral-vibe.headless`, driver
  `swallowtail.mistral-vibe.headless`, axis `mistral-vibe.release`, exact
  release `2.24.2`, behavior `mistral-vibe.headless.stdio-streaming-v1`
- **Candidate mapping:** caller-selected positive maximum turns `1..=8`; this
  is an evidence candidate, not a prequalified public domain
- **Current mapping:** every run sends `--prompt <text> --output streaming
  --max-turns 8 --trust --agent plan --workdir <working resource>`
- **Limit boundary:** exact source installs `TurnLimitMiddleware` and checks a
  step-derived count before a turn. Card 145 owns the exact assistant-turn
  definition, increment/check order, off-by-one behavior, and observable stop.
- **Model capability profile:** exact-release, evidence-first, fail closed on
  value, version, plan/evidence, driver, command, count, terminal, stream,
  partial-event, or lifecycle ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, setup, credential/account
  inspection, catalogue call, provider request, live Vibe prompt, browser
  login, archive extraction, or paid work. Current official public-source
  inspection and secret-free exact-tag/repository fixtures are allowed by card
  145.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-mistral-vibe`, `effigy validate:focused
  swallowtail-adapter-mistral-vibe`, `effigy package:verify-affected
  swallowtail-adapter-mistral-vibe`, `effigy check:examples`, `effigy
  qa:routes`, `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/
  next-action index gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 376 inherited god-file findings: 330 warnings and
  46 errors; stale graph index; one generated-in-src warning. Keep inherited
  findings separate from lane-created findings.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, and `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/052-mistral-vibe-headless-max-turns.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/research/150-mistral-vibe-headless-2-24-2-identity.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/research/199-mistral-vibe-headless-max-turns-evidence.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/guides/mistral-vibe-headless-prepared-integration.md`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/src/command.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/src/prepared.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/src/prepared/run.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/src/driver.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/src/driver/events.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.24.2/`

## Boundaries

Keep this pass inside the named runway:

- **In scope:** `crates/swallowtail-adapter-mistral-vibe/**` for exact adapter-
  local caller-decreasing selection, prepared input, immutable plan/evidence,
  driver/command agreement, limit terminal handling, and deterministic tests;
  `docs/guides/mistral-vibe-headless-prepared-integration.md`; Research 199;
  g04.052; cards 145-147; the reserved g04.052 route-local closeout; applicable
  package examples, fixtures, and unreleased public-API baseline; current
  official Vibe documentation; exact official `v2.24.2` source; secret-free
  command, stream, stderr, exit, counter, partial-event, cancellation, deadline,
  failure, and cleanup fixtures
- **Out of scope:** zero, negative, fractional, raised, or unbounded public
  values unless card 145 proves them and the roadmap is explicitly revised;
  `--max-price`; `--max-tokens`; tool budgets; agent, approval, trust, prompt,
  output, workdir, auth, model, or credential selection; Vibe ACP; TUI;
  continue/resume; teleport; setup; portable output/reasoning/context/billing
  controls; generic provider settings; another Vibe version or route; live
  work; currentness; `CHANGELOG.md`; shared architecture; Contract 029;
  route/feature matrices; programme/front doors/indexes; release, publication,
  merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract change,
  stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, process, security, billing, or compatibility decision.
- Do not represent a turn limit as Contract 040 `OutputTokenLimit`, a generic
  budget capability, or proof that the provider completed less work.
- Do not normalize upstream argparse breadth into the public API. Research 199
  owns the admitted subset. Caller omission stays fixed `8`; upstream flag
  omission stays forbidden.
- Do not silently strengthen process exit, stderr, or partial streaming output
  into successful `end_turn`. Research 199 owns terminal classification.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- **Repo constraints:** follow `/Users/tom/Dev/projects/swallowtail/AGENTS.md`
  and the canonical architecture/contracts named above. Work in one meaningful
  batch, use Effigy selectors, and keep glue-light reporting.

## Important Context

- **Planning lineage:** Research 150 and the realized route freeze exact
  `2.24.2` and fixed `--max-turns 8`. g04.052 adds no version range, route, or
  transport. It assesses one native flag the route already emits.
- **Official evidence:** start with the current
  [Mistral Vibe README](https://github.com/mistralai/mistral-vibe/blob/main/README.md)
  and exact [Mistral Vibe v2.24.2](https://github.com/mistralai/mistral-vibe/tree/v2.24.2).
  Record retrieval dates and complete fetched-body/source digests. Do not send
  a provider request.
- **Exact release identity:** GitHub lightweight tag `v2.24.2` commit
  `5e6aa0f6beb3454454f4c1de74a7652ba577ab05`; PyPI sdist SHA-256
  `be62b3148a9640ab2d72ab9849a40499d1680aa59589b01deb62c5eb08df269d`.
- **Current source truth:** `command.rs` fixes `--max-turns 8`; caller
  omission does not omit the flag. Exact route fixtures already freeze
  programmatic-limit stderr and driver classification.
- **Candidate-domain burden:** exact source accepts more than the proposed API.
  Classify caller omission, `1..=8`, zero, negatives, fractions, overflow,
  values above eight, and upstream flag omission separately.
- **Counting burden:** exact source's turn middleware checks
  `context.stats.steps - 1` before a turn. Freeze what a step represents, when
  it increments, the exact at-limit boundary, and whether retries, tools, or
  compaction affect it. Do not infer from names.
- **Zero burden:** the exact upstream test demonstrates immediate stop for
  `max_turns=0`. Decide whether that has any useful route meaning; the planned
  public candidate remains positive `1..=8`.
- **Terminal burden:** Research 150 records `ProgrammaticLimitError`, stderr,
  exit 1, and Swallowtail bounded-limit mapping. Re-freeze exact stream,
  partial-event, exit, diagnostic, terminal, cancellation, deadline, and
  joined-cleanup behavior before binding.
- **Preservation burden:** current constructors, omission argv, streaming
  decoder, plan agent, trust, workdir, local access, mandatory host deadline,
  cancellation, failure, cleanup, and one-child tests remain authoritative.
- **Honest stop:** an empty Research 199 deliver-now set is a successful
  evidence result. Close cards 146-147 as blocked, finish the route-local stop
  record, validate, and open the evidence PR.
- **Generation boundary:** do not close or roll over g04. After merge the
  orchestrator reconciles g04.052 and reassesses remaining inventory.
  Generation closure requires later explicit operator direction.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. New route work does not pre-empt per-route feature work.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 145's exact domain/counting/terminal decision.
  Continue automatically only for a non-empty deliver-now set and no stop
  condition, then report after the complete cards 146-147 implementation and
  validation chunk.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Start by reading this handoff from the top. Before broad repository reads, run
the quick worktree-safety preflight in `## Completion Protocol`. If the current
context is a clean, dedicated, non-`main` registered worktree, use it
immediately, record its actual path and branch, and do not compare its generated
identity with the fallback above. If it is unusable, use the named worktree if
it matches; only then read `.agents.local.env` and follow its required container
setting. Never fall back to `/tmp` or `TMPDIR`.

Then read `AGENTS.md`, g04.052, cards 145-147, Research 150 and 199, the
Mistral Vibe prepared guide, exact command/preparation/driver/stream/terminal
source and fixtures, and the canonical contracts from the selected worker
worktree.

Take card 145 as one coherent evidence chunk. Use current official docs, exact
official tag source, and deterministic repository evidence; do not send a live
request. If Research 199 has no deliver-now set, close cards 146-147 as
blocked, finish the route-local stop record, validate, and open the evidence
PR. If an exact set survives, execute cards 146-147 in order and open one
implementation PR. At each natural pause, tell the operator what changed, what
validation ran, what remains, and whether a planning decision is needed.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe: `git rev-parse --show-toplevel`, `git branch
   --show-current`, `git status --porcelain`, and `git worktree list
   --porcelain`.
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
   `ae0c300b0c4be4d6cc90a553fc15ca68bd783ed3` and is clean before editing. If
   `origin/main` moved, use current pushed main only when it contains that
   planning base; otherwise stop and report divergence.

### Work the cards

1. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan` once at
   startup. Keep inherited doctor findings separate.
2. Read card 145 and its named refs completely. Freeze official/exact-tag
   evidence and promote Research 199. Do not edit production code during the
   evidence card.
3. Report the exact deliver-now or stop table to the operator. Continue to card
   146 only if Research 199 has a non-empty exact set and no stop condition.
4. If continuing, implement cards 146 and 147 as one meaningful code/test/docs
   batch. Preserve every fixed route boundary.
5. Update only route-local worker surfaces. In the reserved closeout, list the
   shared architecture, Contract 029, route/feature matrix, programme, indexes,
   changelog, milestone, and Next Task changes the orchestrator must apply
   after merge. Do not propose g04 closure.
6. Run the complete card-specific validation once after the coherent batch.
   Record exact pass/fail counts and any inherited baseline.

### PR loop

1. Review `git diff`, `git diff --check`, branch name, and worktree state.
   Commit the worker batch with a concise message. Push the worker branch.
2. Open one PR against current `main`. The PR body must name g04.052, cards
   completed or blocked, Research 199 disposition, exact route/version,
   selected and omitted values, counting/terminal truth, validation, shared-
   closeout delta, and every explicit withhold.
3. Do not merge. Report the PR URL and exact head SHA to the operator for the
   orchestrator's review loop.
4. If review requests changes, keep the same branch, worktree, PR, and lane.
   Fix only in-scope issues, rerun proportionate validation, push, and report
   the new exact head.
5. Do not restack or merge unless the operator explicitly asks in a later
   message. The orchestrator owns exact-head review, CI state, fast-forward
   restacking, merge, and shared closeout.

### Completion report

Return the PR URL, exact head SHA, actual branch/worktree, Research 199
deliver-now or stop table, cards completed/blocked, exact validation, inherited
baseline, shared-closeout delta, and any unresolved decision. Keep the report
glue-light. Do not claim merge, release, currentness movement, or g04 closure.
