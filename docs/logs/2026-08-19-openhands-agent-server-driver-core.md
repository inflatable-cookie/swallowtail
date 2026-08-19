# 2026-08-19 OpenHands Agent Server Driver Core

## Result

Card 288 added package `swallowtail-adapter-openhands` and the smallest
`openhands.agent-server` driver. Discovery is exact
`openhands-agent-server.package` `1.42.1` with claim
`openhands.agent-server.package-window-1` and behavior
`openhands.agent-server.loopback-http-ws-v1`. Spawn is
`python -m openhands.agent_server --host 127.0.0.1 --port 0`. Stdin
closes without a control channel. Live HTTP/WebSocket conversation stays
unwired; fixture events prove decode, terminals, cancellation, deadline,
and joined cleanup. V0 Socket.IO, Contract 035, the Python SDK,
Docker/hosted sandbox, wildcard bind, ambient session/LLM keys, and
`NeverConfirm` stay out.

Current source is 38 packages and 45 production routes. The OpenHands
driver is realized without a prepared facade or production claim.
Immutable `v0.3.2` stays 30 packages and 36 routes.

`effigy validate:focused swallowtail-adapter-openhands` passed (28 tests,
Clippy). No live install, login, or provider prompt.

## Next

Implement the OpenHands Agent Server prepared facade (card 289).
