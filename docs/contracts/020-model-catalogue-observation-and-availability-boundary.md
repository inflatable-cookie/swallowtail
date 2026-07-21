# 020 Model Catalogue Observation And Availability Boundary

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Represent provider catalogue results as bounded, source-scoped observations
without turning mutable discovery data into entitlement, runtime capability,
route selection, or consumer policy.

## Observation Scope

Every catalogue result belongs to one exact configured instance, catalogue
driver, service or endpoint audience, execution host, access profile, and
observation operation. A regional or account-scoped provider catalogue retains
that scope. Results from another region, audience, account, driver, or
observation time cannot be substituted silently.

An entry may identify one reported model and include mutable presentation or
typed observations. Catalogue presence proves only that the bound source
reported the entry. It does not prove:

- runtime or model permission
- entitlement, subscription, agreement, or billing readiness
- regional invocation availability
- quota, rate, capacity, health, or successful request acceptance
- implementation by a selected Swallowtail operation facade
- existence of a configured model route
- a default model, provider, endpoint, or routing preference

Consumers may use catalogue evidence in their own policy. Swallowtail does not
make that decision while discovering models.

## Identity Separation

Model id, provider id, model route, provider resource reference, catalogue
driver, integration family, configured instance, and endpoint audience remain
separate identities.

A provider display name is not a provider id unless the exact catalogue
contract defines it as one. A provider resource ARN, URL, deployment id, or
other provider reference does not replace the public model id or authorize a
route. A gateway catalogue may report models from multiple underlying
providers without collapsing the gateway family into those providers.

## Typed Catalogue Observations

Provider-neutral model metadata may carry source-scoped observations for:

- input and output modalities
- provider lifecycle status and timestamps
- provider-advertised streaming behavior
- inference or serving types
- customization types
- token limits and other safe availability metadata already allowed by
  Contract 014

Known common values use typed common vocabulary. A provider value newer than
that vocabulary is retained only as a bounded, namespaced provider observation
when it is safe and useful. It never becomes a known common value by string
similarity. Missing fields mean unknown.

Provider lifecycle status is not universal runtime state. Active, legacy,
deprecated, extended-access, or end-of-life evidence remains tied to the
provider source and observation time. It cannot by itself enable or prohibit
invocation. Optional lifecycle timestamps carry the same source scope and do
not authorize billing or support assumptions.

Advertised streaming, modality, inference, and customization observations are
not Swallowtail runtime `Capability` claims. A driver or facade declares its
implemented capabilities independently. Discovery cannot widen an operation
request, configured route, or preflight plan.

## Unknowns, Drift, And Bounds

Generated SDK and provider enums are open to drift. Unknown semantic values
that are safe catalogue metadata are preserved as bounded, namespaced
observations. Unknown values never imply support. Invalid required identity,
unsafe data, malformed structure, or exceeded bounds fails the catalogue
operation rather than producing a misleading entry.

Adapters fix maximum entry counts and maximum lengths for identifiers,
presentation fields, provider-defined observation codes, and repeated
observation collections. They do not silently truncate a catalogue and call it
complete. Raw provider responses, credentials, endpoint values, and SDK
objects remain outside public metadata and diagnostics.

## Operation Lifecycle

Catalogue pagination is operation-specific. A paginated provider traversal is
bounded, cancellable, deadline-aware, fixed to one preflight binding, and
distinct from inference retry. A non-paginated operation remains bounded by
entry and field limits.

SDK retry, redirect, endpoint failover, adaptive delay, or hedging defaults do
not widen one catalogue request. Unless an explicit later policy says
otherwise, a catalogue operation makes one attempt per provider request. The
outer deadline covers client construction, credential resolution, every
authorized request, response collection, projection, scoped work join, and
credential release.

Cancellation stops local work and joins owned work. It does not claim that a
remote service failed to observe an already transmitted request.

## Service And Access Separation

Catalogue and inference drivers may share an explicit credential mechanism
while retaining distinct configured access profiles, endpoint audiences,
permissions, clients, and operation roles. An inference grant cannot be reused
as a catalogue grant. Catalogue authorization cannot be treated as runtime
authorization.

A catalogue driver performs only its declared discovery operations. It cannot
invoke a model, accept a third-party agreement, initiate a subscription, call
a marketplace, modify account access, or perform provider onboarding as a
side effect of discovery.

## First Bedrock Control-Plane Subset

The first proof binds:

- official `aws-sdk-bedrock = 1.148.0`
- native `ListFoundationModels`
- one exact host-approved regional Bedrock control-plane endpoint and AWS
  region
- one separately configured catalogue access profile
- one host-authorized delegated AWS credential provider
- no provider, modality, inference-type, or customization filter
- one non-paginated service request and one SDK attempt
- bounded projection of model id, model name, provider name, modalities,
  response-streaming evidence, inference types, customization types, and model
  lifecycle

The model ARN remains adapter-private provider evidence in this first subset.
The native control-plane result does not become a configured Bedrock Runtime
route.

The separately documented `bedrock-mantle` OpenAI-compatible `/models` surface
is excluded. It has a different endpoint and protocol boundary and requires a
separate driver decision.

## Bedrock Access Evidence

The first driver requires the exact `bedrock:ListFoundationModels` control-
plane permission posture. It calls no runtime inference, AWS Marketplace,
model agreement, or provider onboarding operation.

AWS credential validity, IAM authorization, account eligibility, Marketplace
subscription, third-party provider prerequisites, runtime permission, regional
invocation availability, quota, rate, billing, and request acceptance remain
independent evidence. The catalogue driver claims none of them from a returned
model summary.

## Conformance

Deterministic generated-type fixtures must prove:

- the exact SDK version, request shape, non-paginated output, summaries, and
  typed errors
- exact configured instance, host, endpoint audience, region, access profile,
  delegated credential provider, and catalogue role binding
- no ambient credential, region, endpoint, profile, filter, route, retry, or
  model fallback
- one request and one SDK attempt
- bounded known and unknown observation projection
- missing optional evidence remains unknown
- invalid identities, structural drift, and overflow fail safely
- catalogue presence does not construct or mutate a model route
- cancellation, deadline, joined private work, and delegated credential
  release
- endpoint, credential, raw SDK response, ARN, and provider payload redaction

Default conformance uses no AWS credential, provider account, network request,
Marketplace action, or paid inference.

## Acceptance

- catalogue evidence retains its exact source and observation scope
- mutable provider evidence cannot become implicit runtime capability or
  entitlement
- provider resource references cannot replace model or route identity
- non-paginated catalogues are still explicitly bounded
- catalogue and inference authority remain independent
- discovery has no subscription, agreement, onboarding, or inference side
  effect
