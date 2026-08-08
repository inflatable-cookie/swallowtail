# 163 Runtime Plan-Family Consolidation

Status: done
Closeout: 2026-08-08
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

- [x] one shared core exists with per-role validation rules
- [x] the seven modules use it with unchanged behavior
- [x] validation drift between roles is gone
- [x] focused runtime rounds and the full workspace round pass

## Closeout

### Shared core (`runtime/src/plan_family.rs`)

- `PlanRule<A>` + `check_plan_rules`: ordered per-role validation rule tables;
  each role's `validate_plan` is now an explicit table (management 7 rules,
  reconciliation 9, run reconciliation 6, recovered cleanup 6, catalogue 7,
  import 8 shared rules + its candidate/attachment/working-resource tail).
  First-failing rule yields the exact original code and message, preserving
  failure ordering and diagnostics.
- shared `failure`, `validate_agreement_matches_plan`,
  `validate_execution_services`: every role's public request/execution
  validators are thin wrappers over these.
- `plan_family!` macro generates the plan, prepared-evidence, and typed
  request structs (public shape byte-identical) for the uniform roles:
  reconciliation, run reconciliation, and recovered cleanup use plan +
  prepared + request; catalogue uses plan only (its request carries a
  cursor); management's three typed requests replace the `typed_request!`
  macro (action check first, then scope, matching the original ordering).
- recorded as hand-written with the shared core: import's plan (three
  fields, `source_catalogue`) and request (extra `provider_session_ref`
  accessor); catalogue's cursor-carrying request; import's prepared evidence
  stays in `prepared.rs` to preserve the public API baseline.

### Drift made explicit

The audit's reconciliation-vs-management drift is now visible as tables:
reconciliation requires harness-interaction evidence, ambient read access
policy, and a working-resource service; management requires a scoped task
service plus action/initial-state rules; import and catalogue require
task + working-resource together. No role's acceptance changed.

### `.ok()` masking fixed

`attachment_fingerprint_for_checkpoint` now returns the real
`SessionResumeBindingPersistenceFailure` instead of `Option`; checkpoint
export and restore map the fingerprint failure into an `AttachmentMismatch`
failure carrying the fingerprint's own diagnostic, so the fingerprint
failure kind is no longer destroyed.

### Validation

- `effigy validate:focused swallowtail-runtime` passed
- `effigy test:rust`: 1505 tests passed
- `effigy package:api`: 28 packages at the unchanged v0.3.0 candidate
  baseline
- workspace suites: 138 green; runtime clippy clean

## Stop Conditions

- stop if any role's request, plan, or failure behavior changes

## Auto-Continuation

Yes, to card 164 after acceptance.

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy test:rust`, `effigy package:api`
