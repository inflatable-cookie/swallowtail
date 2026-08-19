# 011 Route Readiness And Connection Admission

Status: draft
Owner: Tom
Updated: 2026-08-19
Research: 168

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

## Inventory Fold

Research 168 maps each consumer surface onto an existing record, a gap, or a
consumer overlay. The whole lifecycle is not already covered. 047 stays the
selection snapshot.

Material facts:

- `SignInAction` is unused by production adapters
- `DiscoveryDriver` cannot list addable routes; consumers import adapter crates
- `AccessStatus` / 047 credential posture already match readiness chips
- ACP `authenticate` is not login
- `ExecutionLayer` is not Hosted / Installed / Local runtime
- no host port exists for browser, loopback callback, or device-code display

Crate placement, pending implementation:

- records in `swallowtail-core`
- roles and store trait in `swallowtail-runtime`
- optional in-memory and JSON-file adapters in `swallowtail-host-local`
- addable descriptors adapter-local, like prepared facades

First-proof routes, pending the post-tag implementation tranche:

- hosted API key: Anthropic Messages
- hosted interactive OAuth: Anthropic or Claude subscription, whichever can
  be proved without extracting secrets
- installed: Codex app-server
- local runtime: Ollama attach

047 may later carry optional overlay presentation metadata. That must not
change `Ready` / `NotReady`. Accent color stays consumer-only.

The pre-facade source tag remains g04.003. Contract 036's hypothesis is
`v0.3.3` unless that inventory finds a break.

## Contract Targets

New contract, next after 056, owning:

- addable-route catalog
- credential-field descriptors
- sign-in loop through host ports
- store port
- readiness refresh
- authenticated-subject observation
- config-field descriptors
- model-presentation overlay

Amendments, only to keep seams:

| Contract | Bound |
| --- | --- |
| 006 | subject observation and sign-in loop versus credential status |
| 008 | addable route versus discovered candidate versus configured instance |
| 010 | host ports for URL open, loopback callback, device-code display |
| 014 | field descriptors versus credential leases |
| 015 / 017 | delegated harness activation stays distinct from login |
| 029 / 032 | instance update observation reuses claims |
| 037 | preparation remains after admission |
| 047 | no emails, tokens, or targets; overlay does not change selection readiness |

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

- architecture: planned connection-lifecycle placement beside 037 and 047
  (noted in system architecture; not realized)
- contracts: new readiness/admission contract after the g04.003 tag;
  006/008/010/014/015/017/029/032/037/047 amendments only at the named seams
- roadmaps: g04.003 source tag, then contract and implementation
- logs: inventory and contract-target closeout
