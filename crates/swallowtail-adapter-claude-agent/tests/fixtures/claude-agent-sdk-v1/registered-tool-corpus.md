# 289 Claude Agent SDK registered-tool transport corpus

Status: evidence preparation; no runtime or support claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 Card116
Base: `6c52b9f970b38838f8e4918d32b536dc26fb0823`
Authority: Card116; the g05.035 dispatch manifest; Contracts 060, 061, and
063; Card084; Research 278, 280, and 288

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
[custom tools](https://code.claude.com/docs/en/agent-sdk/custom-tools), and
[approval handling](https://code.claude.com/docs/en/agent-sdk/user-input).

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

1. Card114 must prove the reusable Contract060-derived registered-tool kernel,
   including registration snapshot, lease, live admission, correlation,
   bounded result, progress, reconnect, no-replay, cancellation, and joined
   teardown fixtures.
2. Card115 must prove the selected skill/reference transport without changing
   the route's exact operation binding.
3. The Claude lane needs a provider-free fake-SDK fixture for one exact
   Contract063 registered MCP selection: snapshot-to-provider name mapping,
   admission, dispatch, typed result, denial, cancellation, unknown outcome,
   duplicate/late result rejection, and cleanup.
4. If the route uses an SDK MCP handler as the carrier, a pinned 0.3.259
   artifact fixture must prove the sidecar callback can send and receive the
   exact bound call without exposing raw credentials, paths, endpoints, or
   provider content. Static declarations do not prove this.
5. If the route uses HTTP/SSE, a pinned route corpus must identify the exact
   protocol/version, auth boundary, startup/status timing, call/result bounds,
   cancellation, reconnect, and joined cleanup. Current docs alone are not
   enough.
6. After114/115, one exact-head review must inspect the mounted/callable path,
   not only type fixtures. A real disposable server/result remains a later,
   separately authorized route gate.

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
- wait for cards114-115 before any adapter wiring or support disposition.

No runtime source, public API baseline, guide, route matrix, contract, claim,
WatcherBridge path, Claude Code behavior, credential, or provider state changed
for this preparation.
