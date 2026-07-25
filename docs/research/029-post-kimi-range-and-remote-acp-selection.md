# 029 Post-Kimi-Range And Remote ACP Selection

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

Which remaining provider or transport proof adds the most architectural
information after Kimi capability-range conformance?

## Method

Sources were accessed 2026-07-24. Evidence includes official ACP lifecycle and
transport records, the maintained ACP Rust SDK repository and published crate
source, official provider or maintained-project documentation, and realized
Swallowtail descriptors.

No login, credential, provider request, installation, update, container,
remote agent, model download, or live inference was used.

## Realized Inventory

Swallowtail has 21 production driver descriptors. They use 16 exact integration
family ids across 14 provider or runtime brands.

| Family and route | Layer and operation | Transport and topology | Version posture |
| --- | --- | --- | --- |
| `alibaba-model-studio` conversation | direct interactive | hosted HTTPS/SSE | dated facade; no claim |
| `anthropic` Messages | direct structured | hosted HTTP/SSE | dated API facade; no claim |
| `anthropic` Managed Agents | harness structured | provider-managed HTTPS/SSE | dated beta facade; no claim |
| `amazon-bedrock` Runtime | direct structured | in-process Rust SDK/EventStream | exact SDK pin; no claim |
| `amazon-bedrock` catalogue | direct catalogue | in-process Rust control-plane SDK | exact SDK pin; no claim |
| `codex` exec | harness structured | local or remote-authoritative CLI process | qualified six-month range; unverified newer |
| `codex` app-server | harness interactive | local or remote-authoritative JSONL-RPC process | qualified six-month range; unverified newer |
| `deepseek` V4 | locally continued direct interactive | hosted HTTP/SSE | exact opaque facade; qualified-only |
| `gemini-cli` ACP | harness interactive | local or remote-authoritative ACP stdio | exact `0.51.0`; no claim |
| `gemini` Live | direct realtime interactive | hosted raw WebSocket | dated preview facade; no claim |
| `kimi-code` ACP | persistent harness interactive | local or remote-authoritative ACP stdio | exact `0.28.1` and `0.29.0`; unverified newer |
| `kimi-platform` K3 | direct structured | hosted HTTP/SSE | dated facade; no claim |
| `llama.cpp` attached | direct structured and catalogue | attached HTTP/SSE | exact build/facade; no claim |
| `llama.cpp` owned | direct structured, catalogue, serving | owned process plus HTTP/SSE | exact build/facade; no claim |
| `ollama` native | direct structured and catalogue | attached local or remote-authoritative HTTP/NDJSON | qualified `0.14.0..=0.32.1`; qualified-only |
| `openai` background Responses | direct structured | hosted HTTP/SSE with one reattachment | dated facade; no claim |
| `openai` Realtime | direct realtime interactive | hosted WebSocket | dated facade; no claim |
| `opencode` server | harness interactive | attached local or remote-authoritative HTTP/SSE | qualified published range; unverified newer |
| `pi` RPC | harness interactive | local or remote-authoritative strict-LF JSONL process | exact `0.80.10`; qualified-only |
| `qwen-code` headless | harness structured | local or remote-authoritative streaming-JSON CLI | exact `0.19.11`; no claim |
| `xai` Responses | direct interactive | hosted WebSocket | dated facade; no claim |

The twelve common profiles cover one-shot CLI, long-lived RPC or process ACP,
persistent process ACP, attached network harness, hosted direct API,
provider-managed remote harness, connection-scoped direct session, locally
continued direct inference, realtime media, and attached or owned self-hosted
runtimes.

Remaining architectural gaps:

- remote ACP connection transport
- interactive authentication lifecycle
- a provider-supported local harness SDK usable from Rust without an unowned
  language bridge
- persistent owned serving beyond the completed ephemeral proof

Further version-range maintenance remains necessary, but it now deepens a
represented boundary rather than adding a missing one.

## Material Evidence Delta

### Remote ACP crossed the implementation threshold

The ACP Streamable HTTP and WebSocket transport RFD is Active. It requires
clients to support HTTP/2 Streamable HTTP and WebSocket, uses one `/acp`
endpoint, keeps connection and session identity distinct, and leaves
authentication orthogonal to the transport.

The maintained Rust SDK now publishes
`agent-client-protocol-http = 2.0.0`. Its source provides an HTTP/SSE client,
WebSocket selection for `ws` and `wss` endpoints, a custom `reqwest::Client`
injection point for HTTP, and connection close through the normal run
lifecycle.

This corrects Research 028's conclusion that maintained SDK support had not
landed.

Limits remain material:

- the RFD leaves reconnect, retry, liveness, and affinity behavior to
  implementers
- v1 has no in-flight replay or stream resumption
- protocol-version headers and later hardening remain unfinished
- authentication and status RFDs remain Draft
- the SDK's HTTP client needs an explicitly cookie-enabled client for affinity
- the published client uses unbounded internal channels for SSE delivery and
  the core SDK `Channel` is also unbounded
- the published HTTP dependency disables reqwest's HTTP/2 feature even though
  the Active RFD requires HTTP/2
- the RFD requires cookie handling for both HTTP and WebSocket; the SDK's
  WebSocket path does not expose equivalent custom client or cookie-store
  injection and does not retain upgrade-response headers
- SDK package `2.0.0`, ACP wire version 1, RFD revision, and remote agent
  artifact version are independent
- SDK error bodies and endpoint details must be sanitized at the Swallowtail
  boundary
- the maintained client therefore cannot be the first production transport
  actor under Contract 035's bounded-state rule

Evidence:

- [ACP RFD lifecycle](https://agentclientprotocol.com/rfds/updates)
- [Streamable HTTP and WebSocket transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
- [ACP Rust SDK repository](https://github.com/agentclientprotocol/rust-sdk)
- [`agent-client-protocol-http` 2.0.0](https://docs.rs/crate/agent-client-protocol-http/2.0.0)
- [HTTP client API](https://docs.rs/agent-client-protocol-http/2.0.0/agent_client_protocol_http/struct.HttpClient.html)
- [HTTP client source](https://docs.rs/agent-client-protocol-http/2.0.0/src/agent_client_protocol_http/client.rs.html)

### Other candidates add less new information

Grok Build now documents both streaming JSON and ACP stdio plus optional
sandboxing. Those routes repeat Swallowtail's realized structured-CLI and
process-ACP shapes. They remain useful provider breadth, not the next
architectural proof.

Claude Agent SDK and Cursor SDK remain Python or TypeScript embeddings. Claude
bundles Claude Code and recommends container isolation for production hosting.
Cursor remains a public-beta SDK. Both would add a foreign runtime before
proving the available Rust-native shared transport.

vLLM and SGLang would add deployment breadth but largely repeat the attached
HTTP serving boundary with heavier operational weight. Persistent owned
serving also approaches Monkey's authority boundary and needs a separate
ownership decision.

Evidence:

- [Grok Build overview](https://docs.x.ai/build/overview)
- [Grok CLI reference](https://docs.x.ai/build/cli/reference)
- [Grok headless scripting](https://docs.x.ai/build/cli/headless-scripting)
- [Claude Agent SDK overview](https://code.claude.com/docs/en/agent-sdk/overview)
- [Claude Agent SDK hosting](https://code.claude.com/docs/en/agent-sdk/hosting)
- [Cursor SDK release](https://cursor.com/changelog/sdk-release)
- [Cursor changelog](https://cursor.com/changelog/page/1)

## Ranking

| Rank | Boundary | New pressure | Decision |
| --- | --- | --- | --- |
| 1 | remote ACP | shared network protocol, long-lived HTTP/SSE or WebSocket, affinity, connection/session separation | select |
| 2 | Grok Build | first-party xAI harness breadth | later; repeats proven local transports |
| 3 | Claude or Cursor SDK | embedded local agent runtime | later; foreign runtime and maturity weight |
| 4 | interactive ACP auth | login and status lifecycle | wait; current RFDs are Draft |
| 5 | persistent owned serving | durable local serving authority | later; heavy and close to Monkey boundary |
| 6 | more installed ranges | maintenance depth | continue after the missing transport proof |

## Decision

Select a provider-neutral remote ACP transport proof.

The first proof is deliberately narrow:

- one exact host-approved `http`/`https` or `ws`/`wss` endpoint
- explicit transport selection from the endpoint; no negotiation or fallback
- ACP wire version 1
- exact `agent-client-protocol-http = 2.0.0` and matching core SDK pin
- one operation-owned connection and one active remote session
- HTTP/2 Streamable HTTP/SSE and WebSocket deterministic fixtures
- connection id, ACP session id, and Swallowtail operation id remain distinct
- connection-scoped opaque cookie state for HTTP and WebSocket affinity
- unauthenticated access only
- `ExperimentalObserved` support authority and explicit consumer opt-in
- local and remote-authoritative execution-host conformance
- explicit close and joined reader, callback, connection, and credential-free
  cleanup

Source audit during card 134 confirms the maintained transport client cannot
meet the combined HTTP/2, bounded-queue, and WebSocket-cookie rules. The shared
crate must use the exact maintained core ACP schema privately while owning
bounded HTTP/SSE and WebSocket physical transport actors. The maintained HTTP
crate remains an exact cross-check dependency and server oracle, not the sole
client implementation. If the core schema cannot be used without leaking SDK
types or unbounded actors, implementation stops.

The first proof excludes:

- a generic ACP provider or integration-family registration
- live public endpoints
- authentication headers, query credentials, subprotocol credentials,
  interactive login, or credential mutation
- automatic reconnect, retry, replay, resumption, failover, or transport
  fallback
- connection pooling, multiplexing, or global clients
- a provider/model/agent selection
- a stable remote-ACP support claim
- treating the SDK package version as an observed interface-version range

The reusable implementation belongs in a separate
`swallowtail-transport-acp-remote` crate. `swallowtail-protocol-acp` remains
the provider-neutral wire and fixture boundary without acquiring HTTP runtime
dependencies. Provider adapters may later compose the transport with their
own family, agent, access, capability, and version evidence.

Remote ACP adds a thirteenth conformance profile. The existing process ACP
profile owns process framing and process cleanup; the attached network-harness
profile lacks ACP connection, session, callback, affinity, and no-replay
semantics. Combining those assertions would hide a real transport difference.

## Version Posture

Contract 029 still governs installed or runtime-observed interface versions.
The first remote transport has no runtime protocol-version header to qualify
as a maintained range.

Record these axes separately:

- ACP wire version negotiated by `initialize`
- remote transport RFD revision and lifecycle status
- compile-time HTTP transport SDK version
- compile-time ACP core SDK version
- remote agent implementation and artifact version when later known
- configured endpoint and instance revision

Unknown newer remote agents may be observed only after a provider-specific
adapter supplies its own version and capability evidence. The shared transport
does not invent a guaranteed or unverified-newer range.

## Promotion

- durable boundary: Contract 035
- implementation sequence: g01 roadmap 045 and cards 133-135
- first ready task: card 133, shared records, independent corpus, and the
  thirteenth conformance profile
- next planning checkpoint: select the first provider-specific remote ACP
  adapter only after the shared transport proof closes
