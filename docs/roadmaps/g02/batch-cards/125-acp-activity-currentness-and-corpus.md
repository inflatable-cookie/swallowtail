# 125 ACP Activity Currentness And Corpus

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../037-acp-observable-agent-activity.md`
Depends on: card 124

## Goal

Freeze current ACP activity semantics and exact Claude Agent, Gemini CLI, and
Kimi Code behavior milestones before shared projection.

## Scope

1. Revalidate the maintained ACP schema, SDK, and transport authority.
2. Revalidate each selected harness's exact schema and version range.
3. Freeze bounded fixtures for message chunks, thought chunks, plans, tool
   calls, tool updates, usage, modes, commands, unknown updates, permission
   requests, and completion.
4. Preserve provider-specific fields and exclusions separately.
5. Cover stdio framing and the existing explicit remote transport corpus.
6. Make no production behavior change.

## Out Of Scope

- Grok Build activation
- provider identity inferred from ACP
- production mapping
- live harness installation or authentication

## Acceptance Criteria

- [ ] current ACP authority and exact schema version are recorded
- [ ] every guaranteed harness segment has fixture provenance
- [ ] thought chunks are classified as provider-visible display content or
      excluded
- [ ] tool and plan lifecycle semantics are explicit
- [ ] stdio and remote transport identity remain separate
- [ ] unknown and malformed updates have deterministic outcomes

## Validation

- ACP protocol fixture tests
- adapter fixture decoders
- `effigy qa:docs`
- `effigy check:rust`

## Stop Conditions

- Stop if current ACP semantics conflict with Contract 044.
- Keep provider-specific decoding outside the shared boundary when required.

## Auto-Continuation

Continue to card 126 only after the shared and provider-specific corpus is
complete.

