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

The projection must also distinguish controls fixed when a session starts from
controls that the active session can negotiate later. A consumer needs to know
whether changing a value updates the current session, applies only to a later
turn, or requires a new session.

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
- which controls are session-start-only, per-turn, or negotiable on an active
  session, including the exact safe transition point
- which facts came from route qualification, model catalogue evidence,
  configured-instance readiness, preparation, or live negotiation

Today a consumer could approximate this by joining 047 routes, model
observations, capability constraints, presentation overlays, and input types.
That duplicates Swallowtail semantics downstream and invites incorrect UI:
showing a route-wide feature for an incompatible model, treating catalogue
observation as operation authority, offering a control whose values are not
qualified, or hiding a temporarily unavailable feature as unsupported.

## Recommended Shape For Promotion Research

Prefer one public projection facade with three lifecycle-appropriate views over
one unstructured feature bag:

1. **Selection summary** — safe, bounded feature summaries for configured
   instances and model rows. This supports model-picker badges, filtering, and
   explanatory availability states.
2. **Session-start controls** — exact controls applicable while preparing or
   opening the selected instance, route, model, and operation shape. The view
   says which values become fixed for that session and whether changing one
   requires a replacement session.
3. **Active-session controls** — exact controls the bound session can negotiate
   after opening, with current and provider-effective state, allowed transition
   points, and safe unavailable reasons. This view must distinguish between
   changes allowed between turns and any separately proved mid-turn steering.

Live negotiated observations may refine or supplement the third view after a
session opens. They must not be presented as pre-session guarantees. A
post-open option list is not proof that the current session accepts mutation.

The three views should be assembled from the same semantic vocabulary and
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
- lifecycle and mutability: session-start-only, per-turn, between-turn
  negotiation, explicitly proved mid-turn negotiation, or observation-only
- compatibility constraints for model, route, access mode, and operation shape
- optional bounded fallback label, help text, and grouping hint

The descriptor should not become arbitrary JSON, raw provider flags, or a
consumer UI component. Provider-native extensions need bounded namespaced
identity and explicit evidence. The consuming app owns visual layout,
localization, persistence of preferences, and product policy.

### Session Lifecycle Semantics

Do not represent lifecycle as a generic `mutable` boolean. At minimum, the
consumer projection must distinguish:

- **session-start-only** — selected before open or resume and fixed once the
  session is active; a changed value requires a new session
- **per-turn** — supplied for one later turn without changing session setup
- **between-turn negotiable** — may update the active session while it has no
  in-flight turn, through an exact route mechanism
- **mid-turn negotiable** — may steer an active turn only when separately
  qualified; never inferred from between-turn support
- **post-open observation-only** — visible after opening but not consumer
  mutable

For active-session negotiation, the facade needs a typed companion state for
requested, pending, provider-confirmed effective, and rejected values. It must
also publish whether the session must be idle, whether the next turn observes
the change, and whether failure leaves the prior value effective. Provider
acknowledgement cannot be inferred from a successful local setter call.

### State And Authority

The projection is descriptive. It does not authorize execution, choose a
model, create a route, supply a default, or bypass preflight. The actual
request must still pass the existing prepared-plan and capability checks.

Current selected values belong in separate consumer or session state. Keeping
descriptors separate from values lets several applications render the same
route truth without Swallowtail owning their composer state. The cohesive API
may return descriptor and active-session state together, but must not collapse
requested and provider-effective values.

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
5. Which current routes have an exact in-session mutation and acknowledgement
   mechanism, and which merely advertise values after opening?
6. Does this amend Contracts 037, 047, and 057, or need one dedicated contract
   that composes them without changing their authority?
7. What snapshot identity and refresh signal lets a consumer replace stale
   projections safely?

## Promotion Gate

Before implementation, run a census across production routes and every public
consumer-settable option. Map each item to its current source, scope, value
domain, evidence strength, and lifecycle. Use the live per-route feature
inventory as evidence, not runtime authority.

## Parallel Evidence Dispatch (2026-08-30)

The operator authorized this census to run beside the Claude Code `2.1.251`
currentness worker. Parallel safety requires an evidence-only triage lane. The
worker may update this note and add
`2026-08-30-consumer-route-feature-and-option-projection-census.csv` under
`docs/triage/`. It must not edit code, contracts, research/log/roadmap indexes,
the active Next Task, or currentness/watcher cards.

The census covers every production route and every public consumer-selectable
or consumer-observable feature/control reachable through current Swallowtail
types. Each row records at least:

- configured route and operation shape
- semantic feature or control identity
- current public source type and owning package
- value kind, enumerable values or bounds, and omission semantics
- route/model/access/resource constraints
- lifecycle: selection-summary, session-start-only, per-turn,
  between-turn-negotiable, mid-turn-negotiable, or post-open-observation-only
- requested, pending, effective, rejected, or descriptor-only state support
- evidence source and strength
- present consumer projection surface, if any
- exact gap, ambiguity, or unsafe inference a cohesive facade must avoid

Use repository code, contracts, guides, route matrices, and frozen research as
evidence. Do not contact providers, run prompts, authenticate, install tools,
or treat documentation matrices as runtime authority. Avoid adapter-by-adapter
prose when several rows share one exact type or lifecycle rule, but do not
flatten distinct routes or provider-native controls.

The worker returns one evidence PR and stops. The note remains `Status: open`.
After review and merge, the orchestrator decides whether to promote the census
into numbered research and a contract/roadmap lane. No research number or
implementation authority is reserved by this dispatch.

Promote only when the contract can prove:

- a consumer can render route/model feature summaries without adapter-specific
  knowledge
- a composer receives only controls valid for the exact selected prepared
  operation
- unsupported, unavailable, conditional, unknown, and negotiated-only states
  remain distinct
- session-start-only, per-turn, between-turn, mid-turn, and observation-only
  controls remain distinct, with no inferred mutation authority
- projected control values and omission semantics agree with preflight and
  execution validation
- active-session negotiation keeps requested, pending, effective, and rejected
  state truthful
- provider-native extensions remain bounded and namespaced
- consumer presentation and routing policy remain downstream

Related evidence:

- [`advanced-route-features.md`](2026-08-21-advanced-route-features.md)
- [`route-readiness-facade.md`](2026-08-19-route-readiness-facade.md)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [Contract 057](../contracts/057-route-readiness-and-connection-admission.md)

This note is triage, not execution authority.
