# 084 Provider Recovered Resource Cleanup Kernel

Status: ready
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
