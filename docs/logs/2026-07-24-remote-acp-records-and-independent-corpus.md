# Remote ACP Records And Independent Corpus

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/133-remote-acp-records-and-independent-corpus.md`

## Outcome

Contract 035's provider-neutral records and thirteenth conformance profile are
realized. No production client, provider registration, credential path, or
live endpoint was added.

`swallowtail-core` now carries exact remote transport selection, bounded
connection-private cookie affinity, frame/correlation/stream limits, and
separate ACP wire, RFD, transport SDK, and core SDK evidence. A portable
`Unauthenticated` credential mechanism is distinct from
`LocalUnauthenticated`, topology, compute placement, entitlement, and
metering.

`swallowtail-testkit` adds the `RemoteAcpHarness` profile without widening
process ACP or attached-network profiles. It proves exact experimental access,
network-only transport authority, callback exchange, no implicit recovery or
fallback, and joined work under local and remote-authoritative execution-host
identities.

## Independent Corpus

The raw corpus pins:

- ACP wire version 1
- Active transport RFD as observed 2026-07-24
- `agent-client-protocol-http = 2.0.0`
- `agent-client-protocol = 2.0.0`
- no remote-agent or runtime-interface range claim

HTTP/SSE fixtures cover initialize, connection and session streams, `202`
acceptance, correlation, affinity cookies, cancellation, invalid headers,
disconnect, and explicit `DELETE`. WebSocket fixtures cover the upgrade,
upgrade-response cookie retention, full-duplex correlation, callbacks,
cancellation, disconnect, and explicit joined close.

The fixture parser is test-only and independent of the future production
client. It bounds transcript, record, and count sizes and emits only closed
error variants.

## Evidence Correction

The Active RFD requires clients to accept, store, and return cookies for both
Streamable HTTP and WebSocket. Contract 035, Research 029, roadmap 045, and
cards 133-134 were corrected before implementation. WebSocket therefore uses
the same connection-private bounded affinity posture; it does not gain
reconnect, authentication, or cross-connection cookie reuse.

## Validation

- `cargo test -p swallowtail-core` — 47 passed
- `cargo test -p swallowtail-testkit --test conformance_profiles` — 14 passed
- `cargo test -p swallowtail-protocol-acp --test remote_transport_fixtures` —
  8 passed
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy doctor`
- `git diff --check`

All pass. Doctor remains at the inherited 19 oversized-file findings: 12
warnings and seven errors.

## Continuation

Card 134 is ready. It adds `swallowtail-transport-acp-remote`, keeps maintained
SDK and runtime dependencies private, implements exact HTTP/SSE and WebSocket
selection, and proves bounded connection-private state plus joined cleanup
against the independent corpus.
