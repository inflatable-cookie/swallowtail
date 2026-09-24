# 337 OpenCode ACP 1.18.18–1.18.32 Identity

Status: complete; identity and surface freeze; evidence only; no claim, matrix,
admission, or provider change
Owner: Tom
Date: 2026-09-22
Lane: g06.015

## Question

Does OpenCode's ACP server have a freezable identity and a selected surface the
sibling implementation lane can be compiled against, and does `session/new`'s
`mcpServers` accept a **consumer-supplied** entry — in which transports, and
with what scope? That last answer decides whether `opencode.acp` can reach the
production MCP through Longhorn's stdio carrier or directly.

## Headline Result

**`opencode.acp` is a `direct-http` candidate, not a stdio-only carrier
dependent.** OpenCode's ACP `session/new` accepts a client-declared
`mcpServers` list whose ACP entry may be `stdio` (`name`/`command`/`args`/`env`),
`http` (`name`/`url`/`headers`), or `sse` (`name`/`url`/`headers`); it
advertises `mcpCapabilities: { http: true, sse: true }`, maps an `http` or `sse`
entry to one internal `remote` config carrying the declared URL and headers, and
connects it with a **StreamableHTTP** transport (falling back to SSE) that
forwards the declared headers. The production MCP shape — loopback URL,
per-instance bearer header, stateless streamable HTTP — is therefore
representable on this route without Longhorn's stdio carrier.

This is a provider-capability finding. Whether Swallowtail may expose a
consumer-supplied HTTP MCP placement in production is a contract question, not
settled here (§Typed Gaps).

## Method

Frozen artifacts and published documentation only. npm launcher and platform
tarballs, the GitHub tag source archives for `v1.18.18` and `v1.18.32`, the
selected published-tag source files for every stable `v1.18.19..=v1.18.31`, and
the ACP SDK tarball were downloaded, SHA-256 hashed, and extracted in `/tmp`.
Downloaded executables were never run. The only host CLI use was
`opencode --version` and `opencode acp --help`. No prompt, login, ACP
initialize, session, install, host update, or network-permission action
occurred.

The window is proved per hop, not from a commit-range summary: every selected
published-tag source file was fetched at every stable ref and hashed
independently (`window-ledger.json`), and the two endpoint values were
cross-checked against the downloaded tag archives. The published tags diverge
at some hops (Research 332 records `v1.18.30` → `v1.18.31` diverged), so
selected-file content identity is the authority here, not ancestry.

The installed binary was compared with the official platform package rather
than with source: `~/.opencode/bin/opencode` (143 182 562 bytes) is
byte-identical to `opencode-darwin-arm64@1.18.18`'s `bin/opencode`
(`4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`), so the
installed observation and the official 1.18.18 artifact are one identity.

## Official And Installed Identity

At observation the official channels agree: npm `latest` `opencode-ai@1.18.32`
and GitHub `v1.18.32`. The first unpublished stable is `1.18.33`.

| Version | npm published | npm tarball SHA-256 | GitHub tag commit | source archive SHA-256 |
| --- | --- | --- | --- | --- |
| `1.18.18` | 2026-08-13T01:13:43.814Z | `3e76f908fc945e26c0f305462755c4bee37ee52c221ccb02df6bb2bcbcbe3071` | `31406ccc51b4bd2a4e1e086b2bcaa5f7f804f26d` | `9962680e6ea7b59e002b2940a1f33f31f147fea4e976df2ea5501bc70ed2fb83` |
| `1.18.32` | 2026-09-21T22:50:42.463Z | `454fbb032ade95a21323891138d4b425573f67631e7e888e516da893dc4be8ba` | `545f51d26cc39a907d2867492d498d9607ea5fa4` | `65e95c9a6666ca65bbd17de1e7cecddac1504e66eeebbcfaf5ac68f97e6f392b` |

Platform package `opencode-darwin-arm64`: `1.18.18` tarball
`9d62f72654b27c4fae220ce09fcea4e1364ff52225086feab94a35c6b0c0b4a7` with
`bin/opencode` `4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`;
`1.18.32` tarball
`a1707bb6cc9deaccca501c4097f1580b8af70dfc227f194ae0f576f32abbdebe` with
`bin/opencode` `a3c45d4e1d6620b436851f1ef6b25c71befcf06a382e279a1eb1c2196424395e`.
The npm launcher package ships four files at both endpoints; `LICENSE`,
`bin/opencode.exe`, and `postinstall.mjs` are byte-identical and only
`package.json` changes. npm metadata carries no `gitHead`, so the npm artifact
is release authority and GitHub tags are independent implementation evidence.

The embedded ACP SDK is `@agentclientprotocol/sdk` **0.21.0** at every stable
`1.18.18..=1.18.32` (same `packages/opencode/package.json` dependency pin), and
its `PROTOCOL_VERSION` constant is `1`.

The installed observation is version `1.18.18`, absent from `PATH`, not
installed or updated by this lane, and matching the official artifact exactly.
Its compiled ACP `initialize` response, `mcpConfig`, MCP registration, and
`connectRemote` are the same code as the 1.18.18 source, so the installed
observation agrees with the frozen official surface.

## Window Identity

Nineteen selected source files were hashed at every stable from `v1.18.18`
through `v1.18.32`. Fifteen are byte-identical across the whole window,
including the entire `mcpServers` chain, `initialize`, the entrypoint, session
and permission handling, cancellation, and error mapping. The whole
selected-surface delta is **one hop, `1.18.30` → `1.18.31`**, in three files:

- `packages/opencode/src/acp/service.ts` — `session/load`, `session/resume`, and
  `session/fork` now restore model, effort variant, and mode from the durable
  backing session and message history instead of only message history, and
  `session/set_config_option` plus `unstable_setSessionModel` emit a
  `config_option_update` notification.
- `packages/opencode/src/acp/config-option.ts` — the `default` effort sentinel
  joins the selectable `thought_level` options.
- `packages/opencode/src/acp/event.ts` — a reasoning chunk's `messageId` becomes
  the reasoning part id instead of the message id.

Classification: **compatible extension.** The delta adds one observable
`session/update` frame and rebinds one update field value; it removes no method,
capability, transport, or literal, and it leaves the `mcpServers` handoff
byte-identical. `packages/opencode/package.json` changes at every hop, but only
its version and non-ACP dependency pins; the ACP SDK pin is constant.

## Selected ACP Surface

Entrypoint is `opencode acp` (`start ACP (Agent Client Protocol) server`) over
newline-delimited JSON-RPC on the child's stdin/stdout. It accepts `--cwd`
(default `process.cwd()`), the network options `--port` (default `0`),
`--hostname` (default `127.0.0.1`), `--mdns`, `--mdns-domain`, and `--cors`,
and the global `--print-logs`, `--log-level {DEBUG,INFO,WARN,ERROR}`, and
`--pure`. It sets `OPENCODE_CLIENT=acp` and starts its own loopback OpenCode
server before serving ACP.

`initialize` returns `protocolVersion: 1`, hard-coded — OpenCode does not
negotiate down to the client's requested version — with
`agentCapabilities { loadSession: true, mcpCapabilities { http: true, sse: true },
promptCapabilities { embeddedContext: true, image: true }, sessionCapabilities
{ close, fork, list, resume } }`, one auth method `opencode-login`, and
`agentInfo` carrying the running package version. It reads
`clientCapabilities._meta["terminal-auth"]` to enrich that auth method.

Session methods: `session/new`, `session/load`, `session/list`,
`session/resume`, `session/close`, `session/fork` (unstable),
`session/set_config_option`, `session/set_mode`, `session/set_model`
(unstable), `session/prompt`, `session/cancel`.

Prompt response stop reasons are `end_turn`, `cancelled` (abort),
`max_tokens` (output length), and `refusal` (content filter);
`max_turn_requests` is never emitted. A provider-auth failure maps to
`authRequired`; any other assistant failure maps to `internalError`.

Updates emitted on `session/update`: `agent_message_chunk`,
`agent_thought_chunk`, `user_message_chunk` (replay), `tool_call`,
`tool_call_update`, `available_commands_update`, `config_option_update`
(1.18.31+), and `usage_update`.

Permissions: the agent calls `session/request_permission` with `once`
(`allow_once`), `always` (`allow_always`), and `reject` (`reject_once`) options,
queued per session. With no client callback the pending permission is
auto-rejected. An `edit` permission may additionally write the proposed content
through `fs/write_text_file` when the client advertises it. `fs/read_text_file`,
terminal methods, and elicitation are not used.

`session/cancel` aborts the backing session best effort and the turn returns
`cancelled`. stdin EOF exits the process with code `0`; `session/close` removes
ACP session state and aborts the backing session, and process exit kills the
internal server.

## The `mcpServers` Handoff

| ACP entry | Required fields | Backing config | Connect path |
| --- | --- | --- | --- |
| `stdio` | `name`, `command`, `args`, `env` | `{ type: "local", command: [command, ...args], environment }` | stdio child with the declared environment merged over the process environment |
| `http` | `name`, `url`, `headers` | `{ type: "remote", url, headers }` | StreamableHTTP, then SSE, forwarding the declared headers on each request |
| `sse` | `name`, `url`, `headers` | `{ type: "remote", url, headers }` | StreamableHTTP, then SSE, forwarding the declared headers on each request |

- **Transports a client may declare**: all three. `initialize` advertises
  `mcpCapabilities: { http: true, sse: true }` and the ACP SDK union accepts the
  two URL-bearing shapes.
- **Is a stdio entry representable**: yes.
- **Is a streamable-HTTP entry with per-instance bearer representable**: yes.
  The `headers` list is converted to a name/value record and passed as
  `requestInit.headers`. The ACP `http` and `sse` discriminants are dropped
  during mapping, so an `http` entry is served by the StreamableHTTP transport
  first — the production shape.
- **Is the entry per session**: it is *declared* per session. `mcpServers` is
  read on `session/new`, `session/load`, `session/resume`, and `session/fork`,
  and registration is deduplicated per session by name plus stable config. The
  *backing registration* is not per session: `sdk.mcp.add` stores it on the
  instance/directory state keyed by server name with last-write-wins, and
  `session/close` clears only the ACP-side dedupe set, not the backing server.
  Two sessions declaring the same server name with different configs share one
  backing slot; the later declaration wins.

The provider capability therefore does not require Longhorn's stdio carrier. A
consumer-supplied loopback StreamableHTTP URL with a bearer header is
representable and wired to a StreamableHTTP connect attempt. What remains
unproven provider-free is only that a live loopback StreamableHTTP server
connects, that its tools are listed, and that a real turn can call them —
ordinary live-gate evidence, not a representability question.

## Proposed Claim Shape (Not Applied)

Evidence only; no claim, matrix row, or admission moves here. For g06.016 and
contract review:

- **Route id**: `opencode.acp`, kept strictly separate from `opencode.http`.
- **Axis (proposed)**: `opencode.executable` — the launched CLI package release,
  distinct from `opencode.server`, the attached-server axis the HTTP route
  binds. Contract 029's separate-version-axes rule requires the distinction.
- **Claim id**: `opencode.acp.executable-window-1`.
- **Behavior revisions**: `opencode.acp-v1.client-mcp-servers-v1` for
  `1.18.18..=1.18.30` and `opencode.acp-v1.client-mcp-servers-v2` for
  `1.18.31..=1.18.32`, the only difference being the `config_option_update`
  emission and the reasoning `messageId` binding.
- **Support posture**: `AllowUnverified`.
- **Segments**: `1.18.18..=1.18.30` and `1.18.31..=1.18.32`, both maintained;
  no interior gap (every stable in the range is published).
- **Unverified newer**: `1.18.33`.
- **What g06.016 must build**: the ACP stdio driver and prepared facade; the
  `initialize`/session/prompt/update/permission/cancel mapping; and the
  consumer-declared MCP seam for the transports the contract admits.
- **Contract gate**: Contract 063 admits exactly three MCP placements, all
  stdio-shaped and Swallowtail-owned (the watcher family, the `claude-agent.sdk`
  mediated-stdio carrier, and the `grok-build.acp` client-MCP courier). A
  consumer-supplied URL-plus-header placement is a *new* shape. g06.016 must not
  wire one into production until a contract admits the placement; the stdio
  entry is the currently admitted shape.

## Typed Gaps

- **Live honouring of the remote entry.** The frozen source proves
  representability, mapping, and a StreamableHTTP connect attempt, not that a
  live loopback streamable-HTTP server with a bearer header connects and is
  callable. Reopen: a separately authorized ACP live gate that declares one
  remote entry and exercises one tool call. No live packet exists, so the
  feature-matrix MCP cells cannot become `Yes` on this evidence.
- **Contract admission of a consumer-supplied HTTP MCP placement.** Named
  above; an escalation to the operator/Chatterbox through the implementation
  lane, not a provider question.
- **Same-name backing collisions.** Per-session declaration maps onto a
  name-keyed instance-scoped registration with last-write-wins and no
  unregister on close. The implementation lane must decide whether the driver
  bounds declarations to one route-owned name or surfaces this collision
  honestly.
- **`protocolVersion` is not negotiated.** OpenCode always answers `1`
  regardless of the client's requested version. A driver must not infer that
  its requested revision was accepted.
- **Plugin loading.** `opencode acp` runs the host's plugins unless `--pure` is
  passed. Plugin loading is not part of the selected ACP surface; a production
  route must decide whether to pin `--pure`.
- **The 1.18.31 boundary.** The window's only behaviour delta lands at
  `1.18.31`; if g06.016 compiles one behavior revision it should compile for
  `v2` and treat the `v1` shape as accepted-but-older.

## Stop Conditions

None fired.

- The ACP revision does not diverge from the routes already qualified: the wire
  is ACP protocol version `1` on the same ACP SDK family the other ACP routes
  use.
- `mcpServers` handling *is* settled provider-free at the representability and
  wiring level, so the "cannot be settled" stop does not apply; the live
  question is a bounded typed gap.
- The official 1.18.18 artifact and the installed binary are the same identity,
  and the 1.18.18 vs 1.18.32 selected-surface delta is bounded and classified,
  so the "official and installed disagree" stop does not apply.
- No new driver shape beyond the existing ACP driver and the MCP seam is needed.
  The contract gate above is a placement-admission question, not an unbuilt
  driver mechanism, and it binds the implementation lane rather than blocking
  this evidence lane.

## Non-Claims

No adapter, driver, route, claim, matrix row, or admission was added or changed.
No production claim is made for `opencode.acp`. No live session, prompt, login,
install, host update, release, or tag occurred. Downloaded artifacts were
hashed and never executed. `opencode.http` and OpenCode web search were not
touched and no `opencode.http` fact is used as ACP evidence. No claim is made
about `subscriptions`/`listen` or Longhorn resources. The route's HTTP
representability is a provider capability; it is not a Swallowtail production
admission.

## Sources

- npm registry `opencode-ai`, `opencode-darwin-arm64` packuments, tarballs, and
  published integrity, for `1.18.18` and `1.18.32`, plus the published stables
  `1.18.19..=1.18.31`
- GitHub releases and tag refs `v1.18.18..=v1.18.32` at
  `https://github.com/anomalyco/opencode`
- npm `@agentclientprotocol/sdk@0.21.0` and its published JSON schema
- installed `~/.opencode/bin/opencode` `1.18.18` (`--version`,
  `acp --help`, artifact hash)
- frozen corpus
  `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.18.32/`
  (`identity.json`, `surface.json`, `window-ledger.json`)
