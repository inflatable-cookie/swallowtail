# 163 Runtime Plan-Family Consolidation

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../053-claim-and-surface-consistency.md`
Depends on: card 162

## Goal

Consolidate the seven hand-rolled plan/agreement/request skeletons in the
runtime into one shared core with per-role validation tables.

## Scope

1. Extract one agreement/plan/request core in `swallowtail-runtime` covering
   the shared skeleton used by:
   `provider_session_operation`, `provider_session_reconciliation`,
   `provider_session_import`, `provider_run_reconciliation`,
   `provider_recovered_resource_cleanup`, `settled_session_restoration`, and
   `working_state_restoration`.
2. Replace the divergent shapes, including the `typed_request!` macro in
   `provider_session_operation.rs` and the hand-written sibling in
   `provider_session_reconciliation.rs`.
3. Express per-role validation differences as explicit tables or rules, and
   fix the drift the audit found (for example the reconciliation
   working-resource and ambient-harness checks versus the management service
   set).
4. Fix the lossy `.ok()` masking in `session_binding/persistence.rs:14-16`
   and `provider_operation_checkpoint.rs:84-89,124-129` so the real
   fingerprint failure kind is not destroyed.

## Out Of Scope

- public API, request, plan, or diagnostic changes
- provider or route behavior changes

## Acceptance

- [ ] one shared core exists with per-role validation rules
- [ ] the seven modules use it with unchanged behavior
- [ ] validation drift between roles is gone
- [ ] focused runtime rounds and the full workspace round pass

## Stop Conditions

- stop if any role's request, plan, or failure behavior changes

## Auto-Continuation

Yes, to card 164 after acceptance.

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy test:rust`, `effigy package:api`
