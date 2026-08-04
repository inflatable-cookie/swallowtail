# 086 Anthropic Recovery Prepared And Package Acceptance

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Depends on: card 085

## Goal

Close exact Managed Agents recovery behind explicit prepared profiles, public
guidance, deterministic proof, and independently compiling packages.

## Scope

1. Expose separate ordinary and recoverable Managed Agents profiles.
2. Freeze checkpoint-before-dispatch, bounded pagination, waiting, terminal,
   ambiguous termination, and cleanup-order corpus cases.
3. Document consumer persistence, reconciliation, callback non-authority, and
   explicit cleanup sequencing.
4. Reconcile route and solution feature truth.
5. Run focused, affected-package, docs, and extracted-package acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
- `effigy package:verify-affected swallowtail-adapter-anthropic swallowtail-runtime`
- `effigy qa:docs`

## Stop Conditions

- no authenticated provider run or paid inference
- stop if prepared evidence conflates observation, callback, interrupt, or cleanup

## Auto-Continuation

Complete g03.033 and return to the g03 compatibility-maintenance checkpoint.

## Closeout

- documented separate ordinary and opt-in recoverable prepared run profiles
- documented opaque checkpoint and cleanup-binding persistence, exact route
  restoration, read-only reconciliation, callback non-authority, and explicit
  inactive cleanup sequencing
- updated the compile-tested example with ordinary run, recoverable run,
  reconciliation, and recovered-cleanup preparation paths
- reconciled the route matrix, architecture, and Contract 048 with realized
  Anthropic recovery; the combined feature CSV already carried the correct
  provider-managed-recovery and owned-cleanup `Yes` values
- split five touched files below Effigy's oversized warning threshold
- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
  — 188 tests passed; focused package check passed
- `effigy package:verify-affected swallowtail-adapter-anthropic swallowtail-runtime`
  — both independently extracted packages compiled
- `effigy qa:docs` — passed

`effigy doctor` still reports known repository-wide structural debt: 206
findings, including 19 errors. None of this card's split files remains in that
report. No authenticated provider run, paid inference, or live resource effect
ran.
