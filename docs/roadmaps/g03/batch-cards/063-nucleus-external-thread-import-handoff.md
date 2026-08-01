# 063 Nucleus External Thread Import Handoff

Status: planned
Owner: Tom
Created: 2026-08-01
Milestone: `../023-provider-session-import-acceptance-and-handoff.md`
Depends on: card 062

## Goal

Produce a bounded Nucleus adoption handoff for external-thread browsing and
explicit import while leaving all consumer persistence and presentation in
Nucleus.

## Scope

1. Describe the consumer flow: prepare catalogue, browse, select, import,
   replay, persist, and resume.
2. Define consumer-owned provider-binding mapping, replay persistence,
   duplicate detection, and stale-candidate handling.
3. Define unsupported-route, partial-history, and reauthorization UX needs.
4. Identify deterministic fixtures Nucleus can consume without a live provider.
5. Record the handoff without editing Nucleus.

## Out Of Scope

- any Nucleus, Soundcheck, or other consumer edit
- a Swallowtail database, repository scanner, thread model, or UI
- background polling, bidirectional synchronization, or merge policy
- provider-session management binding persistence

## Acceptance Criteria

- [ ] the handoff maps each consumer responsibility explicitly
- [ ] imported history and future events have a clear deduplication boundary
- [ ] unsupported and stale routes cannot appear resumable
- [ ] the handoff identifies the exact prepared Swallowtail entry points
- [ ] no consumer policy enters Swallowtail contracts
- [ ] the sole Next Task returns to the g03 evidence gate

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no consumer checkout or test suite

## Auto-Continuation

No. Return to the g03 evidence gate after publishing the handoff.
