# 052 Consumer And Operator Integration Documentation

Status: active
Owner: Tom
Updated: 2026-08-05

## Purpose

Make every production route and portable feature usable by an integrating
agent or operator without reverse-engineering adapter source, provider-native
payloads, or consumer-specific workarounds.

## Authority

Guides explain realized APIs. Contracts, architecture, route matrices, and
qualified version evidence remain authoritative when a guide is stale or
ambiguous.

The guide map owns documentation traceability, not provider or feature truth.
Adding a guide cannot widen a route capability, access posture, version range,
authority, lifecycle, or recovery claim.

## Route Coverage

Every production route ID in the provider route matrix must map to one
canonical route guide. One guide may cover multiple explicitly separated
routes behind the same public facade.

A complete route guide names:

- when to choose and reject the route
- package, public facade, driver ID, operation roles, and transport
- installation or attached-service prerequisites
- supported version posture and unverified-newer behavior
- authentication, billing, credential, endpoint, executable, and environment
  ownership without exposing secrets
- required host services, working resources, access policy, and isolation
- discovery, preparation, model selection, and bound-operation sequence
- each supported operation shape and its event-drain contract
- generation controls, inputs, tools, callbacks, activity, and usage where
  applicable
- cancellation, terminal, cleanup, persistence, continuation, reconciliation,
  restoration, and management truth
- portable failure handling and exact diagnostic escape hatch
- unsupported capabilities and the evidence gate for promotion
- one compiling normal-path example and relevant deterministic validation
- live or authenticated checks only as separately gated optional evidence

Composite guides must keep route-specific differences legible. A capability
available on one branch cannot appear available on the whole facade.

## Feature Coverage

Every feature column in the provider solution feature matrix must map to a
canonical task-oriented guide or guide section. Portable features outside that
matrix, including configured provider instances, observable activity, task
lists, subagents, restoration, and failure classification, need the same map.

A complete feature guide names:

- the portable records, roles, handles, outcomes, and prepared entry points
- route applicability and exact capability differences
- operation ordering, correlation, ownership, bounds, and persistence
- consumer responsibilities and forbidden inferences
- terminal, cancellation, cleanup, callback, and failure interaction
- a compiling consumer example or an explicitly linked route example
- deterministic conformance or package validation

Feature guides do not create a universal provider router, prompt API, retry
policy, fallback, credential workflow, or persistence model.

## Examples And Validation

Example code is part of documentation evidence. Normal-path examples must
compile through `effigy check:examples`. Documentation links, front doors, and
the sole roadmap next task must pass `effigy qa:docs`.

`effigy qa:guides` owns the deterministic route, feature-header, canonical
guide, guide-index, coverage-state, and example traceability check. It is also
part of `effigy qa:docs` so ordinary guide changes cannot bypass it.

Coverage acceptance must also compare the guide map with:

- every production route in `provider-route-matrix.md`
- every feature header in `provider-solution-feature-matrix.csv`
- every route guide and referenced example

Missing, partial, and complete are explicit states. A route or feature cannot
be marked complete merely because its API appears in generated Rust docs, a
matrix cell, a contract, a source test, or a release handoff.

## Audience Boundary

Consumer guidance targets application agents and developers. Operator
guidance covers installation, authentication posture, attached services,
version checks, optional live probes, and safe diagnostics. Adapter-authoring
and release-validation guidance stays separately labelled.

No guide may require credential values, provider-native payload parsing,
authenticated testing, destructive management work, or live prompts for its
deterministic acceptance.

## Acceptance

- all 33 current production route IDs map to a complete route guide
- all 34 current feature columns map to a complete feature guide or section
- portable features outside the matrix have explicit guide ownership
- every route has a compiling normal-path example or documented not-applicable
  operation posture
- guide coverage is checked against the route and feature inventories
- deterministic docs and example validation passes without live provider work
