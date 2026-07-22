# 020 Post-Direct Breadth And Realtime Media Selection

Status: promoted
Owner: Tom
Updated: 2026-07-22

## Question

Which remaining provider or transport route adds the most useful new
architecture after sixteen production drivers and ten provider-neutral
conformance profiles?

## Method

Sources were accessed 2026-07-22. Official provider documentation, maintained
protocol documentation, and the official Grok npm package registry entry are
treated as evidence. Existing same-day Research 019 remains current for
DeepSeek and Z.AI. No account, credential store, paid inference, live provider
session, browser login, or installed harness was used.

Support authority follows Contract 006. Preview and beta routes remain visible
even when provider-published.

## Realized Coverage

The repository has sixteen separately registered production drivers:

| Driver | Layer and operation | Transport | Distinct pressure already proved |
| --- | --- | --- | --- |
| Codex exec | harness structured run | structured CLI | bounded JSONL process |
| Codex app-server | harness interactive session and catalogue | JSONL-RPC stdio | callbacks, discovery, resume, workspace profiles |
| OpenCode server | harness interactive session and catalogue | HTTP/SSE | attached network harness |
| Anthropic Messages | direct structured run and catalogue | HTTP/SSE | provider-native Messages mapping |
| Anthropic Managed Agents | harness structured run | HTTPS/SSE | durable remote resources and recovery |
| Bedrock Runtime | direct structured run | Rust SDK/EventStream | embedded SDK and delegated cloud identity |
| Bedrock catalogue | direct catalogue | Rust SDK | separate regional control plane |
| Gemini CLI | harness interactive session | ACP stdio | read callbacks and ACP negotiation |
| Kimi Code | harness interactive session | ACP stdio | load, replay, resume, writes, delegated login |
| Kimi Platform | direct structured run and catalogue | HTTP/SSE | compatible chat with provider-owned semantics |
| llama.cpp attached | direct structured run and catalogue | HTTP/SSE | external self-hosted deployment |
| llama.cpp owned | direct serving, catalogue, and structured run | process plus HTTP/SSE | artifact lease and owned ephemeral serving |
| OpenAI background Responses | direct structured run | HTTP/SSE | temporary retention, reattachment, native cancel |
| Qwen Code headless | harness structured run | stream-JSON CLI | native budgets and explicit ambient isolation |
| xAI Responses | direct interactive session | WebSocket | connection continuation and billed cost |
| Alibaba Model Studio | direct interactive session | HTTPS/SSE | durable conversation and ordered deletion |

The ten common profiles cover one-shot CLI, long-lived RPC, baseline ACP,
persistent ACP, hosted direct API, connection-scoped direct session,
provider-managed harness, attached network harness, attached self-hosted, and
owned self-hosted behavior.

The important missing mechanism is continuous media exchange. Current
operation content is bounded text. Attachments are finite leased inputs. No
runtime port transports ordered binary media chunks into and out of an active
session, and no conformance profile proves realtime media cancellation or
joined duplex cleanup.

## Current Candidate Evidence

### OpenAI Realtime is GA and materially new

The provider-supported Realtime API is GA. Its server-to-server route uses
`wss://api.openai.com/v1/realtime`, bearer authentication with a public OpenAI
API key, and API-account usage billing. It is not ChatGPT or Codex access.

`gpt-realtime-2.1` accepts text, audio, and image input and returns text or
audio. A WebSocket client exchanges ordered JSON events and base64 audio
chunks. Sessions retain a conversation, accept input-audio buffer append and
commit events, emit audio and transcript deltas, expose rate-limit and usage
evidence, support native response cancellation, and have a 60-minute maximum
lifetime.

The first proof can stay server-side, API-key backed, audio-only, manual-turn,
connection-scoped, and resource-free. WebRTC, SIP, ephemeral client secrets,
automatic voice activity detection, images, tools, stored prompts, browser
devices, and live authentication remain later routes or capabilities.

Evidence: [Realtime and audio](https://developers.openai.com/api/docs/guides/realtime),
[WebSocket](https://developers.openai.com/api/docs/guides/realtime-websocket),
[conversations](https://developers.openai.com/api/docs/guides/realtime-conversations),
and [GPT-Realtime-2.1](https://developers.openai.com/api/docs/models/gpt-realtime-2.1).

### Gemini Live is the strongest second realtime proof

Gemini Live exposes a provider-published bidirectional WebSocket with text,
audio, and video input, native audio output, session resumption handles, and
roughly ten-minute connection rollover. Standard or authorization API keys are
project-bound; constrained ephemeral tokens are minted for client-side Live
API access. Free, prepay, and postpay billing tiers remain separate.

The route currently uses preview APIs and model
`gemini-3.1-flash-live-preview`. It adds valuable resumption and derived-
credential pressure, but it should follow a stable common realtime-media
boundary rather than define that boundary alone.

Evidence: [Live WebSocket](https://ai.google.dev/gemini-api/docs/live-api/get-started-websocket),
[session management](https://ai.google.dev/gemini-api/docs/live-api/session-management),
[ephemeral tokens](https://ai.google.dev/gemini-api/docs/live-api/ephemeral-tokens),
[API keys](https://ai.google.dev/gemini-api/docs/api-key), and
[billing](https://ai.google.dev/gemini-api/docs/billing).

### Grok Build is current but repeats realized harness shapes

Grok Build is now a provider-supported coding harness with interactive,
headless JSON or streaming JSON, and ACP stdio surfaces. The stable npm tag is
`@xai-official/grok@0.2.106`; later `0.2.107-0.2.110` publications remain on
the alpha tag.

Browser OIDC, device OAuth, an external credential command, and API-key access
are distinct. Sessions persist under Grok-owned local state. Auto-update can be
disabled. Permissions and the optional native sandbox remain independent; the
sandbox is off by default. These are useful provider-breadth facts, but Codex,
Qwen, Gemini, and Kimi already prove the relevant structured CLI, ACP,
persistent session, delegated auth, permission, native-budget, and optional-
sandbox boundaries.

Evidence: [Grok Build](https://docs.x.ai/build/overview),
[headless and ACP](https://docs.x.ai/build/cli/headless-scripting),
[CLI reference](https://docs.x.ai/build/cli/reference),
[enterprise auth](https://docs.x.ai/build/enterprise), and official npm
metadata for `@xai-official/grok@0.2.106` observed 2026-07-22.

### Remote ACP remains premature

ACP's Streamable HTTP and WebSocket transport remains a Draft RFD. Its current
design requires HTTP/2, separates connection and session streams, leaves
authentication orthogonal, and still lists protocol-version, reference-
implementation, reconnect, resumability, and hardening work. Swallowtail must
not claim a stable shared remote transport yet.

Evidence: [remote transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
and [RFD updates](https://agentclientprotocol.com/rfds/updates).

### Cursor remains an operator policy gate

Cursor's Background Agents API remains beta. It uses an account-scoped bearer
key at `https://api.cursor.com`, usage-based pricing, GitHub repository access,
remote mutation, follow-up prompts, and durable agent resources. Selecting it
would set repository, branch, pull-request, remote-workspace, and deletion
policy not owned by current Swallowtail authority.

Evidence: [Cursor Background Agents API](https://docs.cursor.com/background-agent/api/overview).

### Compatible chat and local-runtime breadth are lower information

DeepSeek V4 and Z.AI GLM-5.1 general APIs remain provider-supported bearer-key,
usage-billed Chat Completions routes. DeepSeek's legacy `deepseek-chat` and
`deepseek-reasoner` aliases retire on 2026-07-24. Z.AI keeps its general API
separate from Coding Plan. Both need adapter-owned corpora, but neither changes
the shared transport or lifecycle boundary.

Ollama, vLLM, and SGLang remain useful attached deployments. Native model pull,
create, copy, and delete operations cross model acquisition and durable serving
policy. That work remains outside Swallowtail unless a later contract preserves
Monkey and operator ownership explicitly.

Evidence: [DeepSeek](https://api-docs.deepseek.com/quick_start/pricing),
[Z.AI quick start](https://docs.z.ai/guides/overview/quick-start),
[GLM-5.1](https://docs.z.ai/guides/llm/glm-5.1),
[Ollama](https://docs.ollama.com/api/introduction),
[vLLM](https://docs.vllm.ai/en/latest/serving/online_serving/), and
[SGLang](https://docs.sglang.io/docs/basic_usage/overview).

## Access And Authority Comparison

| Candidate | Endpoint or transport | Credential | Entitlement and metering | Authority | Result |
| --- | --- | --- | --- | --- | --- |
| OpenAI Realtime | public `wss` Realtime API | public API key; workload identity separately supported | API-account usage billing | provider, GA | select |
| Gemini Live | Generative Language WebSocket | project API/auth key or constrained ephemeral token | free, prepay, or postpay project billing | provider, preview | second realtime proof |
| Grok Build | local CLI or ACP stdio; provider inference proxy | browser/device OIDC, external helper, or API key | product/team or public-API path; keep separate | provider | later harness breadth |
| remote ACP | draft HTTP/2 SSE or WebSocket | transport-specific and orthogonal | route-specific | experimental draft | wait |
| Cursor Background Agents | `https://api.cursor.com` | account bearer key | plan limits and usage pricing | provider beta | operator gate |
| DeepSeek V4 | `https://api.deepseek.com` | DeepSeek API key | topped-up usage balance | provider | later compatible mapping |
| Z.AI GLM-5.1 | general `api.z.ai` endpoint | Z.AI API key | general API usage billing | provider | later compatible mapping |
| Ollama/vLLM/SGLang | configured local or remote deployment | deployment-specific | local or hosted compute | maintainer/deployment | later native-runtime mapping |

## Ranking

| Rank | Candidate | New shared pressure | Main limit |
| --- | --- | --- | --- |
| 1 | OpenAI Realtime WebSocket | ordered duplex audio, media formats, chunk bounds, native response cancel, media cleanup | new runtime records required |
| 2 | Gemini Live | realtime portability, connection rollover, session handles, constrained ephemeral credentials | preview surface |
| 3 | Grok Build | current xAI harness, layered auth, provider-native sandbox | repeats CLI and ACP profiles |
| 4 | remote ACP | shared remote connection/session topology | Draft RFD |
| 5 | Cursor Background Agents | repository-backed remote mutation | unresolved product authority |
| 6 | DeepSeek and Z.AI | provider breadth and model semantics | repeats compatible chat |
| 7 | additional native runtimes | local ecosystem breadth | repeats deployment lifecycle or crosses serving ownership |

## Decision

Select the provider-supported OpenAI Realtime GA WebSocket with exact model
`gpt-realtime-2.1` as the next proof.

The first subset is one resource-free direct interactive session with:

- public OpenAI API-key access and API usage billing
- server-to-server WebSocket at the exact Realtime audience
- preflight-bound mono PCM input and output formats
- manual input-audio append and commit
- maximum two successful serial audio turns
- ordered output-audio and transcript deltas, terminal usage, rate evidence,
  and request correlation
- one active response and native response cancellation
- connection-scoped state with no reconnect, resume, storage, or replay
- joined reader, writer, timer, connection, and credential cleanup

Any cancelled or deadline-expired audio response ends the session after native
cancel acknowledgement. This avoids claiming playback-aware conversation
truncation when Swallowtail does not own audio playback position.

The proof excludes WebRTC, SIP, ephemeral client secrets, automatic voice
activity detection, barge-in continuation, images, text turns, tools, MCP,
stored prompts, provider fallback, model aliases, retry, browser devices,
audio capture, playback, resampling, transcoding, jitter buffers, and live
authentication from default QA.

## Required Promotion

Contract 026 must settle:

- realtime media as a capability-specific interactive-session role, not a new
  execution layer or generic prompt API
- exact media format, bounded redacted chunk, sequence, input commit, output
  delta, transcript, usage, rate, and terminal records
- consumer ownership of device capture, playback, buffering, conversion, and
  played-position truth
- one active response, native cancel truth, connection invalidation, and
  playback-unaware interruption rules
- session-scoped endpoint and credential lifetime with joined duplex cleanup
- no implicit WebRTC, ephemeral credential, reconnect, resume, or fallback

No provisional product spec is required. The operation remains an interactive
direct-inference session; the new role prevents existing text session handles
from gaining a silent media method.

## Promotion

- durable realtime-media boundary: Contract 026
- implementation sequence: g01 roadmap 031 and cards 091-093
- second provider proof after the shared boundary: Gemini Live remains first in
  the later realtime portability queue
- deferred provider breadth: Grok Build, DeepSeek, and Z.AI remain exact later
  candidates
- deferred policy gate: Cursor repository-backed agents
- deferred protocol gate: ACP remote transport
