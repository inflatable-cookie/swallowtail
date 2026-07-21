# 005 Post-Tranche Coverage Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Question

Which remaining provider or transport proof adds the most architectural
information after Codex structured CLI and app-server, OpenCode HTTP/SSE,
Anthropic hosted HTTP/SSE, Gemini ACP, and attached llama.cpp?

## Method

Official provider documentation, maintained-project documentation, current
release records, repository contracts, and the adjacent Monkey authority
surfaces were checked on 2026-07-20. No credential store was inspected, no
provider account was queried, and no authenticated inference request was made.

Candidates were compared on transport novelty, access authority, lifecycle,
fixture stability, missing shared contracts, and value to both harness-heavy
and direct-inference consumers.

## Evidence Deltas

### xAI Responses WebSocket

xAI now documents a provider-supported WebSocket mode for the generally
available Responses API at `wss://api.x.ai/v1/responses`. This is not a second
encoding of one HTTP request:

- one connection carries several serial `response.create` turns
- the latest response remains in a connection-local cache
- a follow-up sends only new input plus `previous_response_id`
- `store=false` and Zero Data Retention can still use the connection-local
  continuation path
- a connection processes turns serially and queues an extra provider request
  rather than multiplexing
- one connection has a documented 25-minute maximum lifetime
- reconnect can continue only from provider-stored state; `store=false` or ZDR
  requires a new chain with consumer-supplied full context
- provider failures can evict the continuation point
- `previous_response_not_found` and `websocket_connection_limit_reached` are
  explicit protocol errors

The documented event types and order match Responses streaming. The first
proof can therefore freeze a bounded text-only event subset without enabling
provider tools, background work, warmup, storage, or automatic reconnect.

Authentication is an xAI API key sent as a bearer credential to the xAI API
audience. Keys are team-bound and may carry endpoint and model ACLs. Credits,
model entitlement, rate limits, and key validity remain separate observations.
This is a provider-supported public API route, not Grok product OAuth or a
harness-maintained subscription endpoint.

xAI also reports `cost_in_usd_ticks` in each inference response. The provider
defines this as the exact charged amount for that request, inclusive of
discounts and server-side tool calls. Streaming observations are cumulative;
the final value is authoritative for that request. One USD is exactly
10,000,000,000 ticks. This is materially different from catalogue price
metadata or a consumer estimate and exposes the missing provider-billed-cost
boundary deferred by Contract 014.

The current Models API can list language models available to the authenticating
key and reports ids, aliases, modalities, fingerprints, and price metadata. It
uses HTTP, not the Responses WebSocket. The first WebSocket proof does not
silently derive a second endpoint or combine two configured instances. It uses
one explicit model route. A later xAI catalogue driver may bind its own exact
HTTP endpoint and transport identity.

Evidence:

- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI Responses API](https://docs.x.ai/developers/rest-api-reference/inference/chat)
- [xAI cost tracking](https://docs.x.ai/developers/cost-tracking)
- [xAI Models API](https://docs.x.ai/developers/rest-api-reference/inference/models)
- [xAI API authorization](https://docs.x.ai/developers/rest-api-reference/management/auth)
- [xAI rate limits](https://docs.x.ai/developers/rate-limits)

### Second ACP Agent

Kimi Code CLI is now the strongest second ACP candidate. The earlier Python
`MoonshotAI/kimi-cli` project is being wound down in favor of the current
TypeScript `MoonshotAI/kimi-code` project. Release `0.28.0` was current on
2026-07-20 and publishes `kimi acp` over JSON-RPC stdio.

Its maintained ACP matrix reports SDK `0.23.0`, stable new, load, resume,
prompt, cancel, list, mode/configuration, image, embedded-resource, permission,
filesystem-read, and filesystem-write surfaces. Native session close and
terminal reverse RPC remain absent. The CLI reuses prior Kimi login state; ACP
clients do not own login.

This is good portability evidence for `swallowtail-protocol-acp`, especially
capability drift, load versus resume, and a second agent implementation. It is
not the next proof because a safe first subset would repeat the already proven
new-session/read/cancel stdio shape. Using its new capabilities would first
need a contract for provider-owned persistent session state, replay ordering,
write callbacks, and delegated Kimi login without inspecting the credential
store.

Evidence:

- [Kimi Code CLI repository](https://github.com/MoonshotAI/kimi-code)
- [Kimi ACP capability matrix](https://moonshotai.github.io/kimi-code/en/reference/kimi-acp)
- [Kimi Code IDE and access guidance](https://moonshotai.github.io/kimi-code/en/guides/ides)
- [Kimi Code `0.28.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.28.0)

### SDK-Native Routes

Current SDK candidates do not yet justify a Rust SDK driver:

- Qwen Code's Python SDK is explicitly experimental and drives an external
  `qwen` process through stream JSON.
- Qwen Code's TypeScript SDK is experimental, requires the CLI, and adds
  language-runtime, embedded-MCP, permission, and inherited-environment policy.
- Kimi Agent SDK offers Go, Node.js, and Python clients around Kimi Code, but no
  Rust package. It retains the CLI as execution engine.

These SDKs expose useful harness lifecycle, but from Swallowtail's Rust process
they would require a language bridge or repeat an underlying process transport.
They do not yet prove the missing in-process Rust lifecycle shape. Do not call a
sidecar bridge an SDK-native driver. Recheck when a maintained Rust SDK or an
explicit cross-language embedding boundary exists.

Evidence:

- [Qwen Code Python SDK](https://qwenlm.github.io/qwen-code-docs/en/developers/sdk-python/)
- [Qwen Code TypeScript SDK](https://qwenlm.github.io/qwen-code-docs/en/developers/sdk-typescript/)
- [Kimi Agent SDK](https://github.com/MoonshotAI/kimi-agent-sdk)

### Owned Self-Hosted Lifecycle

Contract 007 already permits a serving driver to launch a service only under an
explicit host grant. The synthetic owned-self-hosted profile proves that common
shape. A production proof could extend the pinned llama.cpp route with an
operator-supplied artifact, host-owned ephemeral process, readiness deadline,
and stop/join cleanup. It must not download, convert, license, move, or select
the model artifact.

The adjacent Monkey repository now owns a working `monkey serve` OpenAI Chat
facade and `monkey up`/`monkey down` warm-server lifecycle. Swallowtail must
attach to that service if it later integrates Monkey; it must not duplicate or
take over Monkey's model loading, PID, readiness, or serving policy. This makes
owned llama.cpp lifecycle a valid later host-mechanism proof, but not a reason
to absorb Monkey.

Owned serving remains lower-information than xAI WebSocket because attached
llama.cpp already proved the model/deployment/facade boundary and the shared
runtime already has an owned synthetic profile. The missing work is concrete
host process and resource authority, not a new execution layer or wire
lifecycle.

Evidence:

- [llama.cpp server `b9910`](https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/README.md)
- `/Users/tom/Dev/projects/monkey/docs/roadmaps/g10/043-openai-chat-serving.md`
- `/Users/tom/Dev/projects/monkey/docs/logs/2026-07/18-g12-011-monkey-up.md`

## Comparison

| Candidate | New information | Main contract pressure | Fixture posture | Rank |
| --- | --- | --- | --- | --- |
| xAI Responses WebSocket | connection-scoped direct session, serial turn chain, expiry/reconnect, exact billed cost | resource-free direct sessions, connection-bound leases, continuation/cancel semantics, billed-cost evidence | deterministic fake WebSocket; live key separately gated | 1 |
| Kimi Code ACP `0.28.0` | second agent, ACP portability, load/resume, write callbacks | persistent provider sessions, replay, delegated login, write authority | deterministic ACP corpus; no installed binary | 2 |
| owned llama.cpp | concrete start/readiness/stop and artifact lease | launch authority, readiness, failure teardown, resource bounds | existing pinned binary/facade; artifact operator-supplied | 3 |
| SDK-native | language-native callback and lifecycle surface | Rust embedding or explicit language bridge | current candidates wrap CLI and lack Rust package | 4 |

Additional HTTP/SSE hosted providers and one-shot JSONL harnesses remain useful
coverage. They rank after these shapes unless provider evidence exposes a new
access or lifecycle boundary.

## Recommendation

Select xAI Responses WebSocket as the next proof.

The first route is a direct-model interactive session over one host-approved
WebSocket endpoint. It uses one provider-supported xAI API key, one exact
configured model route, `store=false`, text input/output, no provider or client
tools, one active turn, no warmup, no background mode, no automatic retry, and
no resume after the socket closes.

This selection does not establish unsettled consumer routing policy. xAI public
API access is already in operator scope, the provider and endpoint authority are
explicit, deterministic fixtures lead, and live authentication stays gated.

## Required Promotion

Contract 016 must settle before implementation:

- interactive direct inference without a fake working resource
- session-scoped endpoint and credential leases
- serial one-active-turn behavior and provider continuation correlation
- cancellation by socket close when no text-response cancel method is
  documented
- no implicit reconnect, replay, retry, storage, or durable resume
- provider-authoritative billed-cost evidence distinct from usage, catalogue
  price, entitlement, rate, and consumer accounting

## Concrete Sequence

1. xAI Responses WebSocket direct-session fixtures, driver, and conformance.
2. Kimi Code ACP currentness/fixture contract, then a second-agent portability
   proof centered on load/resume rather than repeating Gemini's subset.
3. Host-owned ephemeral llama.cpp lifecycle, without artifact acquisition or
   Monkey ownership.
4. Recheck SDK-native candidates; proceed only with a real Rust embedding or an
   explicitly contracted language bridge.
5. Expand hosted HTTP/SSE and one-shot CLI coverage by provider value and access
   authority, not protocol similarity.

## Promotion

- durable direct-session and billed-cost behavior: Contract 016
- delivery sequence: g01 roadmap 017 and cards 052-054
- next implementation batch: card 052
