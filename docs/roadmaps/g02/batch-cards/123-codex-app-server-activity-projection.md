# 123 Codex App-Server Activity Projection

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../036-codex-observable-activity-fidelity.md`
Depends on: card 122

## Goal

Map Codex app-server's documented rich event surface into portable observable
activity.

## Scope

1. Map stable item identity and start, update, and completion lifecycle.
2. Map assistant commentary, final answers, plans, reasoning summaries,
   commands, files, tools, search, images, subagents, review, compaction,
   tasks, and hooks where qualified.
3. Correlate approvals, callbacks, dynamic tools, and request resolution.
4. Preserve command output, exit status, duration, and file diffs as bounded
   operation content.
5. Emit bounded namespaced unknown activity for safely classified additive
   semantic events.
6. Publish the exact app-server activity profile through prepared evidence.
7. Preserve all access, workspace, cancellation, deadline, session, and
   cleanup behavior.

## Out Of Scope

- raw reasoning text
- undocumented provider fields
- exec mapping
- consumer work-log persistence or UI

## Acceptance Criteria

- [x] non-message completed items no longer become empty progress
- [x] every delta has exact item ownership
- [x] completed items remain authoritative
- [x] approval and tool completion remain separate
- [x] file changes use current diff truth, not deprecated output emulation
- [x] unknown semantic additions remain visible without raw payload
- [x] every maintained version segment passes
- [x] read-only and bounded-workspace facades remain distinct

## Result

- Added a dedicated Codex app-server projector for stable item lifecycle,
  assistant phases, readable reasoning summaries, plans, commands, file
  changes, MCP and dynamic tools, search, images, collaboration, review,
  compaction, hooks, provider requests, and bounded namespaced unknowns.
- Preserved command text, cwd, bounded output, exit status, duration, and file
  diffs as redacted-by-default operation content.
- Added separate callback-correlated consumer-tool activity and
  provider-request-correlated request activity. Approval/request lifecycle
  does not stand in for tool completion.
- Kept raw reasoning deltas and reasoning content out of portable activity.
- Added `ProviderUnspecified` assistant phase for older qualified messages.
  It carries identity-only disclosure rather than guessed commentary or
  final-answer content.
- Prepared read-only and bounded-workspace sessions now require ordered
  streaming and publish an exact immutable app-server activity profile.
- Stable `0.146.0` inherits the `0.145.0` profile without widening it.

## Validation Evidence

- Codex activity projector unit tests — 3 passed
- Codex app-server integration tests — 15 passed
- Codex prepared facade tests — 19 passed
- runtime activity and event-buffer tests — 14 passed
- complete Codex adapter suite — 120 passed
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`
- `effigy doctor` — unchanged 111 findings: 83 warnings and 28 errors

## Validation

- focused Codex app-server fixture tests
- Codex prepared facade and range suites
- `effigy check:rust`
- `effigy package:api`

## Stop Conditions

- Stop on unsafe content disclosure or identity mismatch.
- Fail closed rather than attach a semantic delta to an inferred item.

## Auto-Continuation

Continue to card 124 after app-server range and facade conformance passes.
