# ACP v1 Remote Transport 2.0.0 Fixture

Independent raw loopback evidence for the Active ACP Streamable HTTP and
WebSocket transport RFD.

Pins:

- ACP wire version 1
- transport RFD revision observed 2026-07-24
- `agent-client-protocol-http` `2.0.0`
- `agent-client-protocol` `2.0.0`

The fixture does not use either SDK as its parser or server oracle. Endpoint,
connection, session, cookie, and payload values are synthetic private
placeholders.

Covered:

- HTTP/2 initialize, connection and session SSE streams, 202 routing,
  callbacks, cancellation, cookie affinity, invalid headers, disconnect, and
  DELETE close
- WebSocket upgrade cookies, initialize-first ordering, full-duplex callbacks,
  cancellation, disconnect, and explicit close
- no retry, reconnect, replay, resumption, pooling, multiplexing, redirect, or
  transport fallback

Excluded:

- live network access
- authentication
- provider, model, agent, or configured-instance support claims
- a runtime interface-version range
