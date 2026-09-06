# 084 Claude SDK Client MCP Servers

Status: ready; serial after card 083's merge
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 083 merged; card 080 tool admission; Contract 013 tool exclusion

## Goal

Consumer-declared MCP servers on `claude-agent.sdk` open, with strict config, per-server status observation, and every MCP tool call admitted through the same per-call mediation as native tools. No server authority beyond the declared set.

## Scope

1. Freeze the `0.3.259` `mcpServers` config shape and the status query (`mcpServerStatus` or equivalent) with anchors. Stdio servers first; SSE/HTTP only if the SDK proves them without extra credentials.
2. Additive prepared input on `ClaudeAgentSdkSessionProfile`: an ordered set of declared servers (name, command, args, env allowlist keys). `strictMcpConfig` stays true. Server env is built by the same deny-by-default allowlist as the sidecar child env; no inherited `process.env`.
3. MCP tool names (`mcp__<server>__<tool>`) join the admitted set explicitly per server; unadmitted MCP tools are withheld into `disallowedTools`; every admitted MCP call goes through `canUseTool` mediation exactly like `Edit` or `Bash` (card 080/081 semantics). No MCP tool is ever auto-allowed.
4. Open evidence carries per-server status (connected, failed with typed code, pending); a declared server that fails to connect fails the open typed unless the profile marks it optional.
5. Fake-SDK proofs: a fake stdio MCP server connects, its tool is admitted and mediated, a denied call never reaches the server, an undeclared server name is rejected before the SDK is constructed, a failing required server fails open.
6. Guide, matrix, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Managed MCP servers (`managedMcpServers`, withheld in Research 280); OAuth-backed remote servers; MCP resources or prompts; any change to the default profile.

## Acceptance Criteria

- [ ] `mcpServers` and status surfaces frozen with anchors
- [ ] declared servers are additive profile input; default profile unchanged
- [ ] every MCP tool call is mediated; none auto-allowed
- [ ] per-server status in open evidence; required-server failure is typed
- [ ] fake-SDK proofs as listed; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no process runs and no tool executes that the consumer did not declare and the host did not admit. Smallest counterexample: an MCP tool call that bypasses `canUseTool`.

## Stop Conditions

The SDK auto-allows MCP tools in a way `canUseTool` cannot intercept (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review.
