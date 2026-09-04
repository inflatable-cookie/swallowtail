# 076 Contract 061 Kimi Compound Acknowledgement Gate

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed cards 033, 070, and 073; retained Kimi gate evidence; current `main`

## Goal

Settle the one remaining planning blocker for card 034: a generic Contract 061
representation for the compound Kimi reasoning/Plan acknowledgement row that
preserves each half's state, represents terminally not-dispatched Plan truth,
and requires no adapter downcast or invented pending acknowledgement.

No Rust changes.

## Scope

1. Re-verify the fixed reasoning-first, Plan-second driver order and all
   reachable success and failure branches on current `main`.
2. Freeze the exact half-state matrix: absent, effective with exact provider
   token, rejected with exact provider token, and requested but terminally not
   dispatched. Prove which states are unreachable for each half.
3. Propose one minimal additive generic representation. Name every public type,
   variant, constructor/accessor, bound, admission rule, composer rule,
   diagnostic, and testkit assertion verbatim.
4. Keep `ConsumerRouteStateSupport::with_pending()` reserved for proven
   pending acknowledgement. A terminally undispatched Plan half must not use
   it and must not imply future arrival.
5. Preserve the existing row identity
   `feature.active-session-reasoning-and-plan-ack`, exact per-half association,
   exact provider values, route applicability, and observation-only authority.
6. Compare the proposal against current Contract 061 and decide explicitly
   whether it needs an amendment and shared runtime baseline before card 034.
7. Rewrite the retained Kimi gate note down to this accepted design and the
   exact downstream sequence. Fill this card's `## Result`.
8. Stop after one reviewable planning PR.

## Out Of Scope

Rust; card 034 implementation; provider-operation vocabulary already delivered
by card 073; Kimi version/containment claims; changing driver order; extra
provider work; adapter-only downcast escape hatches; another route or Candidate.

## Acceptance Criteria

- every reachable half/state combination is explicit and code-referenced
- a generic consumer can associate each exact token with its exact state
- terminally undispatched is distinct from pending, absent, and rejected
- one additive public shape is named exactly, with bounds and failure behavior
- contract/runtime/testkit impact and downstream serial edges are explicit
- zero Rust changes

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the shape reports only acknowledgement work that occurred and keeps
each half bound to its own state.

Smallest counterexample: an undispatched Plan half marked pending, row-level
flags that cannot associate state to half, a speculative reasoning state made
reachable, or an adapter downcast needed to interpret the row.

## Auto-Continuation

No. Chatterbox promotes the accepted result. Card 034 remains not ready until
any required contract/runtime baseline is merged.

## Stop Conditions

- materially different viable public shapes remain after analysis
- the design cannot stay additive
- a bound or generic state meaning requires operator policy not fixed by
  existing Contract 061 authority
- current code contradicts the retained reachability matrix

## Evidence

- [retained Kimi gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [card 034](034-contract-061-kimi-package-completion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)

