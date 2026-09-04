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

## Result

Complete. The planning blocker is refreshed against the required factual
baseline commit
`c2e07762fc116d95c4ed051c3f8f1b2c3acc3d35`, not the stale sibling
`40e0cbea11fe29ba160913efaea1763dcb32466f`.

### Preflight and preserved reachability

- `git fetch origin` completed. The required
  `c2e07762fc116d95c4ed051c3f8f1b2c3acc3d35` is an ancestor of the fetched
  `origin/main`; origin advanced through `99e91aa8`, `629b2d16`, and
  `904785d4` while this repair was in flight. The sibling `40e0cbea` is not
  used as factual authority. The worker tree was clean before editing and
  the committed handoff declares `worker-pr-loop`, `planning`, and
  orchestrator dispatch authority.
- Against the required baseline, `swallowtail-adapter-kimi/src/driver.rs:75-86`
  remains reasoning-first and
  Plan-second. The reasoning confirmation is awaited and checked before
  `mode::prepare_plan_mode`, the Plan write, or the Plan confirmation.
- Reasoning remains `Absent`, `Effective(exact provider token)`, or
  `Rejected(exact provider token)` when requested. `RequestedNotDispatched` is
  unreachable for reasoning: a requested reasoning half that is not confirmed
  terminates the lifecycle before any projected outcome can expose it.
- Plan remains `Absent`, `Effective("plan")`, or
  `Rejected("default" | "auto" | "yolo")`. Its provider domain is frozen by
  `driver/mode.rs:10` to `default`, `plan`, `auto`, `yolo`, so Plan tokens are
  always admitted and within the retained bound.
- A requested Plan half becomes `RequestedNotDispatched` only when reasoning
  has already rejected and the terminal `?` path skipped Plan dispatch. It is
  not pending, does not imply a future acknowledgement, and never calls
  `ConsumerRouteStateSupport::with_pending()`.
- Missing, malformed, duplicated, ambiguous, unadvertised, transport, setup,
  foreign, and unretainable confirmations remain ordinary `Runtime` failure
  with no contribution. The retained note's pre-lifecycle mismatch branch and
  projection-only `DeclaredEffort`/requested-`"on"` branch remain unchanged.

### Accepted generic representation

Add one additive runtime-owned value and attach it to the existing row; do not
change `feature.active-session-reasoning-and-plan-ack` identity,
applicability, source identity, lifecycle, observation-only posture, or the
existing row-level state-support API.

```rust
pub enum ConsumerRouteAcknowledgementState {
    Absent,
    Effective(ConsumerRouteEnumerableValue),
    Rejected(ConsumerRouteEnumerableValue),
    RequestedNotDispatched,
}

pub struct ConsumerRouteCompoundAcknowledgement {
    reasoning: ConsumerRouteAcknowledgementState,
    plan: ConsumerRouteAcknowledgementState,
}

impl ConsumerRouteAcknowledgementState {
    pub const fn absent() -> Self;
    pub fn effective(value: ConsumerRouteEnumerableValue) -> Self;
    pub fn rejected(value: ConsumerRouteEnumerableValue) -> Self;
    pub const fn requested_not_dispatched() -> Self;
}

impl ConsumerRouteCompoundAcknowledgement {
    pub fn new(
        reasoning: ConsumerRouteAcknowledgementState,
        plan: ConsumerRouteAcknowledgementState,
    ) -> Result<Self, ConsumerRouteProjectionFailure>;
    pub const fn reasoning(&self) -> &ConsumerRouteAcknowledgementState;
    pub const fn plan(&self) -> &ConsumerRouteAcknowledgementState;
}
```

The row gains additive `with_compound_acknowledgement` and
`compound_acknowledgement` constructor/accessor methods. A generic consumer
reads `reasoning()` and `plan()` directly; no Kimi type or downcast is needed.
`ConsumerRouteEnumerableValue::new` remains the shared constructor and keeps
its existing 512-byte UTF-8, non-blank, control-free admission. Kimi applies
its already-retained adapter-local 128-byte exact-token bound and its exact
per-half admitted sets before constructing the generic value. It never exposes
the ACP payload or substitutes normalized `"on"` for the provider token.

`new` rejects impossible combinations with the existing
`ConsumerRouteProjectionFailureKind::ValueDomainInvalid` kind and the new
diagnostic `swallowtail.consumer_route_projection.acknowledgement_state_invalid`.
It rejects `RequestedNotDispatched` on reasoning, rejects it on Plan unless
reasoning is `Rejected`, and rejects `Effective`/`Rejected` values in an
`Absent` half. It accepts absent/effective/rejected exact values and the one
terminal Plan not-dispatched pairing above. No constructor sets pending.

The composer admits the attached value only on the exact existing row
identity, exact `kimi-code.acp` applicability, an
`ActiveSessionObservation` source, and completed post-open acknowledgement
evidence. It preserves the two half fields during snapshot replacement and
counts the compound value as one active-session row. It rejects the value on
prepared sources, provider-operation sources, a mismatched row identity,
missing active source, or non-observation evidence with the existing
applicability/source/evidence diagnostics. The row remains descriptive and
observation-only: the compound value authorizes no request, mutation, routing,
or acknowledgement.

### Required contract/runtime/testkit impact

Contract 061 needs one additive amendment stating that a compound
acknowledgement may carry independently state-associated halves, that exact
provider tokens are attached to only `Effective` or `Rejected`, and that
terminally undispatched is distinct from `Absent`, `Rejected`, and `pending`.
No existing source kind, lifecycle, view, state-support flag, fixed maximum,
or authority meaning changes.

The shared runtime baseline must export the two names above, the row
constructor/accessor pair, and the fixed diagnostic. The shared testkit must
add and call these exact assertions:

- `assert_compound_acknowledgement_associates_each_half_state`
- `assert_compound_acknowledgement_preserves_exact_provider_values`
- `assert_compound_acknowledgement_terminal_not_dispatched_is_distinct`
- `assert_compound_acknowledgement_rejects_impossible_half_combinations`
- `assert_compound_acknowledgement_preserves_reasoning_first_order`
- `assert_compound_acknowledgement_requires_observation_source`

The assertions cover effective reasoning plus rejected Plan, rejected
reasoning plus `RequestedNotDispatched` Plan, absent halves, exact token
association, rejection of speculative `RequestedNotObserved` reasoning, and
the absence of `with_pending()` for the terminal Plan state. The runtime
baseline is additive and must precede Card 034; Card 034 then adds only the
Kimi adapter projection and its route fixtures.

### Downstream decision and stop

The retained Kimi gate note is preserved unchanged as evidence. Chatterbox
should return the semantic decisions above to the operator, amend Contract
061, and dispatch one shared runtime/testkit baseline. Card 034 stays planned
and not ready until that baseline is merged. This card authorizes no Rust,
contract, architecture, adapter, or other triage-note edit and stops after
this one reviewable planning PR.

Zero Rust changes. Changed files are this card and the append-only
`PAPERCUTS.md` entry.
