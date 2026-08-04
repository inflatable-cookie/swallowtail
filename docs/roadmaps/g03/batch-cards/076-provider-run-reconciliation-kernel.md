# 076 Provider Run Reconciliation Kernel

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../030-openai-background-run-reconciliation-and-detachment.md`
Depends on: card 075

## Goal

Persist and reconcile one exact provider-owned structured run without
manufacturing provider-session identity.

## Scope

1. Add bounded persisted `ProviderRunCheckpoint` records.
2. Bind runtime run, provider run, opaque cursor, and prepared route fingerprint.
3. Carry qualified checkpoints on runtime events.
4. Add the distinct run reconciliation capability, role, plan, request, and outcome.
5. Freeze corruption, drift, output-bound, and exact-correlation failures.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`

## Stop Conditions

- stop if the route binding cannot exclude credential material while preserving exact attachment
- stop if run reconciliation requires provider-session identity or state-changing authority

## Auto-Continuation

Continue to card 077 when the portable conformance surface passes.
