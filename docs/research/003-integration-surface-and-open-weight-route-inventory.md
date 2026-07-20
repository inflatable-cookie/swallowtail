# 003 Integration Surface and Open-Weight Route Inventory

Status: promoted
Owner: Tom
Updated: 2026-07-19

## Question

Which current harness, direct-inference, protocol, and self-hosted surfaces must
constrain Swallowtail before it chooses runtime traits?

## Method

Sources were accessed 2026-07-19. Provider documentation, maintained project
documentation, protocol specifications, and installed CLI help are treated as
evidence. Marketing names are not treated as transports or capability claims.

Support authority uses Contract 006:

- **provider** — published and supported by the provider
- **maintainer** — published and maintained by the owning integration project
- **experimental** — explicitly unstable or insufficiently specified
- **prohibited** — documented as outside authorized use

## Result

Swallowtail must cover at least seven materially different shapes:

1. a one-shot CLI with terminal JSON or streamed JSONL
2. a long-lived subprocess with bidirectional JSONL or JSON-RPC
3. an ACP agent over stdio or another advertised transport
4. a harness-owned HTTP, SSE, or WebSocket service
5. an in-process SDK controlling a harness or agent loop
6. a direct hosted model API
7. a self-hosted serving runtime loading an independently identified model
   artifact

No single family spans these shapes consistently. “OpenAI-compatible” narrows
wire translation but does not prove tools, reasoning, schemas, multimodality,
usage reporting, cancellation, or session semantics.

## Harness Inventory

| Family and driver candidate | Operation and transport | Access | Important capability or limit | Authority |
| --- | --- | --- | --- | --- |
| Codex interactive CLI | interactive local process | ChatGPT OAuth or Platform API key | sessions, workspace tools, approvals, resume | provider |
| Codex `exec` | bounded process; text or JSONL events | same Codex access profiles | streaming events, schemas, cancellation by process control | provider |
| Codex app-server | long-lived JSON-RPC service; stdio and remote modes | host/Codex auth context | threads, turns, events, approvals, model discovery, interruption | provider; some modes experimental |
| Codex SDK | in-process client controlling local app-server | same Codex access profiles | preserves harness lifecycle rather than becoming direct inference | provider |
| Claude Code interactive and `-p` | interactive or bounded process; text, JSON, stream JSON | Claude subscription OAuth, Console key, cloud identity, gateway | sessions, tools, permissions, partial events, resume | provider |
| Claude Agent SDK | SDK around the Claude Code agent loop | supported API or eligible subscription access | programmatic harness control; not a raw Messages client | provider |
| OpenCode TUI/run | interactive or bounded process | provider-specific OAuth and keys | sessions, agents, tools, provider routing | maintainer |
| OpenCode server and SDK | long-lived HTTP server with OpenAPI-generated SDK | optional HTTP basic auth plus provider access | session and event APIs; multiple clients may attach | maintainer |
| OpenCode ACP | ACP subprocess | provider access delegated to OpenCode | standard client-agent lifecycle plus OpenCode behavior | maintainer |
| Cursor Agent CLI | interactive or bounded process; text, JSON, streamed JSONL | Cursor login or API key | session IDs, resume, tools, write policy, MCP | provider |
| Pi coding-agent CLI | interactive, print, JSON event, or bidirectional JSONL RPC | multiple provider OAuth/key profiles | asynchronous prompt acceptance, steering, cancellation, model switching, session persistence | maintainer |
| Pi SDK packages | in-process TypeScript agent/session and unified model API | provider-specific | harness and direct-model packages are already separate | maintainer |
| Kimi Code CLI | interactive or bounded process | Kimi OAuth | sessions, tools, model selection | provider |
| Kimi ACP | ACP subprocess | delegated Kimi login | negotiated session and tool lifecycle | provider |
| Kimi server | REST and WebSocket service with published OpenAPI/AsyncAPI | delegated Kimi login | remote-capable service surface | provider |
| Qwen Code CLI | interactive or bounded process; text, JSON, streamed JSONL | API key; Coding Plan API key supported | partial messages, tools, system prompt, limits, sessions | provider/open-source maintainer |
| Qwen Code SDK | TypeScript, Python, and Java SDKs | same configured provider route | in-process harness control | provider/open-source maintainer |
| Qwen Code daemon | ACP over HTTP and SSE | bearer token required for remote bind | experimental remote service | experimental |
| Gemini CLI | interactive or headless process; text, JSON, streamed JSON | Google login or supported API/cloud credentials | sessions, tools, resume, checkpointing | provider |
| Gemini CLI ACP | JSON-RPC ACP over stdio | delegated Gemini CLI access | new/load session, prompt, cancel, model change, client filesystem | provider/experimental surface |

Primary evidence:

- [Codex CLI reference](https://developers.openai.com/codex/cli/reference/),
  [app-server protocol](https://developers.openai.com/codex/app-server/), and
  [Codex SDK](https://developers.openai.com/codex/sdk/)
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
  and [Agent SDK overview](https://platform.claude.com/docs/en/agent-sdk/overview)
- [OpenCode server](https://opencode.ai/docs/server/),
  [SDK](https://opencode.ai/docs/sdk/), and
  [ACP command](https://opencode.ai/docs/cli/#acp)
- [Cursor headless mode](https://docs.cursor.com/en/cli/headless) and
  [output schema](https://docs.cursor.com/en/cli/reference/output-format)
- [Pi project](https://github.com/earendil-works/pi),
  [RPC protocol](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/rpc.md),
  and [JSON event mode](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/json.md)
- [Kimi Code documentation](https://www.kimi.com/code/docs/en/) and
  [CLI reference](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-command)
- [Qwen Code documentation](https://qwenlm.github.io/qwen-code-docs/en/),
  [headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/),
  and [Qwen Code repository](https://github.com/QwenLM/qwen-code)
- [Gemini CLI repository](https://github.com/google-gemini/gemini-cli) and
  [ACP mode](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/acp-mode.md)

### Installed Evidence

Read-only checks on 2026-07-19 found:

| Binary | Version | Confirmed local surface |
| --- | --- | --- |
| `codex` | 0.144.6 | interactive, `exec`, app-server, MCP server, resume, remote app-server |
| `claude` | 2.1.187 | interactive, print mode, streamed JSON, partial messages, resume |
| `opencode` | 1.14.48 | TUI, run, ACP, HTTP server, attach, session import/export |
| `cursor-agent` | 2026.07.01-41b2de7 | interactive, print, JSON/stream JSON, resume, workspace roots |
| `llama-server` | version flag unavailable | local HTTP serving binary present |

Installed help confirms surface availability, not authentication readiness or
full protocol stability. No credential stores were inspected.

### Codex Exec Revalidation

`codex-cli` 0.144.6 help and the current Codex manual were rechecked on
2026-07-19 before the first driver implementation.

- `codex exec` accepts a prompt argument or stdin and can emit JSONL events.
- model and sandbox selection are explicit command inputs.
- ephemeral operation and a schema-file input are supported.
- access may use the saved Codex login or an invocation-scoped API key.
- the exec surface does not expose model discovery; Codex app-server owns that
  later proof.
- image and schema inputs require process-usable host paths. The first proof is
  therefore text-only until host-authorized path materialization is defined.

This revalidation narrows the first driver without changing the seven runtime
shapes above.

### Codex App-Server Revalidation

The current Codex manual, installed `codex-cli` 0.144.6 help, and its generated
non-experimental JSON schemas were checked on 2026-07-19.

- the default transport is bidirectional JSONL over stdio; WebSocket and Unix
  socket modes remain separate transport drivers, with WebSocket documented as
  experimental and unsupported
- every connection requires one `initialize` request followed by the
  `initialized` notification
- stable methods cover `model/list`, `thread/start`, `thread/resume`,
  `turn/start`, and `turn/interrupt`
- model listing is paginated and returns stable model ids plus mutable display
  metadata
- thread and turn responses carry provider ids independently from Swallowtail
  request, session, and turn ids
- turn notifications carry thread and turn correlation; completed agent-message
  items contain normalized final text
- app-server may initiate approval, tool, elicitation, and attestation requests
  that require a correlated client response
- generated schemas are CLI-version-specific and include raw host paths in
  some responses, so adapters must parse narrowly and keep unneeded fields
  private

The first session proof therefore owns one local stdio process, uses only the
stable API, fixes a read-only/no-approval posture, and rejects server-initiated
requests. Callback exchange and remote transports remain later capability work.

## Direct Hosted Inference Inventory

| Family and route | Protocol | Credential and entitlement | Notes | Authority |
| --- | --- | --- | --- | --- |
| OpenAI public API | Responses HTTP/SSE and SDKs | Platform API key or workload identity; usage billing | direct inference; separate from ChatGPT billing | provider |
| Codex subscription endpoint through OpenCode | Responses-shaped HTTP | ChatGPT OAuth; subscription metering | product-scoped direct route maintained by OpenCode | maintainer |
| Anthropic public API | Messages HTTP/SSE and SDKs | API key, cloud identity, or gateway; usage billing | arbitrary Claude subscription OAuth is not an authorized public API credential | provider; OAuth misuse prohibited |
| xAI | Responses and legacy-compatible HTTP, SDKs, streaming and WebSocket modes | xAI API key and API credits | tools, structured output, reasoning, remote MCP and hosted tools vary by model | provider |
| Kimi Code API | OpenAI- and Anthropic-compatible HTTP | membership-backed API key | subscription endpoint and key are distinct from Kimi Platform | provider |
| Kimi Platform | provider API | Platform key; usage billing | separate audience and metering from Kimi Code | provider |
| Z.AI GLM general API | OpenAI-compatible HTTP | Z.AI API key; usage billing | general endpoint is distinct from Coding Plan | provider |
| Z.AI GLM Coding Plan | OpenAI- and Anthropic-compatible coding endpoints | API key; subscription allowance | limited to supported coding-tool audience; client identity and endpoint matter | provider |
| Alibaba Model Studio Qwen | OpenAI-compatible HTTP and provider APIs | workspace API key; usage or provisioned throughput | region, workspace authorization, and model route alter access | provider |
| DeepSeek API | OpenAI-compatible Chat Completions and Anthropic-compatible interface | DeepSeek API key; usage billing | streaming, tool calls, JSON output, reasoning behavior vary by model | provider |

Primary evidence:

- [OpenAI API reference](https://developers.openai.com/api/reference/overview)
- [Anthropic API overview](https://platform.claude.com/docs/en/api/overview)
- [xAI API overview](https://docs.x.ai/overview)
- [Kimi Code overview](https://www.kimi.com/code/docs/en/) and
  [third-party agent setup](https://www.kimi.com/code/docs/en/third-party-tools/other-coding-agents)
- [Z.AI Coding Plan overview](https://docs.z.ai/devpack/overview),
  [Claude Code setup](https://docs.z.ai/devpack/tool/claude), and
  [other coding tools](https://docs.z.ai/devpack/tool/others)
- [Alibaba Model Studio overview](https://help.aliyun.com/en/model-studio/what-is-model-studio)
  and [first Qwen API call](https://help.aliyun.com/en/model-studio/first-api-call-to-qwen)
- [DeepSeek chat completion API](https://api-docs.deepseek.com/api/create-chat-completion)
  and [function calling](https://api-docs.deepseek.com/guides/function_calling/)

## Open-Weight And Self-Hosted Inventory

GLM, Qwen, and DeepSeek expose downloadable model artifacts as well as hosted
services. A model family is therefore not an adapter family. The same artifact
can be loaded by several serving runtimes, with different templates, parsers,
quantization, context limits, and protocol facades.

| Concern | Examples | Swallowtail identity |
| --- | --- | --- |
| Model artifact | Qwen checkpoints, GLM checkpoints, DeepSeek checkpoints, a specific quantization | artifact identity and license metadata |
| Serving runtime driver | Ollama, llama.cpp, vLLM, SGLang, future Monkey runtime | driver with native lifecycle and capabilities |
| Configured deployment | local process, LAN server, remote GPU host, managed endpoint | configured instance and execution host |
| Protocol facade | native Ollama API, OpenAI-compatible API, Anthropic-compatible API | transport/protocol capability of that instance |
| Selected model | one loaded model, alias, adapter, quantization, or reasoning profile | model route with observed capabilities |

### Model Families

- [Qwen's official model repository](https://github.com/QwenLM/Qwen3) publishes
  weights and documents Transformers, llama.cpp, Ollama, SGLang, vLLM, and
  other deployments. It also records serving-specific reasoning limitations.
- [DeepSeek's official model repository](https://github.com/deepseek-ai/DeepSeek-V3)
  publishes weights and points to several independent serving stacks. The
  hosted DeepSeek API is a separate configured route.
- [GLM's official model repository](https://github.com/zai-org/GLM-4.5)
  publishes weights and local/deployment guidance. Z.AI hosted general and
  Coding Plan endpoints remain separate routes.

These families belong in the model and route inventory even when Swallowtail
does not need a model-family-specific transport driver.

### Serving Runtimes

| Runtime | Surface | Capability consequence |
| --- | --- | --- |
| Ollama | native HTTP API, OpenAI-compatible endpoints, Python/JS libraries; local and cloud base URLs | streaming, tools, thinking, and structured output are model-dependent; API is stable-intent but not strictly versioned |
| llama.cpp | local CLI and HTTP server with OpenAI-, Responses-, and Anthropic-style routes | chat template, Jinja tool parser, reasoning parser, and model build determine effective support |
| vLLM | high-throughput OpenAI-compatible server | tool and reasoning parsers are explicit deployment configuration |
| SGLang | OpenAI-, Anthropic-, Ollama-compatible, and native APIs | protocol choice and parser configuration are instance capabilities, not model-family facts |
| Monkey | future local-model family or runtime | requires its own evidence and contract before driver work |

Evidence:

- [Ollama API introduction](https://docs.ollama.com/api/introduction),
  [streaming](https://docs.ollama.com/api/streaming), and
  [structured generation](https://docs.ollama.com/api/generate)
- [llama.cpp server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- [vLLM OpenAI-compatible server](https://docs.vllm.ai/en/latest/serving/openai_compatible_server/)
- [SGLang compatible APIs](https://docs.sglang.io/docs/basic_usage/openai_api_completions)

## Shared Protocol

[ACP v1](https://agentclientprotocol.com/protocol/v1/overview) is the clearest
cross-harness protocol candidate. It defines JSON-RPC initialization,
authentication, new/load session, prompt/update, permissions, filesystem and
terminal callbacks, cancellation, plans, modes, and extensibility.

ACP is a transport and lifecycle baseline, not a universal adapter. Drivers
still own launching or locating the agent, credentials, advertised optional
capabilities, provider extensions, persistence, recovery, compatibility, and
diagnostics.

Qwen Code, Gemini CLI, OpenCode, and Kimi provide useful ACP evidence. Their
non-ACP surfaces remain independently valuable drivers.

## Runtime Constraints Promoted

1. Driver identity is more durable than a family-to-transport assumption.
2. Process supervision cannot be mandatory for HTTP-only or in-process SDK
   drivers.
3. A one-shot process and a long-lived bidirectional process need different
   lifecycle capabilities.
4. Sessions, resume, cancellation, steering, permissions, tool callbacks, and
   schemas are negotiated capabilities rather than universal methods.
5. Harness SDKs remain harness execution even when embedded in-process.
6. Model artifacts, serving runtimes, configured deployments, protocol
   facades, and routes remain separate.
7. Compatible HTTP schemas do not justify shared capability claims without
   conformance evidence.
8. Credential, entitlement, endpoint audience, support authority, topology,
   and execution layer remain explicit during routing and fallback.

## Research Gaps

- Grok Build now presents a provider harness surface, but its stable automation
  and remote-control contract needs a dedicated driver study.
- Qwen Code daemon and some ACP implementations are experimental; exact version
  negotiation and recovery need conformance probes.
- Cursor documents structured CLI output but no general long-lived local RPC
  surface was verified.
- Hosted provider model catalogs and subscription terms change too quickly to
  freeze in core records.
- Self-hosted tool/reasoning correctness needs fixture probes per runtime,
  model artifact, template, parser, and version combination.
- Monkey's eventual role as model artifact manager, serving runtime, or gateway
  remains a separate architectural decision.

## Promotion

- durable artifact/runtime/deployment separation: Contract 007
- cross-adapter runtime decision inputs: g01 card 008
- provider-specific conformance work: later implementation roadmaps
