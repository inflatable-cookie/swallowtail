# 170 Second-Proof Addable-Route Inventory

Status: accepted
Owner: Tom
Date: 2026-08-20
Lane: g04.015

## Question

Which production routes can take a second addable-route tranche on the
shapes already proved by Anthropic Messages, Codex app-server, and Ollama
attach, without hosted URL-open OAuth or secret extraction?

## Method

Repository-local inventory of `deepseek.continuation`, `claude-agent.acp`,
and `llama-cpp.attached` against Contract 057. No live provider, install,
or login work. Hosted interactive OAuth stays a remaining gate.

## Shared Facts

- First-proof addable descriptors exist only for `anthropic.messages`,
  `codex.app-server`, and `ollama.attached`.
- Remaining production routes stay on the prepared-facade path until a
  named adapter-local descriptor lands.
- Preparation still starts after a configured instance exists. Second-proof
  must not fold 057 admission into `prepare_*`.
- Overlay still cannot invent a catalogue `provider_id`.
- Hosted URL-open OAuth is not a candidate in this tranche.

## DeepSeek Continuation

| Surface | Current record | 057 mapping |
| --- | --- | --- |
| Route / driver | `deepseek.continuation`; `swallowtail.deepseek.direct`; family from the direct descriptor; transport OpenAI-compatible HTTPS/SSE | Adapter-local addable descriptor. Topology **hosted**, not `ExecutionLayer::DirectModelInference` |
| Availability | Descriptor absence means the crate is unlinked | `Available` when Credential host service exists; `Unavailable(HostService)` otherwise |
| Credential | `CredentialMechanism::ApiKey`, pay-as-you-go, audience `api.deepseek.com`, required `CredentialRef` | Secret API-key field. Guide forbids reading environment values into portable records. Do not invent an env name |
| Sign-in | None. Preparation rejects non-API-key profiles. Guide rejects OAuth, proxies, and Anthropic facades | API-key collection only. No URL-open, loopback, or device-code ports |
| Config | Exact host-approved `https://api.deepseek.com` target | Opaque `ApiEndpoint` `ConfigFieldRef`. Prepare still takes host `InstanceTargetRef` |
| Prepare | `prepare_deepseek_direct` | Reuse after admission |
| Overlay | Catalogue entries already set `provider_id` `deepseek` | Overlay keys work. Do not change catalogue identity |
| Subject | No account observation | `Absent`. Do not probe Open Platform for identity |
| 029 / 032 | Opaque exact `deepseek-openai-chat-2026-07-22`, `QualifiedOnly` | Update observation can reuse `deepseek_facade_claim`; 032 unobserved |
| Refresh | Caller-asserted or observed at prepare | Host-supplied `AccessStatus`. Enablement stays independent |

This is the smallest second hosted API-key proof. It is not hosted OAuth.

## Claude Agent ACP

| Surface | Current record | 057 mapping |
| --- | --- | --- |
| Route / driver | `claude-agent.acp`; `swallowtail.claude-agent.acp`; ACP v1 stdio | Adapter-local addable descriptor. Topology **installed**, not `ExecutionLayer`. Do not advertise `claude-code.headless` or `claude-code.response-only` from this row |
| Availability | Installed-executable probe already exists | `Available` when Process host service exists; `Unavailable(HostService)` otherwise. Discovery candidates are not catalog rows |
| Credential | Two explicit profiles: `ApiKey` pay-as-you-go with `CredentialRef`, or `LocalUnauthenticated` subscription with no credential | First installed proof was Codex ChatGPT with **no credential field**. Map the addable row to the local subscription profile only. API-key billing stays a separate explicit profile, not this row. Do not extract keychain bytes |
| Sign-in | Local subscription is inherited process/login state | Not hosted URL-open OAuth. No browser ports on this row |
| Config | `InstalledExecutableTarget` plus environment | Opaque binary-path and environment config refs |
| Prepare | `prepare_claude_agent` after executable observation | Reuse after admission |
| Overlay | ACP model options are session-negotiated; no Messages-style catalogue `provider_id` | Leave unmarked. Do not invent a provider id |
| Subject | No facade observation of Claude account email | `Absent` |
| 029 / 032 | Maintained `0.53.0..=0.70.0` excluding `0.58.0`, unverified-newer allowed | Reuse the existing claim plus 032 observation |
| Refresh | Caller-asserted access | Host-supplied `AccessStatus`. Do not scrape login files |

This is the second installed proof. It is not the hosted OAuth gate.

## llama.cpp Attached

| Surface | Current record | 057 mapping |
| --- | --- | --- |
| Route / driver | `llama-cpp.attached`; `swallowtail.llama-cpp.attached-openai-chat`; OpenAI-compatible HTTP/SSE | Adapter-local addable descriptor. Topology **local-runtime**. Do not advertise `llama-cpp.owned` |
| Availability | Prepare health-probes an operator-owned server | `Available` when Network host service exists; `Unavailable(HostService)` otherwise. The addable row does not probe. Swallowtail does not start or stop the server |
| Credential | `LocalUnauthenticated`, no `CredentialRef` | No credential field. No sign-in loop |
| Config | Host-approved endpoint target | Opaque `ApiEndpoint` `ConfigFieldRef` |
| Prepare | `prepare_llama_cpp_attached` | Reuse after admission. Exact opaque b9910/f5525f7e7 binding. No unverified-newer |
| Overlay | Catalogue entries omit `provider_id` | Stay unmarked. Do not invent a provider id |
| Subject | No account | `Absent` |
| 029 / 032 | `llama_cpp_attached_runtime_claim`; exact opaque attached runtime | Update observation can reuse the claim; 032 unobserved unless an executable is supplied |
| Refresh | Host-supplied `AccessStatus` from later observation | No credential dimension. Do not treat owned serving as attach |

This is the second local-runtime proof. Owned ephemeral serving stays a
different route.

## First Implementation Tranche

Inventory does not contradict a hosted API-key second proof first:

- DeepSeek continuation
- adapter-local hosted descriptor
- secret API-key field, no invented env name
- opaque endpoint config
- 057 API-key collection, no browser ports
- existing `prepare_deepseek_direct` after admission
- subject `Absent`
- overlay keys already possible because catalogue rows carry `deepseek`

Leave Claude Agent ACP, llama.cpp attached, and hosted OAuth behind that
tranche. Do not mark the remaining production routes as addable.

## Stop Check

Nothing here requires Swallowtail to store secrets, embed a browser, or
run a server. OpenHands stays without a production route.
