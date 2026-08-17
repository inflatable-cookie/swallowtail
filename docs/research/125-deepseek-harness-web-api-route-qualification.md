# 125 DeepSeek Harness Web `/api` Route Qualification

Status: promoted
Owner: Tom
Date: 2026-08-17

## Question

Should Swallowtail add a second DeepSeek Harness route over the Web GUI
host's `/api` RPC and WebSocket mux, and which first subset can it honestly
claim without flattening onto JSON-RPC, ACP, or Open Platform continuation?

## Method

Sources, same product RC as Research 124:

- public repository `deepseek-ai/deepseek-harness` at clone
  `47f943859bef60e4160492346772ded9b24f765a`
- npm `@deepseek-ai/dsh@0.1.0-rc.6`
- published CLI, API gateway, apiproxy, webserver, and browser-trust notes
- isolated probe checkout outside the Swallowtail tree

No `dsh web` process was booted in this record. Qualification is
source-and-docs. Live handshake, method allowlist proof, and Ollama smoke
belong to the implementation tranche. Prompts, credentials, session ids, and
raw transcripts stay out of this file.

JSON-RPC `deepseek-harness.jsonrpc` remains the live-proven one-shot
structured run from Research 124 and g03.069. This record does not reopen
that pin.

## Spawn And Pin

Web `/api` is not the JSON-RPC binary. The host is the Node CLI:

- launch: `dsh web` / `dsh --profile web`
- default bind: `http://127.0.0.1:3080`
- flags: `--port`, repeatable `--trusted-host`, `--patch` for Cordis
- CLI currently rejects `--host 0.0.0.0`

The production runner needs built package and frontend artifacts. Swallowtail
must spawn host-approved `dsh`, not wrap a browser, and not reuse
`dsh-jsonrpc-agent-pkg-macos-arm64`.

Version axis is distinct from `deepseek-harness.runtime-bin`. Pin exact npm
`@deepseek-ai/dsh@0.1.0-rc.6` on a new axis such as `deepseek-harness.web`.
`dsh -V` is launcher metadata. `/api` has no protocol version field;
`host.describe` is not the compatibility axis. Unverified-newer stays off
while the product is an RC.

Host-approved Cordis remains required. Swallowtail still does not ship a
`danger-full-access` default.

## Wire

POST `/api/<method>` carries a JSON-RPC-like envelope
(`type`/`rpcId`/`method`/`payload`). HTTP status is carrier-only (404, 415,
400, 500). Business errors return HTTP 200 with an error branch. Every POST
must declare `application/json` or the carrier answers 415.

Downlink is WebSocket mux (`/api/events.mux`, `/api/events.host`), not the
JSON-RPC stdio `session.event` stream. `GET /api/session.export` is a host
ZIP download of raw session artifacts, not an RPC.

There is no OpenAPI, no bearer token, and no independently versioned client.
Trust is the browser-deputy fence: loopback or `trustedHosts` `Host`,
same-origin `Origin` when present, refuse `sec-fetch-site: cross-site`. That
is not authentication. Swallowtail owned processes bind loopback only.

## Method Map And Allowlist

The unary map includes session, subagent, host filesystem, workspace, skill,
agent-preset, goal, settings, credentials, and llm methods. Credentials and
settings share `/api` with prompt. `session.prompt` can drive bash.
`llm.discoverModels` may carry an `apiKey` on the request envelope.

First Swallowtail allowlist:

- `session.list`, `session.search`, `session.create`
- `session.history`
- `session.models`
- `session.prompt`, `session.cancel`
- `session.fork`
- `workspace.list`, `workspace.archiveSession`
- `host.describe` as bind/liveness metadata only

Hard deny:

- `settings.*`, `credentials.*`, `llm.*`
- `host.pickDirectory`, `host.listDirectory`, `host.createDirectory`,
  `host.openPath`
- `agentPreset.read`, `copy`, `openDocument`, `remove`
- `GET /api/session.export`
- `command.execute`, goals, attachments, queue edits, subagent control,
  `session.selectModel` / `rename` until separately evidenced

Unknown methods fail closed.

## History And Cancel

`session.history` is documented to read an attached session or inspect a cold
log **without resuming or publishing an Agent**, paging on append-origin
message boundaries. That is a Contract 054 candidate. It is not yet
qualified: corpus must prove no Agent resume, no prompt side effect, and no
credential or reasoning leakage into diagnostics.

`session.cancel` aborts the active turn and keeps pending inbox work. Native
cancel exists on this surface. JSON-RPC remains process-kill only.

`session.fork` publishes a child from a turn/end anchor. `workspace.archiveSession`
hides a session from grouping without deleting its log. Neither is restore or
hard-delete.

## Route Decision

Add a second route in the existing package, Kimi-local-server shaped:

- package: `swallowtail-adapter-deepseek-harness`
- family: `deepseek-harness`
- route: `deepseek-harness.local-server`
- driver: `swallowtail.deepseek-harness.local-server`
- transport: owned `dsh web` on loopback HTTP + WebSocket
- version axis: `deepseek-harness.web`
- first qualified point: exact `@deepseek-ai/dsh@0.1.0-rc.6`

Keep `deepseek-harness.jsonrpc` unchanged. Do not merge onto
`deepseek.continuation`. Do not start ACP, headless CLI, or the browser UI.

## First Production Subset

Owned loopback `dsh web` with host-approved Cordis:

- catalogue: `session.list` / `session.search` / `workspace.list`
- control-free history candidate: `session.history`
- one structured prompt with mux events and native `session.cancel`
- fork and archive as native session-lifecycle methods
- session model listing via `session.models`, not `llm.models`

Live proof may use host-local Ollama. That does not qualify
`deepseek-official`.

## Deferred

- ACP stdio
- JSON-RPC session-id continuity on the stdio route
- attachments, queue, subagents, skills, presets, goals, commands
- settings, credentials, llm configuration plane
- host filesystem and directory picker
- session-log ZIP export
- restore, hard-delete, Contract 054 support until history proof
- bearer auth or non-loopback bind
- unverified-newer, version bump, tag, registry

## Contract Fit

No new provider-neutral contract is required to start. Existing owned-process,
prepared-evidence, catalogue, structured-run, archive, activity, and
fail-closed diagnostic contracts already bound the portable roles. The method
allowlist and loopback Host fence are adapter-local, like JSON-RPC Cordis
admission.

Contract 054 stays unsupported until `session.history` proves control-free on
the qualified pin. Contract 036 still requires architecture and route-matrix
honesty when the additive route lands; immutable `v0.3.2` must not be
described as containing it.

## Recommendation

Compile g03.070 for `deepseek-harness.local-server` at exact
`@deepseek-ai/dsh@0.1.0-rc.6`. Keep JSON-RPC. First tranche is allowlisted
catalogue, history, prompt, cancel, fork, and archive. Promote only after a
redacted corpus freezes the pin, fence, and control-free history claim.
