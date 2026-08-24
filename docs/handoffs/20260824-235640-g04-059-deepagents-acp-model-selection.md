---
title: g04.059 Deep Agents ACP model-selection worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-24
updated: 2026-08-24
planning_base: 386027e78915f26f3c2020e0c0d6bb639f8eace6
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260824-235640-g04-059-deepagents-acp-model-selection.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining promoted per-route feature inventory
after g04.058 and selected exact `deepagents.acp` model selection. g04.059 is
compiled. Implementation has not started. The ready runway begins with exact
`deepagents-acp@0.1.25` parser, provider, access, fallback, confirmation, and
lifecycle evidence; cards 165-166 are conditional on a non-empty Research 206
deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

The production route owns one exact ACP child per prepared session but starts
it with no extra argv. Current official LangChain docs advertise `--model` and
`provider:model`, so the selected transport appears able to choose the model
at the exact child boundary instead of silently relying on an upstream default.

That surface is not qualified for exact `0.1.25`. Provider grammar, access-key
agreement, fallback, aliases, effective confirmation, and restoration truth
must come from the exact package. A safe empty set is better than exposing a
string that can choose an unproved provider, fall back, or mismatch the
host-owned key posture.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `386027e78915f26f3c2020e0c0d6bb639f8eace6`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `386027e78915f26f3c2020e0c0d6bb639f8eace6` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.059, cards 164-166,
  Research 206 reservation, compilation log, route-local closeout reservation,
  inventory reassessment, programme boundary, and the sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-059-deepagents-model-selection-20260824-235640`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-059-deepagents-model-selection-20260824-235640`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-059-deepagents-model-selection-20260824-235640 -b agent/g04-059-deepagents-model-selection-20260824-235640 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion; existing contracts are
  the complete authority and no contract edit is planned
- **Roadmap milestone:**
  `docs/roadmaps/g04/059-deepagents-acp-model-selection.md`
- **Ready cards, in order:** card 164, then conditional card 165, then
  conditional card 166
- **Allowed runway:** exact Deep Agents ACP model-selection evidence, then only
  Research 206 deliver-now prepared binding and route-local acceptance
- **Remaining card budget:** three serial cards; cards 165-166 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share exact package evidence, prepared
  inputs, access-profile agreement, plan/request state, child command, fixtures,
  guide, research, and closeout; they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 023, 029, 033, 037, 041, and 052
- **Route identity:** `deepagents.acp`, driver
  `swallowtail.deepagents.acp`, axis `deepagents-acp.package`, exact qualified
  npm package `0.1.25`, `QualifiedOnly`
- **Current mapping:** one host-approved `deepagents-acp` child per session;
  no extra argv; child cwd from the read-only working resource; initialize,
  `session/new`, one bounded `session/prompt`; no model input
- **Current access:** local unauthenticated
  `deepagents_provider_api_key_access_profile`; host owns Anthropic or OpenAI
  API keys in the isolated child environment; Swallowtail opens no credential
  lease and does not bind key bytes
- **Current lifecycle:** one in-process session/child; no load or resume;
  fresh working-state restoration is context-losing; cancellation and close
  join owned turn, connection, task, process, and working-resource state
- **Model capability profile:** model selection is currently unsupported;
  omission owns no upstream default claim; advertised `--model` is not yet
  qualified or effective truth
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no installation, `npx`, ACP server execution, login,
  host-key inspection, credential capture/materialization, authenticated
  provider prompt, external inference request, or paid work. Current official
  public docs, exact public npm artifact/source, existing fixtures, and
  secret-free local parser/unit inspection are allowed by card 164.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-deepagents`, `effigy validate:focused
  swallowtail-adapter-deepagents`, `effigy package:verify-affected
  swallowtail-adapter-deepagents`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, `effigy doctor`, and `git diff --check`
- **Known doctor baseline:** inherited 378 god-file findings: 332 warnings and
  46 errors; stale graph index; one generated-in-src warning. New tests must be
  focused and must not increase the finding/error counts.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index gates,
  and `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-deepagents/**` for exact
  provider/model value, prepared session input, capability/constraint and
  plan/request/access agreement, one `--model <provider:model>` mapping,
  admitted confirmation, safe failures, lifecycle propagation, fixtures,
  example, and package API baseline;
  `docs/guides/deepagents-acp-prepared-integration.md`; Research 206; g04.059;
  cards 164-166; the reserved g04.059 route-local closeout; current official
  LangChain Deep Agents ACP/model docs; exact public
  `deepagents-acp@0.1.25` artifact/source; existing secret-free fixtures and
  local source/parser/unit evidence
- **Out of scope:** a generic provider settings map; arbitrary argv or an
  unconstrained model string; live/moving model catalogue; model quality or
  portability claim; `--skills`, `--memory`, `--workspace`, `--name`,
  `--debug`, or `--log-file`; library embed; `npx`; registry package `0.1.7`;
  another Deep Agents route; API-key read/injection/lease/persistence; login or
  provider prompt; model/provider fallback; MCP/tool/permission/resource/
  persistence/continuation expansion; containment claims; currentness;
  `CHANGELOG.md`; shared architecture/contracts; release, publication, merge,
  generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, access, security, persistence, or compatibility decision.
- Current official docs are leads. Exact `0.1.25` source/artifact evidence owns
  parser, provider, default, alias, fallback, auth, and lifecycle claims.
- Provider/model selection must agree with explicit prepared host-owned access
  evidence before spawn. Never inspect or materialize API-key bytes.
- Missing or wrong credentials, missing integration packages, invalid provider
  or model, and provider rejection must not retry an alternate provider/model
  or ambient default.
- Dispatch is not accepted/effective/observed truth. Require an exact wire
  confirmation if Research 206 finds one; otherwise expose only the exact
  dispatch-only claim Research 206 admits. Stop if neither is supportable.
- Omission must preserve the current empty argv and must not acquire an
  upstream default-model claim.
- Selection is immutable for one owned child/session. There is no per-turn
  model change, load/resume, transcript recovery, or durable persistence.
- Fresh restoration may reassert the prepared value only if Research 206
  admits it and access/version evidence still agrees; it remains
  context-losing.
- The route remains `AmbientHost`. Model selection does not add filesystem or
  descendant-process containment, resource access, host callbacks, MCP, tool,
  permission, or provider-session authority.
- Failure and cancellation join every owned process/task and do not imply
  provider-state deletion.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, the canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 153 selected the first-party npm ACP route;
  Research 157 froze exact `0.1.25` identity and source-derived protocol;
  Research 159 retained it as exact `QualifiedOnly`. g04.058 stopped a separate
  Antigravity profile family and left the programme pointer ready for
  reassessment.
- **Official lead:** current LangChain ACP docs advertise `--model` / `-m`;
  current model docs describe `provider:model`. These pages can drift and do
  not prove `0.1.25`.
- **Existing exact artifact:** Research 157 downloaded the 11-file
  `deepagents-acp@0.1.25` tarball and recorded npm identity/integrity. Reuse or
  reverify that artifact. Freeze new complete file/specimen digests in
  Research 206 rather than relying on current docs.
- **Existing command:** `crates/swallowtail-adapter-deepagents/src/command.rs`
  returns an empty vector and explicitly forbids `--model` in tests.
- **Existing fixtures:**
  `crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.25/`
  freeze identity, initialize, session/new, prompt, cancel, permission,
  negative cases, and corpus plan. The negative corpus currently treats
  `--model` as unselected.
- **Identity caveat:** initialize `agentInfo.version` is constructor default
  `0.0.1`, not npm package `0.1.25`; present `agentInfo.name` must remain
  `deepagents-acp`. Do not compare the ACP version field to the package axis.
- **Access caveat:** current preparation uses one generic local provider-key
  access profile. Research 206 must decide whether exact provider agreement can
  be represented under existing contracts. If not, stop; do not infer which
  host key exists from environment contents or provider failure.
- **Auth truth:** missing host Anthropic or OpenAI key currently maps provider
  error text to `swallowtail.deepagents.acp.host_auth_required`. This is not
  proof of selected-provider agreement and must not become a fallback trigger.
- **ACP truth:** the route currently validates agent name but claims no model
  field in initialize/session/events. Source inspection must find exact
  confirmation or support an explicitly dispatch-only result.
- **Current permission truth:** ACP permission requests are observed and
  cancelled; `allow_always` / `allow-always` are never selected. Host fs
  callbacks are rejected. Local child writes remain outside a bounded-write
  claim.
- **Decision preference:** a narrow adapter-local provider/model value is
  acceptable only for Research 206's exact rows. Do not create a generic
  cross-provider model type or settings map.
- **Open tensions:** exact provider prefixes and model grammar may differ from
  current docs; suffixes may pass through unvalidated; the default may change;
  model construction may throw only after spawn; ACP may expose no effective
  model; the generic access profile may not prove provider agreement.
- **Report after:** card 164 and Research 206 are complete, then after the
  binding/acceptance batch if the deliver-now set is non-empty
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight in `## Completion Protocol`. If the current context is a clean,
dedicated, non-`main` registered worktree, use it immediately, record its actual
path/branch, and do not create another worktree because its generated name
differs from this file.

Read `AGENTS.md`, g04.059, cards 164-166, Research 153/157/159, the Deep Agents
guide, system architecture, and Contracts 011/023/029/033/037/041/052. Execute
card 164 first. Promote Research 206 with a non-empty exact table or an honest
empty set. Continue automatically only when its gate is satisfied.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad reads: `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare it with the placeholders or create a
   second worktree merely because names differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if absent. Create a unique worktree/branch there from pushed
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path. Never clean,
   reset, stash-over, or discard another checkout. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm `git merge-base --is-ancestor
   386027e78915f26f3c2020e0c0d6bb639f8eace6 HEAD` succeeds, and confirm this
   handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline and do not run the full planned workspace suite.

### While you work

- Execute cards 164-166 in order. Stop after card 164 when Research 206 is
  empty or a named gate fails. An evidence stop is a complete worker outcome.
- Keep commits aligned with meaningful chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the card-specific final validation. If code executes, run
   `cargo fmt -p swallowtail-adapter-deepagents`, `effigy validate:focused
   swallowtail-adapter-deepagents`, `effigy package:verify-affected
   swallowtail-adapter-deepagents`, `effigy check:examples`, `effigy qa:routes`,
   `effigy qa:northstar`, the relevant docs index gates, `effigy package:api`,
   `effigy doctor`, and `git diff --check`. If card 164 stops with docs only,
   run its named focused/docs/diff gates and record why code-only gates did not
   apply.
2. Update Research 206, milestone/cards, Deep Agents guide/matrix only as
   warranted, reserved closeout, programme, triage, and sole Next Task. Keep
   g04 open.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The planning
   base above predates this handoff commit and is intentionally not
   self-referential.
5. In the PR body, link the milestone, cards, Research 206, changed surfaces,
   exact evidence, validation, and unresolved items.
6. Report the PR URL and exact head SHA to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator and worker may share one GitHub identity. Formal self-approval
is then unavailable; the orchestrator posts the evidence-backed verdict as a
PR comment. If changes are requested, make only those changes on this branch,
push again, and report back through the operator. Requested changes: none yet.
The operator must explicitly authorise any merge.

- **Closeout refs:** Research 206; g04.059; cards 164-166; reserved g04.059
  closeout; Deep Agents guide/matrix only when warranted; research/logs/
  roadmaps/g04/batch-card/Next Task front doors; triage reassessment; g04 stays
  open

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, research, log,
triage, and Next Task state honest. If the work is blocked, record the blocker
and stop rather than making the handoff look more complete than it is.
