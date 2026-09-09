# 301 Claude SDK Registered-Tool Live Acceptance

Status: complete; promoted as Card 153
Owner: Swallowtail Chatterbox
Created: 2026-09-09

## Question

Does the existing `claude-agent.sdk` mediated-stdio registered-tool route work
against the exact supported provider tuple with real call/result correlation
and the required negative controls?

## Evidence

Desktop Card 318 ran against exact source-linked Swallowtail
`24f88fb8a1328aa0e85b9c91989962ba32b9c590`. The frozen tuple was SDK
`0.3.259`, native `2.1.259` on Darwin arm64 with digest
`884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898`,
Node `22.23.2`, sidecar source tag
`swallowtail-claude-agent-sdk-sidecar@0.4.4`, private-loopback plus
mediated-stdio, MCP `2025-11-25`, `claude-sonnet-5`, default permission, and
persistence disabled.

Exactly four fresh opens and four prompt turns ran, with no retry, reconnect,
respawn, repeated attempt, or model fallback. The Allow case authenticated MCP
before readiness, dispatched `desktop/reconcile` exactly once with unchanged
`{}`, correlated the fixed `{"ok":true}` result, and completed. Deny,
cancellation while pending, and stale/foreign callback rejection dispatched
zero times. Deny completed; the two adverse controls ended provider-failed as
expected and rejected the stale/foreign callback. Every attempt used Card 315's
immutable courier
`sha256:12db9fe39fe6928c179ee0e83afc96a7005717e0ef23a00fc25300c886f69720`
and Card 313's route-qualified degraded cleanup oracle: reapers joined and no
lease, listener, or process survived.

The immutable capsule is
`docs/proofs/claude-credited-registered-tool-qualification.capsule.json` in
Desktop, SHA-256
`c4de15a8a8b4a47996dc94c9cf7610e56a311a1b52266d156309cf944231c1f6`.
Provider-free validation passed 36/36 and the linked Swallowtail mediated-stdio
suite passed 15/15 before acceptance.

Evidence chain:

- Northstar task `4356b461-cea3-4079-85cf-d9e63a1bb178`
- Desktop PR 181 accepted head
  `18b70c917ccbc551f94596d364bf2f1916cc3065`
- independent review comment `5599408741`
- merge `807f7a3f916605e170b73f25812663fc0d9c295c`
- canonical Desktop closeout
  `03e71e90d5280550c175c85a4ac4202cca7d9da6`

## Decision

The capsule satisfies Card 132's pass branch for the exact tuple it ran.
Swallowtail may replace the route's pending real-gate disposition with a
qualified exact-tuple claim and make only the `registered_tools` and
`consumer_tool_exchange` matrix cells available. No broader SDK/native/Node,
model, transport, permission, skill, persistence, or platform claim follows.

## Consequence

Card 153 consumes the evidence provider-free, freezes the identities in route
tests, and reconciles Contract 061, the Claude SDK guide, and the feature
matrix. It authorizes no provider call, Desktop mutation, candidate, tag, or
release action.
