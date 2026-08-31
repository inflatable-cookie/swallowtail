# g05.008 Consumer Route Feature And Control Projection

Status: complete; Contract 061 active; implementation unplanned
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Depends on: reviewed consumer projection census; operator decisions recorded
Vision tags: consumer integration, explicit selection, route truth
Contract refs: 037, 047, 057; 061
Spec: archived 012
Planning state: card 021 complete; implementation unplanned

## Problem

Consumers can assemble route, model, capability, readiness, preparation, and
negotiated-session records, but no cohesive public projection preserves their
exact source and lifecycle truth. The reviewed 767-row census closes the
evidence gate. The operator selected one dedicated composing contract and
deferred a closed availability-reason taxonomy.

## Generation Runway Goal

Promote Spec 012 into Contract 061 without writing Rust, changing existing
contract authority, or selecting an implementation tranche.

## Goals

- [x] write Contract 061 from Spec 012
- [x] preserve Contracts 037, 047, and 057 unchanged
- [x] archive Spec 012 only after the contract and index surfaces are active
- [x] return to planning reassessment before any implementation roadmap

## Non-Goals

- Rust types, crate/module placement, adapters, fixtures, or public API
- amendments to Contracts 037, 047, or 057
- a closed exhaustive availability-reason taxonomy
- route enumeration, selection, defaults, routing, fallback, or consumer UI
- provider contact, live evidence, compatibility work, or route claims
- reopening watcher, skill-visibility, PR 127, or parked Bedrock work

## Execution Plan

### Batch 8.1 — Composing Contract Promotion

- [x] Execute ready card 021.
- [x] create Contract 061 with the three projection views, exact source and
      snapshot binding, lifecycle truth, safe-reason posture, and fail-closed
      composition rules
- [x] update contract/spec/log/roadmap indexes and archive Spec 012
- [x] close with implementation unplanned and one post-contract reassessment

## Acceptance Criteria

- [x] Contract 061 is active and composes 037, 047, and 057 without amending
      them
- [x] catalogue, capability, readiness, preparation, and negotiated evidence
      retain separate authority
- [x] selection-summary, session-start, per-turn, and active-session truth do
      not collapse
- [x] source dimensions plus bounded safe reasons remain; no exhaustive
      availability-reason enum is claimed
- [x] every Spec 012 review-oracle counterexample has an explicit failure or
      withholding point
- [x] no production code, architecture realization claim, provider work, or
      implementation card follows

## Decision Gates

- Stop if the contract needs an amendment to 037, 047, or 057.
- Stop if exact projection semantics require a new product-policy choice.
- Stop if a closed availability taxonomy, umbrella registry, consumer UI
  schema, or execution authority is introduced.
- Stop if contract review cannot identify the failure point for a stale,
  cross-boundary, or observation-as-mutation counterexample.

## Batch Cards

- [021 Consumer Route Feature And Control Projection Contract](batch-cards/021-consumer-route-feature-and-control-projection-contract.md)

## Outcome

Contract 061 is active. It composes Contracts 037, 047, and 057 without
amending them, keeps the three projection views and per-turn truth distinct,
binds one immutable snapshot to exact instance revision, route, model,
operation shape, and source evidence identity, preserves source availability
dimensions plus bounded safe reasons without an exhaustive taxonomy, and names
four fail-closed points covering every Spec 012 review-oracle counterexample.
Spec 012 is archived. No Rust, manifest, public API baseline, architecture
realization claim, or implementation card follows.

## References

- [Spec 012](../../specs/archive/012-consumer-route-feature-and-control-projection.md)
- [Contract 061](../../contracts/061-consumer-route-feature-and-control-projection.md)
- [Projection triage and census synthesis](../../triage/2026-08-30-consumer-route-feature-and-option-projection.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 047 Configured Provider Instance Catalogue](../../contracts/047-configured-provider-instance-catalogue.md)
- [Contract 057 Route Readiness And Connection Admission](../../contracts/057-route-readiness-and-connection-admission.md)
