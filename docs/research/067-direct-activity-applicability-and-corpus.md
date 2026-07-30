# 067 Direct Activity Applicability And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

What ordinary observable activity can Swallowtail guarantee across every
production direct, attached, realtime, catalogue, inventory, and serving
route?

## Method

Evidence was checked on 2026-07-29.

- rechecked current official provider or maintained-project documentation
- inspected every exact qualified request, decoder, corpus, and exclusion
- kept current upstream richness separate from the qualified Swallowtail route
- reused deterministic positive, unavailable, unknown, malformed, and failure
  fixtures
- classified every non-harness production route and every non-inference
  operation

No credential, account, provider request, paid inference, model download,
attached runtime, executable, or consumer repository was used.

## Inventory

There are 13 non-harness production routes.

| Route | Ordinary operation | Profile | Available activity | Exact absence |
| --- | --- | --- | --- | --- |
| `anthropic.messages` | structured run | available | assistant lifecycle; provider web-search lifecycle when selected | no qualified reasoning summary |
| `anthropic.messages` | interactive session | available | assistant lifecycle; consumer-tool call lifecycle | no provider tool in this role; no qualified reasoning summary |
| `kimi-platform.chat` | structured run | available | assistant deltas; client-visible K3 thought deltas | no qualified tools |
| `deepseek.continuation` | structured run | available | assistant deltas | no tools in the one-request role; reasoning is private |
| `deepseek.continuation` | interactive session | available | completed consumer-tool call; assistant deltas | reasoning is private continuation |
| `alibaba.conversations` | structured run | available | native response, item, content, and assistant-text lifecycle | selected request disables reasoning and tools |
| `alibaba.conversations` | interactive session | available | one native assistant-item lifecycle per provider turn | selected request disables reasoning and tools |
| `openai.background` | structured run | available | response and assistant-text lifecycle across one reattachment | reasoning effort does not request a readable summary; tools excluded |
| `xai.responses-websocket` | structured run | available | response and assistant-item lifecycle | tools excluded; encrypted thinking is private |
| `xai.responses-websocket` | interactive session | available | one assistant-item lifecycle per serial response | tools excluded; encrypted thinking is private |
| `bedrock.runtime` | structured run | available | message and assistant-text lifecycle | reasoning and tool SDK variants rejected |
| `ollama.attached` | structured run | available | assistant deltas and completion | guaranteed behavior segment excludes thinking and tools |
| `ollama.attached` | interactive session | available | one assistant lifecycle per committed replay turn | guaranteed behavior segment excludes thinking and tools |
| `llama-cpp.attached` | structured run | available | assistant deltas and completion | exact deployment disables parsed reasoning and tools |

The machine corpus records 14 positive ordinary profiles because several
drivers expose both structured and interactive roles.

## Tool Ownership

Only two selected direct routes expose tools.

- Anthropic structured inference may select provider-executed web search.
  `server_tool_use` and `web_search_tool_result` remain provider-owned.
- Anthropic direct continuation exposes a declared client tool. Swallowtail
  correlates the provider call with the consumer callback; it does not claim
  that Anthropic executed the tool.
- DeepSeek direct continuation exposes one declared function call. The
  consumer executes it and supplies the correlated result.

No command, file, plan, task, subagent, shell, or harness-owned tool kind is
valid on these direct routes.

## Reasoning Disclosure

The routes divide into three exact cases.

### Client-visible thought updates

Kimi K3 documents separate streaming `reasoning_content` and final-answer
`content` deltas and demonstrates printing both. The exact K3 corpus already
freezes that separation. The thought stream may map to portable
`ReasoningSummary` as provider-intended readable display. The portable name
does not claim complete reasoning or hidden chain-of-thought.

### Private continuation

DeepSeek V4 requires `reasoning_content` to be replayed with later tool
continuations. Contract 030 already makes it adapter-private, bounded,
zeroized continuation state. It never becomes activity.

xAI may return encrypted thinking for stateless continuation. Encrypted
content has no readable display surface and remains private.

### Unqualified or disabled

- Alibaba's current Responses API exposes explicit reasoning-summary events,
  but the qualified route fixes `reasoning.effort=none`.
- Anthropic's current Messages API can stream summarized thinking when the
  request selects `display: "summarized"`. The qualified route does not enable
  thinking.
- OpenAI background accepts reasoning effort, but the selected request and
  corpus do not request or qualify a readable summary.
- Bedrock's selected SDK decoder rejects `ReasoningContent`.
- Ollama's guaranteed `native-text-v1` segment rejects `thinking`.
- llama.cpp's exact attached deployment fixes `--reasoning-format none`.

These remain unavailable. Current upstream capability is not a route claim.

## Current Richer Surfaces

Current evidence shows useful later work without changing this tranche.

| Surface | Current official evidence | Current Swallowtail result |
| --- | --- | --- |
| Alibaba Responses | reasoning-summary, provider-tool, MCP, search, and code lifecycle events | later separately qualified facade; exact route remains text-only |
| Anthropic Messages | summarized thinking, fine-grained client-tool input, server-tool lifecycle | tool mappings selected; summarized thinking remains later input and corpus work |
| Kimi K3 | reasoning deltas, tools, vision, structured output, dynamic tools | reasoning mapping selected; tools and other inputs remain outside exact route |
| Ollama native chat | thinking and tool calls in current response schema | guaranteed range stays `native-text-v1` |
| llama.cpp server | parsed reasoning and function calling in current server | exact b9910 deployment stays text-only |
| xAI Responses | tool-oriented WebSocket continuation and encrypted thinking | exact route stays assistant-only |
| Gemini Live | current raw WebSocket also documents consumer-executed function calls | exact audio route sends no tools |
| OpenAI Realtime | current session surface supports function calling and text or audio modes | exact audio-only route sends no tools |

New upstream features need their own interface revision, request selection,
dated corpus, prepared profile, and disclosure decision. They cannot appear
under permitted-unverified-newer execution.

## Not Applicable

Ordinary observable activity is not applicable to:

- six route-local catalogue or inventory operations: Anthropic, Kimi Platform,
  DeepSeek, Bedrock control plane, Ollama inventory, and llama.cpp attached
- four auxiliary catalogue branches: Alibaba deployable models, Gemini
  Models, OpenAI Models, and xAI Models
- OpenAI Realtime and Gemini Live selected media sessions
- llama.cpp owned serving lifecycle

Realtime audio, transcript, input commit, interruption, response status, and
rollover stay under Contract 026. Catalogue, inventory, readiness, usage, rate,
quota, cost, and serving observations keep their existing typed evidence
roles. They do not become agent activity.

This produces 13 not-applicable operation profiles. `Unavailable` remains
different: it describes an ordinary route whose selected interface lacks one
activity kind. `NotApplicable` means the operation has no ordinary
agent-activity role.

## Corpus

`swallowtail-testkit/tests/fixtures/direct-activity-applicability.json`
machine-freezes:

- all 13 non-harness production routes
- 14 positive text-operation profiles
- 13 not-applicable catalogue, inventory, realtime, and serving operations
- exact assistant, reasoning, provider-tool, and consumer-tool truth
- ownership, lifecycle, correlation, disclosure, and absence boundaries
- paths to existing positive, unavailable, unknown, malformed, and failure
  fixtures
- one exact mapping order for the text tranche

The focused test reads the existing provider corpora directly. Bedrock remains
a typed SDK fixture because generated EventStream values are the provider
surface; it does not need a counterfeit JSON wire transcript.

## Contract Fit

Contracts 026, 030, and 044 already settle the result.

- direct inference may expose assistant, reasoning-summary, consumer-tool, and
  provider-tool activity
- private continuation never becomes display content
- realtime media remains separate except for independently qualified tools
- catalogue and serving-only operations have no activity profile
- current upstream additions do not widen a qualified facade

No contract or architecture delta is required before implementation.

## Mapping Selection

Card 133 should map only the 14 positive text profiles, in this order:

1. Alibaba native assistant-item lifecycle
2. Anthropic assistant, provider-search, and consumer-tool lifecycle
3. DeepSeek assistant and consumer-tool lifecycle
4. Kimi assistant and reasoning-summary deltas
5. OpenAI background assistant lifecycle across reattachment
6. xAI assistant lifecycle
7. Bedrock assistant lifecycle
8. Ollama assistant lifecycle
9. llama.cpp assistant lifecycle

Card 134 should prove the two realtime not-applicable profiles and the
catalogue, inventory, and serving boundaries without flattening media or
provider observations.

## Sources

- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [Anthropic streaming Messages](https://platform.claude.com/docs/en/build-with-claude/streaming)
- [Anthropic server tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/server-tools)
- [Amazon Bedrock ConverseStream](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html)
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)
- [DeepSeek thinking mode](https://api-docs.deepseek.com/guides/thinking_mode/)
- [Kimi K3](https://platform.kimi.ai/docs/guide/kimi-k3-quickstart)
- [Kimi Chat Completions](https://platform.kimi.ai/docs/api/chat)
- [OpenAI background mode](https://developers.openai.com/api/docs/guides/background)
- [OpenAI Realtime conversations](https://developers.openai.com/api/docs/guides/realtime-conversations)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI text generation and encrypted thinking](https://docs.x.ai/developers/model-capabilities/text/generate-text)
- [Gemini Live raw WebSocket](https://ai.google.dev/gemini-api/docs/live-api/get-started-websocket)
- [Ollama native chat](https://docs.ollama.com/api/chat)
- [Ollama streaming](https://docs.ollama.com/capabilities/streaming)
- [llama.cpp server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
