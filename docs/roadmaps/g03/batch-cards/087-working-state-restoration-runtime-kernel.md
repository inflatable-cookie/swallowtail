# 087 Working-State Restoration Runtime Kernel

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../034-working-state-restoration-facade.md`
Depends on: card 086

## Goal

Realize one consuming prepared facade over existing session reconciliation, run
reconciliation, and continuation recovery outcomes.

## Scope

1. Add explicit method and outcome vocabulary.
2. Add a continuation-recovery outcome with runtime turn, replay, and live session.
3. Add an object-safe exact-once prepared restoration operation.
4. Prove outcome preservation and consuming execution with provider-free tests.

## Validation

- `effigy validate:focused swallowtail-runtime`

## Stop Conditions

- stop if the facade requires provider routing or flattened provider state
- stop if recovery can claim terminal truth

## Auto-Continuation

Continue to card 088 when the runtime facade passes.

## Closeout

- added method, outcome, continuation-recovery, object-safe operation, and
  consuming prepared-facade vocabulary
- proved visible method selection and failure without fallback
- 134 focused runtime tests passed
