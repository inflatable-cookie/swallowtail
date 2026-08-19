# 2026-08-19 OpenHands Agent Server Prepared Facade

## Result

Card 289 added `prepare_openhands_agent_server` and one owned loopback
structured-run operation on `swallowtail-adapter-openhands`. Preflight
names `swallowtail.openhands.agent-server` and exact
`openhands-agent-server.package` `1.42.1`. Access stays host-owned
`LocalUnauthenticated` with entitlement `Unknown`. Swallowtail does not
bind a credential lease, select a model route, pass `NeverConfirm`, bind
`0.0.0.0`, or flatten V0 Socket.IO, Contract 035, or the Python SDK.
Spawn stays `python -m openhands.agent_server --host 127.0.0.1 --port 0`.
Live HTTP/WebSocket conversation stays unwired; fixture events prove
terminal cleanup against the prepared plan. Missing working-resource
authority, `openhands.acp` axis, and unqualified packages fail before
spawn. Current source stays 38 packages and 45 production routes.

`effigy validate:focused swallowtail-adapter-openhands` (31 tests) and
`effigy package:verify-affected swallowtail-adapter-openhands` passed. No
live install, login, or provider prompt.

## Next

Implement the OpenHands Agent Server package and route acceptance
(card 290).
