# 258 g04 Closeout Inventory And Deferred Disposition

Status: completed
Owner: Tom
Created: 2026-08-28
Milestone: `../091-generation-closeout-and-g05-cutover.md`

## Goal

Prove g04 can close without losing unfinished authority or turning parked and
standing work into active backlog.

## Scope

1. Reconcile all g04 milestone and batch-card statuses.
2. Confirm the per-route ledger has no active qualification or delivery row.
3. Preserve Bedrock items 79-80 as parked.
4. Preserve Contract 029 currentness as standing and generation-independent.
5. Keep hosted OAuth, OpenHands, Aider, and Kiro headless parked outside the
   active runway.
6. Promote the sole open product triage family only as g05 evidence work.

## Acceptance Criteria

- [x] g04 has 91 roadmaps after the closeout milestone
- [x] the 85-item ledger remains exactly 83 closed and two parked
- [x] no ready g04 card remains
- [x] parked route surfaces keep no implied revisit date
- [x] no provider, consumer, or release behavior changes

## Validation

- roadmap, card, inventory, backlog, triage, and standing-lane reconciliation
- `git diff --check`

## Auto-Continuation

Yes, into card 259.
