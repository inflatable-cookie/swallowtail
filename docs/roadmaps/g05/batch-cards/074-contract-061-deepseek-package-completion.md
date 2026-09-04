# 074 Contract 061 DeepSeek And DeepSeek Harness Package Completion

Status: complete
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed cards 066 and 073; Contract 061 as amended for provider-operation observation; current `main`

## Goal

Complete Candidate I as one exact 47-row Contract 061 tranche across
`swallowtail-adapter-deepseek` and `swallowtail-adapter-deepseek-harness`.
Publish the two completed local-server catalogue/history operations through
the provider-operation view added by card 073. Keep the six matrix-only rows
withheld and change no shared runtime vocabulary.

## Scope

1. Implement deterministic contribution ledgers for exactly 19
   `deepseek.continuation`, 11 `deepseek-harness.jsonrpc`, and 17
   `deepseek-harness.local-server` census tuples.
2. Emit 41 rows and construction-time withhold 6 rows, matching the accepted
   Candidate I audit. No filter or exception list is permitted.
3. Use ordinary prepared contributions for the 39 already representable rows.
   Admit local-server `control.provider-session-catalogue` and
   `control.provider-session-history` only from matching successful completed
   provider-operation outcomes, through distinct prepared and outcome source
   identities and the card 073 provider-operation view.
4. Do not present preparation, an open session, a failed query, payload
   contents, or documentation as completed-operation observation.
5. Preserve driver, pagination, cleanup, continuation, failure, and provider
   contact behavior. Projection adds no provider work.
6. Correct only the source-reference cells in
   `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`
   recorded by the accepted audit for imported core/runtime types,
   `ActivityObservation`, `ModelRoute`, and the two retained effective
   outcomes. Change no census row identity or support disposition.
7. Add provider-free, mutation-sensitive fixtures and assertions for every
   route ledger, both operation observations, the six withholdings, source
   separation, applicability, and fixed maxima.
8. Stop after one reviewable two-package PR.

## Out Of Scope

Shared runtime/core/testkit API changes; Contract or architecture changes;
Kimi, Gemini, Grok, OpenCode, or another Candidate; provider contact; live
sessions; version claims; payload projection; new operation shapes.

## Acceptance Criteria

- all 47 tuples appear exactly once as emitted or withheld
- totals are 41 emitted / 6 withheld / 47 reconciled
- the two local-server observed controls exist only in the provider-operation
  view and only after matching successful completed outcomes
- prepared and outcome sources remain distinct and cannot share an ID
- the accepted census-source corrections land without changing row meaning
- semantic API changes are zero and no provider work is added

## Validation

- `cargo fmt -p swallowtail-adapter-deepseek -p swallowtail-adapter-deepseek-harness -- --check`
- `effigy validate:focused swallowtail-adapter-deepseek swallowtail-adapter-deepseek-harness`
- `effigy package:verify-affected swallowtail-adapter-deepseek swallowtail-adapter-deepseek-harness`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Review Oracle

Invariant: every published row is backed by exact route-local evidence at the
lifecycle where it is claimed.

Smallest counterexample: either observed control appears from prepared
evidence, a failed or mismatched outcome publishes a row, one of the six
withholdings is emitted, or the 47-row ledger needs an exception.

## Auto-Continuation

No. Stop for exact-head review.

## Stop Conditions

- card 073's public surface cannot represent either operation exactly
- an adapter requires new shared vocabulary, a larger fixed maximum, or a
  changed composer rule
- projection would change provider work, paging, cleanup, or failure behavior
- the exact 41/6/47 ledger does not reconcile

## Evidence

- Candidate I audit note `docs/triage/20260904-140002-contract-061-candidate-i-audit.md`, pruned after this card closed; preserved in Git history
- [card 066](066-contract-061-candidate-i-breadth-audit.md)
- [card 073](073-contract-061-provider-operation-observation-baseline.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)

## Result

Completed Candidate I across both DeepSeek adapter packages. Exact prepared
facades now publish 39 route-local rows. Six matrix-only rows remain withheld
at construction. Completed local-server catalogue and history outcomes admit
the final two rows only through provider-operation observation, with distinct
prepared and outcome source identities. The ledger remains exact: 41 emitted,
6 withheld, 47 reconciled.

Corrected only the accepted census source references: imported shared types,
`ActivityObservation`, the direct `ModelRoute` construction site, and the
retained fork/archive outcome source. Provider execution, paging, cleanup,
continuation, failure, and contact behavior are unchanged.

Merged through PR 217 at `8cb811f2`; the exact 41/6/47 Candidate I tranche is
now part of the realized Contract 061 coverage.
