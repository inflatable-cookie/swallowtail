# g05.035 Shared Harness Capability And Producer Boundary

Status: planned; architecture and contract promotion required before implementation
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-07
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
Desktop Spec 010 at `30a338f2` now settles the bilateral ownership split;
Longhorn PR 22 is aligned and independently passed at head `6ce4aa1b`.
Bilateral ownership alignment is complete; Swallowtail promotion gates remain.
Card 084 merged through PR 255 at `8a377c1b` after independent review of
`f52c48c6`. It is merged, untagged stdio-only Claude SDK evidence, not released
`v0.4.3` behavior or centralized bridge delivery. The old branch at `7cb08b1f`
remains archival history. No duplicate Card 084 or producer lane is allowed.

## Execution Plan

- [ ] **Batch A — architecture and contract promotion.** Independently review
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

## Next Planning Gate

Independent review of Research 288 and Spec 014. Longhorn PR 22 is already
aligned and independently accepted at `6ce4aa1b`. Chatterbox may then promote
accepted architecture/contracts and recompile this roadmap into exact ready
batch cards. Implementation authority is currently **no**.
