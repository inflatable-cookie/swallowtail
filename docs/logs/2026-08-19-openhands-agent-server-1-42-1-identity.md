# 2026-08-19 OpenHands Agent Server 1.42.1 Identity

## Result

Card 287 froze official OpenHands Agent Server identity at PyPI
`openhands-agent-server==1.42.1` without installing, starting the
server, or sending a prompt. The selected wire is
`python -m openhands.agent_server --host 127.0.0.1 --port N`: health
on `GET /health` / `/ready` / `/server_info`, conversations on
`POST /api/conversations`, events on `/sockets/events/{id}`. Not V0
Socket.IO, not Contract 035, not the Python SDK. Swallowtail always
passes loopback host and `AlwaysConfirm`; omitting `--host` with a
session key binds `0.0.0.0`, and omitting confirmation policy inherits
`NeverConfirm`. Named fixtures live under the future adapter tree. No
production claim.

## Validation

- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`

## Next

Implement the OpenHands Agent Server driver core (card 288).
