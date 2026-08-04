# 084 Provider Recovered Resource Cleanup Kernel

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Depends on: card 083

## Goal

Represent provider-input wait and explicitly clean one exact driver-owned
resource group recovered after process loss without widening reconciliation.

## Scope

1. Add `WaitingForProviderInput` to run reconciliation state.
2. Add bounded versioned persisted owned-resource cleanup bindings.
3. Bind runtime run, provider run, exact resources, and prepared attachment.
4. Add a separate capability, operation shape, driver role, plan, request, and
   effect-truth outcome for recovered cleanup.
5. Freeze corruption, drift, cross-operation, active-resource, cancellation,
   deadline, and partial-effect conformance.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`

## Stop Conditions

- stop if cleanup can be admitted by a reconciliation checkpoint or raw id
- stop if the portable role implies interrupt, retry, or provider-specific order

## Auto-Continuation

Continue to card 085 when the provider-neutral contract and conformance pass.

## Closeout

- added non-terminal `WaitingForProviderInput` run reconciliation truth
- added a bounded, versioned, integrity-checked cleanup binding distinct from
  the read-only run checkpoint
- bound runtime run, provider run, typed resource kinds, opaque exact provider
  resources, route fingerprint, and prepared access attachment
- added the separate cleanup capability, operation shape, role, cancellation
  scope, immutable plan/request, and effect-truth outcome
- froze malformed, oversized, version, corruption, route drift,
  cross-operation, cancellation-scope, deadline, active-resource, and partial-
  effect rejection or preservation
- `effigy validate:focused swallowtail-core swallowtail-runtime` — 194 tests passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime` — both
  extracted packages compiled

No authenticated provider work ran. Card 085 is ready.
