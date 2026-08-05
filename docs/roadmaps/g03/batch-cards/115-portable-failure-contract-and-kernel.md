# 115 Portable Failure Contract And Kernel

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../041-portable-failure-classification.md`
Depends on: card 114

## Goal

Promote and realize the provider-neutral failure classification without
breaking existing diagnostic or terminal construction.

## Scope

1. Promote Contract 051.
2. Add origin, kind, recovery, and classification records to core.
3. Add classified construction while keeping unknown the safe default.
4. Add a borrowed terminal failure view with exact terminal source.
5. Preserve preparation, cleanup, callback, and activity boundaries.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`

## Stop Conditions

- stop if classification requires parsing provider prose or raw payloads
- stop if terminal status or cleanup truth must be flattened

## Auto-Continuation

Continue to card 116 when common records and focused tests pass.

## Completion

- Contract 051 defines origin, kind, and bounded recovery evidence without
  replacing exact safe diagnostic codes.
- `SafeDiagnostic` now carries an all-unknown default classification and an
  evidence-backed classified constructor path.
- runtime terminal outcomes expose a borrowed failure view while provider,
  host, and runtime terminal sources remain distinct.
- warning-or-error activity may carry the same safe diagnostic; other activity
  kinds reject it.
- focused core and runtime validation passed: 206 tests plus package checks.
