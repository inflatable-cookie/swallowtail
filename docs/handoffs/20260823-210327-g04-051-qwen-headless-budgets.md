---
title: g04.051 Qwen headless turn/tool budget worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260823-210327-g04-051-qwen-headless-budgets.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator closed the post-g04.050 assessment, received the operator's
selection of exact Qwen Code `0.21.15` caller-decreasing turn/tool budgets, and
compiled g04.051. Implementation has not started. The ready runway begins with
exact current official and package evidence; cards 143-144 are conditional on
a non-empty Research 198 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

`qwen.headless` already sends `--max-session-turns 24` and
`--max-tool-calls 16` on every structured-run and turn child. Consumers cannot
select a smaller execution envelope. Exact Qwen Code `0.21.15` implements both
limits, including pre-tool enforcement, but its zero-tool behavior, turn
accounting, counter lifetime, and process/stream terminal shape must be mapped
to Swallowtail without inference.

The target is narrow: typed caller-decreasing budgets, exact per-child
dispatch, omission compatibility, and truthful terminal behavior. It is not a
portable generation control and does not change wall time, deadlines, tools,
approval, credentials, models, reasoning qualification, or currentness.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `68f34ab06550b3029d494a128c1676474dfe0cef`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `68f34ab06550b3029d494a128c1676474dfe0cef` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Done:** g04.051, cards 142-144, Research 198 reservation, compilation log,
  route-local closeout reservation, triage selection, and the sole Next Task
  are published on `main`
- **Still open:** card 142 exact evidence; conditional cards 143-144; worker PR,
  review, merge, and orchestrator shared closeout. g04 remains active.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-051-qwen-headless-budgets-20260823-210327`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-051-qwen-headless-budgets-20260823-210327`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-051-qwen-headless-budgets-20260823-210327 -b agent/g04-051-qwen-headless-budgets-20260823-210327 origin/main`
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
  `docs/roadmaps/g04/051-qwen-headless-turn-and-tool-budgets.md`
- **Current batch card:**
  `docs/roadmaps/g04/batch-cards/142-qwen-headless-turn-and-tool-budget-evidence.md`
- **Ready cards, in order:** card 142, then conditional card 143, then
  conditional card 144
- **Allowed runway:** exact Qwen Code `0.21.15` turn/tool-budget evidence, then
  only Research 198 deliver-now adapter-local caller-decreasing binding
- **Remaining continuation envelope:** three serial cards; cards 143-144 run
  only after their named evidence and implementation gates
- **Lane budget / pause signal:** one PR. Stop after card 142 if Research 198
  has no deliver-now row or if shared contract/currentness work is needed.
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  008, 011, 029, 033, 037, and 052
- **Route identity:** `qwen.headless`, driver
  `swallowtail.qwen.headless`, axis `qwen-code.package`, exact feature point
  `0.21.15`
- **Candidate mapping:** caller-selected turn limits `1..=24` and tool-call
  limits `0..=16`; these are evidence candidates, not prequalified public
  domains
- **Current mapping:** every applicable command sends
  `--max-wall-time 60s --max-tool-calls 16 --max-session-turns 24`
- **Child boundary:** structured run, first turn, exact `--resume` child, and
  fresh replacement; counters may be child-local and must not be described as
  operation-wide without proof
- **Reasoning composition:** exact package `0.21.15`, provider
  `alibaba-modelstudio`, models `qwen3.8-max` and
  `qwen3.8-max-preview`, canonical `low|medium|high|xhigh|max`, using the
  existing initialize/set-effort-before-user control exchange
- **Model capability profile:** exact-package, evidence-first, fail closed on
  budget, version, profile, plan/evidence, driver, command, child, counter,
  terminal, reasoning, or lifecycle ambiguity
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no package install, login, credential/account
  inspection, catalogue call, provider request, live Qwen prompt, browser
  login, or paid work. Current official public-source inspection and secret-
  free exact-package/repository fixtures are allowed by card 142.
- **Required validation:** card-specific gates plus final
  `cargo fmt -p swallowtail-adapter-qwen`, `effigy validate:focused
  swallowtail-adapter-qwen`, `effigy package:verify-affected
  swallowtail-adapter-qwen`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, and `git diff --check`
- **Known doctor baseline:** 376 inherited god-file findings: 330 warnings and
  46 errors; stale graph index; one generated-in-src warning. Keep inherited
  findings separate from lane-created findings.
- **Planning validation:** `effigy qa:docs`, `effigy qa:northstar`, and
  `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge
- **Key files:**
  - `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/051-qwen-headless-turn-and-tool-budgets.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/research/198-qwen-headless-turn-and-tool-budget-evidence.md`
  - `/Users/tom/Dev/projects/swallowtail/docs/guides/qwen-headless-prepared-integration.md`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/command.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/prepared_profile/input.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/prepared_profile/plan.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/prepared_profile/run.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/prepared_profile/session.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/session/turn.rs`
  - `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-qwen/src/events/terminal.rs`

## Boundaries

Keep this pass inside the named runway:

- **In scope:** `crates/swallowtail-adapter-qwen/**` for exact adapter-local
  caller-decreasing selection, prepared input, immutable plan/evidence,
  driver/command agreement, every admitted child, terminal handling, and
  deterministic tests; `docs/guides/qwen-headless-prepared-integration.md`;
  Research 198; g04.051; cards 142-144; the reserved g04.051 route-local
  closeout; applicable `swallowtail-adapter-qwen` examples and unreleased
  public-API baseline; current official Qwen headless documentation; exact
  official `v0.21.15` source; secret-free command, stream, stderr, exit,
  counter, cancellation, deadline, failure, cleanup, run, resume, replacement,
  and reasoning-control fixtures
- **Out of scope:** raised turn/tool bounds; upstream unlimited `-1`; selectable
  wall time; approval/permission changes; tool allowlist/exclusion changes;
  agent/subagent tools; JSON-schema structured output; model/provider selection;
  reasoning-value changes; ambient settings or synthetic config roots;
  portable output/reasoning/context/billing controls; generic provider settings;
  another Qwen version or route; live work; currentness; `CHANGELOG.md`; shared
  architecture; Contract 029; route/feature matrices; programme/front doors/
  indexes; release, publication, merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract change,
  stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, process, security, billing, or compatibility decision.
- Do not represent these execution limits as Contract 040 portable generation
  controls or as proof that the provider completed less work.
- Do not infer operation-wide counters from a process-local Qwen counter. Name
  child reset and replacement truth exactly.
- Do not normalize upstream parser breadth into the public API. Research 198
  owns the exact admitted subset; omission remains current `24` / `16`.
- Do not silently strengthen a nonzero exit or stderr diagnostic into a stable
  semantic stream event. Research 198 owns terminal classification.
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

- **Planning lineage:** g04.026 qualified package `0.21.15`; g04.041 added an
  exact model-qualified reasoning control on that point. g04.051 adds no new
  version or reasoning value. It assesses the fixed native turn/tool flags the
  route already emits.
- **Official evidence:** start with current [Qwen Headless Mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
  and exact [Qwen Code v0.21.15](https://github.com/QwenLM/qwen-code/tree/v0.21.15).
  Record retrieval dates and complete fetched-body/source digests. Do not send
  a provider request.
- **Exact package identity:** `@qwen-code/qwen-code@0.21.15`, npm tarball
  SHA-256 `8d405b065888b7000a6989d99c2d79257cd8f9f5b68e9078fb76484527351b9a`,
  GitHub commit `5dce2515a778f9cf2013168962b4fbc3454636e3`.
- **Current source truth:** `command.rs` fixes wall time `60s`, tool calls `16`,
  and session turns `24`, plus the read-only allowlist and exclusions. Every
  turn is a joined child; resumed turns use private exact `--resume`.
- **Candidate-domain burden:** exact package source accepts more than the
  proposed API. Swallowtail should admit only useful caller-decreasing values.
  Card 142 must explicitly classify turns `1..=24`, tool calls `0..=16`, all
  omissions, invalid forms, raised values, and upstream unlimited behavior.
- **Zero-tool burden:** exact source appears to abort before the first tool
  dispatch when the limit is zero. Prove whether this produces useful, truthful
  route behavior and how partial assistant intent is represented.
- **Turn burden:** reconcile Qwen's user/model/tool turn definition with one
  prompt per Swallowtail child. Prove reset/lifetime across first, resumed, and
  fresh replacement children; do not infer session-wide accumulation.
- **Terminal burden:** official docs name exit 53 for turn overrun and 55 for
  tool-call overrun. Exact `stream-json` paths may still expose plain stderr and
  process failure rather than one semantic event. Freeze the actual package and
  current Swallowtail classification before binding.
- **Upstream exceptions:** Qwen's JSON-schema `structured_output` tool and
  subagent inner calls have special accounting. The current route selects no
  JSON schema and excludes agent tools. Record them as not applicable; do not
  widen scope to exercise them.
- **Reasoning burden:** selected budgets must compose with the existing private
  initialize/set-effort acknowledgement and still send the user record only
  after exact reasoning acceptance. Omitted reasoning keeps text-stdin shape.
- **Preservation burden:** current constructors, omission argv, native wall
  bound, mandatory host deadline, tool set, safe mode, environment, credentials,
  model route, cancellation, deadline, failure, cleanup, resume, and replacement
  tests remain authoritative.
- **Honest stop:** an empty Research 198 deliver-now set is a successful
  evidence result. Close cards 143-144 as blocked, finish the route-local stop
  record, validate, and open the evidence PR.
- **Generation boundary:** do not close or roll over g04. After merge the
  orchestrator reconciles g04.051 and reassesses remaining inventory. Generation
  closure requires later explicit operator direction.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. Keep unrelated route families out of routine reporting.
- **Known baseline:** do not claim or repair inherited doctor findings unless
  this lane creates distinct friction. Record new recurring Northstar friction
  in `PAPERCUTS.md`.
- **Report after:** card 142's exact domain/profile/lifetime/terminal decision.
  Continue automatically only for a non-empty deliver-now set and no stop
  condition, then report after the complete cards 143-144 implementation and
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

Then read `AGENTS.md`, g04.051, cards 142-144, Research 017, 173, 189, and 198,
the Qwen prepared guide, exact command/preparation/driver/run/session/turn/
reasoning/terminal source and fixtures, and the canonical contracts from the
selected worker worktree.

Take card 142 as one coherent evidence chunk. Use current official docs, exact
official package source, and deterministic repository evidence; do not send a
live request. If Research 198 has no deliver-now set, close cards 143-144 as
blocked, finish the route-local stop record, validate, and open the evidence
PR. If an exact set survives, execute cards 143-144 in order and open one
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
   `68f34ab06550b3029d494a128c1676474dfe0cef` and is clean before editing.
   If `origin/main` moved, use the current pushed main only when it contains
   that planning base; otherwise stop and report the divergence.

### Work the cards

1. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan` once at
   startup. Keep inherited doctor findings separate.
2. Read card 142 and its named refs completely. Freeze official/exact-package
   evidence and promote Research 198. Do not edit production code during the
   evidence card.
3. Report the exact deliver-now or stop table to the operator. Continue to card
   143 only if Research 198 has a non-empty exact set and no stop condition
   fired.
4. If continuing, implement cards 143 and 144 as one meaningful code/test/docs
   batch. Preserve every fixed runtime and reasoning boundary.
5. Update only route-local worker surfaces. In the reserved closeout, list the
   shared architecture, Contract 029, route/feature matrix, programme, indexes,
   changelog, milestone, and Next Task changes the orchestrator must apply
   after merge. Do not propose g04 closure unless explicitly instructed later.
6. Run the complete card-specific validation once after the coherent batch.
   Record exact pass/fail counts and any inherited baseline.

### PR loop

1. Review `git diff`, `git diff --check`, branch name, and worktree state.
   Commit the worker batch with a concise message. Push the worker branch.
2. Open one PR against current `main`. The PR body must name g04.051, cards
   completed or blocked, Research 198 disposition, exact route/version,
   selected and omitted values, counter/terminal truth, validation, shared-
   closeout delta, and every explicit withhold.
3. Do not merge. Report the PR URL and exact head SHA to the operator for the
   orchestrator's review loop.
4. If review requests changes, keep the same branch, worktree, PR, and lane.
   Fix only in-scope issues, rerun proportionate validation, push, and report
   the new exact head.
5. Do not restack or merge unless the operator explicitly asks in a later
   message. The orchestrator owns exact-head review, CI state, fast-forward
   merge, shared closeout, and roadmap reconciliation.

### Before you finish

1. Confirm card 142 and Research 198 record the exact evidence disposition.
2. Confirm cards 143-144 and g04.051 reflect completed work or an honest stop.
3. Confirm the reserved route-local closeout records actual validation, PR,
   head, unresolved risks, and the shared closeout delta without claiming
   merge.
4. State whether the continuation envelope ended after evidence or after all
   three cards. There is no later worker card in this handoff.
5. Leave the operator one clear next task: review the exact PR head. After
   merge, the orchestrator—not this worker—reconciles g04.051 and selects the
   next planning move. g04 stays open until explicit operator direction.
