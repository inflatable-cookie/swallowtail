# 287 OpenHands Agent Server Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../093-openhands-agent-server-route.md`
Depends on: Card 286; Research 153

## Goal

Freeze source and route-shape evidence for OpenHands Agent Server. Candidate route is `openhands.agent-server`. Do not edit production claims.

## Scope

Record official executable/server identity, transport, selected help/schema/event samples, authentication and working-resource authority, topology, persistence, cancellation, cleanup, and overlap with existing routes. Name the smallest deterministic corpus. First topology is an owned local loopback child, not Docker/hosted sandbox or Contract 035 remote ACP.

## Out Of Scope

driver implementation, prepared API, production matrix changes, live provider work, installation, login, and version-range claims

## Acceptance Criteria

- [x] Source identity and route disposition are explicit.
- [x] Route adds material information gain or records why it does not.
- [x] Authority and cleanup boundaries are named.
- [x] No claim changes before driver work.

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if the surface is only a UI/TUI, wrapper/fork without divergence, or hidden remote/provider state is needed to establish the route.

## Auto-Continuation

Continue to card 288 only after route identity is admitted.

## Evidence

Research 154; `docs/logs/2026-08-19-openhands-agent-server-1-42-1-identity.md`;
fixtures under `crates/swallowtail-adapter-openhands/tests/fixtures/openhands-agent-server-1.42.1/`.
`effigy qa:northstar` passed. No production claim.
