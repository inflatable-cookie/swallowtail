# 154 OpenHands Agent Server 1.42.1 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 287

## Question

Is official PyPI `openhands-agent-server==1.42.1` a distinct owned
loopback HTTP/WebSocket wire that can freeze `openhands.agent-server`
without flattening onto V0 Socket.IO, Contract 035 remote ACP, the
Python SDK `Conversation` class, Docker/hosted sandbox, or
`switch_acp_model`?

## Method

Reconciled Research 153 with official Agent Server docs, GitHub
`OpenHands/software-agent-sdk` tag `v1.42.1`
(`167c1f924ac8a8acbeb0432bf9b1fcf77d5c2497`), PyPI
`openhands-agent-server==1.42.1` (published 2026-08-12), companion
`openhands-sdk==1.42.1`, and the GitHub release OpenAPI asset.

Downloaded the wheel, sdist, and `openapi.json`. Did not `pip install`.
Did not start the server. Did not send `POST /api/conversations`. Did
not open a WebSocket. Did not log in or send a provider prompt. Host
`python3` is 3.9.6; `openhands.agent_server` is absent.

Observed versions are not qualified claims. No production claim in this
record.

## Identity

| Surface | Value |
| --- | --- |
| Route | `openhands.agent-server` |
| Axis (provisional) | `openhands-agent-server.package` |
| Package (provisional) | `swallowtail-adapter-openhands` (create in card 288) |
| PyPI | `openhands-agent-server==1.42.1`, wheel 2026-08-12T14:04:35.031755Z |
| Wheel SHA-256 | `772a73b19684acab5f9f61b1c244f156052625ade51a5e48a424b3c13039f7a7` |
| Sdist SHA-256 | `449a62da7a16ec4cd90c611812b763d6e02ee331b412e09316fa6ab5c289afcc` |
| Requires-Python | `>=3.12` |
| Console script | `agent-server = openhands.agent_server.__main__:main` |
| `Requires-Dist` | `openhands-sdk` **unpinned**; PyPI latest that day is also `1.42.1` |
| Git tag | `v1.42.1` at `167c1f924ac8a8acbeb0432bf9b1fcf77d5c2497` |
| OpenAPI asset | GitHub release `v1.42.1` SHA-256 `d548f9d1589f72f556a849e8382a49f6b836c8b35ad63d70d575aff2066860e9` |
| OpenAPI `info.version` | `1.42.1` (docs YAML snippet saying `0.1.0` is stale) |
| OpenAPI paths | 95 `/api/*` routes; health and WebSockets are **not** in that asset |
| Host | Python 3.9.6 at `/usr/bin/python3`; no `openhands.agent_server`; no `agent-server` binary |

Standalone GitHub binaries (`agent-server-1.42.1-macos-arm64` and
siblings) exist. Not the first bind.

Swallowtail binds a host-approved Python `>=3.12` (or the `agent-server`
console script from such an interpreter). It does not wrap `/usr/bin/python3`
on this host.

## Selected wire

Entrypoint (always pass loopback; never rely on the default):

```
python -m openhands.agent_server --host 127.0.0.1 --port <host-approved port>
```

`--host` defaults to `127.0.0.1` only when no session API key is set.
If `SESSION_API_KEY` / `OH_SESSION_API_KEYS_*` is set and `--host` is
omitted, the process binds `0.0.0.0`. An explicit wildcard without a
key logs a warning. Swallowtail always passes `--host 127.0.0.1`. It
does not bind `0.0.0.0` / `::`.

Health (wheel `server_details_router.py`; no `/api` prefix; no session
key):

- `GET /health` → `{status:"ok"}`
- `GET /alive` → `{status:"ok"}`
- `GET /ready` → `{status:"ready"}` or HTTP 503 `{status:"initializing"}`
- `GET /server_info` → `version` is the `openhands-agent-server` dist
  version; also reports SDK/tools/workspace versions, git sha, and
  usable tools. Native extras stay private.

REST under `/api`. Optional `X-Session-API-Key`. Cookie
`oh_workspace_session_key` is unselected.

- `POST /api/conversations` — `StartConversationRequest`; required
  `workspace` (`kind` const `LocalWorkspace`, required `working_dir`)
- `POST /api/conversations/{id}/run|pause|interrupt` — no body
- `GET/POST /api/conversations/{id}/events` — `SendMessageRequest`
  (`content[]` Text/Image, `role` default user, `run` default false)
- `DELETE /api/conversations/{id}`

WebSocket from the wheel, not the GitHub tree README:

- selected: `/sockets/events/{conversation_id}`
- unmapped: `/sockets/bash-events`
- stale docs path `/conversations/{id}/events/socket` is not the wire

Auth preference: first frame `{"type":"auth","session_api_key":"..."}`;
header `X-Session-API-Key` for non-browser clients; query
`session_api_key` is deprecated and unselected.

Events are JSON objects with a `kind` discriminator (OpenAPI lists 135
consts). Unknown kinds fail-closed. First corpus names
`MessageEvent`, `StreamingDeltaEvent`, `ConversationStateUpdateEvent`,
`PauseEvent`, `InterruptEvent`, `FinishAction`, `AgentErrorEvent`,
`ConversationErrorEvent`, and `ServerErrorEvent`. Native fields stay
private.

`ConversationExecutionStatus`: `idle`, `running`, `paused`,
`waiting_for_confirmation`, `finished`, `error`, `stuck`, `deleting`.
Map `finished` → `end_turn`; interrupt/kill → `cancelled`; `error` →
`error`; `stuck` / `max_iterations` → bounded limit.

First useful op:

1. spawn the owned child with explicit `--host 127.0.0.1`
2. `GET /health` and `GET /ready`; `GET /server_info` version `1.42.1`
3. optional bounded `POST /api/conversations` then run, decode selected
   event kinds
4. join or kill the child (`DELETE` conversation is extra)

Health-only is the smallest no-LLM probe. A conversation needs a caller
LLM credential reference on `agent.llm`. Swallowtail does not log in
and does not run ChatGPT `subscription_login`.

## Authority

Omitting `confirmation_policy` inherits server default `NeverConfirm`
(auto-continue; same class as `--yolo`). Swallowtail must pass
`{"kind":"AlwaysConfirm"}`. It must not pass `NeverConfirm` or
`ConfirmRisky`, and it must not omit the field.

`max_iterations` defaults to 500. Swallowtail must pass a positive
bound. Fixture example `1` is not production policy; the driver maps
the host process deadline.

Do not inherit ambient `SESSION_API_KEY`, `OH_SESSION_API_KEYS_*`,
`OH_SECRET_KEY`, or `LLM_API_KEY`. Swallowtail does not mint session
API keys. Unauthenticated loopback is allowed for the owned child when
bound to `127.0.0.1`. If the host grants a session key, still pass
`--host 127.0.0.1`.

Working resource is `LocalWorkspace.working_dir`. Isolation is the
owned child's ambient-host filesystem. Cleanup is join or kill that
process. Disconnect-only is not the first topology.

## Unmapped on this corpus

V0 Socket.IO `InitSessionRequest`, Contract 035 remote ACP,
`POST .../switch_acp_model`, Python SDK `Conversation` / `Workspace`,
Docker sandbox, hosted API sandbox, `/sockets/bash-events`,
bash/file/git routers, settings/secrets, OpenAI subscription device
login, VSCode/desktop, plugins/marketplace, `RemoteWorkspace`,
`browser_tool_set`, CORS / `0.0.0.0` bind, standalone GitHub binaries,
and attaching to a caller-owned remote server.

## Decision

Admit `openhands.agent-server` as an owned loopback HTTP/WebSocket
child. Freeze identity and named fixtures under
`crates/swallowtail-adapter-openhands/tests/fixtures/openhands-agent-server-1.42.1/`.
Card 288 may create the package and driver. No production claim in this
card. Counts stay 37 packages / 45 production routes.

## Non-goals

- installing OpenHands or starting a live server
- live conversation, login, or LLM key use
- Docker/hosted sandbox, ACP canvas, Python SDK embed
- version-range claims, package creation, matrix edits
