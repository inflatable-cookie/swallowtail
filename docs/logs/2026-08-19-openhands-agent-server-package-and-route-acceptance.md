# 2026-08-19 OpenHands Agent Server Package And Route Acceptance

## Result

Card 290 deferred `openhands.agent-server`. Identity, driver, and
`prepare_openhands_agent_server` stay on `swallowtail-adapter-openhands`.
Live HTTP/WebSocket conversation stays unwired; `start_run` fail-closes
with `swallowtail.openhands.agent_server.live_http_unwired` before spawn.
That is not a production conversation route. No guide, example, or
matrix row.

Current source stays 38 packages and 45 production routes. Immutable
`v0.3.2` stays 30 packages and 36 routes. Exact frozen identity remains
PyPI `openhands-agent-server==1.42.1`. Spawn remains
`python -m openhands.agent_server --host 127.0.0.1 --port 0`.
V0 Socket.IO, Contract 035, the Python SDK, Docker/hosted sandbox,
wildcard bind, and `NeverConfirm` stay out.

Host Python `3.9.6` cannot run the `>=3.12` wheel. Live install, login,
and prompt were not justified.

## Validation

- `effigy validate:focused swallowtail-adapter-openhands` (31 tests)
- `effigy package:verify-affected swallowtail-adapter-openhands`
- `effigy check:examples`
- `effigy qa:docs` (45 production routes; no OpenHands guide or example)

## Next

Implement the Kiro ACP identity corpus (card 291).
