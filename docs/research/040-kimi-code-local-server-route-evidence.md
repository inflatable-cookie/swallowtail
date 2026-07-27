# 040 Kimi Code Local Server Route Evidence

Status: promoted
Owner: Tom
Date: 2026-07-27

## Question

Can Swallowtail add a second Kimi Code driver over the local backend used by
Kimi's Web UI, and does that route provide provider-session archive, restore,
or deletion unavailable through Kimi ACP?

## Method

Evidence was checked against current official documentation and the exact
annotated `0.28.1` and `0.29.0` release tags. The two tagged trees were
compared for command lifecycle, REST session routes, WebSocket framing,
authentication, metadata, and persistent server discovery.

No Kimi installation, login, credential, provider request, live server,
workspace mutation, or session effect was used.

## Currentness Correction

Research 028 correctly preferred ACP for the first Kimi proof, but its
description of the Web backend as only a bundled-UI surface is now too
conservative.

Current Kimi Code documentation explicitly describes `kimi web` as a
foreground local server exposing:

- one REST API
- one WebSocket API
- generated OpenAPI at `GET /openapi.json`
- generated AsyncAPI at `GET /asyncapi.json`
- the bundled Web UI from the same origin

This is a documented Kimi Code integration surface. It remains fast-moving and
local-harness-specific, not a stable shared protocol or public hosted API.
Exact release qualification is still required.

Source:
[Kimi command reference](https://moonshotai.github.io/kimi-code/en/reference/kimi-command.html)

## Separate Route Identity

The route must be distinct from Kimi ACP:

| Dimension | Kimi ACP | Kimi local server |
| --- | --- | --- |
| Driver route | `kimi-code.acp` | `kimi-code.local-server` |
| Transport | ACP v1 JSON-RPC stdio | local REST plus WebSocket |
| Startup | `kimi acp` | `kimi web --no-open` |
| Protocol authority | shared ACP | Kimi-maintained OpenAPI and AsyncAPI |
| Access | delegated Kimi login plus process authority | local bearer-token lease plus endpoint and process authority |
| Provider lifecycle | list/load/resume; no close/delete | archive and restore; no hard-delete route |

The same Kimi executable and state root can underlie both routes. That does not
make their driver, transport, access, configured-instance, or management
bindings interchangeable.

## Exact Release Evidence

The existing Kimi executable claim already qualifies two exact points:

| Release | Annotated tag | Peeled source commit |
| --- | --- | --- |
| `0.28.1` | `0032545b65f95c139ecba5a48ba1b911844e1ffe` | `efacf0452d46f5dbd67499eabc053869495d5213` |
| `0.29.0` | `03c34eefa49513e6216390a9773326077a37f414` | `8bf5bacba9e524c38fb808c0122070037ead25a8` |

Both exact trees document and implement the same selected local-server
boundary:

- foreground `kimi web`, with clean `SIGINT` and `SIGTERM` shutdown
- loopback default and `--no-open`
- bearer authentication on REST, WebSocket, OpenAPI, and AsyncAPI
- `GET /api/v1/healthz` as unauthenticated liveness only
- `GET /api/v1/meta` with exact `server_version`, `server_id`, start time,
  backend revision, and declared capabilities
- REST session creation, listing, lookup, prompt submission, abort, archive,
  and restore
- WebSocket protocol version `2`, per-session durable cursors, server hello,
  subscriptions, resynchronization, and bounded event framing
- one persistent `server.token` under the selected Kimi home
- live instance records under the selected Kimi home, including host, port,
  pid, server id, heartbeat, and host version

The exact REST session contract exposes:

- `POST /api/v1/sessions/{session_id}:archive`
- `POST /api/v1/sessions/{session_id}:restore`

It exposes no session hard-delete route. The deprecated
`deleteSessionResponseSchema` name aliases the archive response and must not be
interpreted as deletion.

Sources:

- [`0.28.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
- [`0.29.0` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.0)
- [`0.28.1` REST session contract](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.28.1/packages/protocol/src/rest/session.ts)
- [`0.29.0` REST session contract](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/protocol/src/rest/session.ts)
- [`0.29.0` server session routes](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/routes/sessions.ts)
- [`0.29.0` WebSocket control protocol](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/protocol/ws-control.ts)

## Access And Topology

The first route should support two explicit topologies:

1. attached: the host supplies one approved loopback endpoint and one bearer
   credential lease for an already-running Kimi server
2. owned foreground: the host starts `kimi web --no-open` from one exact
   approved executable, binds one approved loopback endpoint, obtains the
   provider-generated bearer through an opaque credential lease, waits for
   health and metadata readiness, then joins the child on close

Swallowtail must not:

- disable authentication
- expose the token through diagnostics, endpoint URLs, or process output
- scan arbitrary home directories
- assume the first default port
- attach to an unverified sibling instance
- read or mutate session storage directly
- claim filesystem or descendant-process containment
- stop an externally owned server

The server's filesystem and shell reach remain Kimi harness authority under the
selected session access posture. The HTTP boundary does not create a sandbox.

## Provider-Session Management

The local-server route can qualify reversible archive and restore for one
inactive bound session. It cannot qualify deletion.

An ACP-created session may be visible to a server using the same Kimi state
root, but the ACP binding cannot be reused directly. Cross-transport management
requires one explicit consumer-authorized import that proves:

- the same exact Kimi executable release
- the same execution host
- the same Kimi state-root resource identity
- the exact provider session reference
- an authenticated server whose metadata matches the expected release and
  configured instance
- successful target lookup before a new local-server management binding is
  issued

A raw id, filesystem path, list result, or matching provider family is
insufficient.

## Interactive Route

REST plus WebSocket can support a second full Kimi interactive driver:

- create and resume
- prompt and ordered streamed events
- approvals and structured questions
- interruption
- session status and replay cursors
- archive and restore after handle close

This is a separate implementation tranche. The lifecycle proof should land
first because it is smaller, closes the current Kimi gap, and validates
endpoint, credential, version, effect, and owned-process boundaries before the
larger event surface is adopted.

## Recommendation

Add `kimi-code.local-server` as a second driver inside
`swallowtail-adapter-kimi`.

Sequence:

1. exact `0.28.1` and `0.29.0` REST, WebSocket, auth, metadata, and lifecycle
   corpus
2. attached and owned-foreground host composition
3. archive and restore prepared operations
4. explicit ACP-to-server management-binding import and cross-host conformance
5. interactive session driver
6. provider-wide matrix and package closeout

Keep `kimi-code.acp` unchanged and explicitly unsupported for provider-session
management. Keep delete unsupported on both Kimi routes until a maintained
surface documents and qualifies a real deletion operation.

## Promotion

- Contract 038 carries the separate-route, archive/restore, import-authority,
  and no-delete rules.
- System architecture records the contracted local-server route and same-crate
  driver boundary.
- Roadmap g02.020 and cards 061-065 sequence corpus, lifecycle, import,
  interactive execution, and acceptance.
