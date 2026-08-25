---
title: g04.065 Claude Code headless Ultracode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
planning_base: 9d26ec9c8b42e65f27310ae2c62d60e28a0d8fb9
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260825-215324-g04-065-claude-code-headless-ultracode.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, per-route-feature]
---

## What This Thread Was Doing

The orchestrator merged and reconciled Kimi headless v2, then reassessed the
blocked Kimi reasoning-effort cards. Those cards stay blocked: qualifying the
v2 stream repaired route identity and lifecycle truth, but added no effort
confirmation, model-effort snapshot, ambient-config authority, or fail-closed
child-environment binding.

The remaining per-route inventory was then reassessed. The next bounded lead is
Claude Code headless Ultracode. Current official documentation describes
`--effort ultracode` as a product setting that sends `xhigh` and enables
dynamic workflow orchestration. Exact qualified `2.1.241` help does not list
`ultracode` among its advertised effort values, so this starts as an evidence
gate rather than an implementation assumption.

g04.065 and cards 181-183 form one serial worker lane. Card 181 owns exact
package, parser, model, entitlement, workflow, tool, process, persistence,
output, and lifecycle evidence. Cards 182-183 run only if Research 212 admits a
non-empty exact table whose product-specific behavior fits the existing route.
An honest evidence stop is a complete result.

This is the complete handoff from the planning/orchestrator thread to one
bounded implementation thread. Start from this file without a copied
transcript or second prompt. Do not create internal subagents or parallel
worker lanes; the operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md`. Read the `effigy` skill before validation.

## Why It Matters

The per-route programme exists to turn known feature gaps into exact route
truth, not to leave them in a research list. Ultracode is valuable only if
Swallowtail can preserve what Claude Code actually means by it. Treating it as
a seventh `ReasoningMode`, an alias for ordinary `xhigh`, or a generic “fast”
toggle would erase the dynamic-workflow behavior and violate Contract 040.

The harder boundary is topology. The production route owns one read-only
Plan-mode prompt with fixed `Read,Glob,Grep`, empty MCP, no session persistence,
one selected model, and joined child cleanup. If Ultracode can silently add
tools, teammates, children, persistence, or unbounded internal activity, that
must either be proved contained by the exact selected command or stop the lane.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:**
  `9d26ec9c8b42e65f27310ae2c62d60e28a0d8fb9`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/065-claude-code-headless-ultracode.md`,
  cards 181-183, Research 212 reservation, compilation log, closeout
  reservation, programme/triage/index updates, and the sole Next Task
- **Worker branch:**
  `agent/g04-065-claude-code-headless-ultracode-20260825-215324`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-065-claude-code-headless-ultracode-20260825-215324`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-065-claude-code-headless-ultracode-20260825-215324
  -b agent/g04-065-claude-code-headless-ultracode-20260825-215324
  origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active spec lane:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/per-route-feature-completion.md`
- **Roadmap milestone:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/065-claude-code-headless-ultracode.md`
- **Ready cards, in order:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/181-claude-code-headless-ultracode-evidence.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/182-claude-code-headless-ultracode-binding.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/183-claude-code-headless-ultracode-acceptance.md`
- **Allowed runway:** one route/control family: `claude-code.headless`
  Ultracode on exact qualified Claude Code points, starting with the evidence
  relationship between the documented first-support lead and `2.1.241`
- **Remaining card budget:** three serial cards; stop after 181 unless Research
  212 admits a non-empty exact table and no decision gate fires
- **Dispatch topology:** one serial worker lane; all cards share the same
  adapter, package corpus, command builder, prepared facade, guide, API
  baseline, and closeout surfaces
- **Parallel safety check:** no parallel lane is authorized because the cards
  mutate the same adapter and truth surfaces
- **Route identity:** route `claude-code.headless`, driver
  `swallowtail.claude-code.headless`, axis
  `claude-code.headless-stream-json`, behavior
  `claude-code.headless.stream-json.v1`
- **Qualified window:** `2.1.220..=2.1.241`, semantic ordering,
  `AllowUnverified`; exact first Ultracode support is not yet qualified
- **Existing command:** one `claude -p` stream-JSON child, explicit selected
  model, optional ordinary effort, Plan mode, `Read,Glob,Grep`, user/project/
  local setting sources, empty strict MCP, and no session persistence
- **Candidate:** adapter-local Ultracode opt-in only; not a portable capability,
  not `ReasoningMode::xhigh`, and mutually exclusive with a separate effort
  selection
- **Canonical local refs:**
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/011-runtime-conformance-profiles.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/029-interface-version-qualification-and-compatibility.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/033-harness-configuration-posture.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/040-generation-control-application-and-enforcement.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/052-consumer-and-operator-integration-documentation.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/202-claude-code-2-1-241-identity.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/212-claude-code-headless-ultracode-evidence.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/guides/claude-agent-prepared-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-21-advanced-route-features.md`
- **Route-local source leads:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/prepared_code/profile.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/claude_code_command.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/claude_code_selection.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/claude_code.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/claude_code_activity.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/src/claude_code_pump.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/tests/claude_code_structured_run.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/identity.json`
- **Model capability profile:** frontier implementation and protocol/topology
  audit; deterministic official source/package and fixture evidence only
- **Tool/runtime restrictions:** official public source and packages may be
  downloaded and extracted in disposable `/tmp`; do not install or update
  Claude Code, inspect account state, capture credentials, run a provider
  prompt, or use paid inference. An extracted binary may run only for local
  version/help/parser cases proved to stop before authentication/provider work.
- **Required validation:** `cargo fmt -p
  swallowtail-adapter-claude-agent`, `effigy validate:focused
  swallowtail-adapter-claude-agent`, `effigy package:verify-affected
  swallowtail-adapter-claude-agent`, `effigy check:examples`, `effigy
  qa:routes`, `effigy qa:northstar`, `effigy package:api`, relevant research/
  log/roadmap/g04/batch-card and sole-next-action index gates, `effigy doctor`,
  and `git diff --check`; run conditional implementation gates only if cards
  182-183 execute
- **Known doctor baseline:** 378 god-file findings: 332 warnings and 46 errors;
  stale graph index; one generated-in-src warning. Do not increase it.
- **Planning validation:** `effigy tasks`, `effigy doctor`, `effigy test
  --plan`, `effigy qa:docs`, `effigy qa:northstar`, all affected indexes,
  sole-next-action, and `git diff --check` ran. Docs and Northstar passed;
  doctor reproduced the inherited baseline.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Please keep this run inside the named runway:

- **In scope:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-claude-agent/**`
  for exact package/parser evidence, selected-command corpus, adapter-local
  Ultracode preparation/binding, command construction, route-local tests,
  fixtures, example and API baseline only when Research 212 admits delivery;
  Research 212; g04.065; cards 181-183; Claude guide; route/feature matrices;
  changelog only if public behavior warrants it; programme, triage, reserved
  closeout, indexes, and sole Next Task
- **Out of scope:** adding `ultracode` to portable `ReasoningMode`; ordinary
  effort vocabulary changes; Fast mode; Agent teams; teammate/custom-agent,
  autocompact, JSON Schema, budget, permission, sandbox, raw settings, generic
  argv/environment, response-only, Claude Agent ACP, Anthropic Messages or
  Managed Agents; another route/version family; shared contracts/runtime;
  live provider work; release, publication, merge, rollover, or g04 closure
- Card 181 makes no production claim edit. Reconfirm the exact evidence point
  and first supported package before inspecting behavior. Do not qualify a
  moving current version or broaden the Contract 029 window in this feature
  lane.
- Reconcile official docs with exact package behavior. Documentation names are
  leads; exact `2.1.241` help does not currently advertise `ultracode`.
- Distinguish parser acceptance, planned dispatch, child argv, provider
  acceptance, effective Ultracode, and later observation. No-auth parser cases
  cannot prove provider acceptance or effectiveness.
- Freeze model, entitlement, billing, settings precedence, unsupported-value,
  alias, clamp, fallback, and omission truth. If those facts require account or
  provider work, record an empty row and stop.
- Audit dynamic workflows, not just argv. If the selected command can add
  tools, spawn unjoined processes, enable teammate state, persist sessions,
  bypass Plan mode, or disclose internal workflow content, stop unless exact
  deterministic evidence proves the effect is disabled or bounded inside the
  current route.
- Keep Ultracode adapter-local. If delivery is admitted, use a closed opaque
  opt-in and reject a separate portable effort selection before spawn. Do not
  expose raw settings or rely on ambient config to select the mode.
- Preserve ordinary effort and omission behavior. Omitted Ultracode must leave
  the current command byte-equivalent.
- Preserve the selected model, working resource, access, Plan mode, fixed
  tools, empty MCP, no-session persistence, retention, activity, usage,
  cancellation, deadline, terminal, stop, and joined cleanup truth.
- Do not expose hidden reasoning, workflow prompts, internal traces, or
  unqualified teammate/subagent detail as output or activity.
- Default QA must not resolve credentials, inspect account state, install
  Claude, run a provider prompt, contact the provider, or use paid inference.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean, reset, or stash over unrelated state.
- Do not create subagents or parallel workers. Do not merge the PR. Merge
  remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, canonical authority, glue-light reporting,
  and Effigy selectors. Work in one meaningful batch.

## Important Context

- **Planning lineage:** g04.055 qualified headless through exact `2.1.241`.
  g04.045 previously showed how exact CLI help can diverge from current docs
  and stopped structured output honestly. g04.065 applies the same
  evidence-first discipline to a product-specific execution setting.
- **Why these cards are ready:** the route, exact version claim, selected-model
  input, child ownership, prepared facade, ordinary effort mapping, and
  deterministic package inspection path already exist. The cards are bounded
  around one exact provider product setting and contain explicit stop gates.
- **Decisions and preferences:** manual operator-harness handoff only; no
  internal subagents. Ultracode stays provider- and route-specific. g04 remains
  open until the operator explicitly closes it.
- **Open tensions:** official docs name the setting while exact help does not;
  dynamic workflows may be implemented behind the same effort flag; model and
  entitlement checks may happen only after authentication; existing
  `Read,Glob,Grep` and Plan constraints may or may not suppress all workflow
  topology; immutable adapter-local evidence must fit without a shared
  capability.
- **Report after:** card 181 and Research 212, then cards 182-183 only if the
  exact deliver-now gate passes
- **Report to:** the operator, who relays progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight below. If the current context is a clean, dedicated, non-`main`
registered worktree, use it immediately, record its actual path/branch, and do
not create another worktree because its generated name differs from this file.

Read `AGENTS.md`, Northstar handoff mode, Effigy, g04.065, cards 181-183,
Research 192/202/212, Contracts 029/033/037/040/044, the Claude prepared guide,
and the route-local files named above. Execute card 181 first. Promote Research
212 with exact selected package/source evidence and one honest decision.
Continue automatically only if its gate admits an adapter-local opt-in without
widening topology or shared authority.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads: `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare it with the placeholder path/branch or
   create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if it is absent. Create a unique worktree and branch under that
   container from `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path;
   never clean, reset, stash over, or discard another checkout's state. If the
   launcher supplied a dirty or `main` worktree, stop and report it rather than
   silently creating a second worktree.
4. From the selected worktree, confirm `HEAD == origin/main`, confirm planning
   commit `9d26ec9c8b42e65f27310ae2c62d60e28a0d8fb9` is an ancestor of
   `HEAD`, and confirm this handoff exists in the selected `HEAD`.
5. Read the named skills, milestone, cards, evidence, contracts, guide, and
   route-local source before editing.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline; do not turn it into unrelated cleanup.

### Execute

1. Execute card 181 only. Recheck current official docs, inspect exact selected
   packages/binaries in disposable `/tmp`, freeze parser and topology evidence,
   add any secret-free corpus, promote Research 212, and name the decision. Make
   no production claim edit yet.
2. If the decision is empty, needs live provider/account evidence, needs raw
   settings/config mutation, widens tools/process topology, or needs a new
   shared contract/public lifecycle, stop honestly. Leave cards 182-183
   unexecuted, close g04.065 as a named evidence stop, update current docs and
   indexes, validate, commit, push, and open the evidence PR.
3. If Research 212 admits exact rows, execute cards 182-183 in the same serial
   lane. Bind the smallest opaque adapter-local opt-in, reject a separate effort
   selection before spawn, preserve omission bytes, and prove every admitted
   workflow/lifecycle boundary.
4. Mark roadmap/card/research/log/index status from actual evidence. Keep g04
   active. Leave one honest sole Next Task for orchestrator continuation.

### Validate and report

1. Run every card-specific gate and the final validation set named in Current
   State. Use specific index selectors. Default validation stays account-,
   credential-, install-, prompt-, and provider-free.
2. Run `git diff --check` against the planning base. Confirm no secrets,
   downloaded packages, executable artifacts, temporary corpus, or
   out-of-scope changes remain.
3. Commit cohesive work, push the selected worker branch, and open one PR
   against the current pushed `main` tip. Do not merge it.
4. In the PR body, link g04.065, cards 181-183, Research 212, changed surfaces,
   evidence, validation, and unresolved items.
5. Report exact branch, PR URL, head SHA, Research 212 decision, delivered rows
   or stop reason, parser/help result, workflow/topology result, card 182/183
   disposition, changed surfaces, validation, inherited baseline, and review
   concerns. State clearly that merge authority remains with the
   operator/orchestrator.

### Review and merge path

The orchestrator will review the PR against canonical refs, exact head, diff,
and checks. When formal self-approval is unavailable, it will post the verdict
as a PR comment. If changes are requested, make only those changes on this
branch, push again, and report back through the operator. The operator must
explicitly authorise any merge.

- **Closeout refs:** g04.065, cards 181-183, Research 212, the reserved g04.065
  closeout, programme, triage, research/log/roadmap indexes, and the sole Next
  Task

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the exact evidence or topology
stop and stop rather than making the handoff look more complete than it is. Do
not create a second handoff.
