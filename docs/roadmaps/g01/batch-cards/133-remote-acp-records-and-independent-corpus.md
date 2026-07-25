# 133 Remote ACP Records And Independent Corpus

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../045-remote-acp-transport-proof.md`

## Objective

Realize Contract 035's shared records and freeze an independent remote ACP
corpus before production client work.

## Governing Refs

- Research 029
- Contracts 005, 006, 009, 010, 011, 014, 015, 019, 029, and 035
- roadmap g01.045

## Scope

1. Add a portable unauthenticated credential posture distinct from local
   topology; migrate no existing local route unless its semantics require it.
2. Add bounded provider-neutral remote ACP transport, connection, affinity,
   and version evidence records.
3. Add a thirteenth `Remote ACP Harness` profile without weakening the process
   ACP or attached network-harness profiles.
4. Freeze raw loopback HTTP/SSE fixtures for initialize, connection id,
   connection stream, session stream, `202` acceptance, correlation, cookie
   affinity, cancellation, disconnect, invalid headers, and explicit close.
5. Freeze raw loopback WebSocket fixtures for upgrade cookies, full-duplex
   request, response, notification, callback, cancellation, disconnect, and
   explicit close.
6. Pin `agent-client-protocol-http = 2.0.0`, matching core SDK `2.0.0`, and ACP
   wire version 1 as separate fixture evidence.
7. Add deliberate violations for retry, reconnect, replay, fallback, identity
   drift, unbounded state, diagnostic leakage, and detached cleanup.

## Boundaries

- no production network client
- no integration-family or provider registration
- no live endpoint, credential, login, or provider request
- no authentication abstraction beyond the explicit unauthenticated posture
- no reconnect, retry, replay, resumption, pooling, or transport fallback
- no raw endpoint, cookie, header, id, frame, or SDK error in stable output
- no consumer edit

## Acceptance Criteria

- [x] portable records express every Contract 035 invariant
- [x] the thirteenth profile is additive and provider-neutral
- [x] raw HTTP/SSE and WebSocket corpora do not depend on the production client
- [x] SDK, wire, RFD, agent, and configured-instance versions remain separate
- [x] unauthenticated access does not imply local topology or local compute
- [x] card 134 can implement without a fresh contract or product decision

## Validation

- focused core, runtime, testkit, and ACP protocol tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Evidence Required

- exact dependency and wire pins
- raw fixture transcripts and expected lifecycle assertions
- focused test counts and outcomes
- changed public surface summary

## Stop Conditions

- the maintained client cannot represent one required transport without
  authentication or an implicit fallback
- a generic provider identity becomes necessary
- raw independent fixtures contradict the Active RFD
- the public record shape would expose secrets or raw protocol payloads

## Auto-Continuation

Yes, after every shared record and independent corpus assertion passes.

## Outcome

Completed 2026-07-24.

- `swallowtail-core` now exposes exact remote ACP transport, affinity, bounds,
  and wire/RFD/SDK version evidence plus a portable unauthenticated access
  posture.
- `swallowtail-testkit` exposes the additive provider-neutral thirteenth
  profile under both execution-host identities.
- `swallowtail-protocol-acp` owns an independent raw HTTP/SSE and WebSocket
  corpus pinned to ACP wire version 1 and SDK `2.0.0`.
- Current RFD evidence corrected the WebSocket boundary: upgrade cookies are
  retained in the same bounded connection-private affinity posture as HTTP
  cookies.
- Focused core, testkit, and corpus tests pass: 47, 14, and 8 respectively.
- Workspace all-target checking, warnings-denied clippy, docs QA, doctor delta,
  and diff checks pass as recorded in the closeout log.

Card 134 remains ready without a new contract or provider choice.
