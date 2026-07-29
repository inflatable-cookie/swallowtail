# 134 Realtime And Non-Applicable Activity Closeout

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../039-direct-inference-activity-truth.md`
Depends on: card 133

## Goal

Prove the boundary between ordinary observable activity, realtime-media
events, catalogue operations, and serving lifecycle.

## Scope

1. Verify OpenAI Realtime and Gemini Live keep audio, transcript, commit,
   interruption, and rollover on their dedicated event model.
2. Correlate only exact shared tool or provider observations selected by card
   132.
3. Prove catalogue-only and serving-only operations expose no agent activity
   capability.
4. Prove attached-runtime inference does not claim server-owned agent work.
5. Complete the direct-route activity inventory.
6. Run direct, realtime, serving, and package-facing regressions.

## Out Of Scope

- a common text and media event abstraction
- new voice routes
- model server ownership changes
- consumer media or chat UI

## Acceptance Criteria

- [ ] realtime response lifecycle remains exact
- [ ] shared activity appears only where independently qualified
- [ ] catalogue and serving-only profiles are not applicable
- [ ] attached runtime ownership remains unchanged
- [ ] no fake agent work appears on direct routes
- [ ] every direct-route classification is machine-checkable

## Validation

- OpenAI Realtime adapter tests
- Gemini Live adapter tests
- attached and owned runtime tests
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:routes`
- `effigy package:api`

## Stop Conditions

- Stop if a shared event would flatten media sequencing.
- Retain the dedicated realtime surface over superficial commonality.

## Auto-Continuation

Continue to card 135 only after roadmap g02.039 closes.

