# 2026-07-20 Gemini ACP Driver

## Outcome

Card 046 is complete. Swallowtail now has a production ACP v1 transport, a
host-owned read-only working-resource callback port, and a pinned Gemini CLI
`0.51.0` interactive driver.

## Realized Boundary

- common ACP code owns bounded NDJSON framing, JSON-RPC message
  classification, correlation-safe request encoding, and explicit errors
- the Gemini adapter owns agent version, access profile, process arguments,
  Plan Mode, session and update mapping, permission behavior, and exclusions
- the runtime owns the `WorkingResourceIo` service identity and redacted read
  request/content wrappers
- the local host owns canonical path resolution, filesystem access, line and
  byte bounds, traversal rejection, and symlink rejection
- prompt and protocol work run only through scoped joined tasks; session or
  turn close stops the owned process when native cancellation cannot settle

## Access Posture

The first instance accepts only the preflight-bound Developer API-key profile
and one host-approved isolated environment. It never calls ACP
`authenticate`. Consumer membership, interactive login, Vertex, gateways,
ambient provider state, resume, MCP, writes, terminals, and mode/model mutation
remain outside the driver.

## Validation

- 5 Gemini adapter mapping and lifecycle-state tests pass
- 10 ACP framing and frozen-fixture tests pass
- 14 local-host tests pass, including canonical read, traversal, and symlink
  rejection
- workspace compilation passes
- no Gemini binary or live credential is required

## Continuation

Card 047 is ready. It adds end-to-end long-lived conformance across local and
remote-authoritative host identities, disconnect and cleanup assertions, and a
separately gated installed/authenticated probe.
