# 2026-08-30 Consumer Route Feature And Option Projection

Status: open
Owner: Tom
Source: operator direction during g05 execution

## Operator Intent

Consuming applications such as Nucleus should be able to ask Swallowtail what
each selectable route and model can do, then render relevant features and
controls in model-picker and composer surfaces.

The interface should be cohesive. Consumers should not need adapter
downcasts, route-specific queries, command-line knowledge, or their own merge
of unrelated capability, catalogue, readiness, and option records.

This is a publication and projection problem. It does not add route features,
change provider behavior, or give Swallowtail ownership of consumer layout or
routing policy.

## Existing Substrate

Substantial typed truth already exists:

- Contract 047 projects configured-instance identity, readiness, exact
  prepared routes, route capability requirements, and model catalogue entries.
- `Capability`, `CapabilityProfile`, and `CapabilityRequirement` carry portable
  route and operation support, including parameterized constraints.
- model catalogue observations carry model-level modalities, reasoning,
  tool-calling, streaming, lifecycle, and provider-defined values when the
  source supplies them.
- Contract 057 carries addable-route descriptors, readiness, configuration
  field descriptors, and model-presentation overlays.
- session and generation input types represent controls that consumers may
  request, including reasoning, harness mode, tools, idioms, structured
  output, attachments, search, and output-token limits.
- negotiated session observations can expose provider-native values such as
  model options after a session is open.
- the provider-route and per-route feature matrices hold qualified evidence,
  but are documentation and QA surfaces rather than a runtime consumer API.

Contract 047 explicitly allows a low-level consumer to assemble these records.
That is not the same as a cohesive application-facing interface.

## Gap

There is no single typed projection that answers, for one exact configured
instance, route, model, operation shape, and current evidence snapshot:

- which semantic features are supported
- which are currently usable, unavailable, unsupported, conditional, unknown,
  or discoverable only after negotiation
- which controls a consumer may offer before opening a session or starting an
  operation
- the valid type, values, bounds, omission behavior, and lifecycle of each
  control
- which facts came from route qualification, model catalogue evidence,
  configured-instance readiness, preparation, or live negotiation

Today a consumer could approximate this by joining 047 routes, model
observations, capability constraints, presentation overlays, and input types.
That duplicates Swallowtail semantics downstream and invites incorrect UI:
showing a route-wide feature for an incompatible model, treating catalogue
observation as operation authority, offering a control whose values are not
qualified, or hiding a temporarily unavailable feature as unsupported.

## Recommended Shape For Promotion Research

Prefer one public projection facade with two lifecycle-appropriate views over
one unstructured feature bag:

1. **Selection summary** — safe, bounded feature summaries for configured
   instances and model rows. This supports model-picker badges, filtering, and
   explanatory availability states.
2. **Prepared operation controls** — exact controls applicable to the selected
   instance, route, model, operation shape, and lifecycle before execution.
   This supports composer controls without widening the prepared plan.

Live negotiated observations may refine or supplement the second view after a
session opens. They must not be presented as pre-session guarantees.

The two views should be assembled from the same semantic vocabulary and
returned through one consumer-facing API family. Consumers should not query
adapters directly after the relevant route evidence has been admitted and
prepared.

### Feature Descriptor

A candidate feature descriptor needs at least:

- stable portable feature or capability identity
- exact configured-instance, route, model, and operation applicability
- support and availability state, with a safe bounded reason when unavailable
- evidence source, strength, and freshness
- lifecycle scope: picker, session creation, turn, run, or post-open
  negotiation
- whether the feature is informational, consumer-selectable, host-controlled,
  operator-controlled, or provider-selected

Route support, current availability, and provider-effective state are distinct
facts. Do not collapse them into one boolean.

### Control Descriptor

A selectable feature also needs a typed control descriptor:

- stable control identity tied to its semantic capability
- value kind such as boolean, enum, bounded integer, bounded text, or structured
  declaration
- allowed values or numeric bounds from qualified evidence
- omission semantics and whether Swallowtail knows a default
- lifecycle and mutability: before session, before turn, or negotiated only
- compatibility constraints for model, route, access mode, and operation shape
- optional bounded fallback label, help text, and grouping hint

The descriptor should not become arbitrary JSON, raw provider flags, or a
consumer UI component. Provider-native extensions need bounded namespaced
identity and explicit evidence. The consuming app owns visual layout,
localization, persistence of preferences, and product policy.

### State And Authority

The projection is descriptive. It does not authorize execution, choose a
model, create a route, supply a default, or bypass preflight. The actual
request must still pass the existing prepared-plan and capability checks.

Current selected values belong in separate consumer or session state. Keeping
descriptors separate from values lets several applications render the same
route truth without Swallowtail owning their composer state.

Snapshots should retain exact source identity and freshness. Readiness,
catalogue, currentness, or negotiated-session changes produce replacement
projections rather than silently mutating prior truth.

## Boundaries

- no umbrella adapter registry or runtime enumeration of unlinked routes
- no adapter-specific downcasting in consumer code
- no raw credentials, paths, commands, environment values, or provider payloads
- no inference of model support from a route-wide capability alone
- no conversion of documentation matrix rows into runtime authority
- no generic UI-schema language or Swallowtail-owned composer layout
- no flattening of unsupported, unavailable, unknown, and unverified states
- no new route claim merely because the projection can describe it

## Promotion Questions

1. Which existing source is authoritative for every currently selectable
   control, and where are values accepted today without an enumerable domain?
2. Can selection summaries be projected entirely from 047 plus model catalogue
   evidence, or do some features require prepared-operation evidence even for
   picker display?
3. Which fallback presentation text belongs in Swallowtail, and which must be
   supplied or localized by the consumer?
4. How should post-open negotiated options appear without making the pre-open
   composer misleading?
5. Does this amend Contracts 037, 047, and 057, or need one dedicated contract
   that composes them without changing their authority?
6. What snapshot identity and refresh signal lets a consumer replace stale
   projections safely?

## Promotion Gate

Before implementation, run a census across production routes and every public
consumer-settable option. Map each item to its current source, scope, value
domain, evidence strength, and lifecycle. Use the live per-route feature
inventory as evidence, not runtime authority.

Promote only when the contract can prove:

- a consumer can render route/model feature summaries without adapter-specific
  knowledge
- a composer receives only controls valid for the exact selected prepared
  operation
- unsupported, unavailable, conditional, unknown, and negotiated-only states
  remain distinct
- projected control values and omission semantics agree with preflight and
  execution validation
- provider-native extensions remain bounded and namespaced
- consumer presentation and routing policy remain downstream

Related evidence:

- [`advanced-route-features.md`](2026-08-21-advanced-route-features.md)
- [`route-readiness-facade.md`](2026-08-19-route-readiness-facade.md)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [Contract 057](../contracts/057-route-readiness-and-connection-admission.md)

This note is triage, not execution authority.
