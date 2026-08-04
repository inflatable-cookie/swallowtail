# 086 Anthropic Recovery Prepared And Package Acceptance

Status: planned
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
