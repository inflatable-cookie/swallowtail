# 298 Claude SDK Zero-Credit Degraded Cleanup

Status: complete; promoted into g05 card 148
Owner: Tom
Created: 2026-09-08

## Finding

Desktop Card 312 reached a successful `claude-agent.sdk` open while the
operator-confirmed account remained at zero credit. Readiness was `ready`, MCP
status was `connected`, the courier was authenticated before readiness, and a
session identity was returned. No prompt or turn ran, so this proves only that
credit exhaustion was not enforced or surfaced at open. It says nothing about
the first prompt turn.

Immediate close returned `CleanupOutcome::Degraded`. Task reapers joined and
zero registered-tool leases, operation-bridge listeners, or survivors
remained. That exact shape is the Contract 019 route-qualified ordinary macOS
posture: confirmed root-only completion after the descendant termination
attempt is `Degraded`; only attested `OwnedTreeEmpty` may be `Clean`.

Desktop's diagnostic runner accepted only `Clean | NotApplicable`, so it
collapsed the expected `Degraded` posture into
`diagnostic.cleanup_unconfirmed`. This is a consumer proof-oracle mismatch.
The immutable capsule remains truthful and must not be rewritten.

## Frozen Identity

Desktop task `22fa30cb-a2da-4120-94cb-be260c433923`; PR 174 accepted head
`a935727e3f25e7df0b11325bd60ba5b7d0fba5da`; independent review comment
`5591728802`; merge `7646db459d3fb8f194729a46b56acdfa49d41fb2`; canonical
closeout `1a8652b64c22415e6c3a9499f8cd2b7d434816af`.

Capsule SHA-256:
`9f6af32bc0011e01f2d84071c5990a9a657e660b9d90d31e802f13755799cf33`.
Exact source-linked Swallowtail:
`0d120067cd260b1f5127835cab0b1a3ad020a29d`.

The tuple remains SDK `0.3.259`, native `2.1.259` Darwin arm64, Node
`22.23.2`, sidecar `0.4.4`, MCP `2025-11-25`, `claude-sonnet-5`, default
permission, persistence false, and mediated stdio/private loopback.

## Disposition

Card 148 repairs the Desktop provider-free proof oracle. It must retain
`degraded` exactly, distinguish it from `Clean`, and accept it only as the
already-qualified route/platform posture with the other cleanup invariants
intact. `Failed`, unconfirmed root completion, surviving resources, unjoined
reapers, or dispatcher activity remain defects.

No new live run is authorized. Both Contract 061 cells remain unqualified.
Top-up and the credited registered-tool suite remain separate.
