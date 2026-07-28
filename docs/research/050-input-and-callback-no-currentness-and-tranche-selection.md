# 050 Input And Callback `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 74 starting attachment, consumer-tool, approval-or-question, and
external-search `No` cells can the selected route support? Which first tranche
tests the missing boundaries without flattening provider-owned behavior into
consumer authority?

## Method

Evidence was accessed 2026-07-28.

- parsed the canonical 22-solution CSV by exact route and feature
- checked all 74 starting cells against realized prepared plans, validation,
  fixtures, and public facades
- checked exact tags or revisions for maintained harnesses and attached
  runtimes
- used current official provider documentation for hosted and opaque facades
- kept composite-solution branches separate
- used no executable, account, credential, provider request, container, or
  model server

The concurrent Claude Agent one-shot permission conversion starts from a
`Partial` cell. It is not one of these 74 starting `No` cells.

No audited `No` has a realized prepared path. The matrix is therefore
internally correct. The dispositions below classify future conversion or
honest absence; they do not claim current Swallowtail support.

## Dispositions

| Code | Cells | Meaning |
| --- | ---: | --- |
| `R` | 4 | Exact selected surface fits existing portable contracts; route corpus and implementation remain |
| `C` | 36 | Upstream support exists; shared contract detail, model/version qualification, or route corpus remains |
| `P` | 5 | Only one branch of a composite solution qualifies; implementation can make the solution `Partial`, not `Yes` |
| `H` | 3 | Upstream support exists, but xAI remains operator-held |
| `U` | 25 | Exact selected route exposes no qualifying operation-private feature |
| `M` | 1 | The input belongs to realtime-media transport, not the attachment operation shape |
| **Total** | **74** | Every starting `No` exactly once |

`R`, `C`, and `P` contain 45 actionable cells. Three more remain held. A `P`
conversion is useful but cannot be promoted to solution-level `Yes` until both
listed route branches qualify.

## Exact Cell Audit

A dash means the cell was already `Yes` or `Partial`; it is not part of the
74-cell audit.

| Route | Attachments | Consumer tools | Approval or question | External search | Exact finding |
| --- | --- | --- | --- | --- | --- |
| `qwen.headless` | `U` | `U` | `U` | `U` | Exact `0.19.11` headless accepts prompt text and fixed non-interactive policy. Tool discovery and later web-search work are ambient or newer surfaces, not an operation-private callback or exact selected-version search channel. |
| `alibaba.conversations` | `C` | `C` | `U` | `C` | Current Responses supports qualified image/file inputs, custom functions, and provider web search. It exposes no consumer approval exchange. |
| `bedrock.catalogue; bedrock.runtime` | `C` | `C` | `U` | `U` | Converse supports model-qualified image/document blocks and client-side tools. Provider search belongs to different Bedrock APIs or model-specific sibling behavior, not the selected Runtime EventStream route. |
| `claude-agent.acp` | `C` | `C` | — | `C` | ACP can negotiate image prompts and MCP servers; the exact agent has provider-owned search. Image, MCP bridge, model, and permission policy still need exact range corpora. |
| `claude-code.headless` | `C` | `C` | `C` | `C` | Stream JSON can carry structured user content; exact CLI controls expose MCP configuration and a permission-prompt tool; WebSearch is provider-owned. Each needs operation-private configuration and exact `2.1.220` evidence. |
| `anthropic.managed-agent` | `C` | — | `C` | `C` | Managed sessions accept text, image, and document inputs; permission policies can pause for an allow-or-deny response; managed tools include web search. |
| `anthropic.messages` | `C` | `C` | `U` | `C` | Messages accepts image content, client tools, and Anthropic server-side web search. The API does not ask the consumer to approve model-selected calls. |
| `pi.rpc` | `R` | `U` | — | `U` | Exact `0.80.10` RPC prompt, steer, and follow-up messages accept base64 image content. Extensions and skills are process configuration, not RPC tool registration or provider search. |
| `deepseek.continuation` | `U` | — | `U` | `U` | Exact `deepseek-v4-pro` Chat input is text-only. The route supports client tools but no approval exchange or native search tool. |
| `gemini-cli.acp + gemini-cli.headless` | `P` | `P` | — | `C` | ACP can negotiate images and MCP servers; the selected headless branch is text-first and disables extensions and MCP. Provider Google search needs exact branch and policy qualification. |
| `gemini.live` | `M` | `C` | `U` | `C` | Live supports media, function calling, and Google Search. Its continuous image/audio/video transport remains `RealtimeMediaSession`, not a portable finite attachment. |
| `llama-cpp.attached` | `C` | `C` | `U` | `U` | Exact `f5525f7e7` supports multimodal input when an `mmproj` is loaded and function calling when the model template qualifies. It has no provider search or approval exchange. |
| `kimi-code.acp + kimi-code.headless` | `P` | `P` | `P` | `C` | Exact `0.29.2` ACP advertises image input, forwards MCP servers, and performs permission/question client requests. Headless has no equivalent response channel and accepts prompt text. Search requires exact host implementation and access evidence. |
| `kimi-code.local-server` | `R` | `U` | — | `C` | Exact local REST messages accept image and text parts. The server exposes approval and question exchange, but not operation-private consumer tool registration. Search depends on the exact server runner and account configuration. |
| `kimi-platform.chat` | `C` | `C` | `U` | `C` | Kimi K3 supports image input and function tools; Kimi Platform exposes official web-search tools. No approval exchange is part of Chat. |
| `ollama.attached` | `C` | `C` | `U` | `U` | Native Chat supports model-qualified images and tool calls. Ollama's hosted web-search API requires separate account and credential authority and is not the attached runtime. |
| `openai.realtime` | `C` | `C` | `U` | `U` | The selected Realtime model accepts image input and function tools. The exact Realtime session does not expose provider web search or a consumer approval request. |
| `openai.background` | `C` | `U` | `U` | `C` | Responses supports image/file input and provider web search. Contract 021 fixes this background route to one inference attempt, so a client-tool result loop does not fit the selected operation. |
| `opencode.http` | `R` | `C` | `R` | `C` | Exact OpenAPI has file message parts plus permission and question response endpoints. Consumer-defined tools require a separately bounded MCP bridge; web search is provider-owned and access-conditioned. |
| `xai.responses-websocket` | `H` | `H` | `U` | `H` | WebSocket response bodies inherit Responses image, function-tool, and web-search fields. The route remains held because the operator has no account for live proof. |

## Feature Boundaries

### Attachments

Attachments are finite operation inputs. They never imply a client path,
workspace lease, model artifact, arbitrary URL, or realtime-media stream.

Provider image, document, and file inputs are model- and media-qualified.
Swallowtail must bind the accepted media set, representation, count, size, and
cleanup to the immutable plan. A host attachment lease may become bounded
bytes, a data URL, an uploaded provider file, or an operation-scoped temporary
file only when the route corpus fixes that conversion.

Gemini Live media stays under Contract 026. Calling it an attachment would
erase connection timing and stream lifecycle.

### Consumer Tool Exchange

Three mechanisms remain separate:

- native inline tool declarations and correlated results
- provider-owned server tools such as web search
- MCP or harness configuration that makes tools available to an agent

Only the first is a direct portable tool exchange. An MCP-backed route can
qualify later, but it needs one operation-scoped bridge with explicit
registration, correlation, cancellation, and joined cleanup. Ambient MCP
configuration is not enough.

Direct APIs also need an operation shape that can accept a tool result and
authorize another inference attempt. A one-attempt structured route cannot
claim exchange merely because its response can contain a function call.
Contract 030 already owns the consumer-authorized continuation shape.

### Approval And Question Exchange

An approval or question is a namespaced provider request under Contract 012,
not a consumer tool call. Observing and stopping, rejecting, answering a
question, selecting a one-shot permission, and selecting a persistent
permission are different strengths.

The first tranche keeps persistent OpenCode approval unavailable. The
consumer may select only an exact offered one-shot answer. Swallowtail
transports and correlates the response; it does not choose it or execute the
provider tool.

### External Search

`ExternalSearchPolicy::Enabled` authorizes a selected provider-owned search
tool only when provider-side external network is separately allowed. It does
not grant arbitrary host networking, web fetching, browser control, consumer
tool execution, or a sibling product's search service.

The plan must retain provider execution authority, tool revision, model
support, organization enablement, access or billing requirements, search
limits, and usage or citation evidence. Search remains disabled by default.

## Current Evidence

### Harnesses And Shared Protocol

- [Qwen Code `0.19.11` settings](https://github.com/QwenLM/qwen-code/blob/v0.19.11/docs/users/configuration/settings.md)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [ACP protocol](https://agentclientprotocol.com/protocol/overview)
- [Claude Agent ACP `0.61.0`](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Pi RPC `0.80.10`](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [Gemini CLI tools](https://geminicli.com/docs/tools/)
- [Kimi Code `0.29.2` ACP reference](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/docs/en/reference/kimi-acp.md)
- [Kimi Code headless mode](https://moonshotai.github.io/kimi-code/en/reference/headless-mode)
- [OpenCode `1.18.4` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)
- [OpenCode server](https://dev.opencode.ai/docs/server/)
- [OpenCode permissions](https://dev.opencode.ai/docs/permissions/)

### Hosted Providers

- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [Alibaba function calling](https://www.alibabacloud.com/help/en/model-studio/qwen-function-calling)
- [Bedrock Converse content blocks](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ContentBlock.html)
- [Bedrock client-side tool use](https://docs.aws.amazon.com/bedrock/latest/userguide/tool-use-client-side.html)
- [Anthropic Managed Agent files](https://platform.claude.com/docs/en/managed-agents/files)
- [Anthropic Managed Agent permissions](https://platform.claude.com/docs/en/managed-agents/permission-policies)
- [Anthropic Managed Agent tools](https://platform.claude.com/docs/en/managed-agents/tools)
- [Anthropic Messages](https://platform.claude.com/docs/en/api/messages/create)
- [Anthropic client and server tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)
- [Anthropic web search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)
- [DeepSeek Chat](https://api-docs.deepseek.com/api/create-chat-completion)
- [Gemini Live model](https://ai.google.dev/gemini-api/docs/models/gemini-3.1-flash-live-preview)
- [Gemini Live tools](https://ai.google.dev/gemini-api/docs/live-api/tools)
- [Kimi API overview](https://www.kimi.com/help/kimi-api/api-overview)
- [Kimi official tools](https://platform.kimi.ai/docs/guide/use-official-tools)
- [OpenAI model catalogue](https://developers.openai.com/api/docs/models)
- [OpenAI Realtime model](https://developers.openai.com/api/docs/models/gpt-realtime-2.1)
- [OpenAI function calling](https://developers.openai.com/api/docs/guides/function-calling)
- [OpenAI web search](https://developers.openai.com/api/docs/guides/tools-web-search)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI image input](https://docs.x.ai/developers/model-capabilities/images/understanding)
- [xAI function calling](https://docs.x.ai/developers/model-capabilities/text/function-calling)
- [xAI web search](https://docs.x.ai/developers/tools/search-tools)

### Attached Runtimes

- [llama.cpp exact server revision](https://github.com/ggml-org/llama.cpp/blob/f5525f7e7/tools/server/README.md)
- [llama.cpp exact function-calling revision](https://github.com/ggml-org/llama.cpp/blob/f5525f7e7/docs/function-calling.md)
- [Ollama vision](https://docs.ollama.com/capabilities/vision)
- [Ollama tool calling](https://docs.ollama.com/capabilities/tool-calling)
- [Ollama hosted web search](https://docs.ollama.com/capabilities/web-search)

## First Tranche

Select six audited cells across three existing adapters:

- Pi RPC: attachments
- OpenCode HTTP: attachments and approval-or-question exchange
- Anthropic Messages: attachments, consumer-tool exchange, and external search

Anthropic client-tool exchange also requires one adjacent
`interactive_session` conversion. It reuses Contract 030's consumer-owned
direct continuation instead of stretching a one-attempt structured run.

The tranche covers:

- base64 image input over installed JSONL RPC
- file-part input and provider-request responses over attached HTTP/SSE
- image input, client tool continuation, and provider-owned search over hosted
  HTTPS/SSE

It uses no new provider, executable, endpoint, credential mechanism, heavy
container, ambient MCP mutation, or live access. It avoids the xAI hold and
does not depend on the concurrent Claude Agent permission work.

## Contract And Corpus Gate

Card 089 should promote one narrow input and callback contract:

- attachment input, host representation, model media support, and cleanup
  remain separate
- direct client tools, provider-owned tools, and MCP bridges remain separate
- one-attempt structured runs cannot claim a tool-result loop
- provider requests preserve namespace, offered options, response strength,
  correlation, and exactly-once semantics
- provider-owned search requires explicit search and provider-network policy
  plus exact tool, model, access, billing, and effect evidence
- version milestones may add or remove any feature without splitting provider
  identity or hard-denying unverified-newer versions

Freeze these offline corpora:

- Pi `0.80.10` image prompt, wrong media, oversize, cancellation, and cleanup
- OpenCode `1.14.48..=1.18.4` file parts, permission and question events,
  one-shot replies, mismatch, duplicate, late response, abort, and cleanup
- Anthropic `2023-06-01` image input, client tool call/result continuation,
  server web search, mixed client/server tools, model or organization
  rejection, cancellation, deadline, usage, and credential-last cleanup

## Promotion

- Classified all 74 starting cells exactly once.
- Found 45 actionable cells, three held xAI cells, 25 exact route absences,
  and one realtime-media operation-shape mismatch.
- Found no realized matrix error.
- Selected a six-cell, three-transport tranche spanning all four audited
  feature columns.
- Made card 089's contract and corpus envelope exact.
