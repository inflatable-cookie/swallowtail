# 079 Contract 061 Compound Acknowledgement Runtime Baseline

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061 compound-acknowledgement amendment of 2026-09-04; the accepted card 076 result; completed cards 022 and 073

## Goal

Realize the generic compound acknowledgement value exactly as card 076's
accepted result fixes it, in `swallowtail-runtime` and `swallowtail-testkit`
only. Card 034 then adds the Kimi projection and route fixtures.

## Scope

1. Add, verbatim from card 076's "Accepted generic representation":
   `ConsumerRouteAcknowledgementState` (`Absent`,
   `Effective(ConsumerRouteEnumerableValue)`,
   `Rejected(ConsumerRouteEnumerableValue)`, `RequestedNotDispatched`) with
   its four constructors, and `ConsumerRouteCompoundAcknowledgement` with
   `new`, `reasoning()`, and `plan()`.
2. Add the additive row methods `with_compound_acknowledgement` and
   `compound_acknowledgement`. Existing row identity, applicability, source,
   lifecycle, observation-only posture, and the row-level state-support API
   stay unchanged; no constructor sets pending.
3. `new` rejects with the existing `ValueDomainInvalid` kind and the new
   diagnostic
   `swallowtail.consumer_route_projection.acknowledgement_state_invalid`:
   `RequestedNotDispatched` on reasoning; `RequestedNotDispatched` on plan
   unless reasoning is `Rejected`; a token on an `Absent` half.
4. Composer admission: the value attaches only on the exact existing row
   identity with an `ActiveSessionObservation` source and completed post-open
   acknowledgement evidence; it counts as one active-session row; it is
   preserved across snapshot replacement; prepared or provider-operation
   sources, mismatched row identity, missing active source, or
   non-observation evidence reject with the existing diagnostics.
5. Add the six testkit assertions with the exact names from card 076 and
   call them from `assert_consumer_route_projection_contract()`.
6. Regenerate the runtime and testkit public API baseline files additively.
7. Keep every adapter untouched. Stop after one reviewable two-package PR.

## Out Of Scope

Adapter changes; card 034; any Kimi type in shared code; a pending state;
Contract 061 text (already amended); provider contact.

## Acceptance Criteria

- [ ] names, constructors, diagnostic, and admission match card 076 verbatim
- [ ] `RequestedNotObserved` is unreachable on reasoning and `with_pending()`
      is never set for the terminal plan state
- [ ] the six assertions pass and the complete suite calls them
- [ ] semantic API diff is additive only

## Validation

- `cargo fmt -p swallowtail-runtime -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Review Oracle

Invariant: each half carries its own state, exact tokens attach only to
effective or rejected halves, and terminal not-dispatched is never pending.
Smallest counterexample: a reachable not-dispatched reasoning half, a token on
an absent half, or a pending flag on the terminal plan state.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.

## Evidence

- [card 076](076-contract-061-kimi-compound-acknowledgement-gate.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [Kimi active-observation gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)

## Result

Complete. `swallowtail-runtime` now carries the accepted independently
state-associated reasoning and Plan acknowledgement value, its exact
reasoning-first admission rules, and the additive row attachment. Compound
truth admits only on the existing active-session reasoning-acknowledgement row
under completed wire observation and exact acknowledgement authority.
Terminal Plan non-dispatch remains distinct from pending and grants no new
operation authority.

`swallowtail-testkit` exports and runs all six named assertions. They prove
per-half association, exact provider values, absent halves, terminal
non-dispatch, impossible-state rejection, reasoning-first order, observation
source requirements, one-row composition, and source-identity replacement.
The runtime and testkit public API baselines contain additions only. Card 034
remains separate and no adapter, core, contract, or reserved closeout surface
changed.
