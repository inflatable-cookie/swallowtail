# 093 Settled Session Attachment Contract

Status: ready
Owner: Tom
Created: 2026-08-05
Milestone: `../036-reconciliation-then-attachment-composition.md`
Depends on: card 092

## Goal

Extend Contract 050 with one explicit observe-then-attach sequence for exact
settled sessions.

## Scope

1. Define eligible, ineligible, failed, unknown, and stale first-phase outcomes.
2. Require both reconciliation and attachment preparation before provider work.
3. Preserve reconciliation evidence beside a distinct live attachment result.
4. Keep load/replay and replay-free resume distinct.
5. Specify exact consuming, cancellation, deadline, and partial-failure truth.

## Validation

- `effigy qa:docs`

## Stop Conditions

- stop if attachment could follow failed, active, waiting, or unknown evidence
- stop if a checkpoint or reconciliation result could mint attachment authority
- stop if route differences cannot fit explicit outcome variants

## Auto-Continuation

Continue to card 094 when the contract is testable without provider policy.
