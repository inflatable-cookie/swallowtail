# 057 Route Readiness And Connection Admission

Status: active
Owner: Tom
Updated: 2026-08-21
Research: 168
Spec: archived 011

## Purpose

Give consuming applications a portable library surface for the pre-session
connection lifecycle: which routes can be added, what they require, how a
configured instance is admitted, how authentication and readiness are
observed, and how that instance's model list is presented.

Contract 047 remains the ready-to-select snapshot. This contract owns the
lifecycle in front of it. Swallowtail stays a library. It does not become a
connection server, UI, router, or secret store.

## Boundary

In:

- addable-route descriptors grouped by hosted, installed, and local-runtime
  topology, with available, unavailable, and unsupported observations
- admission of a configured instance, including several instances of one
  family
- credential-requirement field descriptors and library-owned sign-in loops
  through host ports
- readiness refresh distinct from host enablement
- optional authenticated-subject observation on this facade
- per-instance config field descriptors as opaque host-owned references
- update observations reused from Contract 029
- a persistence port for instance records, secret references, enablement, and
  presentation overlays, plus an optional simple adapter
- a model-presentation overlay bound to exact catalogue identity

Out:

- a Swallowtail daemon, HTTP server, or durable product database
- raw secret storage in portable records
- embedding a browser, system keychain, or OAuth client secret
- routing, fallback, or composer model-selection policy
- accent color and other pure UI chrome
- flattening gateway or cross-provider models into another instance's
  catalogue
- treating Contract 047 as the add-connection facade
- live provider work as a substitute for this contract

## Addable-Route Catalog

Consumers assemble an addable-route catalog from adapter-local descriptors,
the same way they assemble prepared facades. There is no umbrella registry
crate and no runtime inventory of every production route.

A descriptor names one addable route: driver identity, topology group,
credential or sign-in requirements, and whether the route is available,
unavailable, or unsupported on the current host. Topology grouping is
hosted, installed, or local-runtime presentation. It is not Contract 006
`ExecutionLayer` and must not be collapsed onto harness versus direct
inference.

Contract 008 discovery remains a probe of one already selected driver. A
discovered candidate is not an addable-route row and is not a configured
instance. Listing addable routes does not authenticate, persist, or prepare.

Unavailable means the descriptor can name what is missing: an install,
runtime, or host service. Unsupported means the adapter will not offer that
route on this host. Absence of a descriptor means the consumer did not link
that adapter, not that the route is unsupported.

## Admission

Admission takes one addable route plus host-owned configuration and writes a
configured instance through the store port. Several instances of one family
remain distinct configured-instance ids. Admission does not prepare, select a
model, or change 047 readiness.

A discovered candidate still cannot execute. Only an admitted configured
instance can later be prepared under Contract 037.

## Credential-Field Descriptors

When a route collects an API key or similar host-owned secret, this contract
owns field descriptors: label, secret versus public, and optional environment
name. Contract 014 still owns credential leases for an already stored secret.
The host stores secret bytes. Portable records carry only opaque
`CredentialRef` values.

Field descriptors do not authorize reading another audience's secret, mixing
subscription OAuth into a public API key, or treating an environment name as
a resolved value.

## Sign-In Loop

Swallowtail owns start, poll, complete, cancel, and timeout for interactive
OAuth, device OAuth, delegated CLI login, and API-key collection. Host ports
open a URL, bind a loopback callback, display a device code, or spawn an
approved login helper. Token materialization uses existing credential leases.
Swallowtail does not own the browser, keychain, or secret bytes.

Contract 008 `SignInAction` is an advertisement, not permission to execute.
ACP `authenticate` (Contract 015) activates an already authorized harness
credential. Contract 017 delegated login refreshes evidence for an already
configured instance. Neither is this add-connection loop.

A sign-in loop that changes mechanism, account, endpoint audience, or billing
authority fails closed. Success only materializes a credential reference for
the same route and audience that started the loop.

## Store Port

The store interface holds:

- admitted instance records
- secret references, never raw secrets
- enablement as a host preference
- optional instance labels
- presentation overlays

Swallowtail may ship an optional in-memory or JSON-file adapter for tests and
small apps. Consumers may supply SQLite, keychain-backed, or product stores.
A consumer adapter that stores secrets does so under consumer authority. The
portable interface never requires raw secrets.

Enablement is independent of access-status dimensions and of 047 `Ready` /
`NotReady`. A disabled instance may still be ready. An enabled instance may
be `NotReady`.

## Readiness Refresh

Refresh re-observes Contract 006 / 008 access dimensions for one admitted
instance: credential, entitlement, endpoint authorization, runtime readiness,
and support authority. It does not invent an aggregate ready boolean, probe
unrelated instances, or write enablement.

Contract 047 remains an immutable snapshot. After a refresh, the consumer
replaces that snapshot. Refresh is not a watcher inside 047.

## Authenticated Subject

This facade may expose an optional provider-disclosed subject: email, login,
or plan label. The observation is restricted and redacted by default. A
consumer may reveal it for presentation such as blur/unblur. Adapters report
only what the provider actually discloses.

The subject is never a configured-instance id, never a 047 selection field,
never a default diagnostic, and never a routing key.

## Config-Field Descriptors

Per-instance configuration such as binary path, API endpoint, or environment
is described as opaque host-owned field references. Values stay host-private.
Public records do not carry paths, URLs, or env bodies.

After admission, a route-local preparation handoff may retype the exact stored
field reference into the opaque target, executable, environment, or endpoint
reference required by the selected adapter. The host resolves that reference
at the preparation or operation boundary. Admission still does not prepare,
and the handoff never copies the value into a portable record or diagnostic.
Contract 037 remains the boundary that binds the exact target after admission.

## Update Observation

Instance update affordances reuse Contract 029 claims and Contract 032
installed-executable observations. This contract does not create a second
currentness system, install, upgrade, or authenticate.

## Model-Presentation Overlay

Overlay entries key to exact configured-instance and model ids from a
catalogue result. When a row reports a provider id, the marker must match
that provider id too. When a row omits a provider id, the marker keys
instance plus model and must not invent a provider id. They may mark
hidden, ordinal, consumer-default, and favourite.

They cannot invent a model, make `NotReady` selectable, or copy a model from
another instance. Provider catalogue defaults stay distinct from the
consumer-default marker. Mixed gateway rows remain consumer assembly of
several catalogues. Overlay markers do not change 047 `Ready` / `NotReady`.

Accent color and other chrome stay consumer-owned. Optional 047 presentation
metadata may be added later; it must not change selection readiness.

## Host Ports

Contract 010 gains optional interactive sign-in service kinds:

- open a host-approved URL
- bind a loopback callback for one sign-in operation
- display a device code
- spawn an approved login helper already covered by process authority

Missing ports fail the sign-in loop that requires them. Presence of a port
does not start sign-in. The ports never return secret bytes to portable
records.

## Later Implementation

Not realized by this contract. Implementation cards must not treat these
notes as current architecture.

Crate placement:

- records in `swallowtail-core`
- lifecycle roles and the store trait in `swallowtail-runtime`
- optional in-memory and JSON-file adapters in `swallowtail-host-local`
- addable descriptors adapter-local, like prepared facades
- no umbrella registry crate

First-proof routes:

- hosted API key: Anthropic Messages
- hosted interactive OAuth: Anthropic or Claude subscription, whichever can
  be proved without extracting secrets
- installed: Codex app-server
- local runtime: Ollama attach

## Relationship To Other Contracts

| Contract | Bound |
| --- | --- |
| 006 | subject observation and sign-in loop versus credential status |
| 008 | addable route versus discovered candidate versus configured instance |
| 010 | host ports for URL open, loopback callback, device-code display |
| 014 | field descriptors versus credential leases |
| 015 / 017 | delegated harness activation stays distinct from login |
| 020 | overlay keys to catalogue identity; it does not replace the catalogue |
| 029 / 032 | instance update observation reuses claims |
| 037 | preparation remains after admission |
| 047 | no emails, tokens, or targets; overlay does not change selection readiness |

## Conformance

Portable tests must cover:

- addable-route assembly without a registry crate
- topology grouping distinct from `ExecutionLayer`
- multiple admitted instances of one family
- enablement independent of access status and 047 readiness
- field descriptors that never carry secret bytes
- sign-in start, poll, complete, cancel, and timeout through host ports
- fail-closed sign-in when a required host port is missing
- subject observation redacted by default and absent from 047
- overlay hide, ordinal, consumer-default, and favourite bound to exact
  catalogue identity
- overlay refusal to invent a model, copy across instances, or mark
  `NotReady` selectable
- store port persistence of references, labels, enablement, and overlay
  without raw secrets
- refresh replacing a 047 snapshot rather than mutating one

## Acceptance

- a consumer can list addable routes, admit an instance, complete required
  auth through host ports, refresh readiness, and project a model list with
  overlay applied
- 047 snapshots stay free of emails, tokens, and targets
- enablement and readiness remain independent
- display color, composer routing, and gateway flattening stay consumer-owned
- no implementation of this surface ships before annotated `v0.3.3`
