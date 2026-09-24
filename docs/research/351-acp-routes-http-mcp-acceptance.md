# 351 ACP Routes HTTP MCP Acceptance Evidence

Status: complete; evidence only; no claim, available cell, admission, or
provider change
Owner: Tom
Date: 2026-09-24
Lane: g06.031

## Question

For each remaining ACP route, does the **provider** accept a
consumer-supplied streamable-HTTP MCP entry on `session/new` — which forms,
with headers, under which gates? Research 337 is the method. The answer
tells Chatterbox which wiring tasks to compile; it does not move a matrix
cell to available.

## Headline Result

Five routes are **`direct-http-candidate`**: `claude-agent.acp` `0.79.0`,
`copilot-cli.acp` `1.0.80`, `gemini-cli.acp` `0.59.0`, `goose.acp` `1.50.1`,
`kiro.acp` `2.21.4`. Two are **`provider-limitation`**: `cline.acp` `3.0.55`
and `deepagents.acp` `0.1.30`. `grok-build.acp` `1.0.41` stays
**`carrier-required`**: stdio is proven; inbound ACP HTTP mapping is not.

This is provider capability from hashed artifacts. Swallowtail still sends
`mcpServers: []` on every listed ACP route except the Grok stdio courier.
No cell becomes `Yes`. Live honouring is unproven on every candidate.

## Method

Frozen artifacts only. npm/GitHub/installer archives were SHA-256 hashed and
extracted in `/tmp`; downloaded executables were never run. No prompt,
login, ACP initialize, session, install, or host update. Citations are
shipped source (or strings in a hashed binary when source is unpublished).

Identities reuse the committed fixtures: Claude Agent ACP
[0.79.0](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.79.0/identity.json),
Copilot
[1.0.80](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-1.0.80/identity.json),
Gemini CLI
[0.59.0](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.59.0/identity.json),
Goose [Research 328](./328-goose-acp-1-50-1-failure-binding-reopen.md) /
`1.50.0` fixture, Kiro
[2.21.4](../../crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.21.4/identity.json)
(`kirocli-aarch64-linux.tar.xz`
`f582eac0e002b4d11bbd061d41fcb49d1d37626a2c1fe230373fcbf97755df6f`;
`kirocli/bin/kiro-cli-chat`
`13a8a907f86f9ff482990cb7aab22848d591de923427224fff3c851e7ab157f6`),
Deep Agents
[0.1.30](../../crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.30/identity.json),
Cline
[3.0.55](../../crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/identity.json)
(`apps/cli/src/acp/acpAgent.ts`
`248092d41e330ef1898f98b99d35c6713574a7b9305d95601177c07e64db9e71`),
Grok
[1.0.41](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.41/identity.json)
(darwin-arm64 executable
`9c844eb13365180787d9ad22b2b3748a024be8e1ed845253cc114781b31c591d`).

## Classification

| Route | Point | Class | `mcpCapabilities` | Accepted forms | Headers | Gate |
| --- | --- | --- | --- | --- | --- | --- |
| `claude-agent.acp` | `0.79.0` | direct-http-candidate | `{ http: true, sse: true }` | `stdio`, `http`, `sse` | `headers` name/value → record | none on the map |
| `copilot-cli.acp` | `1.0.80` | direct-http-candidate | `{ http: true, sse: true }` | `http`, `sse` only | name/value list forwarded | stdio client entries rejected; name conflict with agent-configured servers skipped |
| `gemini-cli.acp` | `0.59.0` | direct-http-candidate | `{ http: true, sse: true }` | `stdio`, `http`, `sse` | `requestInit.headers` | `authRequired` when no selected auth; HTTP maps to `httpUrl` |
| `goose.acp` | `1.50.1` | direct-http-candidate | `http(true)` only | `stdio`, `http`; SSE rejected | `HttpHeader` → StreamableHttp `headers` | SSE: `"SSE is unsupported, migrate to streamable_http"` |
| `kiro.acp` | `2.21.4` | direct-http-candidate | initialize advertisement unproven (bundled schema default `{http:false,sse:false,acp:false}`) | `stdio`, `http`, `sse` via `convert_stdio` / `convert_http` / `convert_sse` | `RemoteMcpServerConfig` carries `headers` | `"Dropping session-injected MCP servers: MCP disabled by governance"`; ACP entries override agent-config servers |
| `grok-build.acp` | `1.0.41` | carrier-required | not cited from initialize | stdio proven live on exact `1.0.4`/`1.0.5` (Research 295); `McpServerHttp` is on the wire enum | unproven on ACP inbound | Swallowtail courier is stdio `name`/`command`/`args`/`env` only |
| `cline.acp` | `3.0.55` | provider-limitation | not advertised | stores `params.mcpServers`; never connects | n/a | n/a |
| `deepagents.acp` | `0.1.30` | provider-limitation | `{ http: false, sse: false }` | `handleNewSession` ignores `mcpServers` | n/a | n/a |

## Per-Route Evidence

### `claude-agent.acp` `0.79.0` — direct-http-candidate

GitHub tag source `src/acp-agent.ts` (tarball SHA
`8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb`;
npm `dist/acp-agent.js`
`e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c`).

`initialize` advertises `mcpCapabilities: { http: true, sse: true }`
(`src/acp-agent.ts:2109-2112`). `session/new` maps `type === "http" ||
"sse"` to `{ type, url, headers: Object.fromEntries(name/value) }` and
untagged stdio to `{ type: "stdio", command, args, env }`
(`src/acp-agent.ts:7851-7874`). A test declares
`{ name: "linear", type: "http", url: "https://mcp.linear.app/mcp", headers: [] }`.

Swallowtail still sends `mcpServers: []`
(`crates/swallowtail-adapter-claude-agent/src/connection.rs`). The matrix
cell stays `No` / `provider_limitation`.

### `copilot-cli.acp` `1.0.80` — direct-http-candidate

Hashed darwin-arm64 `app.js`. `initialize` advertises
`mcpCapabilities: { http: true, sse: true }`. Helper `hlt()` accepts only
`type === "http" || "sse"` with a URL; stdio is
`"Rejecting non-http/sse MCP server"`. Headers: `n.headers` name/value list
→ record. `startSessionMcp` merges client servers unless the name already
exists on the agent-configured set.

Stdio is **not** a client-accepted form on this route. Production HTTP is
representable without a carrier.

### `gemini-cli.acp` `0.59.0` — direct-http-candidate

Tagged source `packages/cli/src/acp/acpRpcDispatcher.ts:98-101` advertises
`mcpCapabilities: { http: true, sse: true }`.
`acpSessionManager.ts:295-325` maps `http` → `httpUrl`, `sse` → `url`,
headers via `Object.fromEntries`. `mcp-client.ts:1678-1686` puts
`config.headers` on `requestInit.headers` (SSE path; HTTP transport uses
the same helper at `createTransportRequestInit`).

Gate: `acpSessionManager.ts:243-244` throws `authRequired` when no
`selectedAuthType`. Headless on the composite row still disables MCP; this
finding is ACP-only.

### `goose.acp` `1.50.1` — direct-http-candidate

`crates/goose/src/acp/server.rs:1845` `.mcp_capabilities(McpCapabilities::new().http(true))`
(SSE not advertised). `mcp_server_to_extension_config` (`server.rs:503-544`)
maps `McpServer::Http` to `ExtensionConfig::StreamableHttp` with
`headers: http.headers` name/value; `McpServer::Sse` returns
`"SSE is unsupported, migrate to streamable_http"`. The same SSE reject is
in `server/extensions.rs:337-339`. Stdio maps to `ExtensionConfig::Stdio`.

### `kiro.acp` `2.21.4` — direct-http-candidate

No published ACP mapping source. Hashed `kiro-cli-chat` (never executed).

ACP schema union `W9` is `type:"http"` (`name`/`url`/`headers[{name,value}]`),
`type:"sse"` (same), `type:"acp"` (`serverId`), untagged stdio
(`name`/`command`/`args`/`env`). Symbols:
`chat_cli_v2::agent::acp::mcp_conversion::{convert_mcp_server,convert_http,convert_sse,convert_stdio}`,
`AcpSessionBuilder::session_injected_mcp_servers`. `RemoteMcpServerConfig`
includes `headers`. Strings: `"ACP MCP servers override existing servers in
agent config"`; `"Dropping session-injected MCP servers: MCP disabled by
governance"`; `"Failed to convert MCP server, skipping"`
(`session_manager.rs:1374`). Bundled ACP SDK defaults
`mcpCapabilities` to `{http:false,sse:false,acp:false}`; the agent's
initialize advertisement is not a cited literal `true`.

Headers: mapped onto `RemoteMcpServerConfig.headers`. Transport-level
forwarding is that struct, not a reqwest `requestInit` line.

### `grok-build.acp` `1.0.41` — carrier-required

Live stdio courier stands on exact `1.0.4`/`1.0.5` (Research 295/336). At
`1.0.41` the binary contains `McpServerHttp` (4 elements) + `HttpHeader`,
`parse_acp_mcp_servers`, `to_acp_mcp_server`,
`McpServerTransportConfig::StreamableHttp`, and `McpHttpClient`. Native
config HTTP uses `url` + `bearer_token_env_var`, not an ACP headers list.
No cited mapping from inbound `session/new` `McpServerHttp` to
`StreamableHttp`. Schema presence is not acceptance (Research 337 oracle).

The matrix `Yes` stays the stdio courier. HTTP remains unproven on this
wire.

### `cline.acp` `3.0.55` — provider-limitation

`apps/cli/src/acp/acpAgent.ts`: stores `mcpServers: params.mcpServers` on
`SessionState` (`:201`) and never reads it. `ensureSessionManager` /
`buildConfig` (`:675-794`) pass no MCP servers into `createCliCore`. No
`mcpCapabilities` on initialize. Settings-file MCP elsewhere is not the
ACP client seam.

### `deepagents.acp` `0.1.30` — provider-limitation

`dist/cli.js:688-691` advertises `mcpCapabilities: { http: false, sse: false }`.
`handleNewSession` (`:709-747`) never reads `params.mcpServers`. The token
`mcpServers` is absent from `dist/cli.js`.

## Proposed Wiring Outlines (not applied)

Chatterbox compiles implementation tasks. This lane edits no route code.

| Route | Emit on `session/new` | Avoid | Live gate |
| --- | --- | --- | --- |
| `claude-agent.acp` `0.79.0` | `{ type: "http", name, url, headers: [{name,value}] }` | empty list | one loopback streamable-HTTP tool call |
| `copilot-cli.acp` `1.0.80` | `http` or `sse` with unique name | stdio client entries; names that collide with agent-configured servers | same |
| `gemini-cli.acp` `0.59.0` | `http` → `httpUrl` plus headers; complete `authenticate` first | headless MCP-disabled path | same |
| `goose.acp` `1.50.1` | `McpServer::Http` with headers | SSE | same |
| `kiro.acp` `2.21.4` | `type: "http"` with headers; governance must allow MCP | injecting while MCP is governance-disabled | same |

Contract 063 already admits the consumer-supplied URL+header placement.
Live honouring is still required before any cell becomes available
(Research 349 is the OpenCode precedent).

## Matrix Reconciliation

No available cell moves.

- `cline.acp` and `deepagents.acp` `client_mcp_servers`: `producer_gap` →
  `provider_limitation`. Basis:
  [351 TSV](./351-acp-routes-http-mcp-acceptance.tsv). g06.005 no longer
  owns a seam those providers lack.
- `copilot-cli.acp`, `gemini-cli.acp + gemini-cli.headless`, `goose.acp`,
  `kiro.acp`: stay `producer_gap` citing g06.005. Notes record the
  direct-http-candidate finding. Swallowtail still emits `[]`.
- `claude-agent.acp`: stays `No` / `provider_limitation`. 290 basis retargets
  to the 351 TSV. Provider would accept HTTP; the adapter pin remains.
- `grok-build.catalogue + grok-build.acp`: stays `Yes` (stdio courier).
  Notes: Research 351 does not prove ACP HTTP at `1.0.41`.

## Typed Gaps

- Live honouring of every `direct-http-candidate`. Reopen: per-route live
  gate in the OpenCode 349 shape. No live packet exists, so cells cannot
  become `Yes`.
- Kiro initialize `mcpCapabilities` literal. Mapping functions prove
  acceptance; the advertised flag is unproven.
- Grok inbound `parse_acp_mcp_servers` → `StreamableHttp`. Reopen: source
  or a stronger binary citation of that match arm. Until then
  `carrier-required` stands.
- Gemini headless MCP remains disabled on the composite row; wiring is ACP
  only.

## Non-Claims

No route code, contract, available cell, login, install, live session,
release, or tag. Research 336's Swallowtail-seam view is superseded for
these eight routes only by the dated addendum; other 336 rows stand.
