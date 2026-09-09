# g05.035 Shared Harness Capability And Producer Boundary

Status: planned; Batch A promoted (card 114 complete through the folded ledger below); former stubs 130/134 preserved as g05.041/g05.042
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-09
Depends on: Research 288; Spec 014; Contracts 012, 017, 019, 028, 029, 037, 041, 047, 051, 057, 058, 060-062; g05.029 card 084 evidence
Vision tags: consumer integration, tools, MCP, skills, context, permissions, sessions

## Purpose

Deliver one provider-neutral registered tool/server boundary and route-exact
Claude, Codex, and Grok adoption. Reuse current registry, prepared-plan,
host-service, callback, permission, projection, session, and cleanup machinery.
Do not flatten provider behavior or move Longhorn/Desktop semantics into
Swallowtail.

## Current State

Batch A promotes Contract063 and amendments041/060/061/062. Bilateral ownership
is settled at Desktop30a338f2, Longhorn edc21078 and Swallowtail6fa6266b.
Implementation remains unshipped. Card084's stdio-only SDK support is merged
untagged prior art and is reused, not recreated.

## Execution Plan

- [x] **Batch A — architecture and contract promotion.** Independently review
      Spec 014; retain Longhorn PR 22 alignment at PASS head `6ce4aa1b`;
      promote repository ownership, registered capability, the Contract 060
      common-kernel and compatibility amendment, server profile/transport, unique tool kind,
      progress, Allow/Deny, skill/reference, and projection rules. Recompile
      this roadmap.
- [ ] **Batch B — shared kernel and host conformance.** Add the provider-neutral
      registration snapshot, schema namespace/digest, and registered-server
      profile by extending Contract 060's existing operation lease and private
      loopback lifecycle; retain the compatible closed `WatcherBridge` profile;
      mount the Desktop-linked Longhorn dispatch/validation seam, add any
      qualified bounded SSE carrier, ordered progress, safe
      diagnostics, projection rows, and exhaustive provider-free race,
      migration, compatibility, and teardown fixtures. No adapter change,
      stdio/daemon startup, or second registry/lease/listener implementation.
- [ ] **Batch C — Claude routes.** Consume PR 255's merged stdio-only Claude SDK
      attachment after Batch B, then integrate the centralized snapshot and
      Contract 060-derived common lease without duplicating its process,
      admission, or lifecycle work. Retain ACP empty-MCP truth unless its
      separate gate passes; prove a real disposable server/result, options,
      fresh-session limits, Allow/Deny, continuation, skill/reference transport,
      and no unsupported steering or release claim.
- [ ] **Batch D — Codex and Grok routes, parallel where paths do not overlap.**
      Codex maps the common registry to dynamic native tools and keeps direct
      MCP withheld without evidence. Grok maps exact one-shot permission and
      provider-tool observation; consumer tools/MCP remain withheld unless its
      surface gate passes.
- [ ] **Batch E — real acceptance and consumer sequence.** Desktop packages a
      disposable linked Longhorn host; Desktop binds context, skills, references, and product
      receipts. Run the six real acceptance classes per route. Update route and
      feature matrices, guides, public API baselines, source consumer, and
      exact-SHA CI. Release/tag remains separately authorized.

## Independent Delivery Boundaries

| Batch | May deliver alone | Must not imply |
| --- | --- | --- |
| A | reviewed canonical architecture/contracts and a recompiled roadmap | runtime support |
| B | reusable registry/lease/transport APIs and provider-free conformance | any route or provider support |
| C | Claude route support matching its exact direct or mediated mechanism | Codex/Grok parity or generic steering |
| D1 Codex | dynamic-tool integration and exact failure/lifecycle evidence | provider-direct MCP |
| D2 Grok | exact permission integration and an honest MCP/tool disposition | consumer tool support from provider activity |
| E1 Longhorn | transport-neutral validation/dispatch library and generic errors | registry, listener, admission, domain schema/policy, daemon |
| E2 Desktop | linked host packaging, domain names/schemas/implementations, admission, context/skill selection, UX | provider wire or bridge lifetime |

Batch C, D1, and D2 can run in parallel after Batch B when their contract and
route-evidence gates pass. Other production routes reuse Batch B later and do
not delay Claude/Codex/Grok.

## Goals

- [ ] one central registered capability and server lease replaces no existing
      registry and introduces no generic executor
- [ ] Desktop-issued task+attempt maps one-to-one to a Swallowtail operation/
      turn attempt through authenticated host binding, never model arguments
- [ ] Desktop process incarnation plus Swallowtail lease generation is instance
      authority; PID remains diagnostic only
- [ ] native client, MCP, app, and provider-owned tools retain exact identity
- [ ] one namespaced tool identity binds exactly one tool kind per snapshot
- [ ] Contract 060's released watcher profile and Claude Code attachment retain
      behavior while both profiles share one operation-bridge kernel
- [ ] every call and result remains bound to task, session, turn, attempt,
      registration, transport generation, deadline, and cancellation
- [ ] reconnect never replays mutating work and stale callbacks fail closed
- [ ] every dispatch retry creates a fresh Desktop attempt and operation binding
- [ ] exact Allow/Deny behavior is documented per Claude, Codex, and Grok route
- [ ] skill/reference transport composes Contract 062 without claiming model
      compliance
- [ ] queue UX stays consumer-owned; mid-turn steering stays withheld until a
      route passes Contract 028

## Acceptance Criteria

- [ ] Batch A promotions are accepted in independent review before Batch B is
      marked ready
- [ ] Longhorn and Swallowtail ownership tables cite Desktop `30a338f2`, align,
      and pass separate exact-head reviews before either contract promotes
- [ ] shared provider-free conformance proves every lifecycle and security
      counterexample in Spec 014
- [ ] Contract 060 compatibility fixtures prove unchanged watcher admission,
      ready-before-provider order, omission, private material, terminal barrier,
      and joined teardown; no duplicate listener or lease manager exists
- [ ] ordered progress fixtures reject duplicate, regressive, foreign,
      stale-generation, post-cancel, post-terminal, and post-close notifications
- [ ] each route completes disposable workspace edit/reconcile, supported
      Allow/Deny, cancellation, skill with required references, disposable MCP
      server and real result or honest unsupported disposition, app-context
      binding, disconnect/failure, stale-callback rejection, and teardown
- [ ] no route claims parity from text fixtures, simulated approval, upstream
      advertising, or another adapter's implementation
- [ ] no broad shell, outside-path write, persistent permission, raw client
      content, credential, process, environment, or endpoint authority appears
      silently
- [ ] public API, compatibility, route matrices, guides, release, and consumer
      order follow Contracts 029, 036, 052, and 061

## Validation Shape

Batch A uses `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`.
Its review must cover the Contract 060 amendment and compatibility plan. Later cards name exact
package scopes for `effigy validate:focused` and
`effigy package:verify-affected`; shared Batch B includes core, runtime,
testkit, and host-local. Real provider acceptance is opt-in, separately
authorized, one route at a time, after deterministic conformance passes.

## Stop Conditions

- a required architecture or contract rule remains only in Spec 014;
- either producer plan drifts from the independently reviewed bilateral split;
- a route needs raw credentials, paths, environment, broad shell, ambient
  configuration mutation, or unrelated client content;
- provider-direct MCP cannot preserve the common lease and exact per-call
  admission boundary;
- Contract 060 would be copied, bypassed, or incompatibly changed without an
  explicit migration and operator decision;
- a Deny cannot be represented without simulated provider acknowledgement;
- reconnect requires mutating replay; or
- a batch would alter release, provider, global configuration, or consumer
  state without separate authority.

## Retired Card Dispatch (g05.038)

Batch A has independent planning acceptance recorded (see Batch A Planning
Acceptance below). One Swallowtail Coordinator owns this frontier; Desktop
Coordinator supplies consumer requirements and must not create a competing
producer worker. Card-level dispatch is retired: every card below is complete
or stopped, and serial edges are now owned by this task. Former backlog stubs
130 and 134 are preserved as g05.041 and g05.042.

### Folded Card Evidence

- 114 — complete; shared kernel; Batch A promotion gate.
- 115 — complete; bounded bundle/projection transport.
- 116 — complete; Claude route evidence and reviewed merge.
- 117 — complete; Codex dynamic-tool real binding evidence and reviewed merge.
- 118 — complete; qualified implementation or explicit unsupported disposition.
- 125 — complete; provider-free route-binding fixtures on the mounted path.
- 126 — complete; bundle transport fixtures.
- 127 — complete; bundle transport fixtures on the app-server fixture.
- 128 — complete; PR 273 merged at `63e34641`; provider-free four-verdict fake fixtures; live probe not run; packet relayed to Acowtancy Desktop.
- 129 — complete; every cross is provider-limitation-with-evidence or producer-gap-with-task; check enforces it; ranked backlog for pinned routes.
- 131 — complete; PR 277 merged at `f36e1165`; Codex client MCP is a cited provider limitation.
- 133 — complete; admission/invocation split; offline fixtures pass.
- 137 — complete; PR 285 merged at `56ea6008`; unanswered method named from existing capsules.
- 138 — complete; PR 282 merged at `a4b04134`; approvals observable-only on app-server, typed user-input is the answerable exchange.
- 140 — complete; PR 288 merged at `735eab70`; every probe bound re-derived for a live model.
- 142 — complete; PR 291 merged at `d0f2ea3170950629dd2b73866218ae903db0e287`; executable admitted-instance and registered-tool worked example at exact head.
- 143 — complete; PR 293 merged at `7d2dcb169dbee348567fd8308ed9c41fd5bf2d03` (reviewed head `5e7914424a8defb3dea164c150bd0d5ea806f3e9`); exact live-evidence qualification for MCP/tools.
- 144 — complete; PR 294 merged at `cc53c81ad903a562830d721b39a669aee010d037` (reviewed head `67e69e577518fdf0486998d47720e045a541bd7e`); structured failed-open evidence and provider-free diagnosis of stopped card 132.
- 145 — complete; Desktop PR 172 merged at `117e09e0`; bounded `mcp_status_invalid` before provider readiness; Research 297.
- 146 — complete; PR 296 merged at `13dee542e5ef4ab967cb4f0934cc11c35a8768c2` (reviewed head `58847bb00e4d15285af503c7023ff8750d1f1a7f`); exact `0.3.259` MCP-status projection repair.
- 147 — complete; Desktop PR 174 merged at `7646db45`; successful zero-credit open, immediate degraded close; Research 298.
- 148 — complete; Desktop PR 175 merged at `f8bff5a2`; exact route-qualified degraded cleanup accepted provider-free.
- 150 — complete; Desktop PR 178 merged at `dfc9c6a6`; mutable courier race reproduced and repaired; 24/24 churned opens pass.
- 153 — complete; PR 298 merged at `b35e4c38`; reviewed head `34ef6c32`; exact-tuple qualified projection; two matrix cells.

### Held Gates

- Real-route gate (from stopped card 132): Swallowtail supplies the packet only; Desktop runs under its isolated-testing authorization; Contract 061 row and matrix cells move only from the capsule; goal is one real registered call under the frozen tuple.
- First-turn diagnostic (from stopped card 149): Desktop PR 177 merged its valid capsule at `f29eb0c0` (open rejected at MCP startup before prompt; no credit inference; Research 299); card 150 owns the defect provider-free.

Workers require runtime/lifecycle capability; route work requires exact provider
protocol experience. Independent reviewers test adverse lifecycle cases.
Coordinator compiles Longhorn linked-host and Desktop consumer acceptance
handoffs after the API lands; neither needs a second registry or listener.
Release remains exact-SHA operator-gated under Contract036. No current next-task
pointer or unrelated active owner is evidence that this frontier is blocked.

## Grok Card 128 Return — 2026-09-08

Desktop's exact `1.0.4` and `1.0.5` capsules both returned
`accepts_client_mcp`. Research 295 freezes the source-linked Swallowtail SHA,
version receipts, capsule hashes, and Desktop review/merge/closeout chain.
Card 128's decision tree therefore converts its four pending cells to producer
gaps. Card 118 cannot own them because it is complete; card 143 is the live
successor. MCP admission, registered invocation, and consumer exchange are
directly evidenced. Selected-skill delivery was not in the echo probe and must
settle on its own transport evidence. No tag, release, parity, or unrelated
matrix inference follows.

## Claude Card 132 Return — 2026-09-08

Desktop's single authorized exact-tuple primary open returned typed
`open.failed.swallowtail.claude-agent.sdk.open_rejected` before any turn,
registered-tool dispatch, control attempt, or retry. Research 296 freezes the
source-linked SHA, capsule, artifact tuple, and Desktop lifecycle. Card 132 is
stopped and both Contract 061 cells remain unqualified. Card 144 owns the
provider-free diagnosis and an additive observation-derived failed-open
receipt. It is disjoint from Grok card 143 and may run in parallel. Another
live gate remains an operator decision after diagnosis; no tag, release,
feature classification, or broader acceptance follows.

## Claude Credit-Exhaustion Diagnostic — 2026-09-08

The operator confirmed the Claude account is out of usage credit and authorized
one diagnostic open before adding top-up credit. Card 145 uses the merged Card
144 receipt through Desktop's existing runner, exact source-linked Swallowtail
`6a93f1d916945aa2b402df7329dc994570e005c8`, and
`claude-sonnet-5`, the cheapest model in Desktop's admitted Sonnet/Opus
inventory. It runs no prompt, tool call, or control attempt. Any result remains
diagnostic and leaves both Contract 061 cells unqualified. The later credited
qualification suite remains separately authorized after the operator confirms
top-up.

## Claude Card 145 Return — 2026-09-08

Desktop's single diagnostic open returned `typed_failure` at
`sidecar_rejected`, bounded subcode `mcp_status_invalid`, before provider
readiness. Research 297 freezes the exact source, tuple, capsule, PR, review,
merge, and closeout identities. No prompt, tool dispatch, permission callback,
control, retry, reconnect, or respawn occurred; cleanup was confirmed. The
exhausted-credit state was not reached and no quota inference follows.

The outcome exposes a producer contradiction: exact SDK `0.3.259` permits
optional metadata on MCP-status rows, while the strict sidecar rejects part of
that declared shape and the fake SDK does not exercise it. Card 146 owns exact
artifact reconciliation and the smallest safe provider-free repair. The
credited live suite remains held until Card 146 completes, the operator
confirms top-up, and separate live authority is granted. Both Contract 061
cells remain unqualified; no candidate, tag, release, or broader acceptance
follows.

## Claude Repaired Zero-Credit Diagnostic — 2026-09-08

Card 146 confirmed and repaired the SDK `0.3.259` MCP-status projection
mismatch. The operator confirmed credit remains zero and authorized Card 147:
one open against exact source-linked Swallowtail `0d120067`, using
`claude-sonnet-5`, with no prompt, turn, tool, permission callback, control, or
retry. Successful open closes immediately. Every outcome keeps both Contract
061 cells unqualified and carries no candidate, tag, or release authority.

Card 147 returned a successful ready/connected open and immediate `Degraded`
close with joined reapers and zero survivors. Research 298 freezes the result.
Card 148 completed through Desktop PR 175 and repaired the proof oracle
provider-free without changing the capsule. The operator confirmed the balance
remains zero and authorized Card 149 as the fourth and final diagnostic attempt:
prove a separate one-turn mode offline, then run one open and exactly one
`claude-sonnet-5` prompt. No retry, tool/control action, qualification, top-up,
candidate, tag, or release follows automatically.

Card 149 then stopped before provider readiness: the single open returned
`mcp_server_failed`, and the prompt was never submitted. Research 299 freezes
the capsule and treats this as a material registered-MCP startup defect. Card
150 owns provider-free diagnosis across courier build, immutable artifact
acquisition, provider spawn, authenticated connect, and MCP-status projection.
The leading mutable-target race is a hypothesis from Card 139's measured
mechanism, not yet the Card 149 cause. No further live attempt or top-up is
authorized by this repair.

## Claude Card 318 Return — 2026-09-09

Desktop's funded four-control suite passed against exact source-linked
Swallowtail `24f88fb8`. Research 301 freezes the exact SDK/native/Node/sidecar,
transport, MCP, model, capsule, courier, task, PR, review, merge, closeout,
dispatch-count, correlation, and cleanup identities. Allow dispatched the
registered tool exactly once and correlated its result; Deny, cancellation,
and stale/foreign callback rejection dispatched zero times. No retry,
fallback, reconnect, or respawn occurred.

Card 153 consumes this evidence provider-free and may qualify only the exact
`registered_tools` and `consumer_tool_exchange` projections. No further live
Route116/118 read-only evidence preparation is ready now under the same retained
route owners. It must not wait for the evidence it is tasked to obtain. Runtime
adoption remains gated as recorded in the folded ledger above; negative evidence returns to
Chatterbox without inventing support or closing the full integration goal.

## Reconciliation Against The Desktop Matrix — 2026-09-07

The coordinator's read-only capsule at `cbad15a7` reconciled the Desktop
pre-release matrix (18 cells). Merged: Claude permissions, model
inventory/selection, cancellation, packaging; Codex native registered tools,
answerable typed user-input exchange with observe-only approvals (corrected
2026-09-07 by card 138; Research 293), model catalogue/selection, cancellation,
packaging; Grok one-shot permissions, negotiated model options,
turn/permission cancellation, packaging. Absent and promoted as seams: Claude
registered-tool route binding (card 125), Claude and Codex selected-skill
bundle binding (cards 126, 127). Withheld and BLOCKING the release scope (the operator requires all three
routes including MCP/tools/skills): Grok registered tools/MCP (row
`Unavailable / provider_route_evidence_absent`; card 128 built the probe
harness that settled it; the native route is the fallback decision), Grok
selected-skill bundle (`Unavailable / route_dimension_unsupported` until an
ACP labelled-input surface is evidenced, which the same probe records), Grok
pre-session catalogue (`Unavailable / provider_catalogue_unavailable`;
negotiated options at session open are the available form), persistent permission grants on every route
(`Unavailable / one_shot_only`), Codex provider-direct MCP and Claude
HTTP/SSE/in-process/managed MCP (withheld by contract). Desktop-side
corrections returned with the capsule: `PreparedRegisteredToolBinding::open`
not `RegisteredToolPreparation::open`; courier selection binds through
`RegisteredToolSelection::with_proxy_recipe`; Grok and Codex answers go through
`CallbackResponder::respond`, not a `respond_permission` API; Desktop's
`ObservedOnly` declarations on Codex and Grok are its own ceiling, not the
producer's.

### Acceptance Ledger — 2026-09-07

Two different counts, never conflated: producer seams merged on `main`
(14/18 cells, plus 125-127 since delivered) versus Desktop
live acceptance of a cell (0/18 on 2026-09-07; Desktop is the sole integration/test
owner and its runs happen under its own isolated-testing authorization, which
also covers the Swallowtail real-route gates so no separate Swallowtail live
lane exists). No tag readiness is claimed until the full matrix is Desktop-
accepted or the operator changes scope.

### Card 129 Backlog Disposition — 2026-09-07

Ranks 1, 3, 4 (Claude registered tools, consumer-tool exchange, skill bundle):
cards 125 and 126, since delivered. Ranks 7, 8, 10, 11 (Grok): card 128's probe
gated them; the fallback is the operator's native-route decision. Ranks 5
and 12 (Codex `client_mcp_servers`): card 131 settled limitation versus gap;
card 114 is complete and is not a valid reference. Ranks 2, 6, 9, 13
(persistent permission grants): g05.041 stays a planned producer seam; in
the meantime the one-shot permission exchange is answerable on the Claude and
Grok routes, while on Codex the answerable one-shot exchange is typed
user-input only and approvals are observed, then stop the turn (card 138;
Research 293), so a consumer may implement persistence as its own policy over
`CallbackResponder::respond` only where that permission exchange exists, and
Claude's `permissionMode` already exposes the provider's own durable modes.
Desktop is told so; the correction capsule is Research 293.

## Batch A Planning Acceptance — 2026-09-07

Independent read-only planning review accepted the revised design after fixes
for explicit opt-in/dispatch APIs, live admission linearization, retained cleanup
ownership on timeout, and immediately ready route evidence preparation. The
requested dispatch(call, context) signature is present;116/118 status headers
separate ready research from gated runtime. No operator product decision remains.
The first canonical implementation frontier was 114; 115-118 followed the retired manifest (all complete).
Doctor reported existing oversized-file and graph/generated-source findings;
those are orientation findings, not evidence of new runtime validation.

Validation limitation: qa:northstar and diff checks pass. qa:docs reached the
pre-existing former-card-087 stopped-status/index grammar defect. The g05.038
migration owns the bounded checker/index repair; card 114's dispatch waited for that mechanical gate,
not another product or architecture decision.
