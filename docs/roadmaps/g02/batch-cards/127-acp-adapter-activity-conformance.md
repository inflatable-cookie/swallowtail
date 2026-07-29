# 127 ACP Adapter Activity Conformance

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../037-acp-observable-agent-activity.md`
Depends on: card 126

## Goal

Map exact ACP activity through Claude Agent, Gemini CLI, and Kimi Code
prepared operations and close the shared protocol lane.

## Scope

1. Map message, reasoning-summary, plan, tool, and exact provider-specific
   activity in all three adapters.
2. Publish exact route profiles per operation and version segment.
3. Correlate permission requests and callback settlement.
4. Preserve Gemini modes and writes, Kimi load/resume/retention, Claude
   access profiles, and every route's existing lifecycle.
5. Prove local and remote-authoritative host seams where currently supported.
6. Run full ACP adapter and transport regression.

## Out Of Scope

- widening any write or approval authority
- shared provider selection
- Grok Build
- consumer storage or UI

## Acceptance Criteria

- [x] tool and plan updates no longer become empty progress
- [x] message and thought chunks have stable activity ownership
- [x] each adapter exposes its exact profile
- [x] no provider identity comes from the transport
- [x] cancellation, deadlines, callbacks, continuity, and cleanup remain exact
- [x] all guaranteed interface segments pass offline
- [x] unverified-newer admission does not widen activity fidelity

## Validation

- complete Claude Agent adapter tests
- complete Gemini adapter ACP tests
- complete Kimi adapter ACP tests
- remote ACP transport regression
- `effigy check:rust`
- `effigy package:api`

## Stop Conditions

- Stop one adapter on unresolved provider-specific semantic drift.
- Do not weaken the other adapters or shared protocol to force parity.

## Auto-Continuation

Continue to card 128 only after roadmap g02.037 closes.

## Completion Evidence

- Claude Agent, Gemini CLI, and Kimi Code use the shared bounded ACP decoder,
  then make adapter-owned message, thought, plan, tool, warning, metadata, and
  unknown-event decisions.
- Prepared session and run routes publish exact activity profiles bound to
  the qualified behavior revision. Permitted newer versions inherit the last
  qualified profile.
- Existing output, reasoning, permission, callback, cancellation, continuity,
  retention, access, and joined cleanup paths remain separate.
- Adapter projection tests pass for all three routes. ACP driver suites pass
  at 13, 6, and 5 tests. Prepared facade suites pass at 12, 4, and 5 tests,
  including activity-trace conformance.
- `effigy format:check`, `effigy check:rust`, and `effigy package:api` pass.
- Two standalone corpus binaries and the standalone remote-transport test
  were compiled but stopped before test startup by a macOS provenance hold.
  The same exact corpora are exercised by decoder and adapter tests; local
  and remote-authoritative adapter topology suites pass.
