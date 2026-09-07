# 289 Claude Agent SDK registered-tool transport corpus

Status: evidence preparation; no runtime or support claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 Card116
Base: `6c52b9f970b38838f8e4918d32b536dc26fb0823`
Authority: Card116; the g05.035 dispatch manifest; Contracts 060, 061, and
063; Card084; Research 278, 280, and 288

Continuation revalidation: current `origin/main` is
`7090917257e1a6504545d8d30fc9b43e142162c8`, with Cards114-115 merged. This
refresh remains Card116 evidence preparation and does not change the original
artifact base or imply Claude route support.

This record is intentionally fixture-local. The g05.035 manifest reserves
documentation front doors for coordinator closeout; this route-owned surface
holds the exact evidence and machine-readable index without changing those
front doors.

## Question

What exact Claude Agent SDK surface can consume the Contract063 registered-tool
selection after cards114-115, and which callback or transport evidence is still
missing before adapter wiring?

This is a preparation record. It does not widen a route, amend a contract,
change a version claim, or prove a provider turn.

## Method and boundary

Read-only evidence collection used:

- the promoted Swallowtail contracts, Card084 result, Research 278/280/288,
  current Claude SDK adapter source, and provider-free fixtures;
- the exact official npm `@anthropic-ai/claude-agent-sdk@0.3.259` tarball,
  extracted to a temporary directory, never executed; and
- the current official Agent SDK MCP, custom-tool, and approval documentation,
  recorded as descriptive evidence rather than as a version qualification.

The tarball SHA-256 was recomputed as
`0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e`, matching
the committed 0.3.259 fixture. No optional platform package was downloaded or
run. No provider process, prompt, login, credential, account inspection, live
MCP server, host setting, or consumer operation was used.

The current docs are useful for surface discovery but are not a pinned route
corpus. The package declarations and committed provider-free fixtures remain
the exact artifact evidence.

## Exact artifact surface

The pinned package exposes these relevant declarations in `sdk.d.ts`:

| Surface | Exact shape | Evidence class | Prep reading |
| --- | --- | --- | --- |
| Server registration | `Options.mcpServers?: Record<string, McpServerConfig>` | package declaration | A provider attachment map, not a Contract063 snapshot |
| Strictness | `strictMcpConfig?: boolean` | package declaration | Required to suppress project/user/plugin MCP discovery |
| Stdio | `McpStdioServerConfig { command, args?, env?, timeout?, alwaysLoad? }` | package declaration | Existing Card084 route-local attachment |
| SSE | `McpSSEServerConfig { type: 'sse', url, headers?, tools?, timeout?, alwaysLoad? }` | package declaration and current docs | Present, but no pinned route transport corpus |
| Streamable HTTP | `McpHttpServerConfig { type: 'http', url, headers?, tools?, timeout?, alwaysLoad? }` | package declaration and current docs | Present, but no pinned route transport corpus |
| In-process SDK server | `McpSdkServerConfigWithInstance` with a non-serializable `McpServer` instance | package declaration and current docs | Sidecar-local callback; no host binding evidence |
| Status | `Query.mcpServerStatus(): Promise<McpServerStatus[]>` | package declaration | Observation only; status may include `connected`, `failed`, `needs-auth`, `pending`, or `disabled` |
| Admission callback | `canUseTool?: CanUseTool` | package declaration and current docs | Approval/deny gate, not a consumer dispatch/result callback |
| Native process seam | `spawnClaudeCodeProcess?: (options: SpawnOptions) => SpawnedProcess` | package declaration and Research 280 | Process authority; unrelated to registered-tool dispatch |

The exact package also exports `tool()` and `createSdkMcpServer()`. The custom
tool handler returns MCP content, optional structured content, and an error
flag. That is a provider-facing in-process server callback. It does not expose
a Swallowtail host service, operation lease, consumer admission binding, or
Longhorn result path.

The complete frozen declaration gives `CanUseTool` more provider context than
the existing route wire forwards: `toolName`, input, an abort `signal`,
`toolUseID`, `requestId`, and optional prompt metadata (`suggestions`,
`blockedPath`, `decisionReason`, `title`, `displayName`, `description`,
`agentID`, and `matchedAskRule`). It returns `PermissionResult | null`, not a
`RegisteredToolOutcome`. The sidecar deliberately forwards only its own
bounded callback id, `toolName`, and the existing Bash view. That is correct
for the current permission-only route, but it leaves the provider call
correlation and typed result carrier unqualified.

The official documentation records these provider-facing rules:

- MCP names use `mcp__<server-name>__<tool-name>`.
- MCP tools need permission; `allowedTools` auto-approves listed tools, while
  an unapproved call may reach `canUseTool`.
- `tools` restricts built-in tool availability; it does not replace the MCP
  server declaration.
- stdio is a local child process; HTTP/SSE are remote transports; SDK MCP is
  in-process.
- server connection timing differs by transport, and `mcpServerStatus()` is a
  status query rather than a completion or result authority.

Sources: [Agent SDK MCP documentation](https://code.claude.com/docs/en/agent-sdk/mcp),
[custom tools](https://code.claude.com/docs/en/agent-sdk/custom-tools),
[approval handling](https://code.claude.com/docs/en/agent-sdk/user-input),
[MCP lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle),
[MCP schema](https://modelcontextprotocol.io/specification/2025-11-25/schema), and
[MCP tools](https://modelcontextprotocol.io/specification/draft/server/tools).

## Contract063 mapping

| Contract063 element | Claude SDK surface | Fit at this prep boundary | Disposition |
| --- | --- | --- | --- |
| `RegisteredToolSnapshot` | server name, tool name, MCP kind, config, and provider tool spelling | Carries only a provider config and an upstream tool name. It has no schema namespace/digest, registration revision, effect/retry posture, host identity, or Contract063 admission source. | Adapter must consume the central snapshot after114; do not synthesize one from `mcpServers`. |
| `RegisteredToolSelection` | `mcpServers`, `tools`, `disallowedTools`, `strictMcpConfig` | Partial route translation. The provider can receive a declared set, but the SDK does not carry Swallowtail's immutable selection or bridge generation. | Candidate translation only; no wiring in prep. |
| `RegisteredToolDispatcher` | `canUseTool(toolName, input)` | The callback decides allow/deny and may return updated input. It does not dispatch a consumer effect or return a typed consumer result. | Not a dispatcher mapping. Preserve as the existing permission seam. |
| Host-mediated callback | `canUseTool` over the private sidecar wire | Provider-free fixtures prove callback request/response framing and MCP tool-name admission. They do not prove that an admitted MCP call reaches a Contract063 dispatcher. | Reuse for admission only; do not call it common tool execution. |
| In-process callback carrier | `tool()` / `createSdkMcpServer()` handler | The handler can return a provider-visible result, but it runs inside the Node sidecar and contains a live non-serializable SDK object. No host-bound lease or cancellation/result carrier is proven. | Withheld pending an exact private-carrier corpus. |
| MCP execution kind | `mcp__<server>__<tool>` plus provider `tool_result` records | The provider identity is observable and route-local. The existing sidecar projects tool activity, but provider observation is not consumer result authority. | Preserve kind identity; no result claim. |
| App/native client tool | No public Claude Agent SDK consumer-tool registration callback | `canUseTool` is not an app-tool registration API. | Withheld. |
| Provider-owned tool | Claude built-in tools | Existing permission and provider activity paths remain separate from registered consumer tools. | Preserve existing route behavior. |
| Lease, generation, reconnect, and close | SDK process/MCP lifecycle plus `Query` controls | No public SDK shape binds the Contract063 task/session/turn/attempt, transport generation, no-replay rule, or joined bridge teardown. | Card114 kernel evidence is a prerequisite. |

The important negative result is exact: the SDK has provider-facing MCP and
permission surfaces, but no public callback that is equivalent to
`RegisteredToolDispatcher::dispatch(call, context)`.

## Contract063 gap capsule

This is the independent Card116 result after re-reading the frozen 0.3.259
package declarations/source, Card084, the current official vendor docs, the
existing fake SDK, and the merged provider-free Card114 kernel. “Missing” means
that no current Claude SDK field, callback, result, or exact artifact proves
the Contract063 requirement. Card114 now supplies the provider-neutral types
and conformance vocabulary; it does not supply a Claude adapter carrier.

| ID | Required Contract063 artifact | What 0.3.259/Card084 currently proves | Authoritative source or provider-free probe | Smallest permitted acquisition action |
| --- | --- | --- | --- | --- |
| REG-01 | Immutable snapshot: server identity, revision, execution host, source identity/freshness | `mcpServers` is only a provider config map; no snapshot identity or freshness | Contract063 registry section; `runtime/src/registered_tool/snapshot.rs` | Build one route-owned fake declaration against the merged snapshot type and assert identity/freshness survive mapping; no provider process |
| REG-02 | Namespaced tool identity and exactly one execution kind | Provider spelling is `mcp__<server>__<tool>`; no Contract063 `RegisteredToolId`/kind binding | Contract063 tool-kind rules; Card114 `declaration.rs` and `identity.rs` | Provider-free fake `tools/list` fixture with one namespaced mapping; reject alias/kind mismatch |
| REG-03 | Input/output schema namespace, media type/dialect, revision, bounded bytes, digest | `SdkMcpToolDefinition` has a Zod input schema and `CallToolResult`; the current route records neither schema digest nor output schema | Contract063 registry/schema rules; Card114 `schema.rs`/`declaration.rs`; frozen `sdk.d.ts` `SdkMcpToolDefinition` | Add static schema metadata to the fixture and assert digest/bounds against a fake call/result; no SDK execution |
| REG-04 | Effect posture, retry posture, call/result/concurrency/deadline bounds | SDK has per-server timeout and Card084 has local bounds; neither is the Contract063 declaration or no-replay posture | Contract063 results/concurrency rules; Card114 `declaration.rs`/`limits.rs` | Reuse Card114 provider-free bounds fixture with a fake stdio descriptor; do not infer provider limits |
| REG-05 | Transport set and exact protocol-version subset | Package exposes stdio/SSE/HTTP/SDK configs; package peer metadata does not pin the MCP peer or route protocol | Contract063 transport/version rules; Card114 `snapshot.rs`/`readiness.rs`; package `package.json` | Record one explicit route-local carrier/version fixture and reject every unlisted version before any provider work |
| REG-06 | Required host services, credential references, opaque process/environment recipes | SDK config accepts command, args, env, URL, and headers; those are provider attachment fields, not opaque host references | Contract063 host/credential boundary; Card084 sidecar env rules; Card114 `snapshot.rs` | Map only opaque fake recipe/reference ids and assert raw command, URL, headers, and credential material never enter the public snapshot |
| SEL-01 | Exact selection: snapshot revision, selected namespaced tools, carrier, negotiated version, effective bounds | `tools` and `disallowedTools` select provider names only; no immutable prepared selection | Card114 `selection.rs`/`readiness.rs`; Contract063 prepared-plan rule | Provider-free selection fixture with one exact version and one rejection case; no adapter wiring |
| LEASE-01 | Open binding: host, configured instance, scope/turn, admission, lease generation, transport generation, deadline, cancellation, concurrency | SDK `query()`/`Query` has session controls but no Contract063 lease binding | Card114 `call.rs`/`kernel.rs` and Contract060 lease rules | Mount the existing fake registered-tool host and assert the binding fields before a fake call; no Claude sidecar |
| CALL-01 | Typed call envelope: call id, tool id, execution kind, bounded arguments, deadline, bound lease | SDK tool activity exposes provider `tool_use` ids, while the current sidecar projects only `tool_started` and no arguments | Contract063 call fields; Card114 `RegisteredToolCallRequest`/`RegisteredToolCall` | Capture one synthetic provider call record and map it to a Card114 request; prove bounds and identity rejection only |
| CB-01 | Dispatch callback carrying the bound call and returning a host-owned typed outcome | Frozen `CanUseTool(toolName,input,options)` returns allow/deny; `options` has `toolUseID`, `requestId`, and `signal`, but no host dispatcher/result | Frozen `sdk.d.ts` `CanUseTool`; official approval docs; existing `permission.rs` and `fake-sdk.mjs` | Extend only a provider-free fake corpus to show allow → dispatcher request → typed outcome; do not change the sidecar/runtime |
| CB-02 | Permission/result separation, including provider-supported Deny semantics | Card084 proves `canUseTool` allow/deny and deny-never-reaches-fake-server; it does not prove a central dispatch result | Card084 result/tests; Contract063 permission rules | Preserve current admission fixture and add a separate expected “dispatch not called on deny” record; no provider turn |
| MCP-01 | Exact MCP initialize/version handshake: JSON-RPC id, `protocolVersion`, capabilities, client/server implementation info, `initialized` notification | `mcpServers` declarations and status are present; no exact 0.3.259 MCP wire transcript or negotiated version is frozen | MCP lifecycle/schema specification; package peer metadata; Card084 route source | Create a bounded fake stdio transcript with initialize/initialized and one version mismatch; do not start a server |
| MCP-02 | Tool catalogue: `tools/list` request/cursor, tool name/description/input schema/annotations, optional list-change behavior | SDK status may project tool names/descriptions/annotations; the current fake supplies neither an MCP tools/list response nor schema digest | MCP tools/schema specification; frozen `McpServerStatus.tools`; Card084 fake boundary | Add a static tools/list fixture with one deterministic order and schema digest; no live MCP server |
| MCP-03 | Tool call/result carrier: JSON-RPC request id, `tools/call` name/arguments, content blocks, structured content, `isError`, protocol errors | In-process `handler` returns `CallToolResult`; the sidecar never receives a host-owned result, and current fake emits only an empty final SDK result | MCP schema; frozen `SdkMcpToolDefinition.handler`; current `projectMessage`/fake SDK | Add one provider-free call/result transcript covering success, `isError`, malformed result, and unknown id; do not run the SDK |
| RESULT-01 | `RegisteredToolResult` payload plus output-schema digest and exactly-one `RegisteredToolOutcome` | SDK final `result`/`tool_result` observations carry provider status only; no Contract063 output digest or outcome disposition | Card114 `call.rs`; Contract063 result rules; current sidecar tool-ended projection | Map static fake `CallToolResult` content to a bounded opaque result and assert one settlement; no runtime change |
| RESULT-02 | Safe failure taxonomy: unsupported, host/process/readiness, Deny, provider rejection, cancellation, timeout, server failure, invalid result, transport loss, unknown, stale/duplicate/foreign/post-close | Current route has sidecar failure codes and `isError`, but no one-to-one Contract063 outcome taxonomy | Contract063 results/errors; Card114 `failure.rs` and `call.rs` | Build a provider-free disposition matrix from synthetic records; leave unmapped cases explicitly withheld |
| PROG-01 | Non-terminal progress with call/binding/kind/transport generation and monotonic sequence | Sidecar emits an unqualified `{event: "progress"}`; MCP progress is not captured in the fake or route wire | Contract063 progress rules; Card114 `RegisteredToolProgress`; MCP progress schema | Add static progress records with duplicate/regressive/foreign/stale cases and assert rejection; no provider execution |
| CANCEL-01 | Call-scoped cancellation, deadline, and unknown-execution outcome; no automatic mutating replay | `CanUseTool.options.signal` covers approval; `Query.interrupt()` is turn-level; neither returns a typed registered-call disposition | Frozen `CanUseTool`/`Query` declarations; Contract063 cancellation rules; Card114 `RegisteredToolExecutionDisposition` | Hold a fake dispatcher call, inject cancellation/timeout/transport loss, and record `Cancelled`/`Unknown` without replay; provider-free only |
| LIFE-01 | Reconnect/close generation checks, admission freeze, joined readers/callbacks/resources, stale/late/duplicate rejection | SDK has `mcpServerStatus`, `reconnectMcpServer`, `reinitialize`, and `close`; the current private wire has no registered-call generation or joined dispatch carrier | Contract060 lifecycle; Contract063 reconnect/teardown; Card114 `kernel.rs`/host lease tests | Replay synthetic old-generation records against the merged kernel and assert fail-closed close; no sidecar mutation |
| VER-01 | Exact version artifact for the registered route: adapter wire, MCP peer, native CLI, SDK wrapper, harness schema, and feature capability | SDK wrapper `0.3.259`, native `2.1.259`, harness schema `1`, and private wire `...-v1` are frozen; MCP peer is only a range and no registered-tool capability transcript exists | Card084 result; Research 278/280; package manifest; Card114 conformance version | Record the exact package/peer/route version tuple and a synthetic capability response; do not call a provider |

The capsule is exhaustive at the Contract063 boundary: configuration and
permission are present, while snapshot/selection/lease binding, dispatch
callback, MCP call/result carrier, safe outcome mapping, progress, cancellation,
reconnect/close, and exact route-version evidence remain unqualified. The
merged Card114 kernel closes the provider-neutral half of those gaps; it does
not make the Claude SDK surface callable.

### Exact alternative if the common SDK route is impossible

The frozen artifacts do not expose a public common consumer-dispatch callback.
The exact evidence-backed alternative is Card084's qualified native stdio
attachment: a declared local stdio MCP server, explicit child environment,
`strictMcpConfig`, bounded status, and per-call `canUseTool` mediation. This is
route-local prior art, not current Contract063 registered-tool support.

To qualify it as registered-tool dispatch, Card116 would need a separate
route-local stdio mediation profile: the provider's MCP `tools/list` and
`tools/call` records must map one-to-one to Card114's
`RegisteredToolCallRequest` and `RegisteredToolOutcome`, while the host owns
the Contract063 snapshot, lease, generation, schema digest, bounds, cancellation,
and result validation. `canUseTool` remains permission admission only. The
mapping would be Contract063 MCP kind plus an explicitly qualified stdio
transport/version entry, Card116 adapter evidence, and the later Batch C
disposable server/result gate. It must not widen `WatcherBridge` or create a
second registry/lease/listener.

### Chatterbox decision question

Should Card116 qualify that exact route-local stdio mediation profile as the
Claude SDK mapping for Contract063, or keep registered-tool dispatch withheld
until a provider-neutral host-mediated callback carrier is exposed directly by
the adapter?

## Transport disposition

| Transport | Evidence collected | Card116 disposition |
| --- | --- | --- |
| stdio | Exact 0.3.259 declarations; merged Card084 implementation; provider-free status, required/optional failure, name-admission, omission, and `canUseTool` fixtures | Reuse as existing route-local prior art. It is not the Contract063 shared bridge and must not create a second registry, lease, or listener. |
| SSE | Exact declaration and current official docs show `type: 'sse'`, URL, optional headers, deferred/first-turn connection behavior, and `needs-auth` status | Withhold. No exact pinned 0.3.259 transport transcript proves auth non-custody, protocol version, cancellation, reconnect, result bounds, or joined cleanup. |
| Streamable HTTP | Exact declaration and current official docs show `type: 'http'`, URL, optional headers, and the documented streamable-HTTP alias in config files | Withhold. Same missing pinned route corpus; do not remake Card084 or infer common HTTP support from docs. |
| In-process SDK | Exact declaration marks the instance non-serializable; current docs place handlers inside the application/sidecar | Withhold. A sidecar-to-host private callback/result carrier is not present in the current wire. |
| Managed/ambient MCP | `managedMcpServers` is a settings surface; the route passes `settingSources: []` and `strictMcpConfig: true` | Withhold. No ambient settings or managed server authority enters the route. |

## Existing private carrier and preservation ledger

The current sidecar wire is
`swallowtail-claude-agent-sdk-jsonl-v1`. Its provider-free command set is
`open`, `query`, `interrupt`, `set_permission_mode`, `set_model`,
`list_sessions`, and `close`. It already carries a bounded route-local
`can_use_tool` callback and `callback_response` pair. It has no registered-tool
call, result, progress, transport-generation, or Longhorn dispatch record.

The following behavior stays closed during preparation and later integration:

| Invariant | Current evidence | Required preservation |
| --- | --- | --- |
| Default omission | Undefined `mcpServers` becomes an empty set; default open omits `mcpServers` and does not call `mcpServerStatus`. | A central snapshot must be opt-in. Empty selection must remain byte/behavior equivalent to the default profile. |
| Exact server/tool admission | Server and tool names are bounded; `mcp__<server>__<tool>` is constructed explicitly; the reserved `swallowtail-watchers` name is rejected. | Do not alias a central namespaced identity into another tool kind. Do not widen WatcherBridge. |
| Permission | `allowedTools` is never sent; auto-approving modes are rejected; admitted calls use the existing `claude-agent-sdk/can-use-tool` callback namespace. | Central bridge admission must not bypass, persist, or simulate provider approval. |
| Resume | `resume` and `resumeSessionAt` are validated before open; the same declared MCP set is part of the sidecar open options. | A future exact selection must bind to the resumed operation without replay or cross-session inheritance. |
| Watcher/Claude Code | Contract060's `WatcherBridge` and existing Claude Code `--mcp-config` path remain closed and separate. | No shared implementation or fixture may alter watcher omission, ready order, private authority, terminal barrier, or joined cleanup. |

Local evidence anchors:

- [Card084 result](../../../../../docs/roadmaps/g05/batch-cards/084-claude-sdk-client-mcp-servers.md#result)
- [pinned 0.3.259 identity fixture](../claude-agent-sdk-0.3.259/identity.json)
- [pinned declaration excerpts](../claude-agent-sdk-0.3.259/sdk-declarations.d.ts)
- [private wire fixture](./protocol.json)
- [MCP source boundary](../../../../../crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs)
- [provider-free MCP tests](../../../../../crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/mcp.rs)
- [sidecar admission and MCP validation](../../../../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs)

## Missing evidence before wiring

The prep result is a bounded missing-evidence list, not a support claim:

1. Cards114-115 now provide the provider-neutral registered-tool kernel and
   selected skill/reference types on current `origin/main`; this Claude lane
   still lacks the adapter-side mapping and exact carrier evidence listed in
   the gap capsule above.
2. The Claude lane needs a provider-free fake-SDK/MCP fixture for one exact
   Contract063 registered MCP selection: snapshot-to-provider name mapping,
   admission, dispatch, typed result, denial, cancellation, unknown outcome,
   duplicate/late result rejection, and cleanup.
3. If the route uses an SDK MCP handler as the carrier, a pinned 0.3.259
   artifact fixture must prove the sidecar callback can send and receive the
   exact bound call without exposing raw credentials, paths, endpoints, or
   provider content. Static declarations do not prove this.
4. If the route uses HTTP/SSE, a pinned route corpus must identify the exact
   protocol/version, auth boundary, startup/status timing, call/result bounds,
   cancellation, reconnect, and joined cleanup. Current docs alone are not
   enough.
5. One exact-head review must inspect the mounted/callable path after the
   route carrier is implemented, not only type fixtures. A real disposable
   server/result remains a later, separately authorized route gate.

The remaining gap is therefore route evidence and adapter mapping, not another
shared-kernel producer. No Card117 surface or producer is dispatched here.

## Falsification

| Statement | Falsifier | Result |
| --- | --- | --- |
| `canUseTool` is a common consumer dispatcher | A public callback contract returning a host-owned typed tool result rather than allow/deny input | Not present in the pinned declarations or current docs. |
| SDK MCP config proves Contract063 registration | A provider config carrying snapshot revision, schema digest, host binding, lease generation, and result authority | Not present. |
| Static HTTP/SSE documentation qualifies the route | A pinned 0.3.259 route transcript covering protocol, auth, lifecycle, cancellation, and cleanup | Not captured. |
| Card084 proves centralized bridge integration | Contract063 kernel and exact Claude adapter wiring at an accepted head | Not present; Card084 remains route-local untagged prior art. |
| Empty/default behavior changed | A provider-free default open containing MCP config/status or changing existing permission/resume records | Existing fixtures and source retain omission; no change made. |

## Decision

Card116 read-only preparation is complete at this boundary. The exact useful
mapping is:

- retain Card084 stdio MCP support as route-local prior art;
- reuse `canUseTool` only for exact permission admission;
- do not treat `canUseTool` as Contract063 dispatch;
- withhold in-process SDK callback, HTTP, and SSE integration until their
  exact private-carrier or pinned transport corpus exists; and
- consume the merged cards114-115 types only through a route-owned, separately
  qualified carrier; no adapter wiring or support disposition is authorized by
  this preparation.

No runtime source, public API baseline, guide, route matrix, contract, claim,
WatcherBridge path, Claude Code behavior, credential, or provider state changed
for this preparation.
