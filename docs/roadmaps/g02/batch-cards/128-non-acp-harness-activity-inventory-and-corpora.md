# 128 Non-ACP Harness Activity Inventory And Corpora

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../038-non-acp-harness-activity-coverage.md`
Depends on: card 127

## Goal

Classify every remaining production harness activity source and freeze exact
corpora before implementation.

## Scope

1. Audit:
   - OpenCode HTTP/SSE
   - Pi RPC
   - Kimi local server
   - Anthropic Managed Agents
   - Claude Code headless
   - Gemini CLI headless
   - Kimi Code headless
   - Qwen Code headless
2. Revalidate current authoritative documentation and exact maintained
   versions.
3. Record activity kinds, lifecycle fidelity, disclosure, correlation,
   unknown-event posture, and exact absences.
4. Freeze positive, completion-only, unknown, malformed, and failure fixtures.
5. Rank HTTP/server/RPC and headless implementation batches.
6. Make no production behavior change.

## Out Of Scope

- Codex and ACP routes
- direct model APIs
- parsing human terminal prose
- adding providers or transports

## Acceptance Criteria

- [ ] every remaining production harness route is accounted for
- [ ] every unstable claim cites current authoritative evidence
- [ ] all guaranteed version segments have exact corpus coverage or a stop
      gate
- [ ] harness-owned and consumer-owned tool activity remain separate
- [ ] exact absences remain visible
- [ ] implementation order is evidence-backed

## Validation

- focused fixture-decoder tests
- `effigy qa:docs`
- `effigy check:rust`

## Stop Conditions

- Pause one route when no machine-readable semantic source exists.
- Do not infer activity from human-facing stdout or stderr.

## Auto-Continuation

Continue to card 129 only for the exact contract-ready HTTP, server, RPC, and
managed-agent routes selected by the inventory.

