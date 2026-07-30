# 129 HTTP, Server, And RPC Harness Activity

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../038-non-acp-harness-activity-coverage.md`
Depends on: card 128

## Goal

Map exact observable activity for the selected non-ACP HTTP, server,
WebSocket, RPC, and managed-agent harness routes.

## Scope

1. Implement selected OpenCode HTTP/SSE activity mapping.
2. Implement selected Pi RPC activity and UI-relay correlation.
3. Implement selected Kimi local-server REST/WebSocket activity mapping.
4. Implement selected Anthropic Managed Agent authoritative activity mapping.
5. Publish exact route profiles.
6. Preserve provider retention, recovery, reattachment, callbacks, server
   ownership, session management, and cleanup.

## Execution Order

1. Pi RPC native message and tool lifecycle.
2. Kimi local-server cursor, step, tool, subagent, and callback coexistence.
3. OpenCode range-segmented SSE, including exact `1.14.51` thinning.
4. Managed Agents authoritative completion records and split tool ownership.

## Out Of Scope

- headless JSON/JSONL routes
- provider-state or recovery changes
- consumer presentation
- routes not selected by card 128

## Acceptance Criteria

- [x] every selected event has exact activity ownership
- [x] UI relay and model activity remain distinct
- [x] recovery and reattachment do not duplicate activity
- [x] authoritative persisted events retain exact provider ordering
- [x] callbacks and provider-owned tools remain separate
- [x] every profile matches the frozen corpus
- [x] focused adapter regressions pass

## Validation

- selected OpenCode, Pi, Kimi, and Anthropic adapter tests
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`

## Stop Conditions

- Stop one route on unsafe identity, cursor, or disclosure ambiguity.
- Keep non-selected routes unchanged.

## Auto-Continuation

Continue to card 130 after every selected route passes focused conformance.

## Completion Evidence

- Pi RPC projects message, readable thinking, provider-owned tool, compaction,
  warning, and namespaced unknown activity on the existing turn stream. UI
  callbacks remain separate.
- Kimi local server projects accepted WebSocket events only after durable
  cursor admission. Step, message, thought, tool, shell, subagent, task,
  compaction, retry, warning, and unknown truth retain exact route identity.
- OpenCode projects correlated SSE message, reasoning, tool, step, warning,
  and unknown activity. Exact `1.14.51` remains intentionally thin.
- Managed Agents projects authoritative persisted completions after event
  deduplication. Provider and MCP tools remain provider-owned; custom tools
  remain callback exchange.
- Every prepared route publishes an exact observable-activity profile bound
  to its qualified behavior revision. Permitted newer versions inherit the
  last guarantee without widening fidelity.
- Complete Pi, Kimi, OpenCode, and Anthropic adapter suites pass. Rust check,
  lint, public-API, formatting, and docs gates pass.
- No executable, credential, account, server, model request, or paid inference
  was used.
