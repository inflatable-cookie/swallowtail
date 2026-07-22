# 022 Pi RPC And Post-Realtime Provider Coverage Selection

Status: promoted
Owner: Tom
Updated: 2026-07-22

## Question

Which route should follow Gemini Live, and what shared boundary must exist
before production work?

## Method

Sources were accessed 2026-07-22. Official provider and protocol documents,
maintained project repositories, and their official package registry entries
are treated as evidence. No account, credential, browser login, paid request,
installed harness, remote agent, or model-serving deployment was used.

The audit starts from eighteen production descriptors and eleven common
profiles. It ranks missing runtime pressure, practical value, maturity, access
clarity, and implementation weight. Provider count alone does not qualify a
route.

## Realized Inventory

### Production descriptors

| Integration | Driver | Transport and role | Common profile |
| --- | --- | --- | --- |
| Alibaba Model Studio | direct conversation | HTTP/SSE direct interactive session | hosted direct assertions |
| Codex | exec | one-shot JSONL CLI structured run | one-shot structured CLI |
| Codex | app-server | long-lived stdio JSON-RPC harness | long-lived RPC harness |
| OpenCode | attached server | HTTP/SSE harness | attached network harness |
| Anthropic | Messages | HTTP/SSE direct structured run | hosted direct API |
| Anthropic | Managed Agents | HTTP/SSE provider-managed harness | provider-managed remote harness |
| Bedrock | Runtime | embedded Rust SDK/EventStream direct run | hosted direct API |
| Bedrock | catalogue | embedded Rust SDK control plane | catalogue assertion pack |
| Gemini | CLI ACP | ACP v1 stdio harness | long-lived ACP harness |
| Gemini | Live | raw WebSocket realtime direct session | realtime-media direct session |
| Kimi | Code ACP | ACP v1 stdio persistent harness | persistent ACP harness |
| Kimi Platform | Chat Completions | HTTP/SSE direct run | hosted direct API |
| llama.cpp | attached | HTTP/SSE self-hosted direct run | attached self-hosted |
| llama.cpp | owned ephemeral | process plus HTTP/SSE direct run | owned self-hosted |
| OpenAI | background Responses | HTTP/SSE background direct run | hosted direct API plus background assertions |
| OpenAI | Realtime | WebSocket realtime direct session | realtime-media direct session |
| Qwen Code | headless | one-shot streaming-JSON CLI harness | one-shot structured CLI |
| xAI | Responses WebSocket | connection-scoped direct session | connection-scoped direct session |

The eleven common profiles remain:

1. one-shot structured CLI
2. long-lived RPC harness
3. long-lived ACP harness
4. persistent ACP harness
5. attached network harness
6. hosted direct API
7. provider-managed remote harness
8. connection-scoped direct session
9. realtime-media direct session
10. attached self-hosted
11. owned self-hosted

Separate assertion packs cover background execution, harness isolation,
provider conversations, catalogue truth, and planned connection rollover.

## Current Evidence

### Pi RPC is the next harness proof

Pi `0.80.10` publishes a maintained language-neutral RPC mode over strict
LF-delimited JSONL stdio. It separates commands, correlated responses, and
streamed events. The protocol exposes prompt, steering, follow-up, abort,
state, model, retry, session, and extension-UI operations.

Steering and follow-up are materially different scheduling classes. Steering
waits for the current assistant tool work, then enters before the next model
call. Follow-up waits until the agent has no tool or steering work left.
Extension dialogs use a correlated request/response sub-protocol; display-only
UI notifications do not require a response.

Pi explicitly ships no permission popups or built-in sandbox. Operators can
use a container or extension when wanted. That matches Contract 023: the first
route is honest `AmbientHost` communication, not a containment claim.

Pi can bind provider and model explicitly. Its harness identity, downstream
model provider, exact model route, harness-owned authentication, metering, and
support authority must remain separate. The first proof does not choose a
default provider or model.

Evidence: [Pi coding-agent package](https://www.npmjs.com/package/@earendil-works/pi-coding-agent),
[RPC protocol](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/rpc.md),
[SDK and RPC placement](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/sdk.md),
and [coding-agent overview](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/README.md).

### DeepSeek V4 is the strongest direct follow-on

DeepSeek V4 exposes `deepseek-v4-flash` and `deepseek-v4-pro` through
OpenAI- and Anthropic-compatible facades. Legacy `deepseek-chat` and
`deepseek-reasoner` aliases retire on 2026-07-24.

The route is not just another compatible provider when tools and thinking are
included. A tool-calling assistant message carries `reasoning_content`, and
DeepSeek requires that content to be returned on later requests. Omitting it
produces a provider error. That creates a missing direct-inference continuation
boundary: consumer-owned orchestration, portable tool calls, provider-private
continuation evidence, multiple explicit inference attempts, and no generic
agent loop inside Swallowtail.

Do not implement DeepSeek as stateless breadth first. Research and contract the
direct continuation boundary, then freeze exact V4 semantics.

Evidence: [models and alias retirement](https://api-docs.deepseek.com/quick_start/pricing/),
[first call and dual facades](https://api-docs.deepseek.com/guides/function_calling/),
and [thinking-mode tool continuation](https://api-docs.deepseek.com/guides/thinking_mode/).

### Grok Build is useful breadth with lower information gain

Grok Build is now documented as a first-party coding agent with interactive,
headless streaming-JSON, and ACP routes. Browser authentication and API-key
access remain distinct. It also permits explicit custom models.

Those mechanisms are useful but already represented across Codex, Gemini,
Kimi, Qwen, and the shared ACP layer. It follows Pi unless an xAI-specific
harness becomes a consumer priority.

Evidence: [Grok Build overview](https://docs.x.ai/build/overview) and
[headless use](https://docs.x.ai/build/cli/headless-scripting).

### Remote ACP remains Active, not stable

The remote Streamable HTTP and WebSocket RFD is Active. It proposes HTTP/2,
connection- and session-scoped long-lived SSE streams, required WebSocket
support, cookies, and reconnect through `session/load`.

The RFD still assigns reconnect, liveness, and affinity behavior to
implementers, provides no v1 in-flight replay, and describes the maintained
SDK implementation as future work. ACP v1 Rust and TypeScript SDKs are stable
for the current protocol, but that does not stabilize the remote transport.

Evidence: [remote transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
and [RFD lifecycle](https://agentclientprotocol.com/rfds/updates).

### Cursor SDK matured but remains a heavier beta bridge

Cursor's public-beta SDK now exposes the same harness through local and cloud
runtimes. It has TypeScript and Python packages, streamed runs, cancellation,
custom stores, custom tools, auto-review, and local or cloud placement.

The local route avoids mandatory GitHub mutation, but a Rust adapter needs a
foreign-language package bridge and its bundled local runtime. The cloud route
still adds repository, remote workspace, retention, billing, archive, and
deletion policy. Cursor remains useful after the lighter Pi process proof.

Evidence: [SDK release](https://cursor.com/changelog/sdk-release),
[SDK updates](https://cursor.com/changelog/sdk-updates-jun-2026), and
[official cookbook](https://github.com/cursor/cookbook/blob/main/README.md).

### Claude Agent SDK has an unresolved subscription-authority edge

The official Python SDK bundles Claude Code and exposes async queries,
interactive clients, custom tools, hooks, permissions, and session behavior.
API-key use is clear under Anthropic commercial terms.

Current first-party guidance is not clean enough for a subscription-backed
third-party Swallowtail route. Anthropic's legal page says third-party
developers must not offer Claude.ai login or route Free, Pro, or Max
credentials. A later Help Center update says third-party Agent SDK use still
draws from subscription limits while a planned metering change is paused.

Swallowtail can later prove an API-key-only Agent SDK route. It must not treat
installed subscription OAuth as reusable third-party authority until the
provider resolves that boundary.

Evidence: [Python Agent SDK](https://github.com/anthropics/claude-agent-sdk-python),
[authentication restrictions](https://code.claude.com/docs/en/legal-and-compliance),
and [paused subscription change](https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan).

### Z.AI remains exact compatible breadth

Z.AI exposes `glm-5.1` through a general API and a distinct GLM Coding Plan
endpoint. Coding Plan credentials, entitlement, limits, and supported-tool
authority cannot authorize the general API implicitly. The provider also
documents bearer API keys, derived JWT authentication, SSE, tool calls,
reasoning fields, and detailed subscription-limit failures.

The route is useful but repeats the compatible hosted lifecycle until direct
tool continuation is selected.

Evidence: [API introduction](https://docs.z.ai/api-reference/introduction),
[Chat Completions](https://docs.z.ai/api-reference/llm/chat-completion), and
[error evidence](https://docs.z.ai/api-reference/api-code).

### Ollama is the best attached follow-on

Ollama exposes native local HTTP without authentication, optional hosted cloud
access with separate authentication, NDJSON streaming, tool calls, thinking,
usage timings, installed-model inventory, and running-model inventory.

An attach-only Swallowtail driver can observe `/api/tags`, `/api/ps`, and
`/api/chat` without pulling, creating, deleting, loading, or stopping models.
That stays outside Monkey and avoids a container. Tool/thinking continuation
should reuse the later direct continuation contract instead of inventing an
Ollama-only loop.

Evidence: [API placement](https://docs.ollama.com/api/introduction),
[installed models](https://docs.ollama.com/api/tags),
[running models](https://docs.ollama.com/api/ps),
[streaming](https://docs.ollama.com/api/streaming), and
[tool calling](https://docs.ollama.com/capabilities/tool-calling).

### vLLM and SGLang remain deployment breadth

Both runtimes expose OpenAI-compatible serving, model observation, streaming,
and model-dependent tool or reasoning behavior. vLLM also exposes Responses,
batch, embeddings, audio, metrics, LoRA management, and dangerous development
administration. SGLang exposes gateway routing, model discovery, tokenizer,
embedding, reranking, and administrative surfaces.

The first adapter must attach only. Dynamic LoRA, cache, weight, worker,
profiling, sleep, scale, or arbitrary RPC authority belongs to the deployment
operator or Monkey.

Evidence: [vLLM serving](https://docs.vllm.ai/en/stable/serving/openai_compatible_server/),
[vLLM LoRA](https://docs.vllm.ai/en/stable/features/lora/),
[SGLang gateway](https://github.com/sgl-project/sglang/blob/main/docs/advanced_features/sgl_model_gateway.md),
and [SGLang server arguments](https://github.com/sgl-project/sglang/blob/main/docs/advanced_features/server_arguments.md).

## Comparison

| Rank | Route | New pressure | Decision |
| --- | --- | --- | --- |
| 1 | Pi `0.80.10` RPC | steering versus follow-up scheduling, correlated extension UI relay, multi-provider harness identity, explicit ambient posture | select |
| 2 | DeepSeek V4 direct | provider-required reasoning continuation around consumer tools | contract next |
| 3 | Grok Build | first-party harness breadth over headless and ACP | later |
| 4 | Cursor local SDK | local/cloud SDK harness, stores, custom tools | later; heavier beta bridge |
| 5 | Ollama attached | native installed/running catalogue plus local NDJSON | later after continuation contract |
| 6 | remote ACP | shared remote protocol | wait for maintained transport support |
| 7 | Z.AI general API | exact provider semantics and access separation | later breadth |
| 8 | Claude Agent SDK | strong harness surface, unclear third-party subscription authority | API-key-only later or await clarity |
| 9 | vLLM and SGLang | deployment breadth | attach-only later |

## Decision

Select Pi `0.80.10` RPC over stdio as the next proof.

The bounded first subset is:

- maintained `@earendil-works/pi-coding-agent@0.80.10`
- `pi --mode rpc --no-session`
- one exact operator-selected downstream provider and model route
- one explicit harness-owned delegated-auth access profile
- strict LF JSONL command, response, event, and extension-UI framing
- `AmbientHost` isolation; no sandbox or containment claim
- exact working directory with read-intent tools only: `read`, `grep`, `find`,
  and `ls`
- ambient extensions, skills, prompt templates, and context files disabled
- update checks, install/update telemetry, and automatic provider retry
  disabled
- one active model operation, maximum two completed prompts, maximum one
  steering message and one follow-up message
- correlated extension dialog relay and display-only UI observation
- native abort, host deadline, bounded streams, process supervision, and joined
  cleanup

The proof excludes provider or model defaults, login, credential mutation,
session persistence, resume, fork, compaction, bash RPC, model switching,
package installation, arbitrary commands, write tools, custom tools, MCP,
network or filesystem containment, provider retry, cloud placement, and live
authentication from default QA.

## Required Promotion

Contract 028 must fix:

- command acknowledgement versus scheduled or completed model work
- prompt, steering, follow-up, abort, and callback correlation
- extension dialog versus display-only UI behavior
- exact downstream provider, model, access, and metering identity beneath a
  multi-provider harness
- accepted ambient harness state with disabled customization sources
- read-intent tools without a containment claim
- provider-native retry disabled independently from host deadline
- joined command, reader, callback, process, and delegated-auth cleanup

## Promotion

- durable scheduling and UI-relay boundary: Contract 028
- implementation sequence: g01 roadmap 035 and cards 099-101
- first ready task: card 099, shared records and Pi `0.80.10` corpus
- next direct-contract research: DeepSeek V4 reasoning/tool continuation
- later attached route: Ollama without model management
- remote ACP remains gated on maintained transport support

