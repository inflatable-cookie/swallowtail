# 074 Kimi Exact-Turn Reconciliation

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../029-kimi-operation-checkpoint-reconciliation-and-detachment.md`
Depends on: card 073

## Goal

Restore a persisted Kimi cursor and observe one exact turn through the finite
server-acknowledged replay window.

## Scope

1. Add the prepared exact reconciliation surface.
2. Preserve subscribe acknowledgement cursor truth.
3. Validate exact session, cwd, epoch, sequence, and turn.
4. Map exact terminal, waiting, active, and unresolved state honestly.
5. Freeze restart, stale, foreign, resync, and no-side-effect evidence.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-kimi`

Passed in the cards 073-075 validation batch.

## Auto-Continuation

Completed with cards 073 and 075 in the same validation batch.
