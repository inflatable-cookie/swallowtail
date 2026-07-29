# 128 Non-ACP Harness Activity Inventory And Corpora

Status: completed
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

- [x] every remaining production harness route is accounted for
- [x] every unstable claim cites current authoritative evidence
- [x] all guaranteed version segments have exact corpus coverage or a stop
      gate
- [x] harness-owned and consumer-owned tool activity remain separate
- [x] exact absences remain visible
- [x] implementation order is evidence-backed

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

## Completion Evidence

- Research 066 accounts for all eight routes and records exact currentness,
  lifecycle, disclosure, correlation, ownership, unknown posture, and absence.
- Contract 044 now binds activity-affecting partial and preview options into
  immutable prepared route profiles.
- The testkit inventory plus adapter-local fixtures freeze native lifecycle,
  completion-only, partial, unknown, malformed, and failure cases.
- OpenCode's 45 qualified releases, exact `1.14.51` gap, and Kimi's two
  local-server event-schema segments are machine-checked.
- Focused corpus tests: 4 passed.
- `effigy qa:docs` and `effigy check:rust` passed.
- Card 129 is ready. Cards 130-137 remain in bounds.
