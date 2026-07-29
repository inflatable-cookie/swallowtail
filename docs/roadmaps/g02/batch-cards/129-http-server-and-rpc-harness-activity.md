# 129 HTTP, Server, And RPC Harness Activity

Status: ready
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

- [ ] every selected event has exact activity ownership
- [ ] UI relay and model activity remain distinct
- [ ] recovery and reattachment do not duplicate activity
- [ ] authoritative persisted events retain exact provider ordering
- [ ] callbacks and provider-owned tools remain separate
- [ ] every profile matches the frozen corpus
- [ ] focused adapter regressions pass

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
