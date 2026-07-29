# 123 Codex App-Server Activity Projection

Status: planned
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

- [ ] non-message completed items no longer become empty progress
- [ ] every delta has exact item ownership
- [ ] completed items remain authoritative
- [ ] approval and tool completion remain separate
- [ ] file changes use current diff truth, not deprecated output emulation
- [ ] unknown semantic additions remain visible without raw payload
- [ ] every maintained version segment passes
- [ ] read-only and bounded-workspace facades remain distinct

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

