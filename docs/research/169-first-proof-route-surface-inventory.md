# 169 First-Proof Route Surface Inventory

Status: accepted
Owner: Tom
Date: 2026-08-20
Lane: g04.010

## Question

What do Anthropic Messages, Codex app-server, and Ollama attach already
expose, and what Contract 057 still needs before a consumer can add those
routes through the connection-lifecycle facade?

## Method

Repository-local inventory of the three production adapters, their prepared
facades, driver descriptors, access profiles, compatibility claims, and
catalogue identities. No live provider, install, or login work. OAuth
candidate selection is classified, not chosen as a product preference.

## Shared Facts

- No production adapter exposes `AddableRouteDescriptor`.
- No production adapter calls `DriverDescriptor::with_sign_in_actions`.
  `SignInAction` remains an unused advertisement.
- No production adapter registers `HostServiceKind::{UrlOpen,
  LoopbackCallback, DeviceCodeDisplay}`. Those ports exist only on
  host-local sign-in test doubles.
- Contract 037 preparation still starts after a configured instance exists.
  First-proof must not fold 057 admission into `prepare_*`.
- Contract 047 snapshots can already be assembled from prepared evidence.
  Overlay is a projection over that snapshot. Adapters do not implement
  overlay.

## Anthropic Messages

| Surface | Current record | 057 gap |
| --- | --- | --- |
| Route / driver | `anthropic.messages`; `swallowtail.anthropic.direct`; family `anthropic`; transport `http-sse` | Adapter-local addable descriptor. Topology is **hosted**, not `ExecutionLayer::DirectModelInference` |
| Availability | Descriptor absence today means the crate is unlinked | Hosted row is `Available` when admission host services exist; `Unavailable(HostService)` otherwise. Not install/runtime |
| Credential | `CredentialMechanism::ApiKey`, pay-as-you-go, audience `api.anthropic.com`, required `CredentialRef`, `HostServiceKind::Credential` | Credential-field descriptor: secret API key. Optional env name `ANTHROPIC_API_KEY` is a name, not a resolved value. Host stores bytes. 057 API-key collection writes `CredentialRef` |
| Sign-in | None. Preparation rejects non-API-key profiles. Guide rejects Claude subscription on this route | API-key collection only. No URL-open, loopback, or device-code ports |
| Config | Host-approved `InstanceTargetRef` for `api.anthropic.com` | Config-field descriptor: `ApiEndpoint`, opaque `ConfigFieldRef` |
| Discovery | No `DiscoveryAction` on the driver | Do not use Contract 008 discovery as the addable-route row |
| Prepare | `prepare_anthropic_direct` then catalogue / inference / session | Reuse after 057 admission. Do not change the prepared facade |
| 047 | Consumer-assembled snapshot from prepared evidence | Consumer replaces the snapshot after refresh. Do not add 057 fields to 047 |
| Overlay | Catalogue entries already set `provider_id` `anthropic` | Overlay keys work. No adapter overlay code |
| Subject | No account email/login/plan observation | Default `Absent`. Do not probe Messages for identity |
| 029 / 032 | Opaque exact `anthropic-2023-06-01`, `QualifiedOnly`. Not an installed executable | Update observation can reuse the facade claim as unobserved 032. There is no install/upgrade affordance |
| Refresh | Access status is caller-asserted or observed at prepare | Host-supplied `AccessStatus` through `refresh_readiness`. Enablement stays independent |

This is the smallest hosted API-key proof. It does not prove hosted
interactive OAuth.

## Codex App-Server

| Surface | Current record | 057 gap |
| --- | --- | --- |
| Route / driver | `codex.app-server`; `swallowtail.codex.app-server`; family `codex`; transport `jsonl-rpc-stdio` | Adapter-local addable descriptor. Topology is **installed**, not `ExecutionLayer::HarnessInteraction` |
| Availability | Installed-executable probe already exists | `Unavailable(Install)` when the executable is absent. Discovery candidates still are not catalog rows |
| Credential | `codex_chatgpt_subscription_access_profile`: `InteractiveOauth`, subscription allowance, audience `codex`, **no** credential reference. Guide: cached local ChatGPT login, Swallowtail does not login | Do not treat this as hosted URL-open OAuth. Secret-free path is process-inherited login state. 057 delegated CLI login via `Process` may later map to an approved login helper. API-key and enterprise profiles stay separate |
| Sign-in | `SignInAction` unused. No browser ports | Installed proof must not extract ChatGPT tokens |
| Config | `InstalledExecutableTarget` plus `EnvironmentRef` | Config-field descriptors: binary path and opaque env ref. Values stay host-private |
| Discovery | `DiscoveryAction::Probe` + 032 classification against the app-server claim | Keep discovery on the selected driver. 057 lists the addable row first |
| Prepare | `prepare_codex(AppServer)` after executable observation | Reuse after admission |
| 047 | Prepared evidence already projects a snapshot | Unchanged |
| Overlay | `ModelCatalogEntry::new` **without** `provider_id` | Overlay cannot mark those rows. Later Codex proof must either report a catalogue provider id or accept unmarked models |
| Subject | No facade observation of ChatGPT email | Restricted 057 subject later; never a 047 field or instance id |
| 029 / 032 | Semantic `0.80.0..=0.148.0` with exclusions, `AllowUnverified` | `observe_instance_update` can reuse the existing claim plus 032 observation |
| Refresh | Caller-asserted access; helper does not inspect login | Host-supplied `AccessStatus`. Do not scrape login files |

## Ollama Attach

| Surface | Current record | 057 gap |
| --- | --- | --- |
| Route / driver | `ollama.attached`; `swallowtail.ollama.native-attached`; family `ollama`; transport `http-ndjson-native` | Adapter-local addable descriptor. Topology is **local-runtime** |
| Availability | Prepare probes `/api/version` | `Unavailable(Runtime)` when the attached runtime is missing. Swallowtail does not install or start Ollama |
| Credential | `LocalUnauthenticated`, local compute, no `CredentialRef`, no Credential host service | No credential-field descriptor. No sign-in loop |
| Config | Host-approved endpoint target | Config-field descriptor: `ApiEndpoint` |
| Discovery | No `DiscoveryAction` on the descriptor. Prepare observes version, tags, ps, show | Probe stays preparation/refresh, not the addable catalog |
| Prepare | `prepare_ollama_attached` with model tag and digest | Reuse after admission. Model tag/digest stay prepare-time, not 057 admission identity |
| 047 | Snapshot from prepared evidence | Unchanged |
| Overlay | Catalogue entries **without** `provider_id` | Same overlay-key gap as Codex |
| Subject | No account | `Absent` |
| 029 / 032 | `ollama.runtime` `0.14.0..=0.32.14` excluding `0.32.2` and `0.32.10`, `AllowUnverified`. Version from `/api/version`, not an installed-executable observation | Update observation can reuse the claim; 032 is optional/unobserved unless a later card classifies an executable |
| Refresh | Probe already observes runtime version and reachability | Host-supplied `AccessStatus` from that observation. No credential dimension |

## OAuth Evidence (Not A Selection)

Contract 057 still names hosted interactive OAuth as a later first-proof:
Anthropic or Claude subscription, without extracting secrets.

What exists today:

- Anthropic Messages **cannot** be that proof. It requires a public API key
  and rejects subscription profiles.
- Claude Agent ACP / Claude Code headless already use **local** subscription
  state by inheriting process environment. That is installed, not hosted
  URL-open OAuth. Swallowtail does not extract `ANTHROPIC_API_KEY` or
  keychain bytes on the subscription path.
- Codex ChatGPT access is **cached local login**, also not hosted URL-open
  OAuth.

No production adapter currently runs 057 `start_sign_in` with URL-open or
loopback. Hosted interactive OAuth remains a **remaining gate**. Do not
reclassify Claude Agent or Codex ChatGPT as that hosted proof.

## First Tranche

Inventory does not contradict Contract 057's hosted API-key first proof:

- Anthropic Messages
- adapter-local hosted descriptor
- secret API-key field plus opaque endpoint config
- 057 API-key collection, no browser ports
- existing `prepare_anthropic_direct` after admission
- subject `Absent`
- overlay keys already possible because catalogue rows carry `anthropic`

Leave Codex, Ollama, hosted OAuth, and Contract 052 consumer-path work
behind that tranche.

## Stop Check

Nothing in this inventory requires Swallowtail to store secrets, embed a
browser, or run a server. OpenHands stays without a production route.
