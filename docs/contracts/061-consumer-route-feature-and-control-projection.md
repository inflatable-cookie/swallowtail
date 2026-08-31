# 061 Consumer Route Feature And Control Projection

Status: active
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Spec: archived 012
Evidence: reviewed consumer route-feature and option census

## Purpose

Give consuming applications one cohesive descriptive projection of what an
exact configured instance, route, model, and operation supports, and which
controls are valid at each lifecycle point. Consumers must not need adapter
downcasts, provider command knowledge, or their own merge of catalogue,
capability, readiness, preparation, and negotiated-session records.

The projection composes evidence that already exists. It does not authorize an
operation, choose a route or model, invent a default, mutate a session, or own
consumer layout, localization, persistence, or routing policy.

## Composition Authority

This contract is a composing facade. Every source keeps its own authority:

- Contract 047 remains the immutable ready-to-select snapshot.
- Contract 057 remains the addable-route, admission, readiness, subject, and
  overlay lifecycle in front of 047.
- Contract 037 remains the exact preparation and bound-operation boundary.
- Contracts 006, 008, and 020 remain the access-dimension, capability, and
  model-catalogue authorities.
- Contracts 012, 034, 040, and 041 remain the session-option, negotiated
  harness-option, generation-control, and input/callback admission
  authorities.

Contract 061 amends none of them and grants none of them new reach. It
publishes only what those records already prove, under the identity and
lifecycle they already carry. A projected row is evidence, not executable
authority.

The authoritative source classes are the immutable configured-instance and
prepared-operation records, portable capability profiles and constraints,
model-catalogue observations, public runtime request and session-option types,
adapter prepared inputs and validation, and route-driver or wire
acknowledgement evidence. Provider and solution feature matrices remain
documentation and QA cross-checks. They never establish accepted values, exact
model applicability, current availability, or provider-effective state.

## Boundary

In:

- one public projection family with selection-summary, session-start, and
  active-session views
- stable semantic feature and control identity shared by all three views
- exact configured-instance id and revision, route, model where applicable,
  operation shape, access, resource, and evidence applicability
- typed value kind, admitted values or an explicit unenumerated bound,
  omission truth, and lifecycle for controls
- separation of descriptive support, current availability, request, prepared
  intent, provider-effective observation, and rejection
- immutable snapshot identity and replacement semantics
- bounded namespaced provider-native descriptors where portable identity would
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
- converting documentation matrices into runtime authority
- an exhaustive portable availability-reason enum
- new provider features, route claims, compatibility claims, or live evidence
- execution, mutation, acknowledgement, or any preflight bypass

## Projection Views

The three views share one semantic vocabulary and one snapshot identity. They
remain separate surfaces because their evidence and lifecycle differ. A row
admitted to one view gains no standing in another.

### Selection Summary

Project bounded feature summaries for one exact configured instance and model
row. A summary supports picker badges, filtering, and safe explanatory posture.

A selection summary must not infer model applicability from a route-wide
capability, and must not present current usability without the readiness and
prepared evidence that claim requires. Route-wide support, model applicability,
and current availability stay separate facts on the row.

### Session-Start Controls

Project only controls admitted by the exact selected route, model, operation
shape, access mode, resources, and preparation boundary. Each descriptor
retains its value kind, admitted values or bounds, omission behavior, and
whether changing the value requires a replacement session.

A per-turn control is published as per-turn. It is not a session-start
guarantee, and a session-start control is not a per-turn input.

### Active-Session State

Project post-open observations and exact negotiated state without backdating
either into a pre-session guarantee. Requested, pending, provider-confirmed
effective, and rejected values remain distinct.

An observation-only option list does not become a mutable control. Between-turn
or mid-turn mutation exists only where an exact route mechanism is separately
qualified, and the census proves no general mid-turn row. A successful local
setter or prepared builder call is not provider acknowledgement.

## Descriptor Semantics

A feature or control descriptor must retain:

- stable portable or bounded namespaced semantic identity
- exact applicability: configured instance and revision, route, model where
  applicable, operation shape, access mode, and resource constraints
- authoritative source class, evidence identity, and evidence strength
- support and current availability as separate dimensions, never one flattened
  boolean
- value kind, admitted domain or explicit unenumerated bound, and omission
  semantics for controls
- lifecycle: selection-summary, session-start-only, per-turn,
  between-turn-negotiable, separately qualified mid-turn-negotiable, or
  post-open-observation-only
- actor posture: informational, consumer-selectable, host-controlled,
  operator-controlled, provider-selected, or observation-only
- state support: descriptor-only, or the exact subset of requested, pending,
  provider-effective, and rejected the source proves
- a bounded safe reason only where the authoritative source supplies one

Omission never becomes a Swallowtail default. Unknown and unenumerated stay
explicit rather than collapsing into a value, a bound, or absence of the row.

## Bounded Namespaced Extensions

A provider-native descriptor may be published under bounded namespaced identity
when portable identity would flatten route truth. The extension names its exact
route, qualified version segment, and evidence, and carries no raw provider
payload, command, path, or credential material.

A namespaced extension never widens support, availability, or lifecycle. It
cannot be promoted into a portable identity without separately qualified
cross-route evidence, and a provider-native value cannot enter a portable enum
without route, model, version, and evidence qualification.

## Snapshot Identity And Replacement

One projection snapshot binds the exact configured-instance id and revision,
route, model where applicable, operation shape, and the identities of every
source evidence record used to assemble it. The snapshot is immutable.

Readiness, catalogue, currentness, prepared-operation, or negotiated-session
change produces a replacement projection. This contract creates no universal
clock, timestamp, watcher, refresh loop, or health probe. Freshness remains the
truth each authoritative source carries; the facade preserves it and makes
source replacement visible.

The projection inherits the bounds of the records it composes and introduces no
new numeric cap of its own. A separate projection-specific bound is a later
planning decision, not a claim of this contract.

## Availability Dimensions And Bounded Safe Reasons

The projection preserves the existing authoritative source dimensions:
credential, entitlement, endpoint authorization, runtime readiness, support
authority, catalogue result, capability constraint, preparation agreement,
negotiated state, and evidence freshness.

Unsupported, unavailable, conditional, unknown, and negotiated-only meanings
remain distinguishable in projection behavior. A bounded safe reason may
accompany a dimension only where the authoritative source supplies one.

This contract does not claim those words form an exhaustive portable reason
vocabulary, and it does not define one. Absent source truth stays unknown or
absent. Arbitrary provider payloads, raw diagnostics, and consumer presentation
prose stay out of the reason surface.

## Fail-Closed Composition

Projection rejects or withholds rather than repairing disagreement, choosing a
fallback, silently downgrading, or widening a route claim. Cross-instance,
cross-revision, cross-route, cross-model, cross-operation, cross-access, and
stale-source assembly fail closed. Absence remains absence rather than an
unsupported claim.

Four named points make the review oracle directly testable:

| Counterexample | Named point | Required behavior |
| --- | --- | --- |
| Route-wide capability combined with a model or prepared operation that does not admit it | applicability disagreement | reject the row at admission, or publish it without the usable and currently available claim, before the snapshot is published |
| Valid descriptor combined with a stale configured-instance revision or superseded source record | snapshot identity disagreement | reject the mixed assembly; never publish it as current |
| Post-open option list presented as selectable or acknowledged | absent mutation authority | hold the row at observation-only unless an exact route mutation and acknowledgement source is supplied |
| Missing source truth replaced by an exhaustive availability reason | unbounded reason claim | retain unknown or absent source state plus at most a bounded safe reason the source supplied |

The first point may withhold one row. The second rejects the whole assembly,
because snapshot identity is not per-row truth. The third and fourth constrain
what a published row may claim.

## Authority Boundary

The facade is descriptive. An actual request still passes Contract 037
preparation, capability, host-service, access, and plan agreement. A projected
control never bypasses preflight, authorizes execution, or proves provider
acknowledgement. A catalogue or negotiated option list does not authorize
mutation.

Descriptors stay separate from current selected values. The projection may
return descriptor and active-session state together, but never collapses
requested and provider-effective values.

Selection, defaults, routing, fallback, grouping, ordering, display copy,
localization, and preference persistence remain consumer-owned.

## Relationship To Other Contracts

| Contract | Bound |
| --- | --- |
| 006 / 008 | access dimensions and capability constraints stay the source; the projection restates them and adds no readiness |
| 012 / 034 | session options and negotiated harness options keep their own mapping and version qualification |
| 020 | catalogue observation stays observation; it does not authorize model mutation or prove applicability |
| 037 | preparation and bound execution stay the authority; projection is descriptive only |
| 040 / 041 | generation-control and admission truth stay the source of accepted values and bounds |
| 044 | activity observation stays distinct from feature and control projection |
| 047 | remains the immutable selection snapshot; projection composes it and cannot change `Ready` / `NotReady` |
| 057 | remains pre-session admission, readiness, subject, and overlay; overlay markers stay overlay |
| 052 | route and feature documentation stays traceable evidence, not runtime authority |

## Conformance

Portable fixtures must prove:

- one snapshot binds exact instance, revision, route, model, operation shape,
  and source evidence identities, and is replaced rather than mutated
- selection-summary, session-start, per-turn, post-open observation, and exact
  negotiated state stay distinct across the three views
- route-wide, matrix, catalogue, prepared, and negotiated evidence cannot
  silently widen one another
- a route-wide capability with an incompatible model or prepared operation
  reaches the applicability-disagreement point
- a stale configured-instance revision or superseded source record reaches the
  snapshot-identity-disagreement point
- a post-open option list without an exact mutation and acknowledgement source
  stays observation-only
- unknown and absent source truth survive without an invented availability
  reason, and a bounded safe reason appears only when the source supplied one
- support, current availability, and provider-effective state never collapse
  into one boolean
- omission does not produce a default, and unenumerated domains stay explicit
- bounded namespaced extensions carry route, version, and evidence
  qualification and no raw provider payload
- credentials, targets, paths, commands, environment values, emails, and
  provider payloads are absent from every projected row
- projection creates no request, mutation, acknowledgement, watcher, or
  preflight bypass

Live provider work and route claims remain separately authorized evidence. They
are not part of ordinary projection conformance.

## Later Implementation

Not realized by this contract. Rust naming, crate and module placement, public
API baseline, fixtures, and implementation tranche selection remain later
roadmap work. Implementation cards must not treat these rules as current
architecture.

## Acceptance

- a consumer renders route and model feature summaries, session-start controls,
  and active-session state for one exact selection without adapter downcasts
- Contracts 037, 047, and 057 retain their current authority unchanged
- exact instance revision, route, model, operation shape, evidence identity,
  applicability, and lifecycle survive projection
- selection-summary, session-start, per-turn, post-open observation, and exact
  negotiated state remain distinct
- source dimensions and bounded safe reasons remain visible without an
  exhaustive reason taxonomy
- snapshot replacement and cross-boundary rejection are explicit and testable
- every review-oracle counterexample has a named rejection or withholding point
- projection adds no execution, mutation, acknowledgement, routing, default,
  fallback, or provider effect
