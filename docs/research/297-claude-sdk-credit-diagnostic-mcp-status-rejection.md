# 297 Claude SDK Credit Diagnostic MCP-Status Rejection

Status: complete; promoted into g05 card 146
Owner: Tom
Created: 2026-09-08

## Finding

Bovine Desktop's one authorized exhausted-credit diagnostic did not reach an
account or usage decision. Exact source-linked Swallowtail
`6a93f1d916945aa2b402df7329dc994570e005c8` rejected the open before provider
readiness at stage `sidecar_rejected`, with bounded subcode
`mcp_status_invalid` and stable route code
`swallowtail.claude-agent.sdk.open_rejected`.

The capsule records one primary open and zero prompt turns, tool dispatches,
permission callbacks, Deny/cancellation/stale controls, retries, reconnects,
or respawns. Cleanup is `confirmed`: task reapers joined and zero survivors,
operation-bridge listeners, or registered-tool leases remained. Exhausted
credit was therefore not observed and no quota inference is permitted.

## Frozen Identity

Desktop task `9dc9b50f-5bb2-4912-be7d-a6159e8c758d`; PR 172 accepted head
`8ac46df322bec428e744adb5d22fba3a6ad9a7cc`; independent review comment
`5589754115`; merge `117e09e02dafd505d9f1d6e2b1380b56bbb22a19`; canonical
closeout `5d7f1681222222d6f1586176c05733c6f1daf7c3`.

Capsule SHA-256:
`81cb0fd6c5a717e9ca66b73c7a746e66870219ebf2c1330081dfd76549608569`.

The tuple is SDK `@anthropic-ai/claude-agent-sdk@0.3.259`, native `2.1.259`
Darwin arm64 at SHA-256
`884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898`,
Node `22.23.2`, sidecar tag `swallowtail-claude-agent-sdk-sidecar@0.4.4`,
mediated stdio/private loopback, MCP `2025-11-25`, `claude-sonnet-5`, default
permission, persistence false, `strictMcpConfig: true`, empty setting sources,
and omitted `allowedTools`.

## Producer Contradiction

The frozen SDK declaration permits each `McpServerStatus` row to carry
optional `serverInfo`, `error`, `config`, `scope`, and `tools` alongside
`name` and `status`. The current sidecar rejects any row containing `error`,
`config`, or `headers`; its fake SDK returns only `{name, status}`. The live
subcode proves that the current strict projection rejected the real status
result, but it does not prove which row field or shape differed.

This names a provider-free producer investigation, not authority to relax
validation blindly. Card 146 reconciles the exact `0.3.259` declaration and
implementation with the sidecar projection, adds faithful fixtures for every
admitted optional-field/status combination, and repairs only a confirmed
mismatch. Raw configuration, URLs, headers, errors, paths, and tool
descriptions remain non-projectable; safe projection discards them.

## Disposition

Card 145 is complete as a diagnostic evidence stop. Card 146 precedes any
credited qualification suite. Both `claude-agent.sdk -> registered_tools` and
`claude-agent.sdk -> consumer_tool_exchange` remain unqualified. No provider
limitation, quota behavior, qualification, candidate, tag, or release follows.
