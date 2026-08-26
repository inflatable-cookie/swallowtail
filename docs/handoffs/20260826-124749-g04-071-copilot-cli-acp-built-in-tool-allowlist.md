---
title: g04.071 Copilot CLI ACP built-in tool allowlist worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
planning_base: 08d24df25dc242b50be75d0c7ebd97bf63fbb182
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-124749-g04-071-copilot-cli-acp-built-in-tool-allowlist.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, per-route-feature]
---

## What This Thread Was Doing

The orchestrator resumed the sole Northstar pointer after g04.067 stopped and
reassessed the remaining per-route feature inventory. Copilot CLI ACP built-in
tool availability is the strongest bounded next lead.

Current official GitHub documentation exposes server-start
`--available-tools` and `--excluded-tools` filters inherited by every ACP
session. The maintained Swallowtail route owns one exact `1.0.80` child, one
session, immutable preparation, exact argv, permission observe-and-stop
behavior, and fresh context-losing replacement. Existing fixtures and command
tests deliberately record both flags as unmapped.

This is not a prequalified feature. Exact `1.0.80` evidence has not yet proved
parser behavior, fixed built-in identifiers, registry composition, ambient
extension/MCP effects, unknown-name failure, or a useful closed subset.
g04.071 and cards 195-197 therefore form one serial evidence-first lane. Card
195 must promote Research 218 with an exact non-empty table or honest empty
set. Cards 196-197 run only for the admitted rows.

This is the complete handoff from the planning/orchestrator thread to one
bounded implementation thread. Start from this file without a copied
transcript or second prompt. Do not create internal subagents or parallel
worker lanes; the operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md`. Read the `effigy` skill before validation.

## Why It Matters

The per-route programme exists to turn known feature gaps into exact route
truth. A CLI flag does not prove stable identifiers or filtering. A filtered
tool registry does not grant permission. Permission does not prove invocation
or effect. A read-labelled tool does not contain filesystem, process, or
network access.

The safe candidate is a closed adapter-local allowlist, fixed at preparation
for the owned child. Raw names, denylists, shared tool vocabulary, permission
bypass, MCP, extensions, and isolation claims stay out. An empty evidence set
is useful if exact package truth cannot sustain that boundary.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:**
  `08d24df25dc242b50be75d0c7ebd97bf63fbb182`
- **Pushed main verification:** local planning `HEAD` and remote `main` both
  resolved to the planning base before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.071, cards 195-197,
  Research 218 reservation, compilation log, programme, triage,
  generation/g04/batch-card indexes, and sole Next Task
- **Worker branch:**
  `agent/g04-071-copilot-cli-tool-allowlist-20260826-124749`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-071-copilot-cli-tool-allowlist-20260826-124749`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-071-copilot-cli-tool-allowlist-20260826-124749
  -b agent/g04-071-copilot-cli-tool-allowlist-20260826-124749 origin/main`
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
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/071-copilot-cli-acp-built-in-tool-allowlist.md`
- **Ready cards, in order:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/195-copilot-cli-acp-built-in-tool-allowlist-evidence.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/196-copilot-cli-acp-built-in-tool-allowlist-binding.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/197-copilot-cli-acp-built-in-tool-allowlist-acceptance.md`
- **Allowed runway:** one route/control family: exact `1.0.80`
  `copilot-cli.acp` built-in tool availability, narrowed to only closed
  allowlist rows admitted by Research 218
- **Remaining card budget:** three serial cards; stop after 195 unless Research
  218 admits a non-empty exact table and no decision gate fires
- **Dispatch topology:** one serial worker lane; all cards share the same
  adapter, exact package corpus, prepared session, plan/evidence, child command,
  permission path, fixtures, guide, matrices, and closeout surfaces
- **Parallel safety check:** no parallel lane is authorized because the cards
  mutate the same adapter and truth surfaces
- **Route identity:** route `copilot-cli.acp`, driver
  `swallowtail.copilot-cli.acp`, axis `copilot-cli.package`, exact qualified
  package `1.0.80`, behavior `copilot-cli.acp.stdio-v1`, ACP v1 stdio
- **Existing operation shape:** host-account access; one owned
  `copilot --acp --stdio` child; initialize; one `session/new` with cwd and no
  MCP servers; one bounded text prompt; permission requests observed and
  cancelled; joined connection/process/task cleanup
- **Existing isolation posture:** `AmbientHost`; tool filtering cannot change
  or satisfy it
- **Candidate public shape:** only a closed adapter-local profile or typed
  frozen identifier set selected by Research 218; no raw strings and no shared
  provider-tool vocabulary
- **Canonical refs:**
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/011-runtime-conformance-profiles.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/023-harness-operation-isolation-and-native-boundary.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/029-interface-version-qualification-and-compatibility.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/033-harness-configuration-posture.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/052-consumer-and-operator-integration-documentation.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/149-copilot-cli-acp-1-0-80-identity.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/188-copilot-cli-acp-effort-evidence.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/218-copilot-cli-acp-built-in-tool-allowlist-evidence.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/guides/copilot-cli-acp-prepared-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-21-advanced-route-features.md`
- **Route-local source leads:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/command.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/prepared.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/prepared/session.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/driver.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/connection_dispatch.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/src/selection.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/tests/copilot_cli_acp_identity.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/tests/prepared_facade.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/tests/acp_suite.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-1.0.80/protocol.json`
- **Exact official evidence leads:** current official GitHub Copilot CLI ACP
  server, CLI command, and allowing-tools documentation; exact
  `@github/copilot@1.0.80` wrapper and platform package source. Current docs
  corroborate the exact package; they do not replace it.
- **Preliminary lead only:** current docs say `--available-tools` restricts the
  server tool set and takes precedence over `--excluded-tools`. Re-derive
  parser syntax, fixed identifiers, registry assembly, filtering, failures,
  and lifetime from exact `1.0.80`; do not copy the lead as accepted evidence.
- **Model capability profile:** frontier implementation plus minified
  JavaScript source, CLI parser, registry/filter, permission, process lifetime,
  and fail-closed authority audit; deterministic source/fixture work only
- **Tool/runtime restrictions:** official public docs and exact npm artifacts
  may be downloaded and extracted in a disposable temporary directory. Do not
  install or update Copilot CLI, execute its native binary, inspect login or
  account state, capture credentials, initialize ACP, prompt a provider,
  execute a tool, contact an external service, or use paid inference.
- **Required validation:** card 195 runs `effigy validate:focused
  swallowtail-adapter-copilot-cli`, `effigy qa:northstar`, relevant current
  index selectors, and `git diff --check`; conditional cards 196-197 add
  `cargo fmt -p swallowtail-adapter-copilot-cli`, `effigy
  package:verify-affected swallowtail-adapter-copilot-cli`, `effigy
  check:examples`, `effigy package:api`, all named current docs/index gates,
  `effigy doctor`, and diff checks
- **Known doctor baseline:** 378 god-file findings: 332 warnings and 46 errors;
  one generated-in-src warning. Do not increase it.
- **Planning validation:** `effigy tasks`, `effigy qa:northstar`,
  `effigy qa:docs:index:logs`, `effigy qa:docs:index:roadmaps`, `effigy
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
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-copilot-cli/**`
  for exact package/parser/registry/tool/filter/permission evidence and, only
  after Research 218 admits delivery, prepared input, plan/evidence, driver,
  child command, validation, tests, fixtures, example, and API baseline;
  Research 218; g04.071; cards 195-197; Copilot prepared guide; route/feature
  matrices and changelog only where public truth changes; programme, triage,
  closeout, indexes, and sole Next Task
- **Out of scope:** `--excluded-tools`; raw tool names; generic tool or
  permission settings; consumer tools; MCP; extensions; plugins; skills;
  custom agents; slash commands; yolo/allow-all or approval exchange; model or
  effort selection; TCP; login/BYOK/account work; another Copilot route;
  shared contracts/runtime; live provider/tool work; currentness, release,
  publication, merge, rollover, or g04 closure
- Card 195 makes no production claim edit. Reconfirm exact `1.0.80` identity,
  integrity, and source before classifying behavior. Do not qualify moving
  docs/main or broaden the Contract 029 window in this feature lane.
- Freeze exact option arity, delimiter/quoting, repeats, normalization, case,
  duplicates, empty/unknown values, parser failures, available/excluded
  precedence, and the earliest fail-closed point.
- Separate fixed built-ins from model, account, platform, extension, plugin,
  skill, MCP, custom-agent, user-configured, and service-provided registry
  entries. Do not infer a stable identifier from documentation prose.
- Prove a useful closed subset, not only argv transport. If membership or
  enforcement depends on ambient state the existing preparation cannot bind,
  Research 218 should be empty.
- Distinguish requested restriction, startup dispatch, parser acceptance,
  registry filtering, permission request/response, invocation, activity,
  effect, and terminal outcome. Claim only the exact boundary proved.
- Preserve permission observe-and-stop behavior. The allowlist grants no
  permission and must not enable one-shot or persistent approval.
- Preserve `AmbientHost`. A provider tool filter proves no filesystem,
  descendant-process, network, sandbox, read-only, or host containment.
- Preserve exact omitted argv, initialize/session/prompt behavior, access,
  activity, cancellation, deadline, failure, fresh replacement, terminal
  outcome, process ownership, and joined cleanup truth.
- Default QA must not execute the native Copilot binary, resolve login/account
  state, initialize ACP, run a provider prompt or tool, contact an external
  service, or use paid inference.
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

- **Planning lineage:** g04.035-067 delivered or stopped exact route-local
  controls one family at a time. g04.067 is stopped. The programme and triage
  note select g04.071 next. Contract 029 currentness stays a separate standing
  lane and g04 stays open at operator direction.
- **Why these cards are ready:** the route, exact qualified package, owned
  process lifetime, candidate allowlist-only surface, evidence method, stop
  gates, conditional public shape, acceptance, validation, and continuation
  state are named. No tool/profile row is assumed before card 195.
- **Decisions and preferences:** one route and one coherent feature family;
  closed adapter-local selection only; omitted argv stable; unsupported truth
  rejects before spawn; no permission or isolation widening; no live provider
  work; no internal subagents; no g04 closure.
- **Open tensions:** built-in identifiers may be assembled dynamically;
  extensions or MCP may affect the registry before filtering; unknown names may
  be ignored; names may not encode effects; source may not distinguish filter
  dispatch from effective tool visibility. Any of these may yield an empty set.
- **Report after:** card 195 evidence promotion or stop, then the combined cards
  196-197 implementation/acceptance chunk if authorized by Research 218, then
  final pushed PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the startup worktree-safety preflight
below. Then execute card 195 and promote Research 218. Continue to cards
196-197 only for its exact non-empty deliver-now rows.

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
4. Confirm `HEAD` equals `origin/main`, confirm the planning base is an
   ancestor, and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the named skills, milestone, cards, research, contracts,
   guide, and source leads from the selected worker worktree.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline and actual selector plan before work.

### While you work

- Execute cards serially. Card 195 is one meaningful evidence chunk. Continue
  only when its promoted Research 218 table is non-empty and exact.
- Keep commits aligned with evidence, conditional binding, and acceptance
  chunks, not arbitrary model turns.
- Report after each meaningful chunk with changed files, validation actually
  run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, exact package evidence is unavailable, or
  validation changes the plan. Do not turn an open question into architecture.

### When the assigned runway is complete

1. Run the validation named by the executed cards. Use the current Effigy task
   names in the cards; do not substitute stale `docs:index-check` commands.
2. Update Research 218, cards, milestone, guide, matrices if truth changes,
   programme, triage, closeout log, indexes, and sole Next Task honestly. If
   Research 218 is empty, mark cards 196-197 blocked and retain current argv.
3. Push the selected worker branch.
4. Open one reviewable PR against current pushed `main`. The planning base is
   the pre-handoff commit, not the self-referential handoff commit.
5. Link the milestone, cards, Research 218, changed surfaces, evidence,
   validation, exact stop/delivery state, and unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, exact package
evidence, diff, and checks. Formal self-approval may be unavailable; an
evidence-backed PR comment is the canonical review record. Apply only requested
changes on the worker branch. The operator must explicitly authorize merge.

- **Closeout refs:** Research 218; cards 195-197; g04.071; Copilot prepared
  guide; route/feature matrices when truth changes; programme; triage; logs;
  research/log/roadmap/g04/batch-card indexes; sole Next Task

### Handoff closeout

Before calling the runway complete, leave the research, cards, roadmap, log,
and next-task state honest. If blocked, record the blocker and open the
evidence-only PR rather than making the handoff look complete.
