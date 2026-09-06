# g05.035 Shared Harness Capability And Producer Boundary

Status: planned; architecture and contract promotion required before implementation
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Depends on: Research 288; Spec 014; Contracts 012, 017, 019, 028, 029, 037, 041, 047, 051, 057, 058, 060-062; g05.029 card 084 evidence
Vision tags: consumer integration, tools, MCP, skills, context, permissions, sessions

## Purpose

Deliver one provider-neutral registered tool/server boundary and route-exact
Claude, Codex, and Grok adoption. Reuse current registry, prepared-plan,
host-service, callback, permission, projection, session, and cleanup machinery.
Do not flatten provider behavior or move Longhorn/Desktop semantics into
Swallowtail.

## Current State

Research 288 preserves the released capability audit. Spec 014 records the
operator-confirmed ownership split and first concrete architecture. No durable
architecture or contract has been promoted from the spec. Implementation is
blocked. Existing Desktop baseline and rehearsal lanes continue independently.

## Execution Plan

- [ ] **Batch A — architecture and contract promotion.** Independently review
      Spec 014; answer the three cross-repo questions; promote repository
      ownership, registered capability, server lease/transport, tool kind and
      Allow/Deny, skill/reference, and projection rules. Recompile this roadmap.
- [ ] **Batch B — shared kernel and host conformance.** Add the provider-neutral
      registration snapshot, schema namespace/digest, host service, server
      lease, stdio and private HTTP/SSE transport, lifecycle state machine,
      safe diagnostics, projection rows, and exhaustive provider-free race and
      teardown fixtures. No adapter change.
- [ ] **Batch C — Claude routes.** Finish exact Claude SDK client-MCP evidence
      and integrate it behind the common lease; retain ACP empty-MCP truth
      unless its separate gate passes; prove options, fresh-session limits,
      Allow/Deny, continuation, skill/reference transport, and no unsupported
      steering claim.
- [ ] **Batch D — Codex and Grok routes, parallel where paths do not overlap.**
      Codex maps the common registry to dynamic native tools and keeps direct
      MCP withheld without evidence. Grok maps exact one-shot permission and
      provider-tool observation; consumer tools/MCP remain withheld unless its
      surface gate passes.
- [ ] **Batch E — real acceptance and consumer sequence.** Longhorn supplies a
      disposable server; Desktop binds context, skills, references, and product
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
| E1 Longhorn | disposable server and schema contract | provider/session authority |
| E2 Desktop | context/skill selection and UX integration | Swallowtail or Longhorn policy ownership |

Batch C, D1, and D2 can run in parallel after Batch B when their contract and
route-evidence gates pass. Other production routes reuse Batch B later and do
not delay Claude/Codex/Grok.

## Goals

- [ ] one central registered capability and server lease replaces no existing
      registry and introduces no generic executor
- [ ] native client, MCP, app, and provider-owned tools retain exact identity
- [ ] every call and result remains bound to task, session, turn, attempt,
      registration, transport generation, deadline, and cancellation
- [ ] reconnect never replays mutating work and stale callbacks fail closed
- [ ] exact Allow/Deny behavior is documented per Claude, Codex, and Grok route
- [ ] skill/reference transport composes Contract 062 without claiming model
      compliance
- [ ] queue UX stays consumer-owned; mid-turn steering stays withheld until a
      route passes Contract 028

## Acceptance Criteria

- [ ] Batch A promotions are accepted in independent review before Batch B is
      marked ready
- [ ] shared provider-free conformance proves every lifecycle and security
      counterexample in Spec 014
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

Batch A uses `effigy qa:docs` and `git diff --check`. Later cards name exact
package scopes for `effigy validate:focused` and
`effigy package:verify-affected`; shared Batch B includes core, runtime,
testkit, and host-local. Real provider acceptance is opt-in, separately
authorized, one route at a time, after deterministic conformance passes.

## Stop Conditions

- a required architecture or contract rule remains only in Spec 014;
- Longhorn or Desktop authority is assumed without a recorded answer;
- a route needs raw credentials, paths, environment, broad shell, ambient
  configuration mutation, or unrelated client content;
- provider-direct MCP cannot preserve the common lease and exact per-call
  admission boundary;
- a Deny cannot be represented without simulated provider acknowledgement;
- reconnect requires mutating replay; or
- a batch would alter release, provider, global configuration, or consumer
  state without separate authority.

## Next Planning Gate

Independent review of Research 288 and Spec 014. Chatterbox then records the
three cross-repo answers, promotes accepted architecture/contracts, and
recompiles this roadmap into exact ready batch cards. Implementation authority
is currently **no**.
