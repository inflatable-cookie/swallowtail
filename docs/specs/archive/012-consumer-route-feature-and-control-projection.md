# 012 Consumer Route Feature And Control Projection

Status: promoted
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Evidence: consumer route-feature and option census
Roadmap: g05.008
Promoted to: Contract 061

## Purpose

Give consuming applications one cohesive, descriptive projection of what an
exact configured route, model, and operation can do and which controls are
valid at each lifecycle point. Consumers should not need adapter downcasts,
provider command knowledge, or their own merge of catalogue, capability,
readiness, preparation, and negotiated-session records.

The projection does not authorize an operation, choose a route or model,
invent a default, or own consumer UI and routing policy.

## Evidence Baseline

The reviewed census in
[`2026-08-30-consumer-route-feature-and-option-projection-census.csv`](../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
contains 767 rows across all 48 current production routes:

- 555 feature rows
- 203 control rows
- 9 route-audit rows for routes with no route-specific composer control in
  current public types

It distinguishes 553 selection-summary, 142 session-start-only, 9 per-turn,
and 63 post-open-observation-only rows. It finds no proved mid-turn-negotiable
row. Exact requested/pending/effective/rejected acknowledgement exists only on
four named route paths. Matrices remain documentation cross-checks, not runtime
authority.

The source analysis and unsafe-inference register remain in the
[promoted triage note](../../triage/2026-08-30-consumer-route-feature-and-option-projection.md).

## Scope

In:

- one dedicated composing contract after Contract 060
- one public projection family with selection-summary, session-start, and
  active-session views
- stable semantic feature and control identity
- exact configured-instance, revision, route, model when applicable,
  operation-shape, access, resource, and evidence applicability
- typed value kind, admitted values or bounds, omission truth, and lifecycle
- explicit separation of descriptive support, current availability, request,
  prepared intent, provider-effective observation, and rejection
- immutable snapshot identity and replacement semantics
- bounded namespaced provider-native descriptors when portable identity would
  flatten route truth
- bounded safe reasons carried from authoritative source dimensions

Out:

- amendments to Contracts 037, 047, or 057
- an umbrella adapter registry or runtime enumeration of unlinked routes
- adapter-specific downcasts in consumer code
- a generic UI-schema language, composer layout, localization, or product copy
- consumer selection, defaults, routing, fallback, persistence, or policy
- raw credentials, targets, paths, commands, environment values, provider
  payloads, or unbounded diagnostics
- converting matrices into runtime authority
- new provider features, route claims, compatibility claims, or live evidence
- Rust types or implementation before the composing contract is active

## Settled Decisions

The operator settled these choices on 2026-08-31:

1. Promote the reviewed census as g05's next provisional spec and
   contract-discussion lane.
2. Use one dedicated composing contract, reserved as Contract 061. Do not
   amend Contracts 037, 047, or 057.
3. Do not define a closed portable availability-reason taxonomy in the first
   contract. Preserve existing authoritative source dimensions plus bounded
   safe reasons until cross-route evidence proves an exhaustive vocabulary.

Existing authority also settles these defaults:

- Contract 047 remains the immutable ready-to-select snapshot.
- Contract 057 remains the lifecycle in front of 047.
- Contract 037 remains the exact preparation and bound-operation boundary.
- display copy, localization, grouping, preference persistence, routing, and
  composer policy remain consumer-owned.
- a catalogue or negotiated option list does not authorize mutation.

No product-policy fork remains before Contract 061 drafting. Rust naming,
module placement, and implementation tranche selection remain later roadmap
work.

## Projection Views

### Selection Summary

Project bounded feature summaries for one exact configured instance and model
row. A summary may support badges, filtering, and safe explanatory posture. It
must not infer model applicability from a route-wide capability or present
current usability without the required readiness and prepared evidence.

### Session-Start Controls

Project only controls admitted by the exact selected route, model, operation
shape, access, resources, and preparation boundary. Each descriptor retains
its value kind, admitted values or bounds, omission behavior, and whether a
change requires a replacement session.

### Active-Session State

Project post-open observations and exact negotiated state without backdating
them into pre-session guarantees. Requested, pending, provider-confirmed
effective, and rejected values remain distinct. Observation-only option lists
do not become mutable controls. Between-turn or mid-turn mutation exists only
when separately qualified by the exact route mechanism; the census proves no
general mid-turn row.

## Descriptor Semantics

A feature or control descriptor must retain:

- stable portable or bounded namespaced semantic identity
- exact applicability and lifecycle
- authoritative source class and evidence identity
- support and current availability dimensions without one flattened boolean
- value kind, admitted domain or explicit unenumerated bound, and omission
  semantics for controls
- actor posture: informational, consumer-selectable, host-controlled,
  operator-controlled, provider-selected, or observation-only
- safe bounded reason when the authoritative source supplies one

Provider-native values cannot enter a portable enum without route, model,
version, and evidence qualification. Unknown or unenumerated stays explicit.
Omission does not become a Swallowtail default.

## Snapshot And Freshness

One projection snapshot binds the exact configured-instance id and revision,
route, model when applicable, operation shape, and identities of the source
evidence used to assemble it. It is immutable.

Readiness, catalogue, currentness, prepared-operation, or negotiated-session
changes produce a replacement projection. Contract 061 must not invent one
universal clock, timestamp, watcher, or refresh loop. Freshness remains the
truth carried by each authoritative source; the composing facade preserves it
and makes source replacement visible.

Cross-instance, cross-revision, cross-route, cross-model, cross-operation, or
stale-source assembly fails closed. Absence remains absence rather than an
unsupported claim.

## Availability And Safe Reasons

The first contract preserves existing source dimensions such as credential,
entitlement, endpoint authorization, runtime readiness, support authority,
catalogue result, capability constraints, preparation agreement, negotiated
state, and evidence freshness.

Unsupported, unavailable, conditional, unknown, and negotiated-only meanings
must remain distinguishable in projection behavior. Contract 061 must not claim
that those words form an exhaustive portable reason enum. Bounded safe reasons
may accompany the source dimensions; arbitrary provider payloads and consumer
presentation prose may not.

## Authority And Failure Boundary

The facade is descriptive. Actual requests still pass Contract 037 preparation,
capability, host-service, access, and plan agreement. A projected control never
bypasses preflight or proves provider acknowledgement.

Projection rejects or withholds a row when exact source identity or
applicability does not agree. It does not repair disagreement, choose a
fallback, silently downgrade, or widen a route claim.

## Review Oracle

Invariant: a cohesive projection preserves exact source and lifecycle truth
without creating execution or mutation authority.

Smallest adversarial counterexamples:

1. Combine a route-wide capability with a model or prepared operation that
   does not admit it. The projection must reject or withhold the usable row.
2. Combine a valid descriptor with a stale configured-instance revision. The
   projection must reject the mixed snapshot rather than mark it current.
3. Treat a post-open option list as a selectable or acknowledged mutation. The
   projection must keep it observation-only unless an exact mutation and
   acknowledgement path is supplied.
4. Replace missing source truth with an exhaustive availability reason. The
   projection must retain unknown or absent source state plus only a bounded
   safe reason.

Contract 061 must state the expected rejection or withholding point and make
these counterexamples directly testable by later conformance work.

## Acceptance Criteria

- [x] reviewed census covers every current production route and public
      consumer-selectable or observable feature/control found in source
- [x] operator selected a dedicated composing contract
- [x] operator deferred a closed availability-reason taxonomy
- [x] Contracts 037, 047, and 057 retain their current authority
- [x] Contract 061 owns the three views, exact source binding, lifecycle truth,
      safe-reason posture, and fail-closed composition rules
- [x] Contract 061 names testable rejection points for every review-oracle
      counterexample
- [x] the spec is archived only after Contract 061 and its index surfaces are
      active
- [x] implementation remains unplanned until post-contract reassessment

## Promotion Targets

- [Contract 061](../../contracts/061-consumer-route-feature-and-control-projection.md)
  is active
- contract index, summaries, and contracts front door updated
- this spec archived after contract promotion
- g05.008 card 021 closed through the contract-promotion log
- architecture only after code lands; architecture records realized structure
