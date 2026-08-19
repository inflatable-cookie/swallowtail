# 011 Route Readiness And Connection Admission

Status: draft
Owner: Tom
Updated: 2026-08-19

## Purpose

Give consuming applications a portable library surface for the pre-session
connection lifecycle: which routes can be added, what they require, how a
configured instance is admitted, how authentication and readiness are observed,
and how that instance's model list is presented.

Contract 047 remains the ready-to-select snapshot. This spec covers the
lifecycle in front of it. Swallowtail stays a library. It does not become a
connection server, UI, router, or secret store.

## Scope

In:

- addable-route descriptors grouped by hosted, installed, and local-runtime
  topology, with available, unavailable, and unsupported observations
- admission of a configured instance, including several instances of one family
- credential-requirement field descriptors and library-owned sign-in loops
  through host ports
- readiness refresh distinct from host enablement
- optional authenticated-subject observation on the connection facade
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
- flattening gateway or cross-provider models into another instance's catalogue
- treating Contract 047 as the add-connection facade
- live provider work as a planning substitute

## Settled Decisions

1. **Authenticated subject lives on the connection facade, not in 047.**
   T3 Code shows an account email, blurred until clicked. That is presentation.
   Swallowtail may expose an optional provider-disclosed subject (email, login,
   plan label) as a restricted observation: redacted by default, revealable to
   the consumer for blur/unblur. It is never a configured-instance id, never a
   047 selection field, never a default diagnostic, and never a routing key.
   Adapters report only what the provider actually discloses.

2. **Sign-in is library-max, host-executed.** Swallowtail owns start, poll,
   complete, cancel, and timeout for interactive OAuth, device OAuth, delegated
   CLI login, and API-key collection. Host ports open a URL, bind a loopback
   callback, display a device code, or spawn an approved login helper. Token
   materialization uses existing credential leases. Swallowtail does not own
   the browser, keychain, or secret bytes.

3. **Persistence is a Swallowtail port, not Swallowtail-owned app state.**
   Define a store interface for instance records, secret *references*,
   enablement, optional instance labels, and presentation overlays. Optionally
   ship a simple adapter for tests and small apps. Consumers may supply SQLite,
   keychain-backed, or product stores. The portable interface never requires
   raw secrets. A consumer adapter that stores secrets does so under consumer
   authority.

4. **Model hide, reorder, default, and favourite are a bound overlay.**
   Overlay entries key to exact configured-instance, provider, and model ids
   from a catalogue result. They may mark hidden, ordinal, consumer-default,
   and favourite. They cannot invent a model, make `NotReady` selectable, or
   copy a model from another instance. Provider catalogue defaults stay
   distinct from the consumer-default marker. Mixed gateway rows remain
   consumer assembly of several catalogues.

## Remaining Design, Not Product Policy

- crate placement for the store port and simple adapter
- exact sign-in action vocabulary versus today's discovery sign-in actions
- which production routes prove the first tranche (hosted API key, hosted
  OAuth, installed, local runtime)
- whether 047 later admits optional presentation metadata from the overlay
  without changing selection readiness
- `0.3.3` versus `0.4.0` for the pre-facade source tag, under Contract 036

## Acceptance Criteria

- a consumer can list addable routes, admit an instance, complete required
  auth through host ports, refresh readiness, and project a model list with
  overlay applied
- 047 snapshots stay free of emails, tokens, and targets
- enablement and readiness remain independent
- no implementation of this surface ships before the current-source tag named
  by g04.003
- display color, composer routing, and gateway flattening stay consumer-owned

## Promotion Targets

- architecture: connection-lifecycle placement beside 037 and 047
- contracts: new readiness/admission contract; 006/008/014/047 amendments
  only where the selection snapshot and connection facade must stay distinct
- roadmaps: g04.002 spec closeout, g04.003 source tag, then implementation
  after that tag
- logs: operator decisions above
