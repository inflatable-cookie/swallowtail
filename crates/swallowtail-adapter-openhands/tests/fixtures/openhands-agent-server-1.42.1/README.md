# OpenHands Agent Server 1.42.1 identity corpus

Secret-free source identity for `openhands.agent-server` before any
Swallowtail package or claim exists.

Official PyPI `openhands-agent-server==1.42.1` (GitHub
`OpenHands/software-agent-sdk` tag `v1.42.1`) freezes an owned loopback
HTTP/WebSocket child:

`python -m openhands.agent_server --host 127.0.0.1 --port <n>`

Health is `GET /health` / `/alive` / `/ready` / `/server_info`.
Conversations are `POST /api/conversations`. Event stream is
`/sockets/events/{conversation_id}`, not the V0 Socket.IO wire, not
Contract 035 remote ACP, and not the Python SDK `Conversation` class.

No live server. No provider prompt. No install. Host has no
`openhands.agent_server` module; system `python3` is 3.9.6 and cannot
run this package (`requires-python >=3.12`).

No fixture contains a credential, host path, account identity, provider
payload, or real conversation id.
