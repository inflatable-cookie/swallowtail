# 094 Restoration Sequence Runtime Kernel

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../036-reconciliation-then-attachment-composition.md`
Depends on: card 093

## Goal

Realize the consuming observe-then-attach sequence and provider-free
conformance defined by Contract 050.

## Scope

1. Add explicit prepared sequence and outcome vocabulary.
2. Preserve the complete first-phase outcome on every return path.
3. Invoke attachment only for contract-eligible settled evidence.
4. Prove no second dispatch after first-phase failure or ineligible state.
5. Bound consuming execution, cancellation, deadline, and cleanup behavior.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit`

## Stop Conditions

- stop if the kernel needs provider identifiers or route selection
- stop if attachment failure erases successful reconciliation evidence

## Auto-Continuation

Continue to card 095 when provider-free conformance passes.

## Outcome

- added one consuming `PreparedSettledSessionRestoration` sequence
- added distinct loaded and replay-free resumed attachment outcomes
- added phase-aware failures which retain completed reconciliation
- proved eligibility, ordering, first-phase stop, partial failure, and method
  mismatch without provider identity or route selection
- focused runtime and testkit validation passed: 219 tests
