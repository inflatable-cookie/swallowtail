# 085 Anthropic Managed Run Reconciliation And Cleanup

Status: planned
Owner: Tom
Created: 2026-08-04
Milestone: `../033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Depends on: card 084

## Goal

Persist exact Managed Agents recovery authority before work can be lost, then
reconcile and clean the exact recovered resources through separate roles.

## Scope

1. Refactor provisioning so checkpoint and cleanup records become observable
   after exact environment/session creation and before message submission.
2. Add bounded session retrieval plus paginated persisted-event reconciliation.
3. Map active, waiting, exact terminal, interrupt, and ambiguous termination
   evidence according to Contracts 022 and 048.
4. Add inactive-only recovered cleanup: session confirmation before environment deletion.
5. Preserve ordinary run interruption, terminal deletion, callback, retry, and
   lease ordering.

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`

## Stop Conditions

- stop if the checkpoint cannot be persisted before provider work can be lost
- stop on unbounded history, ambiguous exact attribution, or required active interrupt

## Auto-Continuation

Continue to card 086 after route conformance passes.
