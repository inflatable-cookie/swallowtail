# 028 Post-Forward-Compatibility Provider Coverage Selection

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

What should follow the forward-compatibility correction: another provider
transport, or a compatibility proof whose negotiated capabilities change
across releases?

## Method

Sources were accessed 2026-07-24. Evidence includes maintained release
records, tagged source, package locks, official protocol lifecycle records,
official provider documentation, and realized Swallowtail descriptors.

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
| `kimi-code` ACP | persistent harness interactive | local or remote-authoritative ACP stdio | exact `0.28.1`; no claim |
| `kimi-platform` K3 | direct structured | hosted HTTP/SSE | dated facade; no claim |
| `llama.cpp` attached | direct structured and catalogue | attached HTTP/SSE | exact build/facade; no claim |
| `llama.cpp` owned | direct structured, catalogue, serving | owned process plus HTTP/SSE | exact build/facade; no claim |
| `ollama` native | direct structured and catalogue | attached local or remote-authoritative HTTP/NDJSON | qualified `0.14.0..=0.32.1`; qualified-only |
| `openai` background Responses | direct structured | hosted HTTP/SSE with one reattachment | dated facade; no claim |
| `openai` Realtime | direct realtime interactive | hosted WebSocket | dated facade; no claim |
| `opencode` server | harness interactive | attached local or remote-authoritative HTTP/SSE | qualified published range; unverified newer |
| `pi` RPC | harness interactive | local or remote-authoritative strict-LF JSONL process | exact `0.80.10` claim; qualified-only |
| `qwen-code` headless | harness structured | local or remote-authoritative streaming-JSON CLI | exact `0.19.11`; no claim |
| `xai` Responses | direct interactive | hosted WebSocket | dated facade; no claim |

The twelve common profiles cover one-shot CLI, long-lived RPC, ACP,
persistent ACP, attached network harness, hosted direct API, provider-managed
remote harness, connection-scoped direct session, realtime media, attached and
owned self-hosted runtime, and locally continued direct inference.

The original transport plan is represented. Remaining gaps are capability
evolution across harness releases, supported remote ACP, and a provider-
supported local harness SDK boundary usable from Rust without an unowned
language bridge.

## Current Evidence

### Kimi Code `0.29.0` is the strongest next proof

The maintained TypeScript `MoonshotAI/kimi-code` line released `0.29.0` on
2026-07-22. It remains the latest stable release. The existing Swallowtail
route pins `0.28.1`.

Both releases use ACP wire version 1 and lock
`@agentclientprotocol/sdk` `0.23.0`. The Kimi ACP adapter changes from `0.3.4`
to `0.3.5`.

The behavior milestone is narrower and more useful than a new provider name:

- `0.28.1` advertises a `thought_level` select with legacy `off` and `on`
- `0.29.0` advertises `off` plus the selected model's declared effort levels
- models without declared levels retain `off` and `on`
- always-thinking models omit `off`
- legacy `on` remains accepted and maps to the model default
- unsupported concrete levels are rejected before the provider SDK call
- the effective level is re-read and returned after provider normalization

Swallowtail already has typed `ReasoningMode`, `ReasoningSelection`, and
interactive `SessionOptions`. It does not yet have a durable boundary for
applying one portable option through an advertised harness configuration
channel. Contract 015 currently keeps `session/set_config_option` excluded.

Contract 034 closes that gap without adding a generic option map. The first
mapping is reasoning setup on a new session only. Model switching, agent mode,
tool gating, custom agents, load/resume mutation, and arbitrary provider
configuration remain excluded.

The support claim must not infer a continuous semantic interval. It contains
two exact qualified segments:

- `0.28.1`: legacy boolean reasoning behavior
- `0.29.0`: declared effort-level behavior with legacy aliases

`0.28.0`, prereleases, malformed versions, and any point between the two exact
published segments remain outside guaranteed support. Exact stable versions
above `0.29.0` may run as unverified newer through the `0.29.0` private
behavior. Runtime capability drift still fails.

Evidence:

- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
- [`0.28.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.1)
- [`0.29.0` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.0)
- [`0.28.1` ACP option source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.28.1/packages/acp-adapter/src/config-options.ts)
- [`0.29.0` ACP option source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/acp-adapter/src/config-options.ts)
- [`0.29.0` option application](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/acp-adapter/src/session.ts)

### Remote ACP advanced but is not implementation-ready

The Streamable HTTP and WebSocket transport RFD moved from Draft to Active on
2026-07-21. This corrects Research 024.

The transport remains unfinished:

- Goose reference work is in progress
- Rust and TypeScript SDK support follows the reference implementation
- clients must support both HTTP/2 Streamable HTTP and WebSocket
- reconnect, retry, liveness, and affinity remain implementer-owned
- v1 has no in-flight replay or stream resumption
- protocol-version headers and hardening remain later work

Remote ACP is the highest-value future transport proof. Active RFD status is
not a maintained SDK or stable interoperability claim.

Evidence:

- [ACP RFD lifecycle](https://agentclientprotocol.com/rfds/updates)
- [Streamable HTTP and WebSocket RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)

### Grok Build repeats proven local harness shapes

Grok Build remains stable at `0.2.111`. It exposes headless streaming JSON and
ACP stdio. Current CLI documentation also exposes explicit permission rules
and an optional sandbox profile.

That is useful xAI harness breadth. It does not add a transport or lifecycle
missing from Qwen, Gemini, Kimi, Codex, and Pi. Sandbox support remains an
optional provider capability, not a Swallowtail prerequisite.

Evidence:

- [Grok Build overview](https://docs.x.ai/build/overview)
- [CLI reference](https://docs.x.ai/build/cli/reference)
- [stable release pointer](https://x.ai/cli/stable)

### Provider UI servers are not preferred integration authority

Kimi's foreground web server exposes HTTP and WebSocket surfaces for its
bundled UI. Current releases continue changing server endpoints and lifecycle.
The documented external agent-integration route is ACP stdio.

Treating the web backend as a stable third-party harness protocol would create
an owned process-plus-network lifecycle and broad host filesystem surface
without stronger support authority than ACP. Do not select it ahead of the
provider's explicit integration protocol.

Currentness note, 2026-07-27: Research 040 supersedes the authority conclusion,
not the historical tranche ordering. Kimi Code now explicitly documents
`kimi web` as a foreground REST plus WebSocket service with generated OpenAPI
and AsyncAPI. That separate route is eligible for exact-version qualification;
it does not widen ACP.

### Local Agent SDKs remain foreign-runtime bridges

Claude Agent SDK and Cursor SDK expose rich local harness loops through Python
or TypeScript. Cursor remains public beta. Claude bundles a native CLI but
still requires a Python or Node host, and its production hosting guidance
recommends container isolation.

Kimi Agent SDK offers Go, Node, and Python clients around the older Kimi CLI
execution engine. The older Python Kimi CLI is being wound down in favor of
the TypeScript Kimi Code line already used by Swallowtail.

A Rust adapter would therefore own a language bridge, bundled runtime, package
compatibility, and extra lifecycle. That is not the seamless first choice.
The existing Bedrock proof already covers a supported in-process Rust SDK.

Evidence:

- [Claude Agent SDK overview](https://code.claude.com/docs/en/agent-sdk/overview)
- [Claude Agent SDK hosting](https://code.claude.com/docs/en/agent-sdk/hosting)
- [Cursor SDK release](https://cursor.com/changelog/sdk-release)
- [Cursor SDK updates](https://cursor.com/changelog/sdk-updates-jun-2026)
- [Kimi Agent SDK](https://github.com/MoonshotAI/kimi-agent-sdk)
- [Kimi CLI successor notice](https://github.com/MoonshotAI/kimi-cli)

### Heavy self-hosted breadth remains later

vLLM and SGLang would mostly repeat attached serving and compatible HTTP
facades while adding materially heavier deployment. Ollama and llama.cpp
already prove light attach-only and owned local-runtime boundaries. No current
evidence justifies moving them ahead of Kimi capability negotiation.

## Ranking

| Rank | Route | New pressure | Decision |
| --- | --- | --- | --- |
| 1 | Kimi Code `0.28.1` and `0.29.0` | negotiated reasoning capability evolution across exact release milestones | select |
| 2 | remote ACP | shared remote harness transport | wait for maintained SDK support and hardening |
| 3 | Grok Build `0.2.111` | first-party xAI harness breadth | later; ACP and JSONL overlap |
| 4 | Claude or Cursor local SDK | embedded foreign harness runtime | later; bridge weight and authority |
| 5 | Kimi web server | process-plus-network harness | do not prefer provider UI backend over ACP |
| 6 | vLLM or SGLang attached | deployment breadth | later; heavier and largely overlapping |

## Decision

Select the Kimi Code ACP compatibility and reasoning-negotiation tranche.

The bounded first proof is:

- exact qualified `0.28.1` and `0.29.0` segments
- exact installed-executable observation plus ACP handshake corroboration
- unverified-newer execution above `0.29.0`
- ACP wire version 1 and exact SDK `0.23.0` remain separate
- new-session reasoning setup only
- portable `ReasoningMode` mapped privately from `thought_level`
- exact effective-value confirmation before readiness
- existing delegated Kimi access and `AmbientHost` posture
- unchanged new, load, resume, replay, write-callback, cancellation, and joined
  cleanup behavior when session options are empty
- local and remote-authoritative conformance

Excluded:

- installation, update, sign-in, credential mutation, container, or sandbox
- continuous inferred range membership
- automatic provider, model, endpoint, credential, or route selection
- generic provider option transport
- model switching, agent-mode switching, custom agents, global tool gating,
  arbitrary tool policy, and v2 engine expansion
- reasoning mutation on load or resume
- live authentication from default QA

## Promotion

- durable boundary: Contract 034
- implementation sequence: g01 roadmap 043 and cards 129-131
- first ready task: card 129, shared negotiated-option records and Kimi
  multi-release corpus
- remote ACP stays on the evidence watchlist
