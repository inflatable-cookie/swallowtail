# 366 Claude Agent SDK Consumer HTTP MCP Placement Evidence

Status: complete; provider-free evidence; no production code, claim, matrix,
admission, or live-provider change
Owner: Tom
Created: 2026-09-29
Lane: shared-harness-producer-boundary

## Question

Can `claude-agent.sdk` on its qualified tuple — Agent SDK `0.3.270`, native
`2.1.270`, Swallowtail sidecar — carry a consumer-supplied streamable-HTTP MCP
entry (absolute URL plus headers) end to end, so Bovine Desktop could reach
Longhorn's Contract 022 `agent-control` server directly as the ACP routes do
under Contract 063? The record separates what the frozen upstream package
accepts from what Swallowtail's binding and sidecar currently forward, names
the redaction constraints, and gives the smallest wiring change or the named
reason it can't.

## Headline Result

The pinned upstream package **accepts** `type: "http"` and `type: "sse"` MCP
server entries carrying `url` and optional `headers`, and passes the whole
`mcpServers` record **verbatim** to the native CLI as `--mcp-config` JSON. A
consumer-supplied HTTP entry is therefore representable on the provider surface
this route already drives.

Swallowtail's admitted seam **cannot forward one today**. `ClaudeAgentSdkMcpBinding`
models stdio only, and the sidecar refuses any entry that carries a key outside
its stdio set (`mcp_servers_invalid`); `sdkMcpServers` always emits
`type: "stdio"`.

Classification, in Research 336/351 terms:

- **Provider (`0.3.270`): `direct-http-candidate`.** Frozen package artifacts
  show the entry shape accepted and forwarded, the same class Research 351
  assigned to five ACP providers from hashed artifacts.
- **Swallowtail admitted seam: `carrier-required`.** Research 336 classified
  what the Swallowtail seam can carry, and that seam is still stdio-only. The
  two statements are not in conflict: 336 classifies the seam, 351 the
  provider. Research 336's `claude-agent.sdk` "SSE/HTTP not representable"
  finding describes `crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs`,
  not the upstream package, and it stands for the seam until wiring lands.

The smallest wiring change that would emit one consumer-supplied HTTP entry is
named under "Smallest Wiring Change". Live honouring (connected, tool listed,
one completed call) remains a separate authorized gate and is not claimed here.

## Method

Frozen artifacts only: the committed `0.3.270` identity and declaration
fixtures, the pinned npm tarball recovered through its recorded reproduce
steps, the committed sidecar source, and the committed adapter. Nothing was
executed; no login, prompt, host update, native launch, or provider session;
no install into any host. The npm tarball was fetched with `npm pack` and its
hashes checked against the frozen identity before reading:

- `@anthropic-ai/claude-agent-sdk@0.3.270` tarball
  `36e86fc13a1ddc8c026dcf5e5bcf72f4107bcbceaff1a45e7f81c5d44d703575`,
  matching
  [`identity.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/identity.json)
  (`official.tarball_sha256`).
- `package/sdk.mjs` `ad98da1735760c234efb02a1bc6aaa3b25911a773d8334fd1a7a1e6cf46e8b31`,
  1,547,458 bytes, matching the same fixture family
  ([`mcp-protocol.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/mcp-protocol.json)
  `source_file_sha256`). `sdk.mjs` is minified, so citations give byte offsets
  from the hashed file, not line numbers.

The tuple has not moved: the adapter binds `0.3.270` / `2.1.270`
([`sdk.rs:84,88`](../../crates/swallowtail-adapter-claude-agent/src/sdk.rs)),
and no currentness step precedes this evidence.

## What The Frozen Package Accepts

**Declared surface.** The committed declaration excerpt is the pinned
`package/sdk.d.ts`; its header records the exact upstream line ranges.

- `McpHttpServerConfig` is `{ type: 'http'; url: string; headers?:
  Record<string, string>; ... }`
  ([`sdk-declarations.d.ts:310-315`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts);
  upstream `sdk.d.ts:1083-1098`).
- `McpSSEServerConfig` is the same shape with `type: 'sse'`
  ([`:293-298`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts);
  upstream `sdk.d.ts:1202-1217`).
- `McpServerConfig` is
  `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfigWithInstance`
  ([`:346`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts);
  upstream `sdk.d.ts:1118-1122`), and `Options.mcpServers` is
  `Record<string, McpServerConfig>`
  ([`:412`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts);
  upstream `sdk.d.ts:1799-1813`).
- The status surface expects URL-bearing servers: `McpServerStatus.config?` is
  documented "includes URL for HTTP/SSE servers"
  ([`:353` and `:374-376`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts);
  upstream `sdk.d.ts:1124-1168,1170`).

**Runtime path in the pinned bundle.** In `package/sdk.mjs`:

- The MCP server-type discriminant set includes `http` and `sse`:
  `U4t=L(()=>V(["stdio","sse","sse-ide","http","ws","sdk"]))`
  (byte offset 1,286,633).
- The bundle's MCP config schema accepts `http` with a URL and a headers
  record, normalizing `streamable-http` to `http`:
  `YQ=L(()=>C({type:V(["http","streamable-http"]).transform(()=>"http"),url:g(),headers:Z(g(),g()).optional(),...}))`
  (byte offset 1,288,209); the sibling `sse` schema is `ZQ` immediately
  before it.
- The header shape the pinned package requires is a JSON object, not a list:
  the declaration types `headers?: Record<string, string>`
  ([`:313`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/sdk-declarations.d.ts))
  and the bundle schema is `headers:Z(g(),g())`, a string-to-string record.
  Contract 063's placement shape and the ACP wire instead carry an ordered
  `[{name, value}]` list, so the two are not interchangeable and the wiring
  must convert.
- The spawn argv builder pushes the whole record with no transport filtering:
  `if(q&&Object.keys(q).length>0)W.push("--mcp-config",me({mcpServers:q}))`
  (byte offset 918,855), where `q` is `Options.mcpServers` (destructured at
  byte offset 916,865) and `me` is a JSON-stringify wrapper
  (`function me(e,t,n){...return JSON.stringify(e,t,n)}`, byte offset
  838,369).

So the provider-neutral config object recorded in `Options.mcpServers` — type,
URL, and headers included — is serialized whole into the native CLI's
`--mcp-config` argument. Swallowtail's sidecar hands that record to
`query()` and intercepts only the spawn call, forwarding the SDK's argument
list unchanged:
`mcpServers: sdkMcpServers(mcpServers)` and `strictMcpConfig: true`
([sidecar `:1493-1494`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)),
`spawnClaudeCodeProcess` to
[`spawnNative` `:818`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs),
which forwards `args`.

**Boundary of this evidence.** The native `2.1.270` binary is not frozen
in-repo (its ~200 MB platform artifacts are recorded only as `manifest.json`
digests), so its `--mcp-config` parser was not inspected. Acceptance above is
the pinned package's declared and bundled surface plus its verbatim forwarding
to that CLI; it is not a direct native-binary citation and not a live
handshake. Native acceptance is settled provider-free as far as the tuple's
frozen artifacts allow, and definitively only by the live gate's
`connected` + `tools/list` + one call.

## What Swallowtail Forwards Today

- **Binding.** `ClaudeAgentSdkMcpServer` carries `name`, `command`, `args`,
  `env_allowlist_keys`, `tools`, and `optional` — no URL or header field — and
  its sole constructor is `stdio`
  ([`mcp.rs:45-90`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs)).
  `ClaudeAgentSdkMcpBinding` is that vector plus the profile
  ([`mcp.rs:213-231`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs)).
  The doc comment states SSE/HTTP are not representable
  ([`mcp.rs:6-8`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs));
  the guide says the same
  ([`claude-agent-sdk-prepared-integration.md:520-534`](../../docs/guides/claude-agent-sdk-prepared-integration.md)).
- **Open params.** `mcp_servers_params` emits only
  `{name, command, args, envAllowlistKeys, tools, optional}` — no `type` and no
  `url`/`headers`
  ([`startup.rs:744-770`](../../crates/swallowtail-adapter-claude-agent/src/sdk/driver/startup.rs)).
  The driver-side shape is `OpenStdioMcpServer`
  ([`mcp.rs:371-397`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs)).
- **Sidecar admission.** `admittedMcpServers` allow-lists exactly
  `["name","command","args","envAllowlistKeys","env","tools","optional"]` and
  throws `mcp_servers_invalid` on any other key
  ([sidecar `:503-557`, key set at `:517`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs);
  code at `:253`), then `sdkMcpServers` always emits `type: "stdio"`
  ([sidecar `:614-626`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)).
  A consumer-supplied `{type:"http",url,headers}` entry is refused before any
  SDK call.

The route's Contract 063 placement row is still the non-production
mediated-stdio courier
([Contract 063 line 309](../../docs/knowledge/contracts/063-registered-tool-operation-bridge.md)).

## Redaction Constraints

Contract 063 makes URL and header values consumer secrets: they never appear in
failures, diagnostics, receipts, activity, logs, matrices, or prepared-plan
fingerprints; a digest or presence flag is the most a projection may carry
([Contract 063 lines 60-69](../../docs/knowledge/contracts/063-registered-tool-operation-bridge.md)).
The ACP route already implements the pattern: a hand-written `Debug` redacts
`sse`/URL/header values
([`mcp.rs:141-159`](../../crates/swallowtail-adapter-claude-agent/src/mcp.rs)),
and the public encoder's `Debug` redacts the emitted list
([`mcp.rs:161-202`](../../crates/swallowtail-adapter-claude-agent/src/mcp.rs)).

The SDK route already satisfies the equivalent constraints for its stdio
evidence and must extend them to the URL and headers:

- Open evidence carries no value. `ClaudeAgentSdkMcpServerStatus` holds only
  `name`, `kind`, and a typed `failure_code`
  ([`mcp.rs:273-307`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs));
  the sidecar projects exactly `{name,status,failureCode}` and discards
  `serverInfo`, `error`, `config`, `scope`, and `tools`
  ([sidecar `:646-689`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)),
  and the Rust side rejects an `error`, `url`, or `config` key on a returned
  row
  ([`startup.rs:772-826`](../../crates/swallowtail-adapter-claude-agent/src/sdk/driver/startup.rs)).
  New URL/header placement code must not add them to that evidence.
- Failures carry a fixed code and a constant message; `mcp_failure` takes
  `&'static str` only
  ([`mcp.rs:518-520`](../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs)),
  so a URL or header value cannot be interpolated into one.
- The values travel only as sidecar stdin bytes: `encode_command` builds the
  record and the pending map stores the command discriminant, not `params`
  ([`wire.rs:511-521`](../../crates/swallowtail-adapter-claude-agent/src/sdk/wire.rs),
  [`connection.rs:80-126`](../../crates/swallowtail-adapter-claude-agent/src/sdk/connection.rs)).
  A prepared-plan fingerprint is not built in this adapter today, so the
  Contract 063 fingerprint rule is a forward constraint on any placement type.
- **Route-specific exposure.** Because the pinned SDK serializes the record
  into `--mcp-config` on the argv it hands to `spawnClaudeCodeProcess`
  (byte offset 918,855 above), a consumer-supplied URL and per-instance bearer
  header would appear in the native child's argument vector on the host. The
  `claude-agent.acp` route carries the same values in the stdio JSON-RPC body
  instead. That difference is a documented consequence of this route's
  provider seam, not a Swallowtail logging leak; the wiring record must state
  it so a consumer can weigh loopback-only URL plus short-lived bearer.

## Smallest Wiring Change

The shape matches the ACP precedent: one route-owned consumer-supplied HTTP
placement, emitted into the same `Options.mcpServers` record the sidecar
already builds, with omission unchanged. The minimal edit set:

1. `crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs` — add
   `ClaudeAgentSdkRemoteMcpPlacement` (`name`, `url`, `headers`, transport
   `Http`/`Sse`), a route-owned reserved name, structure validation (non-empty
   name, absolute `http`/`https` URL, well-formed header names), an encoder to
   `{type:"http",name,url,headers:[{name,value}]}`, that ordered pair list
   being the sidecar-command shape (it matches the ACP wire and lets the
   route detect a repeated header name), and a hand-written `Debug`
   that redacts URL and header values, mirroring
   [`mcp.rs:15-231`](../../crates/swallowtail-adapter-claude-agent/src/mcp.rs).
   `sse` stays modelled and refused, as on the ACP route.
2. `crates/swallowtail-adapter-claude-agent/src/sdk/prepared.rs` — carry
   `Option<ClaudeAgentSdkRemoteMcpPlacement>` on the SDK preparation and add
   `with_http_mcp_placement`, beside `with_mcp_binding`
   ([`:114-117`](../../crates/swallowtail-adapter-claude-agent/src/sdk/prepared.rs));
   reject it together with the registered-only profile, as
   `registered_only_mcp_conflict` does for stdio
   ([`:207-211`](../../crates/swallowtail-adapter-claude-agent/src/sdk/prepared.rs)).
3. `crates/swallowtail-adapter-claude-agent/src/sdk/driver/startup.rs` — add
   the placement to the open params as its own bounded field (or as one typed
   entry in the existing array), include it in the expected-server set, and
   let `mcp_server_status` accept its status row while still refusing
   `error`/`url`/`config`
   ([`:744-826`](../../crates/swallowtail-adapter-claude-agent/src/sdk/driver/startup.rs)).
4. `crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs`
   — admit the placement (new key or new field) in `admittedMcpServers`, and
   have `sdkMcpServers` **convert the ordered `[{name,value}]` header list into
   the pinned SDK's required `Record<string,string>`** before it reaches
   `query()`, emitting `{type:"http",url,headers:{name:value,...}}`. That
   conversion is the one place the two shapes meet: a record cannot carry a
   repeated header name, so a duplicate must fail closed with
   `mcp_servers_invalid` rather than silently keep a last value, and header
   order is not preserved by the SDK shape. Keep `strictMcpConfig: true` and
   the status projection
   ([`:503-626`, `:1493-1494`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)).
5. **Contract 061 projection** — publish the SDK route's
   `consumer-supplied-http` placement beside the existing process-existence
   contribution
   ([`contribution.rs:103-115`](../../crates/swallowtail-adapter-claude-agent/src/consumer_route_projection/contribution.rs)
   is the ACP session analogue; [`contribution/agent.rs:128-166`](../../crates/swallowtail-adapter-claude-agent/src/consumer_route_projection/contribution/agent.rs)
   is the run analogue).

No kernel, host-service, registry, lease, listener, or Contract 022 change is
implied; the sidecar change is harness-side MCP client configuration, the
production role Contract 063 already names. Emitting the entry still proves
nothing about honouring: the feature cells stay unavailable until a live gate
shows one declared entry `connected`, one tool listed, and one call completed,
in the Research 349/361 shape.

## What This Evidence Does Not Settle

- Native `2.1.270` `--mcp-config` acceptance of `type:"http"` is inferred from
  the pinned package's declared and bundled surface plus verbatim forwarding;
  the binary itself was neither downloaded nor inspected.
- Live honouring on the tuple. No `connect`, `tools/list`, call, turn, or
  cleanup claim is made.
- Whether a real native `mcp_status` row for an HTTP server carries `config`
  or `url`; the sidecar discards those keys regardless
  ([sidecar `:646-689`](../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)).
- `needs-auth` remains a typed open failure, so an OAuth-backed remote server
  is out of scope; a consumer bearer header does not need OAuth and stays in
  scope.

## Reconciliation

- Research 336's `claude-agent.sdk` row is unchanged: the Swallowtail seam is
  still stdio-only, and this record supplies the dated addendum that the
  upstream package does declare HTTP/SSE. 336's own scope note already says
  its classification describes the admitted seam, not what a harness does when
  a consumer drives it outside Swallowtail.
- Research 351's method and vocabulary are reused; the SDK route joins its
  `direct-http-candidate` class on the provider axis while its Swallowtail seam
  stays `carrier-required`.
- Contract 063's placement row for `claude-agent.sdk` (line 309) needs no edit
  now: it describes the seam. A wiring task is the successor that would amend
  it.
- No feature cell, guide, matrix, contract, or admission changes. The guide's
  "maps only consumer-declared stdio servers" statement
  ([`:520-534`](../../docs/guides/claude-agent-sdk-prepared-integration.md))
  remains true of current code.

## Non-Claims

No production code, adapter, sidecar, contract, matrix, ledger, admission,
claim, release, or tag change. No provider login, prompt, install, host
update, native launch, or live session. The npm tarball was fetched and hashed
but never executed; no platform package was fetched. This record is the
provider-free placement answer and the wiring outline; it is not a capability
claim for `claude-agent.sdk`.
