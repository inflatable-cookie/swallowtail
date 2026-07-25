# 045 Remote ACP Transport Proof

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: g01.044
Vision tags: reusable integration substrate, transport diversity, explicit authority
Contract refs: 004, 005, 006, 009, 010, 011, 014, 015, 019, 029, 035
Planning state: contract-ready

## Problem

Swallowtail supports ACP only over owned process stdio. The official remote
transport and maintained Rust client now exist, but their connection,
affinity, lifecycle, support-authority, and no-recovery semantics are not
represented by the current twelve conformance profiles.

## Generation Runway

Advance g01's shared-protocol and hosted-transport goals without selecting a
provider or rolling the generation. Roadmap 045 is the final normal roadmap in
the documented 30-50 range; closeout must include a generation-runway
checkpoint before another material lane is compiled.

## Goals

- [x] Realize provider-neutral remote ACP connection records and a thirteenth
      conformance profile.
- [x] Freeze independent HTTP/SSE and WebSocket loopback corpora against ACP
      wire version 1 and the exact maintained Rust SDK.
- [x] Implement one reusable operation-scoped remote ACP transport crate.
- [x] Prove explicit endpoint selection, affinity, callback exchange,
      cancellation, disconnect, and joined cleanup.
- [x] Preserve provider, authentication, version-range, and consumer-policy
      boundaries.

## Non-Goals

- a generic ACP integration family or provider driver
- a provider, model, agent, or endpoint recommendation
- live authentication or a public remote endpoint
- automatic reconnect, retry, replay, resumption, failover, or transport
  negotiation
- connection pooling, multiplexing, a global executor, or detached tasks
- consumer edits

## Contract Coverage

- Contract 005 keeps transport separate from family, driver, instance, and
  route identity.
- Contracts 009, 014, and 019 govern scoped network work, private SDK runtime,
  cancellation, explicit close, and joined cleanup.
- Contract 015 governs ACP wire and callback semantics.
- Contract 029 prevents the SDK pin from becoming an invented interface range.
- Contract 035 governs the remote connection, affinity, maturity, and first
  proof boundary.

## Execution Plan

### Batch 45.1 — Records And Independent Corpus

- [x] Execute card 133.
- [x] Add generic unauthenticated access posture without conflating it with
      local topology.
- [x] Add remote ACP connection and transport evidence records.
- [x] Freeze raw loopback HTTP/SSE and WebSocket fixtures independent of the
      production client.
- [x] Add the thirteenth provider-neutral profile and deliberate violations.

### Batch 45.2 — Reusable Transport

- [x] Execute card 134.
- [x] Add `swallowtail-transport-acp-remote`.
- [x] Embed exact maintained SDK packages behind the shared boundary.
- [x] Implement exact endpoint selection, HTTP cookie affinity, bounded
      correlation, explicit close, and joined operation-scoped work.

### Batch 45.3 — Portability And Closeout

- [x] Execute card 135.
- [x] Prove HTTP/SSE and WebSocket behavior under local and
      remote-authoritative host identities.
- [x] Cross-check maintained SDK server behavior without making it the sole
      oracle.
- [x] Run focused and full QA, promote realized architecture, and close the
      roadmap.
- [x] Reassess the g01 generation boundary before selecting a
      provider-specific remote ACP adapter.

## Acceptance Criteria

- [x] a reusable production transport exists without a generic provider
- [x] all thirteen provider-neutral profiles pass deterministic fixtures
- [x] HTTP/SSE and WebSocket behavior is explicit and independently tested
- [x] support authority stays experimental and opt-in
- [x] authentication and live-network work remain excluded
- [x] no implicit recovery, fallback, or detached cleanup exists
- [x] architecture, contract, roadmap, log, and front-door state agree
- [x] the next planning checkpoint is explicit

## Risks And Mitigations

- SDK client and server sharing could hide defects: retain raw independent
  loopback fixtures as the primary oracle.
- the maintained client is unbounded, lacks HTTP/2 in its reqwest feature set,
  and drops WebSocket upgrade state: use its exact core schema with private
  bounded physical actors for both transports; retain the HTTP crate as a
  cross-check oracle.
- RFD hardening remains incomplete: publish experimental authority and no
  stability or recovery claim.
- SDK cleanup has a drop fallback: drive explicit close and prove joined normal
  cleanup.

## Planning Gaps

None for the unauthenticated shared transport proof. Provider-specific remote
ACP identity, authentication, version qualification, and support authority
remain a later planning checkpoint.

## Evidence Requirements

- exact RFD and SDK source references
- deterministic raw HTTP/SSE and WebSocket transcripts
- focused core, runtime, testkit, protocol, and transport tests
- workspace all-target check and warnings-denied clippy
- full repository QA at closeout
- doctor delta review and `git diff --check`

## Outcome

The twenty-second workspace crate provides one provider-neutral,
operation-scoped client with explicit HTTP/2 SSE or WebSocket selection.
Independent corpora, bounded physical actors, exact private ACP `2.0.0`
schema, maintained-server cross-checks, and the thirteenth conformance profile
pass under both authoritative host topologies.

Full QA inventories 629 tests: 625 pass and four separately gated probes remain
ignored. Doctor remains at the inherited 19 findings. Roadmap 046 and card 136
own the generation-boundary and provider-coverage checkpoint; no provider or
g02 rollover is selected by this closeout.
