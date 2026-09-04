# 073 Contract 061 Provider-Operation Observation Runtime Baseline

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061 as amended 2026-09-04; the accepted card 070 gate note; completed card 022

## Goal

Realize the provider-operation observation vocabulary, admission, fourth
view, composer pass, fixed maximum, failure kind, and portable assertions
exactly as the
[accepted gate](../../../triage/20260904-161224-contract-061-provider-operation-observation-gate.md)
fixes them, in `swallowtail-runtime` and `swallowtail-testkit` only, without
touching any adapter.

## Scope

1. Add the public names verbatim from the gate's "Required public names":
   `ConsumerRouteProjectionSourceKind::ProviderOperationObservation`,
   `ConsumerRouteLifecycle::PostOperationObservationOnly`,
   `ConsumerRouteSourceClass::ProviderOperationOutcome`,
   `ConsumerRouteEvidenceStrength::CompletedProviderOperation`,
   `ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid`,
   `ConsumerRouteProviderOperationOutcome<'a>`,
   `ConsumerRouteProviderOperationObservation`,
   `ConsumerRouteProviderOperationState`,
   `ConsumerRouteProjectionInput::with_provider_operation_observations`,
   `ConsumerRouteProjection::provider_operation_state`, and
   `MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS = 4`. Existing constructors,
   accessors, and session-scoped meanings stay byte-identical.
2. Implement admission rules 1-8 and the five exact diagnostic codes and
   messages from the gate's "Admission" section. Ordinary
   `ConsumerRouteProjectionContribution::new` rejects the new source kind.
3. Add the fourth composer pass and the composition and replacement rules
   from the gate: co-compose without merging, distinct source IDs, exact
   applicability equality before entry, catalogue and history in separate
   snapshots, extension counting across all four views.
4. Retain the source plan privately on `ProviderSessionCatalogueOutcome` and
   `ProviderSessionHistoryPage` so admission can compare it with the
   supplied prepared evidence, without changing their public constructors
   or accessors.
5. Add the six testkit assertions with the exact names and bodies from the
   gate's "Draft Runtime And Testkit Assertions", and call the new group
   from `assert_consumer_route_projection_contract()`.
6. Update the runtime public API baseline evidence for the additive change.
7. Keep every adapter untouched. Candidate I completion and card 034 emit
   the observed rows in later cards.
8. Stop after one reviewable two-package PR.

## Out Of Scope

Adapter changes; emitting any provider-operation row from DeepSeek harness,
Kimi, or OpenCode; a third operation shape; payload projection; caller
limits; callbacks; registries; Contract 061 text (already amended); candidate
L; provider contact.

## Acceptance Criteria

- [ ] every public name, signature, maximum, diagnostic code, and message
      matches the gate verbatim
- [ ] existing session-scoped names and the existing five-argument
      contribution constructor are unchanged in meaning and signature
- [ ] the six assertions pass and the complete suite calls them
- [ ] a prepared plan, session-shaped evidence, failed outcome, or mismatched
      shape cannot construct an observation
- [ ] the semantic API diff is additive only
- [ ] touched source stays below god-file thresholds and the repository
      scan does not exceed its accepted baseline

## Review Oracle

Invariant: provider-operation observation is additive and outcome-backed.

Counterexamples, each a fail: an existing session-scoped name
reinterpreted; a prepared plan, successful preparation, session handle, or
failed operation admitted as observed outcome; an operation shape other
than catalogue or history admitted; a provider-operation row placed in an
existing view or an existing row placed in the fourth view; prepared and
outcome sources merged or sharing an ID; payload content projected; the
maximum exceeded or caller-replaced.

## Validation

- `cargo fmt -p swallowtail-runtime -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.

## Stop Conditions

- any name, maximum, or rule cannot be realized exactly as the gate fixes
  it (record the gap; return to Chatterbox; do not redesign)
- the change would not be additive on the semantic API baseline
- an adapter edit becomes necessary

## Evidence

- [accepted gate](../../../triage/20260904-161224-contract-061-provider-operation-observation-gate.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [card 022](022-contract-061-composer-and-two-route-vertical.md)
