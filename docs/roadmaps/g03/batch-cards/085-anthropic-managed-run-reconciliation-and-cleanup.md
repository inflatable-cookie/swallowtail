# 085 Anthropic Managed Run Reconciliation And Cleanup

Status: completed
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

## Closeout

- added an explicit recoverable Managed Agents run profile while preserving the
  ordinary delete-on-close profile
- emitted separate exact-run checkpoint and recovered-resource cleanup records
  after environment/session creation and before message submission
- added bounded exact session retrieval and at most eight persisted-event pages
  with a 2,048-event ceiling, cursor-cycle rejection, and contradiction checks
- mapped running, waiting, completed, failed, cancelled, inactive unresolved,
  and unknown state from exact ordered provider evidence
- kept reconciliation read-only and cleanup inactive-only, non-retrying, and
  ordered: confirmed session deletion before environment deletion
- preserved cancellation and elapsed-deadline truth before provider reads or
  cleanup effects
- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
  — 188 tests passed; focused package check passed

No authenticated provider work ran. Card 086 is ready for broader prepared,
public-truth, docs, and extracted-package acceptance.
