---
title: g04.067 OpenCode HTTP web search worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
planning_base: 6409af0c472595a2dcd02a25fff2ddb933da562c
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260826-101859-g04-067-opencode-http-web-search.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, per-route-feature]
---

## What This Thread Was Doing

The orchestrator reconciled g04.066 after PR 65 and reassessed the remaining
per-route feature inventory. It repaired two stale front-door entries: cards
184-186 now sit under completed, and the generation index records g04.066 as
the 57th completed milestone.

OpenCode HTTP `websearch` is the strongest bounded next lead. Exact `v1.18.20`
source registers a native tool, requests a dedicated `websearch` permission,
and accepts ordered permission rules on session creation. The maintained
Swallowtail route already owns that request, deny-first rules, one-shot
permission callbacks, shared search/network policy, and structured and
interactive prompt paths.

This is not a prequalified feature. Exact source also makes availability depend
on provider/backend and environment facts. g04.067 and cards 187-189 therefore
form one serial evidence-first lane. Card 187 must prove an exact host-bindable
availability, permission, policy, and profile row. Cards 188-189 run only when
Research 214 admits a non-empty deliver-now table. An honest empty set is a
complete result.

This is the complete handoff from the planning/orchestrator thread to one
bounded implementation thread. Start from this file without a copied transcript
or second prompt. Do not create internal subagents or parallel worker lanes;
the operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md`. Read the `effigy` skill before validation.

## Why It Matters

The per-route programme exists to turn known feature gaps into exact route
truth. A registered search tool does not prove that the selected attached
server can run it. A permission rule does not prove tool availability. A
callback approval does not grant network authority. A returned tool part does
not prove result quality or model use.

This lane is useful only if those boundaries remain separate. The safe shape
is existing shared `ExternalSearchPolicy::Enabled` with compatible
host-approved network authority, exact prepared evidence, one qualified
session permission row, unchanged disabled omission, and fail-closed rejection
before effects when a required fact is knowable.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:**
  `6409af0c472595a2dcd02a25fff2ddb933da562c`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g04.067, cards 187-189,
  Research 214 reservation, compilation log, closeout reservation, programme,
  triage, generation/g04/batch-card indexes, and sole Next Task
- **Worker branch:**
  `agent/g04-067-opencode-http-web-search-20260826-101859`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-067-opencode-http-web-search-20260826-101859`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-067-opencode-http-web-search-20260826-101859
  -b agent/g04-067-opencode-http-web-search-20260826-101859 origin/main`
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
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/067-opencode-http-web-search.md`
- **Ready cards, in order:**
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/187-opencode-http-web-search-evidence.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/188-opencode-http-web-search-binding.md`, then conditional
  `/Users/tom/Dev/projects/swallowtail/docs/roadmaps/g04/batch-cards/189-opencode-http-web-search-acceptance.md`
- **Allowed runway:** one route/control family: native OpenCode web search on
  `opencode.http`, exact qualified versions and only provider/backend,
  permission, policy, and operation-profile rows admitted by Research 214
- **Remaining card budget:** three serial cards; stop after 187 unless Research
  214 admits a non-empty exact table and no decision gate fires
- **Dispatch topology:** one serial worker lane; all cards share the same
  adapter, exact release corpus, prepared inputs, plan/evidence, session-create
  request, callback/event decoder, guide, matrices, and closeout surfaces
- **Parallel safety check:** no parallel lane is authorized because the cards
  mutate the same adapter and truth surfaces
- **Route identity:** route `opencode.http`, driver
  `swallowtail.opencode.http`, axis `opencode.server`, qualified ceiling
  `1.18.20`, behavior `opencode.http-sse.surface-19`
- **Existing operation shape:** attached server; exact provider/model and
  working resource; private temporary structured-run session or retained
  interactive session; session-create body with wildcard deny/ask followed by
  explicit read/glob/grep allow rules; prompt, SSE, optional one-shot
  permission/question callbacks, abort, and exact cleanup
- **Existing policy posture:** structured runs reject any external search and
  any external network posture other than denied; session inputs likewise make
  no current web-search claim
- **Candidate:** existing shared `ExternalSearchPolicy::Enabled` with the
  compatible host-approved network policy and exact permission/profile binding
  selected by Research 214; no generic OpenCode settings type
- **Canonical refs:**
  `/Users/tom/Dev/projects/swallowtail/AGENTS.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/008-runtime-registration-and-preflight.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/009-async-operation-lifecycle.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/010-execution-host-services-and-inputs.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/011-runtime-conformance-profiles.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/013-interactive-session-access-policy.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/029-interface-version-qualification-and-compatibility.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/033-harness-configuration-posture.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/037-prepared-consumer-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/041-input-callback-and-provider-tool-admission.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/044-observable-agent-activity-and-disclosure.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/contracts/052-consumer-and-operator-integration-documentation.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/176-opencode-http-1-18-20-identity.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/research/214-opencode-http-web-search-evidence.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/guides/generation-controls-and-input-authority.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/guides/opencode-attached-prepared-integration.md`,
  `/Users/tom/Dev/projects/swallowtail/docs/triage/2026-08-21-advanced-route-features.md`
- **Route-local source leads:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/prepared_profile/input.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/prepared_profile/plan.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/prepared_profile/operations.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/protocol/sessions.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/protocol/prompt_and_callbacks.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/protocol/events/provider_requests.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/driver/run/handle.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/src/selection.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/tests/prepared_facade/cases.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/tests/prepared_facade/input_callbacks.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/tests/http_driver.rs`,
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.20/protocol.json`
- **Exact official source leads:** tag `v1.18.20`, especially
  `packages/opencode/src/tool/websearch.ts`, `tool/registry.ts`,
  `effect/runtime-flags.ts`, `permission/index.ts`, `session/session.ts`,
  `session/tools.ts`, and `session/prompt.ts`; current official permissions and
  tools docs are corroboration, not a substitute for the exact tag
- **Preliminary lead only:** the tag chooses between Exa and Parallel search
  paths, reads `OPENCODE_WEBSEARCH_PROVIDER`, `OPENCODE_ENABLE_EXA`, and related
  backend credentials/availability, and asks permission `websearch` against the
  query. Re-derive and digest this in Research 214; do not copy it as accepted
  evidence from the handoff.
- **Model capability profile:** frontier implementation plus exact TypeScript
  source, HTTP/session permission, provider/backend, and fail-closed authority
  audit; deterministic source/fixture work only
- **Tool/runtime restrictions:** official public source and packages may be
  downloaded and extracted in a disposable temporary directory; do not install
  or update OpenCode, mutate the attached server, inspect account/backend keys,
  capture credentials, run a provider prompt, execute hosted search, contact
  an external search backend, or use paid inference
- **Required validation:** card 187 runs `effigy validate:focused
  swallowtail-adapter-opencode`, `effigy qa:northstar`, relevant indexes, and
  `git diff --check`; conditional cards 188-189 add `cargo fmt -p
  swallowtail-adapter-opencode`, `effigy package:verify-affected
  swallowtail-adapter-opencode`, `effigy check:examples`, `effigy qa:routes`,
  `effigy package:api`, all named docs/index gates, and `effigy doctor`
- **Known doctor baseline:** 378 god-file findings: 332 warnings and 46 errors;
  stale graph index; one generated-in-src warning. Do not increase it.
- **Planning validation:** `effigy tasks`, `effigy doctor`, `effigy test
  --plan`, `effigy qa:docs`, `effigy qa:northstar`, `effigy qa:routes`, and
  `git diff --check` ran. Docs, Northstar, routes, indexes, next action, and diff
  passed; doctor reproduced the inherited baseline and stale graph warning.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:**
  `/Users/tom/Dev/projects/swallowtail/crates/swallowtail-adapter-opencode/**`
  for exact release/tool/provider/permission evidence and, only after Research
  214 admits delivery, prepared input, policy/plan/evidence, session-create
  request, validation, protocol/events, tests, fixtures, example and API
  baseline; Research 214; g04.067; cards 187-189; OpenCode prepared guide;
  route/feature matrices and changelog only where public truth changes;
  programme, triage, reserved closeout, indexes, and sole Next Task
- **Out of scope:** web fetch; browser or arbitrary URL access; generic
  OpenCode tool/permission/settings surface; `task` subagents or teams; shell or
  write widening; provider/model/backend selection; environment or attached-
  server mutation; credential injection; another OpenCode transport or route;
  shared contracts/runtime; live provider/account/search work; currentness,
  release, publication, merge, rollover, or g04 closure
- Card 187 makes no production claim edit. Reconfirm exact `v1.18.20` identity
  and source before classifying behavior. Do not qualify moving main or broaden
  the Contract 029 window in this feature lane.
- Treat tool registry presence only as a lead. Freeze the exact availability
  predicates, provider/backend selection, environment gates, credentials,
  fallbacks, request construction, errors, and observable events.
- Prove permission rule order and matching, the selected `allow|ask|deny`
  action, query pattern, callback request/reply, denial, cancellation, and
  terminal behavior. Never treat wildcard `ask` as search authority.
- Distinguish requested policy, prepared capability, tool visibility,
  permission admission, external dispatch, backend acceptance, result
  observation, effective model use, and later activity. Claim only the exact
  boundary proved.
- Ambient attached-server configuration and environment are not host-approved
  evidence. If deterministic availability cannot be bound through existing
  preparation without server mutation, Research 214 should be empty.
- Preserve exact denied/disabled policy, session-create JSON, read/glob/grep
  rules, provider/model binding, reasoning, schema, image, working resource,
  callbacks, retention, activity, usage, cancellation, deadline, terminal,
  private-session deletion, detachment exclusions, and joined cleanup truth.
- Default QA must not resolve search credentials, inspect account/backend
  state, install or configure OpenCode, run a provider prompt, contact a search
  backend, or use paid inference.
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

- **Planning lineage:** g04.035-066 delivered or stopped exact route-local
  controls one family at a time. g04.066 is merged. The programme and triage
  note select g04.067 next. Contract 029 currentness stays a separate standing
  lane and g04 stays open at operator direction.
- **Why these cards are ready:** the route, exact qualified release, prepared
  boundary, candidate native tool/permission, shared policy pair, evidence
  method, stop gates, acceptance shape, validation, and continuation state are
  named. No provider/backend/profile row is assumed before card 187.
- **Decisions and preferences:** one route and one coherent feature family;
  exact typed shared policy; exact session permission; no generic settings;
  disabled omission stable; unsupported truth rejects before effects; no live
  provider/search work and no g04 closure.
- **Open tensions:** a registered tool may be hidden or unusable for the
  selected provider; availability may depend on ambient environment or backend
  credentials; provider selection may not be deterministic from Swallowtail's
  prepared evidence; permission callbacks may occur only after prompt effects;
  SSE may not distinguish backend acceptance from tool output. Any of these may
  yield an empty set.
- **Report after:** card 187 evidence promotion or stop, then the combined cards
  188-189 implementation/acceptance chunk if authorized by Research 214, then
  final pushed PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the startup worktree-safety preflight below.
Then execute card 187 and promote Research 214. Continue to cards 188-189 only
for its exact non-empty deliver-now rows.

## Completion Protocol

1. Confirm the current checkout is a registered worktree, the branch is not
   `main`, `git status --short` is clean, and the planning base is an ancestor
   of `HEAD`. Record the actual worktree and branch in the closeout.
2. Read the named skill instructions, roadmap, cards, Research 176, contracts,
   route guide, production source, fixtures, and current claims before edits.
3. Execute card 187 as one evidence batch. Retrieve and digest exact official
   `v1.18.20` source. Promote Research 214 with a row-by-row table or explicit
   empty set. Run the evidence validation before deciding continuation.
4. If Research 214 is empty or a decision gate fires, mark card 187 complete,
   cards 188-189 blocked, g04.067 stopped, and reconcile the closeout,
   programme, triage, indexes, and sole Next Task. Skip production edits.
5. If Research 214 admits a non-empty set, execute cards 188-189 as one
   meaningful implementation/acceptance batch. Bind only admitted rows. Keep
   disabled omission and all named lifecycle truth exact.
6. Run the card-named focused validation. Run broad or live tests only when the
   cards explicitly authorize them. Compare doctor output with the recorded
   baseline.
7. Update card/milestone/research/closeout status, route/feature truth where it
   changes, programme, triage, indexes, and the sole Next Task. Do not select or
   compile the next route family.
8. Review `git diff --check`, the complete diff, public API baseline where
   applicable, and `git status --short`. Commit in one or a few meaningful
   batches, not micro-commits.
9. Push the worker branch and open one PR against current `main`. Record the
   actual PR URL, branch, worktree, base, and exact head in the closeout without
   claiming merge.
10. Report the evidence decision, delivered or withheld rows, validation,
    doctor delta, PR URL, and exact head to the operator. Stop. Do not merge,
    roll, or close g04.
