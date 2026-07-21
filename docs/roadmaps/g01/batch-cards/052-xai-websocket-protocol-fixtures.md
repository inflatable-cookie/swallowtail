# 052 xAI WebSocket Protocol Fixtures

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../017-xai-responses-websocket-proof.md`

## Objective

Freeze the provider-supported xAI Responses WebSocket subset before runtime or
driver implementation.

## Governing References

- `../../../research/005-post-tranche-coverage-evidence.md`
- `../../../contracts/005-integration-identity-and-transport-diversity.md`
- `../../../contracts/006-execution-layer-and-access-boundary.md`
- `../../../contracts/014-hosted-transport-credential-and-evidence-boundary.md`
- `../../../contracts/016-connection-scoped-direct-sessions-and-billed-cost.md`
- `../017-xai-responses-websocket-proof.md`

## Scope

- one fixture-only `swallowtail-adapter-xai` crate
- exact WebSocket upgrade request and bearer-auth boundary
- first and chained text turns with `store=false`
- ordered Responses events, final output, usage, and `cost_in_usd_ticks`
- provider error, invalid continuation, connection lifetime, unknown event,
  disconnect, cancellation-by-close, and frame bounds
- deterministic loopback fixture with no credential or external network

## Out Of Scope

- production driver or shared runtime changes
- model catalogue HTTP
- tools, search, attachments, structured output, warmup, background mode,
  storage, retry, reconnect, resume, or live authentication

## Ordered Steps

1. Recheck the official WebSocket, Responses, authentication, and cost pages.
2. Record one evidence snapshot and bounded protocol manifest.
3. Add client and server frame fixtures for success and each failure boundary.
4. Add a deterministic fake WebSocket endpoint that enforces handshake,
   route, auth shape, turn order, model, `store=false`, and one active response.
5. Prove parser bounds, cumulative evidence replacement, redaction, and no
   second provider frame for concurrent-turn rejection.
6. Mark card 053 ready only when the driver needs no fresh protocol decision.

## Acceptance Criteria

- [x] fixture identity is an evidence snapshot, not a claimed provider API
      version
- [x] continuation ids remain adapter-private and cannot be supplied by the
      caller
- [x] provider-billed cost is exact integer evidence with its USD scale
- [x] cancellation truthfully closes the session; no text-response cancel frame
      is invented
- [x] card 053 needs no fresh event, auth, storage, or reconnect decision

## Validation

- focused fixture and parser tests
- `cargo clippy -p swallowtail-adapter-xai --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `git diff --check`

## Evidence Required

- official source links and access date in the fixture README
- exact supported and excluded event manifest
- focused test count and failure cases
- explicit statement that no credential or external request was used

## Stop Conditions

- current official docs no longer expose the route as provider-supported
- stable event correlation or terminal semantics cannot be frozen
- the fixture requires provider storage or a live API key

## Auto-Continuation

No. Mark card 053 ready after fixture evidence closes the protocol boundary.

## Completion Evidence

- Rechecked the official xAI WebSocket, Responses, authentication, and cost
  pages on 2026-07-20.
- Added an evidence-snapshot manifest, client frames, ordered server frames,
  provider errors, a bounded fail-closed parser, and a deterministic loopback
  WebSocket endpoint.
- Eight focused tests prove upgrade route and bearer shape, serial first and
  chained turns, private continuation insertion, exact billed ticks,
  cumulative-evidence replacement, correlation and order, unknown events,
  limits, disconnect, and cancellation by close.
- Concurrent turn rejection occurs before a second provider text frame.
- `cargo clippy -p swallowtail-adapter-xai --all-targets -- -D warnings`,
  formatting, and diff checks pass.
- Effigy docs and Northstar checks pass. Doctor remains at the pre-existing 19
  oversized-file findings (12 warnings and seven errors), with no xAI file in
  the report.
- No provider credential, installed provider client, or external inference
  request was used. Card 053 is ready; card 054 remains planned and in bounds.
