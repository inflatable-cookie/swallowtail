# 168 Route Readiness Surface Inventory

Status: accepted
Owner: Tom
Date: 2026-08-19
Lane: g04.001

## Question

Which existing Swallowtail records already cover the Poodle/T3 connection
lifecycle, and where is the gap that Spec 011 must contract?

## Method

Repository-local inventory of core records, runtime roles, Contract 047
admission, host services, and prepared-facade assembly. Poodle specimens and
T3 Code's connection list are evidence. No live provider, install, or login
work.

## Existing Records

| Surface | Where | What it actually does |
| --- | --- | --- |
| `DriverDescriptor` | core `registration` | Static driver identity, roles, layers, host-service needs, discovery and sign-in *advertisements*, compatibility claims |
| `DiscoveryAction` / `DiscoveryOutcome` | core + `DiscoveryDriver` | Probe or refresh one already selected driver. Status: absent, discovered, incompatible, failed. Never promotes a configured instance |
| `SignInAction` | core `registration` | Interactive, device-authorization, or delegate-to-harness *advertisement* |
| `ConfiguredInstance` | core `instance` | Host-admitted instance: opaque target, ownership, access-profile id, facade, versions |
| `AccessProfile` / `AccessStatus` | core `access` | Mechanism, metering, audience, support authority; independent credential, entitlement, endpoint, runtime dimensions. No aggregate ready boolean |
| `CredentialRef` / `CredentialService` | core + runtime host | Opaque host reference; acquire/release leases for an *already stored* secret |
| `InstalledExecutableObservation` | core + Contract 032 | Exact version classified against one claim |
| `InterfaceCompatibilityAssessment` | core + Contract 029 | Qualified, unverified-newer, or incompatible |
| `ModelCatalogDriver` / `ModelCatalogEntry` | core + runtime | Source-scoped models on one prepared instance |
| Prepared facades | Contract 037, adapter crates | Prepare then run one exact operation after the instance exists |
| `ConfiguredProviderInstanceCatalogue` | runtime, Contract 047 | Consumer-assembled *snapshot* of already configured instances. No watcher, probe, persist, or add-connection flow |

Facts that change the later contract:

- no production adapter currently calls `with_sign_in_actions`; the vocabulary
  exists and is unused
- `DiscoveryDriver` discovers candidates of one selected driver, not the
  universe of addable routes
- there is no runtime registry of production routes; consumers import adapter
  crates, as with prepared facades
- 047 `ConfiguredProviderCredentialPosture` already maps to Poodle chips
  (credential ready, runtime unavailable, support prohibited) without emails
  or targets
- 047 `selection_readiness` is derived and conservative; it is not enablement
- ACP `authenticate` (Contract 015) activates an already-authorized harness
  credential. It is not browser or device login
- `ExecutionLayer` is harness versus direct inference, not hosted / installed /
  local-runtime
- `HostServiceKind` has Credential, Process, and Network. It has no browser,
  loopback-callback, or device-code display port

## Consumer Surface Map

| Consumer surface | Existing record | Classification |
| --- | --- | --- |
| Addable-route picker (Hosted / Installed / Local, Available / Unavailable / Unsupported) | `DriverDescriptor`, `SupportAuthority`, `DiscoveryOutcome`, transport/ownership | **New consuming catalog.** Descriptors are adapter-local; availability needs discovery or a static unsupported claim. Grouping is presentation over topology, not `ExecutionLayer` |
| Credential fields (API key paste) | `CredentialMechanism::ApiKey`, `CredentialRef` | **New field descriptors.** Mechanism exists; field schema (label, secret vs public, env name) does not. Host still stores the secret |
| Browser / device / CLI sign-in | `SignInAction`, `CredentialService`, ACP delegated activation | **New runtime loop.** Advertisement and lease acquisition exist; start/poll/complete/cancel and host browser ports do not |
| Admit instance into the list, including several of one family | `ConfiguredInstanceId`, `ConfiguredInstance` | **Reuse plus a store port.** Multiple instances already work as distinct ids. Nothing persists them |
| Enable/disable toggle | none | **Store preference.** Independent of 047 readiness |
| Readiness chips | `AccessStatus`, 047 credential posture | **Reuse, with refresh.** Snapshot today; Spec 011 wants a refresh role over the same dimensions |
| Instance label (Work vs Personal) | none in portable records | **Store field.** Not a provider id |
| Binary path, endpoint, env | `InstanceTargetRef`, host process/env | **New config-field descriptors** over opaque host refs. Values stay host-private |
| Accent color | none | **Consumer overlay.** Out of Spec 011 |
| Authenticated-as email/plan | explicitly forbidden in 047 | **New restricted subject observation** on the connection facade only |
| Version and update affordance | `InstalledExecutableObservation`, Contract 029 claims | **Reuse plus a derived update observation.** Observed version exists. “Official newer than observed” is not a runtime probe today |
| Model list | `ModelCatalogEntry`, 047 bound catalogue | **Reuse** |
| Hide / reorder / default / favourite | none | **Bound overlay** per Spec 011 |
| Composer model pick | 047 `Ready` + bound model ids | **Reuse 047.** Do not replace it |

## Authority Split

Already portable: driver identity, access dimensions, discovery status,
installed version class, prepared routes, 047 selection snapshot.

Host-owned: secret bytes, executable/endpoint/env values, browser placement,
loopback bind, process spawn.

Consumer-assembled today: which adapter crates to link, which instances exist,
when to refresh, UI chrome, composer routing.

Missing portable mechanism: addable-route catalog, credential-field
descriptors, sign-in loop, store port, enablement, instance label,
authenticated subject, config-field descriptors, instance-level update
observation, model overlay.

## Contract Fit

Do not grow 047 into an add-connection facade. Keep it the selection snapshot.

A new contract should own the lifecycle in front of 047: addable descriptors,
admission, sign-in loop, readiness refresh, subject observation, store port,
config fields, overlay.

Amend only to keep seams honest:

- 006 — subject observation and sign-in loop versus credential status
- 008 — addable route versus discovered candidate versus configured instance
- 010 — host ports for URL open, loopback callback, device-code display
- 014 — field descriptors versus credential leases
- 015/017 — delegated harness activation stays distinct from login
- 029/032 — update observation reuses claims; no second currentness system
- 037 — preparation remains after admission
- 047 — still forbids emails, tokens, and targets; overlay must not change
  `Ready`/`NotReady`

## First-Proof Recommendation

Not a product decision: the smallest set that covers the three Poodle groups
plus one OAuth loop.

- hosted API key: Anthropic Messages
- hosted interactive OAuth: Anthropic or Claude subscription, whichever the
  later contract can prove without extracting secrets
- installed harness: Codex app-server
- local runtime: Ollama attach

## Crate Placement Recommendation

- new records in `swallowtail-core`
- lifecycle roles, store trait, and 047-adjacent projection in
  `swallowtail-runtime`
- optional in-memory and JSON-file adapters in `swallowtail-host-local`
- addable descriptors stay adapter-local, like prepared facades
- no umbrella registry crate

A dedicated store crate is unnecessary until a second non-host adapter appears.

## Stop Check

Existing contracts do not cover the whole lifecycle. The lane should continue.
Nothing in this inventory requires Swallowtail to store secrets or run a
server.
