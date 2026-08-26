---
title: g04.072 Grok Build ACP subagents-disabled worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
planning_base: 4d8c6db6ac29ce470bf77e0307051ffd572154f9
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-134340-g04-072-grok-build-acp-subagents-disabled.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, per-route-feature]
---

## What This Thread Was Doing

The orchestrator resumed the sole Northstar pointer after g04.071 stopped and
reassessed the remaining per-route feature inventory. Grok Build ACP launch-
time subagent suppression is the strongest bounded next lead.

Exact installed Grok Build `1.0.5` help accepts `--no-subagents` as a global
option before `agent stdio`. The maintained Swallowtail route already owns one
exact child process, immutable preparation evidence, structured-run and
interactive-session shapes, fresh replacement, cancellation, and joined
cleanup. Current production argv omits the flag.

This is not a prequalified feature. Parser acceptance does not prove that the
flag reaches every ACP subagent registry and spawn path, applies to every
session for the child lifetime, or resists ambient and session-level override.
g04.072 and cards 198-200 therefore form one serial evidence-first lane. Card
198 must promote Research 219 with an exact non-empty table or honest empty
set. Cards 199-200 run only for admitted rows.

This is the complete handoff from the planning/orchestrator thread to one
bounded implementation thread. Start from this file without a copied
transcript or second prompt. Do not create internal subagents or parallel
worker lanes; the operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md`. Read the `effigy` skill before validation.

## Why It Matters

The per-route programme exists to turn known feature gaps into exact route
truth. A launch flag can be a useful caller-decreasing topology restriction,
but only when exact package truth connects dispatch to effective behavior.

The safe candidate is disabled-only and adapter-local, fixed at preparation
for the owned child. It does not expose agent definitions, enable subagents,
add child observation or direct control, grant permission, remove ordinary
process tools, or establish sandbox, filesystem/network, read-only, or OS
descendant-process containment. An empty evidence set is useful if exact
maintained-package truth cannot sustain that boundary.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:**
  `4d8c6db6ac29ce470bf77e0307051ffd572154f9`
- **Pushed main verification:** local planning `HEAD` and remote `main` both
  resolved to the planning base before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.072, cards 198-200,
  Research 219 reservation, compilation log, programme, triage,
  generation/g04/batch-card indexes, and sole Next Task
- **Worker branch:**
  `agent/g04-072-grok-subagents-disabled-20260826-134340`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-072-grok-subagents-disabled-20260826-134340`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-072-grok-subagents-disabled-20260826-134340
  -b agent/g04-072-grok-subagents-disabled-20260826-134340 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active programme:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/per-route-feature-completion.md`
- **Roadmap milestone:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/072-grok-build-acp-subagents-disabled.md`
- **Ready cards, in order:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/198-grok-build-acp-subagents-disabled-evidence.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/199-grok-build-acp-subagents-disabled-binding.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/200-grok-build-acp-subagents-disabled-acceptance.md`
- **Allowed runway:** one route/control family: exact maintained
  `grok-build.acp` launch-time subagent suppression, narrowed to only
  disabled-profile rows admitted by Research 219
- **Remaining card budget:** three serial cards; stop after 198 unless Research
  219 admits a non-empty exact table and no decision gate fires
- **Dispatch topology:** one serial worker lane; all cards share the same Grok
  adapter, exact package corpus, prepared child/session state, plan/evidence,
  process argv, fixtures, guide, matrices, and closeout surfaces
- **Parallel safety check:** no parallel lane is authorized because the cards
  mutate the same adapter and truth surfaces
- **Route identity:** route `grok-build.acp`, driver
  `swallowtail.grok-build.acp`, axis `grok-build.executable`, maintained exact
  packages `1.0.4..=1.0.5`, model `grok-4.6`, ACP v1 stdio
- **Existing operation shapes:** one operation-private structured run or one
  durable interactive session over an owned
  `grok --no-auto-update agent stdio` child; delegated subscription access;
  exact model selection; ambient read-write working resource; activity;
  permission observe-and-stop; joined connection/process/task cleanup
- **Existing topology truth:** no child-topology observation or direct
  operator-control claim; exact `0.2.117` task-control remains private
  compatibility evidence
- **Existing isolation posture:** `AmbientHost`; disabled provider subagents
  cannot change or satisfy OS containment
- **Candidate public shape:** only an adapter-local disabled enum/profile or
  named builder selected by Research 219; no raw boolean/string, explicit
  enabled value, generic topology map, or portable capability
- **Canonical refs:**
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/009-async-operation-lifecycle.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/011-runtime-conformance-profiles.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/023-harness-operation-isolation-and-native-boundary.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/029-interface-version-qualification-and-compatibility.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/033-harness-configuration-posture.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/045-subagent-topology-observation-and-control.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/052-consumer-and-operator-integration-documentation.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/070-grok-build-0-2-114-authenticated-acp-qualification.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/130-grok-1-0-4-milestone-handshake.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/163-grok-1-0-5-identity.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/204-grok-build-acp-reasoning-selection-evidence.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/guides/grok-build-prepared-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-21-advanced-route-features.md`
- **Route-local source leads:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/prepared.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/prepared_profile.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/prepared_profile/plan.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/driver.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/driver/attachment.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/connection.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/src/activity.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/tests/acp/cases/attachment.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/tests/compatibility_corpus.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-4/compatibility.json`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5/compatibility.json`
- **Exact official evidence leads:** current official Grok Build CLI and
  configuration documentation; exact `@xai-official/grok@1.0.4` and `1.0.5`
  wrapper/platform artifacts; exact-version source when provenance matches.
  Moving public source corroborates exact artifacts but does not replace them.
- **Preliminary lead only:** exact installed `1.0.5` accepts
  `grok --no-subagents agent stdio --help`. Re-derive parser placement,
  configuration precedence, application, registry/spawn coverage, override,
  failure, and lifecycle truth from exact artifacts. Do not copy this lead as
  accepted effectiveness evidence.
- **Model capability profile:** frontier implementation plus Rust/native binary
  source, CLI parser, configuration merge, agent/subagent registry, ACP
  lifecycle, process ownership, and fail-closed authority audit; deterministic
  source/fixture work only
- **Tool/runtime restrictions:** official public docs and exact npm artifacts
  may be downloaded and extracted in a disposable temporary directory. Exact
  extracted binaries may run local help, version, parser, and unauthenticated
  no-prompt initialize cases only. Do not install/update/replace Grok, inspect
  login or account state, capture credentials, authenticate, allocate a
  provider session, prompt a provider, execute a tool or subagent, or use paid
  inference.
- **Required validation:** card 198 runs `effigy validate:focused
  swallowtail-adapter-grok`, `effigy qa:northstar`, named current index
  selectors, and `git diff --check`; conditional cards 199-200 add `cargo fmt
  -p swallowtail-adapter-grok`, `effigy package:verify-affected
  swallowtail-adapter-grok`, `effigy check:examples`, `effigy package:api`, all
  named docs/index gates, `effigy doctor`, and diff checks
- **Known doctor baseline:** 378 god-file findings: 332 warnings and 46 errors;
  one generated-in-src warning. Do not increase it.
- **Planning validation:** `effigy tasks`, `effigy qa:northstar`, `effigy
  qa:docs:index:logs`, `effigy qa:docs:index:roadmaps`, `effigy
  qa:docs:index:roadmaps:g04`, `effigy
  qa:docs:index:roadmaps:batch-cards`, `effigy
  qa:docs:next-action:roadmaps`, and `git diff --check` ran and passed. Earlier
  `effigy doctor` reproduced the inherited baseline.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-grok/**` for
  exact package/parser/configuration/registry/spawn/lifecycle evidence and,
  only after Research 219 admits delivery, prepared input, plan/evidence,
  driver, child command, validation, tests, fixtures, example, and API
  baseline; Research 219; g04.072; cards 198-200; Grok prepared guide;
  route/activity/feature matrices and changelog only where public truth
  changes; programme, triage, closeout, indexes, and sole Next Task
- **Out of scope:** enabling subagents; agent definitions or selection;
  provider-owned collaboration controls; child observation or direct control;
  web search, tool allow/deny, permission bypass, model, effort, max-turn, plan,
  schema, or sandbox changes; account/login/credential work; another Grok
  route; shared contracts/runtime; live provider/tool/subagent work;
  currentness, release, publication, merge, rollover, or g04 closure
- Card 198 makes no production claim edit. Reconfirm exact package identity,
  integrity, executable, and source applicability before classifying behavior.
  Do not broaden the Contract 029 window in this feature lane.
- Freeze exact root-option placement, repeats, conflicts, default, environment
  and config precedence, unknown placement, local failures, and earliest fail-
  closed point for every maintained version.
- Trace the flag through exact configuration merge, ACP agent construction,
  every agent/subagent registration and spawn seam, child lifetime, session
  creation, later prompts, operation-private sessions, attachment recovery,
  and replacement. Stored state, plugins, `_meta`, and ambient config need
  explicit dispositions.
- Parser acceptance, help text, and binary strings prove only their exact
  stages. Effective suppression needs exact source/constructor/registry proof
  or another secret-free deterministic observation. If unavailable, Research
  219 should be empty.
- Distinguish requested restriction, argv dispatch, parser acceptance,
  configuration application, registry/tool absence, attempted spawn, provider
  behavior, and OS process containment. Claim only the proved boundary.
- Preserve permission observe-and-stop behavior. The restriction grants no
  permission and must not add callback or approval exchange.
- Preserve `AmbientHost`. Provider subagent suppression proves no filesystem,
  descendant-process, network, sandbox, read-only, or host containment.
- Preserve exact omitted argv, initialize/session/prompt behavior, access,
  model, activity, cancellation, deadline, failure, replacement, terminal
  outcome, process ownership, and joined cleanup truth.
- Default QA must not install/replace Grok, inspect login/account state,
  authenticate, allocate a provider session, run a provider prompt/tool/
  subagent, contact paid inference, or mutate host configuration.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and
  report it through the operator.
- Work only in the selected clean worker worktree. Prefer the current launcher-
  provided worktree and record its actual path/branch; otherwise use the named
  worktree/branch or the recorded local-path fallback created by startup
  preflight. Never edit the orchestrator planning checkout or an unrelated
  dirty checkout.
- Do not create subagents or parallel workers. Do not merge the PR. Merge
  remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g04.035-071 delivered or stopped exact route-local
  controls one family at a time. The last three tool-oriented leads stopped on
  ambient availability. g04.071 is stopped. The programme and triage note
  select g04.072 next. Contract 029 currentness stays a separate standing lane
  and g04 stays open at operator direction.
- **Why these cards are ready:** the route, maintained exact packages, owned
  child lifetime, disabled-only candidate, evidence method, stop gates,
  conditional public shape, acceptance, validation, and continuation state are
  named. No effective suppression row is assumed before card 198.
- **Decisions and preferences:** one route and one coherent feature family;
  disabled-only adapter-local selection; omitted argv stable; unsupported truth
  rejects before spawn; no child-control, permission, or isolation widening;
  no live provider work; no internal subagents; no g04 closure.
- **Open tensions:** exact version source may be unavailable; root global flags
  may parse but not reach the ACP agent; ambient config or stored sessions may
  reintroduce agents; some spawn seams may be model-internal or hidden; no
  deterministic no-provider observation may prove complete suppression. Any of
  these may yield an empty set.
- **Report after:** card 198 evidence promotion or stop, then the combined cards
  199-200 implementation/acceptance chunk if authorized by Research 219, then
  final pushed PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the startup worktree-safety preflight
below. Then execute card 198 and promote Research 219. Continue to cards
199-200 only for its exact non-empty deliver-now rows.

## Completion Protocol

### Before you start

1. Run `git rev-parse --show-toplevel`, `git branch --show-current`, `git
   status --porcelain`, and `git worktree list --porcelain` before broad reads.
2. If the current root is a registered worktree, clean, and not `main`, accept
   it as launcher-provided. Record its actual path/branch. Do not compare it to
   the placeholders or create another worktree merely because they differ.
3. Only if the current context is unusable should you inspect the named
   worktree. If that also cannot be used, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if absent before
   creating a unique worktree/branch there from `origin/main`. Never use
   `/tmp`, `TMPDIR`, or a guessed path. Never clean, reset, stash over, or
   discard another checkout's dirty state. If the launcher supplied a dirty or
   `main` worktree, stop and report it instead of silently creating another.
4. Confirm `HEAD` equals `origin/main`, confirm planning base
   `4d8c6db6ac29ce470bf77e0307051ffd572154f9` is an ancestor, and confirm this
   handoff exists in `HEAD`.
5. Read `AGENTS.md`, the named skills, milestone, cards, research, contracts,
   guide, and source leads from the selected worker worktree.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline and actual selector plan before work.

### While you work

- Execute cards serially. Card 198 is one meaningful evidence chunk. Continue
  only when its promoted Research 219 table is non-empty and exact.
- Keep commits aligned with evidence, conditional binding, and acceptance
  chunks, not arbitrary model turns.
- Report after each meaningful chunk with changed files, validation actually
  run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, exact package evidence is unavailable, or
  validation changes the plan. Do not turn an open question into architecture.

### When the assigned runway is complete

1. Run the validation named by the executed cards. Use the current Effigy task
   names in the cards; do not substitute stale commands.
2. Update Research 219, cards, milestone, guide/matrices if truth changes,
   programme, triage, closeout log, indexes, and sole Next Task honestly. If
   Research 219 is empty, mark cards 199-200 blocked and retain current argv.
3. Push the selected worker branch.
4. Open one reviewable PR against current pushed `main`. The planning base is
   the pre-handoff commit, not the self-referential handoff commit.
5. Link the milestone, cards, Research 219, changed surfaces, evidence,
   validation, exact stop/delivery state, and unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, exact package
evidence, diff, and checks. Formal self-approval may be unavailable; an
evidence-backed PR comment is the canonical review record. Apply only requested
changes on the worker branch. The operator must explicitly authorise any merge.

- **Closeout refs:** Research 219; cards 198-200; g04.072; Grok prepared guide;
  route/activity/feature matrices where truth changes; programme; triage;
  closeout log; indexes; sole Next Task

### Handoff closeout

Before calling the runway complete, leave Research 219, cards, roadmap, log,
and Next Task honest. If exact evidence is empty or blocked, record the stop and
open the evidence-only PR. Keep g04 open. Do not compile another family, merge,
roll the generation, or close g04.
