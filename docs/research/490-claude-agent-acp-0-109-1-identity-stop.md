# 490 Claude Agent ACP 0.109.1 Identity and Qualification Stop

On 2026-11-23, npm `latest`, GitHub stable tags and the ACP registry all pointed
to 0.109.1 for `claude-agent.acp` on the `claude-agent.acp-adapter` axis. The
first published hop, 0.107.0, changes the Agent SDK and its selected native
image on both guarded platforms. Qualification stays at 0.106.0 pending
exact guarded proof.

## Artifact identity

The [frozen ledger](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.109.1/README.md)
covers all eight published stables above 0.106.0, including 0.107.1–0.107.3
and 0.108.1. The ACP, Agent SDK and ACP SDK npm tarballs were downloaded
without installing or executing them. For each, the package name/version,
dependency metadata, npm SHA-512 integrity and npm SHA-1 were verified. The
ledger records computed SHA-256 values, full file identities by adjacent
hop, and npm publication times. All ACP Git heads match exact tags. The
0.106.0 tarball and full tree match [Research 483](483-claude-agent-acp-0-106-0-evidence.md).
The first later unpublished stable is 0.109.2. Preview tags are outside
this stable set.

| ACP | Published UTC | ACP SDK | Agent SDK | Native manifest |
| --- | --- | --- | --- | --- |
| 0.106.0 | 2026-11-10 22:49:36.705 | 2.6.0 | 0.3.322 | 2.1.322 |
| 0.107.0 | 2026-11-13 13:39:38.999 | 2.6.0 | 0.3.323 | 2.1.323 |
| 0.107.1 | 2026-11-16 09:52:58.953 | 2.7.0 | 0.3.323 | 2.1.323 |
| 0.107.2 | 2026-11-16 11:41:46.529 | 2.7.0 | 0.3.325 | 2.1.325 |
| 0.107.3 | 2026-11-16 14:39:18.879 | 2.7.0 | 0.3.325 | 2.1.325 |
| 0.108.0 | 2026-11-17 13:43:51.377 | 2.7.0 | 0.3.326 | 2.1.326 |
| 0.108.1 | 2026-11-18 17:21:24.324 | 2.8.0 | 0.3.326 | 2.1.326 |
| 0.109.0 | 2026-11-19 13:36:23.668 | 2.8.0 | 0.3.328 | 2.1.328 |
| 0.109.1 | 2026-11-21 15:37:49.660 | 2.9.1 | 0.3.330 | 2.1.330 |

The local PATH entry belongs to 0.63.0 by package metadata. Its entry file
is 3,200 bytes with SHA-256 `260aac90bf75f197b93640087c1de66441761d43c2784efa035fdcee60b5dacd`.
It was read without execution and carries no new-hop evidence.

## Selected v1 boundaries

The npm executable still enters at `dist/index.js`. Swallowtail initializes
with protocol version 1. Every hop changes `dist/v1/acp-agent.js`; the
`IdleCompaction` module itself stays byte-identical, with the same literal
v1 import and session construction through 0.109.1. The v1 query still
enables user, project and local settings and supplies `canUseTool`. Those
source facts help locate the guard and the permission seam. Each new
wrapper and dependency graph still needs capture, and native manifests
alone cannot establish whether inherited denial bypasses the callback and
continues the turn as required by v8.

| Hop | Selected or adjacent shipped changes |
| --- | --- |
| 0.107.0 | `v1/acp-agent.js` extracts the load/resume fingerprint and uses a backend-scoped context-window cache. The initial usage size fallback rises from 200,000 to 1,000,000; authoritative model usage can still override it. `result-end.js` adds structured rate-limit error data, and `tool-calls/retracted.js` closes retracted calls through the existing tool update path. The Agent SDK and native identities both change. |
| 0.107.1 | The v1 setup path parses initial config options when preparing a session. `session-effort.js` and `session-model.js` also change. ACP SDK 2.7.0 changes unstable v1 generated schema and SSE transport code. The stdio implementation stays byte-identical at this hop. |
| 0.107.2 | `session-fast-mode.js` sends setting updates through the SDK flag API. The v1 caller changes when it starts or resumes consuming a query and when autonomous or unowned cycles report fast-mode changes. The Agent SDK and native identities change again. |
| 0.107.3 | The flag-gated `hide-claude-auth.js` path adds an Enterprise subscription exception. The v1 restore path reapplies model/config choices with revised validation and keeps the model list narrow when supplied by a gateway. Both SDK pins stay fixed. |
| 0.108.0 | The v1 permission path can settle permission-only tool-call updates and changes streamed tool-result rendering. `mcp-servers.js` adds the client-declared `acp` bridge, which existing Swallowtail HTTP MCP declarations do not select. `claude-cli.js` changes delivery of MCP server configuration for the bundled CLI to SDK initialize. The Agent SDK/native pair changes to 0.3.326/2.1.326. |
| 0.108.1 | v1 load, resume, prompt and close reject child session IDs. Tool-call settlement tracks unanswered server calls and MCP startup notices include servers that could not be bridged. ACP SDK 2.8.0 changes v1 bindings and shared JSON-RPC connection code. |
| 0.109.0 | v1 explicitly cleans up unregistered queries when creation fails and can emit capability-gated prompt suggestions. `cliDelivery` adds settings delivery through SDK initialize for the bundled CLI, with argv still used when an external CLI is selected or conflicting settings arguments are present. The Agent SDK and native identities change. |
| 0.109.1 | The v1 options-only path becomes distinct from regular session reuse and teardown. The root close path tears down the session and clears child announcements. Provider route settings and client system-prompt assembly change before creating a query. ACP SDK 2.9.1 changes v1 schema/bindings and makes its newline-delimited stream drain queued messages before surfacing a read error. The Agent SDK and native identities change again. |

The complete per-file inventory includes preview v2 code, declarations,
source maps and packaging. v2-only handlers are outside the version-1
request path; shared modules and runtime imports remain part of the capture
review. New steering/task-stop, MCP-over-ACP, child-session and prompt
suggestion surfaces do not grant a new Swallowtail operation. The Agent SDK
changes `sdk.mjs` and both native manifests at every new SDK pin, so neither
wrapper byte parity nor native process parity with 0.106.0 applies.

## Decision and adaptation

Keep the published 0.53.0–0.100.0 support segments and the 0.58.0 gap.
Keep the eight exact 0.101.0–0.106.0 points and their existing behavior
revisions. All eight newly observed points remain `UnverifiedNewer`.
Live HTTP MCP honouring remains exact to 0.79.0 and 0.81.2.

The version-currentness planner owns the ACP-only adaptation under
[Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#no-terminal-stop--2026-09-24).
Refresh the then-current stable and reuse these identities to verify
the platform tarballs and actual native binaries. Freeze each exact
guarded graph including the idle veto, spawn parent and arguments, settings
and MCP delivery, and fail-closed custody. Run the prepared Rust route
on authorized trusted disposable Linux x64 and Darwin arm64 runners
using the [Research 483 procedure](../knowledge/operations/claude-agent-acp-admission-preparation.md).
Exercise new/load/resume, explicit callback allow, inherited user/managed
denial without a callback, continuation, usage and terminal ordering,
cancellation/deadline, provider failure, and child exit and close joins.
Add discriminating cases for the changed stdio/error, tool settlement and
query creation paths. Qualify a v8 extension only if the selected public
behavior is supported. Return any required consumer-visible change for
a ruling. No original platform executable ran in this identity pass.

## Sources

- [npm publication history](https://registry.npmjs.org/@agentclientprotocol%2fclaude-agent-acp)
- [ACP adapter tags](https://github.com/agentclientprotocol/claude-agent-acp/tags)
- [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- [Agent SDK package](https://registry.npmjs.org/@anthropic-ai%2fclaude-agent-sdk) and [ACP SDK package](https://registry.npmjs.org/@agentclientprotocol%2fsdk)
