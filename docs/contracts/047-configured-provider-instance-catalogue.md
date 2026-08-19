# 047 Configured Provider Instance Catalogue

Status: active
Owner: Tom
Created: 2026-08-04
Updated: 2026-08-19

## Purpose

Give consumers one portable, bounded projection of configured provider
instances without making Swallowtail choose a provider, model, route, default,
credential, or fallback.

## Assembly And Authority

The catalogue is consumer-assembled from existing Swallowtail records:

- one exact driver descriptor
- one configured instance and revision
- one access profile plus prepared access evidence
- zero or more exact prepared operation routes
- an optional model-catalogue result bound to one prepared model-catalogue
  route

Admission checks identity agreement and removes authority-bearing credential
and target references from the projection. It does not discover providers,
probe credentials, execute a catalogue request, construct a prepared facade,
or retain an operation handle.

Provider adapters remain responsible for exact driver, facade, instance,
version, capability, and route facts. A low-level consumer may assemble the
same records explicitly, subject to the same validation. Consumers cannot use
the catalogue to widen a provider claim.

## Instance Identity

One admitted instance retains:

- configured-instance id and revision
- adapter identity, integration family, and transport family
- protocol facade and instance policy
- execution host and ownership
- exact interface-version bindings
- configured capability profile

Provider identity remains the provider id attached to each model-catalogue
entry. The catalogue does not invent one provider id for a multi-provider
gateway, infer one from an adapter name, or merge model and provider identity.

Display names, grouping chrome, and product labels remain downstream
presentation. Contract 057 may store bound overlay markers for hide, ordinal,
consumer-default, and favourite. Those markers are not this snapshot's fields
and cannot change `Ready` / `NotReady`.

## Credential And Availability Posture

The projection retains only safe access posture:

- access-profile id
- credential mechanism and credential state
- entitlement metering and entitlement state
- endpoint audience and authorization state
- runtime readiness
- support authority
- access-evidence provenance

It never retains a credential reference, lease, token, account identifier,
email, or raw probe response. Authenticated-subject observation belongs to
Contract 057, redacted by default, and does not enter this snapshot.

The exact dimensions remain independently observable. Catalogue selection
readiness is conservative and derived, not caller supplied. An instance is
`Ready` only when all of these are true:

- credential state is `Ready` or `NotRequired`
- entitlement state is `Available`
- endpoint authorization is `Allowed`
- runtime readiness is `Ready`
- support authority is not `Prohibited`
- a bound model catalogue completed successfully with at least one model

Every other combination is `NotReady`. This does not change any underlying
access observation or claim that one failed dimension caused another.
Unavailable, degraded, unknown, unauthenticated, unsupported, empty-model, and
catalogue-failed instances remain visible.

## Prepared Routes

Each admitted route is projected from `PreparedOperationEvidence` and retains
its exact driver role, execution layer, operation shape, capability
requirements, and optional model-route identity. Admission requires exact
agreement on driver, transport, configured instance, revision, host, target,
facade, access status, and access profile.

The target is checked during admission but is not retained by the portable
projection. A route record is evidence, not executable authority. It does not
provide a generic prompt operation or collapse catalogue, structured-run,
interactive-session, realtime, serving, or provider-session roles.

## Model Catalogue Binding

A model-catalogue result is either:

- available with its bounded `ModelCatalogEntry` values
- unavailable with one safe diagnostic
- absent because no result was supplied

Available and unavailable results both identify one exact prepared route whose
driver role is `ModelCatalog`. That source route must be admitted into the same
configured instance. Model entries retain separate model and optional provider
ids plus their existing metadata.

The configured-instance catalogue does not choose a catalogue default, turn a
provider default into a consumer default, select reasoning, create model
routes, or infer model availability for another facade or instance.

## Bounds And Snapshot Truth

The portable projection admits at most:

- 256 configured instances per catalogue
- 64 prepared routes per instance
- 10,000 model entries per instance

Configured-instance ids are unique within one catalogue. Exact duplicate
routes and duplicate `(provider id, model id)` entries are rejected rather
than silently merged.

The catalogue is an immutable snapshot of supplied evidence. It provides no
watcher, registry persistence, refresh loop, hot reload, health probe, routing
policy, default selection, failover, or fallback. A consumer replaces the
snapshot after separately authorized discovery, preparation, catalogue work,
or a Contract 057 readiness refresh.

Contract 057 owns addable-route listing, admission, sign-in, store, subject
observation, and the model-presentation overlay. This catalogue stays the
selection snapshot. Overlay markers must not make `NotReady` selectable.

## Consumer Ownership

The consumer owns:

- which configured records to assemble
- when to refresh their source evidence
- presentation, sorting, grouping, and labels
- explicit provider, instance, model, and reasoning selection
- session defaults and creation of a new session after selection changes
- persistence of consumer preference and session state

Selection must retain exact configured-instance, facade, model, and route
identity. `NotReady` remains non-selectable unless later evidence produces a
new ready snapshot.

## Acceptance

- exact prepared route and model-catalogue bindings survive projection
- unavailable instances remain visible and never report `Ready`
- credentials, target references, emails, provider payloads, and handles are absent
- overlay markers cannot change `Ready` / `NotReady`
- arbitrary cross-instance, cross-facade, cross-access, and cross-driver
  evidence is rejected
- model and provider ids remain separate
- catalogue construction adds no router, default, fallback, or provider effect
