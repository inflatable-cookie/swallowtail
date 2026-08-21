# 171 Further Addable-Route Inventory

Status: accepted
Owner: Tom
Date: 2026-08-21
Lane: g04.022

## Question

Which of the 41 production routes without an addable descriptor can reuse a
proved hosted API-key or installed shape, and which routes stay gated behind a
separate topology, authority, or lifecycle proof?

## Method

Repository-local inventory of every route in the 47-row provider route matrix,
the six realized Contract 057 rows, the prepared route guides, and Research
170. No provider, install, login, billing, or live-runtime work was done.

“Reuse” means the route already has an addable descriptor and can reuse the
Contract 057 admission path plus its existing Contract 037 preparation. “Descriptor
work” means a later adapter-local descriptor can use a proved topology and then
reuse the prepared facade; it does not make the route addable or ready. “Gated”
means this tranche lacks the required proof or an explicit boundary keeps the
route out.

## Authority And Bounds

- Contract 057 owns addable descriptors, admission, credential-field
  references, opaque config fields, readiness refresh, subject observation, and
  the overlay boundary. It does not make a route a server, secret store, or
  registry.
- Contract 037 preparation remains after admission. A descriptor does not
  absorb route-local preparation or operation authority.
- The provider route matrix is the production-route inventory: 47 routes.
- Six rows are already addable. The remaining 41 stay on the prepared-facade
  path until a later named descriptor tranche lands.
- Hosted URL-open OAuth remains parked. A local delegated or inherited login
  is not hosted OAuth.

## Current Addable Baseline — Reuse

These six rows are not new work in this inventory:

| Route | Shape | Reuse after admission |
| --- | --- | --- |
| `anthropic.messages` | hosted API-key | `prepare_anthropic_direct` |
| `deepseek.continuation` | hosted API-key | `prepare_deepseek_direct` |
| `codex.app-server` | installed | `prepare_codex(AppServer)` |
| `claude-agent.acp` | installed | `prepare_claude_agent` |
| `ollama.attached` | local-runtime | `prepare_ollama_attached` |
| `llama-cpp.attached` | local-runtime | `prepare_llama_cpp_attached` |

The addable catalog remains adapter-local and consumer-assembled. No sibling
row is folded into one of these descriptors.

## Remaining Descriptor Work — Proved Hosted And Installed Shapes

These 26 routes have a proved hosted API-key or installed shape and an existing
prepared facade. Their next gap is route-local descriptor and opaque
config-field mapping, with route-specific credential posture preserved. They
are planned candidates, not current addable rows.

| Route | Proved shape and remaining 057 gap |
| --- | --- |
| `kimi-platform.chat` | Reuse the hosted direct API-key shape; add a Platform API-key field, opaque `api.moonshot.ai` endpoint ref, and the existing catalogue/one-attempt prepared path. This is the first post-023 tranche named below. |
| `antigravity.catalogue` | Reuse the approved executable and inherited Google subscription posture; add binary/environment refs and keep catalogue-only operation scope explicit. |
| `antigravity.headless` | Reuse the installed process shape and inherited subscription state; add binary/environment refs without exposing conversation or login state. |
| `cursor-agent.catalogue` | Reuse the installed delegated-subscription shape; add binary/environment refs and preserve the catalogue-only branch. |
| `cursor-agent.acp` | Reuse the installed ACP process shape; add binary/environment refs and explicit workspace authority. Session import and recovery evidence do not transfer from another route. |
| `cursor-agent.headless` | Reuse the installed process shape; add binary/environment refs and explicit run authority. It remains distinct from the Claude Code sibling exclusions. |
| `gemini-cli.acp` | Reuse the installed ACP shape; route-local API-key field and process/environment refs need explicit mapping. This is API-key collection, not hosted URL-open OAuth. |
| `gemini-cli.headless` | Reuse the installed stream-json shape; route-local API-key field and process/environment refs need explicit mapping. |
| `grok-build.acp` | Reuse the installed ACP shape and the advertised delegated `cached_token` posture; add process/environment refs without creating a hosted OAuth loop. |
| `kimi-code.acp` | Reuse the installed ACP shape and delegated membership-auth reference; add process/environment and state-root refs without merging Kimi Platform or local-server authority. |
| `kimi-code.headless` | Reuse the installed stream-json shape and delegated membership-auth reference; add process/environment refs without making a reusable session descriptor. |
| `muse-code.headless` | Reuse the installed signed-payload process shape; add binary/environment refs and keep local account state host-owned. |
| `command-code.headless` | Reuse the installed process shape; add binary/environment and working-resource refs while keeping local account state out of portable records. |
| `cline.acp` | Reuse the installed ACP shape; add binary/environment and working-resource refs with no credential extraction. |
| `cline.headless` | Reuse the installed process shape; add binary/environment and working-resource refs with no credential extraction. |
| `goose.acp` | Reuse the installed ACP shape; add binary/environment and working-resource refs with local config state host-owned. |
| `kiro.acp` | Reuse the installed ACP shape; add binary/environment and working-resource refs with local account state host-owned. |
| `deepagents.acp` | Reuse the installed ACP shape; keep host-owned Anthropic/OpenAI key material behind opaque environment/config refs and do not materialize it as a portable credential. |
| `copilot-cli.acp` | Reuse the installed ACP shape; add binary/environment and working-resource refs with GitHub account state host-owned. |
| `mistral-vibe.headless` | Reuse the installed stream-json shape; add binary/environment and working-resource refs with local config state host-owned. |
| `qoder.headless` | Reuse the installed stream-json shape; add binary/environment and working-resource refs with local config state host-owned. |
| `deepseek-harness.jsonrpc` | Reuse the approved installed artifact shape; add exact binary/config refs while keeping host Cordis, provider, and model selection outside portable records. |
| `zcode.app-server` | Reuse the approved interpreted executable shape; add exact script/runtime and host-settings refs without moving process authority into the catalog. |
| `oh-my-pi.rpc` | Reuse the installed RPC shape; add binary/environment refs and preserve local authentication/configuration and separate catalogue/run/session roles. |
| `pi.rpc` | Reuse the installed RPC shape and delegated harness-auth posture; add binary/environment refs without adding a browser or secret store. |
| `qwen.headless` | Reuse the installed stream-json shape and delegated harness-auth posture; add binary/environment refs without exposing local account state. |

Each candidate still needs its own descriptor proof. Prepared-facade evidence,
installed version claims, or a route guide does not itself create an addable
row.

## First Post-023 Tranche — Descriptor Work

`kimi-platform.chat` is the first implementation tranche after g04.023.
Roadmap [g04.024 Hosted API-Key Kimi Platform Chat](../roadmaps/g04/024-hosted-api-key-kimi-platform-chat.md)
is named only; its implementation cards are not started by this inventory.

The route is the smallest next proof:

- hosted direct HTTP/SSE, matching the existing Anthropic and DeepSeek
  addable shape
- Platform public API key and pay-as-you-go billing, with no hosted OAuth
- exact approved `api.moonshot.ai` endpoint as an opaque config field
- secret API-key field that writes only a `CredentialRef`; do not invent an
  environment-variable name
- existing `prepare_kimi_platform_direct` after admission
- catalogue plus one bounded K3 inference attempt; no provider session,
  continuation, tools, or provider-state management
- subject `Absent`; 047 remains a consumer-assembled snapshot

Research 018 and the Kimi Platform prepared guide already bound the provider
audience, exact facade, model identity, reasoning selection, and no-retry
operation. This tranche adds only the Contract 057 route-entry path later; it
does not widen the direct operation.

## Remaining Gated Routes

These 15 routes remain prepared-only in this lane. “Gated” is a current
promotion boundary, not a claim that a later contract or inventory can never
revisit the route.

| Route | Current shape | Gate |
| --- | --- | --- |
| `codex.exec` | installed structured CLI | Keep it separate from the proved `codex.app-server` row; the current addable catalog proof does not cover exec-only operation semantics. |
| `claude-code.headless` | installed Claude Code stream-json | Explicitly excluded from the `claude-agent.acp` sibling row; no sibling advertisement in this lane. |
| `claude-code.response-only` | installed Claude Code stream-json | Explicitly excluded from the `claude-agent.acp` sibling row; no sibling advertisement in this lane. |
| `alibaba.conversations` | hosted API-key with provider-owned conversation state | Retained conversation/history/management authority is not the proved direct one-attempt shape; keep a separate lifecycle proof before descriptor work. |
| `openai.background` | hosted API-key with retained provider run | Detachment, reconciliation, and provider-run retention add a separate authority shape; do not treat a public API key as sufficient proof. |
| `anthropic.managed-agent` | hosted API-key provider-hosted harness | Managed session/environment cleanup and operator-owned agent-version authority need a separate addable proof. |
| `xai.responses-websocket` | hosted API-key realtime connection | Connection-scoped WebSocket continuation is a separate realtime shape, not the proved hosted HTTP/SSE shape. |
| `openai.realtime` | hosted API-key realtime media | Media-session lifecycle and connection authority are outside the proved direct shape. |
| `gemini.live` | hosted API-key realtime media | Raw Live WebSocket lifecycle is outside the proved direct shape. |
| `bedrock.runtime` | embedded cloud SDK | Delegated cloud identity, SigV4, region/service binding, and SDK EventStream are not a portable public API-key field. |
| `bedrock.catalogue` | embedded cloud SDK control plane | Separate control-plane target, cloud identity, and catalogue authority need a distinct proof from Bedrock Runtime. |
| `kimi-code.local-server` | attached harness network | Loopback server bearer, provider account state, retained operations, and optional ownership are not the proved unauthenticated local-runtime shape. |
| `opencode.http` | attached harness network | Delegated auth, external-server authority, session import, history, reconciliation, and management are a separate attached-harness shape. |
| `deepseek-harness.local-server` | attached/owned harness network | The route owns a `dsh web` child and provider-session operations; live smoke and server ownership remain separate gates. |
| `llama-cpp.owned` | owned local serving | Explicitly separate from `llama-cpp.attached`; this row starts and stops an owned server and must not be advertised from the attached descriptor. |

Hosted OAuth remains parked independently of this table. OpenHands remains
without a production route.

## Decision

The remaining surface is classified without changing Contract 057, the route
matrix, or any adapter crate:

- 6 rows reuse existing addable descriptors.
- 26 rows are later adapter-local descriptor work on proved hosted API-key or
  installed shapes.
- 15 rows remain gated by an explicit route, transport, ownership, or
  authority boundary.
- g04.024 Kimi Platform is the first named implementation after g04.023.
- g04.023 remains the immediate next work. No implementation card from g04.024
  is ready from this inventory.

## Stop Check

Nothing here requires Swallowtail to store secrets, embed a browser, start a
server, or add an OpenHands production route. No hosted OAuth route was
selected.
