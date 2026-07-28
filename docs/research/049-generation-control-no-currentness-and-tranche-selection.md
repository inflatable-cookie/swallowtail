# 049 Generation-Control `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 48 starting output-limit, reasoning-selection, and
structured-output `No` cells can the selected route actually support? Which
first implementation tranche gives useful transport coverage without
overclaiming enforcement?

## Method

Evidence was accessed 2026-07-28.

- parsed the canonical 22-solution CSV by exact route and feature
- checked every `No` against the realized prepared request validation
- checked exact qualified tags or revisions where maintained source exists
- used current official provider documentation for opaque hosted facades
- kept harness configuration, provider request parameters, model capability,
  schema enforcement, and client validation separate
- used no executable, account, credential, provider request, container, or
  model server

The realized adapters rejected every audited control marked `No` at the start
of this audit. There were no matrix false negatives. The cells below are
implementation or honest-absence classifications.

## Dispositions

| Code | Cells | Meaning |
| --- | ---: | --- |
| `R` | 4 | Exact selected surface is ready under existing portable request types |
| `C` | 18 | Upstream control exists; shared contract detail or exact corpus is still required |
| `H` | 3 | Upstream control exists, but the xAI route remains operator-held |
| `U` | 20 | Exact selected operation exposes no qualifying control |
| `S` | 3 | The provider agent version owns the setting; this operation cannot select it |
| **Total** | **48** | Every starting `No` exactly once |

`R`, `C`, and `H` are 25 plausible conversions. They are not current
capability claims.

## Exact Cell Audit

A dash means that the cell was already `Yes` or `Not applicable`; it is not
part of the 48-cell audit.

| Route | Output limit | Reasoning | Structured output | Exact finding |
| --- | --- | --- | --- | --- |
| `qwen.headless` | `C` | `C` | `C` | Exact `0.19.11` has fixed output-limit configuration, reasoning effort, and headless `--json-schema`; application and enforcement differ |
| `alibaba.conversations` | `U` | `C` | `U` | Current Responses route documents reasoning effort but not a qualifying output limit or schema format |
| `bedrock.runtime` | — | `C` | `C` | Converse supports model-specific thinking fields and `outputConfig.textFormat`; both need model and schema-subset evidence |
| `claude-agent.acp` | `U` | — | `U` | ACP config options qualify reasoning only; no portable output maximum or output-schema channel |
| `claude-code.headless` | `U` | — | `U` | Exact CLI exposes effort and stream envelope controls, not a generation maximum or output schema |
| `anthropic.managed-agent` | `S` | `S` | `S` | Operator-owned agent version owns model configuration |
| `anthropic.messages` | — | `C` | `C` | Messages supports model-specific effort and `output_config.format` |
| `pi.rpc` | `U` | `C` | `U` | Exact `0.80.10` confirms `set_thinking_level`; catalogue `maxTokens` is observation, not caller control |
| `deepseek.continuation` | — | — | `U` | `json_object` plus prompt instruction is not schema enforcement |
| `gemini-cli.acp + gemini-cli.headless` | `C` | `C` | `U` | Exact `0.51.0..=0.52.0` model configuration maps to SDK output and thinking controls; neither branch has a qualified operation-private mapping, and JSON output is only a CLI envelope |
| `gemini.live` | `R` | `C` | `U` | Live supports `maxOutputTokens` and model-specific `thinkingConfig`; response schemas are explicitly unsupported |
| `llama-cpp.attached` | — | `C` | `C` | Exact `f5525f7e7` supports request thinking controls and schema-constrained `response_format` |
| `kimi-code.acp + kimi-code.headless` | `U` | — | `U` | Exact `0.29.2` ACP exposes the solution's reasoning selection but no output maximum or structured-output channel; current TypeScript headless exposes none of the three controls |
| `kimi-code.local-server` | `U` | — | `U` | Catalogue output size is observation; local requests expose neither control |
| `kimi-platform.chat` | — | — | `U` | No native schema-enforcement evidence for the selected Chat facade |
| `ollama.attached` | — | `C` | `C` | Chat `think` and `format` are model-dependent and need qualified range segments |
| `codex.app-server; codex.exec` | `U` | — | — | Both qualified Codex branches expose effort; exec owns output schema; neither exposes a caller generation maximum |
| `openai.realtime` | `R` | `U` | `U` | Realtime exposes `max_output_tokens`; selected session shape exposes no reasoning or response-schema control |
| `openai.background` | — | `R` | `R` | Responses accepts reasoning effort and JSON-schema text format |
| `opencode.http` | `U` | `C` | `C` | Exact `1.14.48..=1.18.4` prompt input has `variant` and schema `format`; no stable per-operation output maximum |
| `xai.responses-websocket` | `H` | `H` | `H` | WebSocket `response.create` uses the Responses request body; implementation remains operator-held |

## Control Strength

### Output limits

A qualifying output limit must reach the provider or harness generation
request as a maximum. Catalogue metadata, context limits, client truncation,
turn limits, and tool budgets do not qualify.

Qwen's exact fixed setting qualifies only when automatic escalation is
disabled. Gemini CLI configuration reaches SDK `maxOutputTokens`. Realtime
uses the provider session field directly.

### Reasoning

Reasoning visibility is not selection. Generic modes such as `low` and `high`,
numeric thinking budgets, booleans, model variants, and provider-specific
configuration channels are not interchangeable.

The common `ReasoningMode` remains the consumer selection. Each adapter must
bind an exact model/version mapping and reject unsupported values. A provider
clamp, ignored value, or unconfirmed harness update cannot become the
requested effective mode.

### Kimi executable boundary

The current `kimi-code.headless` route is the TypeScript
`@moonshot-ai/kimi-code` executable. Exact `0.29.2` and current main accept
model, prompt, and output-format inputs but no headless thinking input.

Moonshot's older Python `kimi-cli` line added `--thinking` in `0.51` and now
documents `--thinking` plus `--no-thinking`. That is a different repository,
distribution, implementation, and semantic surface. It cannot become a
`kimi-code.executable` compatibility milestone. Supporting it would require a
separate executable identity and route qualification.

### Structured output

Three enforcement sources are materially different:

- provider-native schema enforcement: Bedrock, Anthropic Messages, llama.cpp,
  Ollama, OpenAI Responses, and xAI Responses
- harness-owned schema tool plus validation or retry: Qwen Code and OpenCode
- prompt convention, JSON envelope, or consumer post-validation: not a
  `StructuredOutput` capability

Schema dialect and supported-key subsets remain exact. Swallowtail transports
the schema and reports the qualified enforcement source; it does not silently
upgrade prompt conventions or make consumer acceptance implicit.

## Exact Evidence

### Harnesses

- [Qwen Code `0.19.11` settings](https://github.com/QwenLM/qwen-code/blob/v0.19.11/docs/users/configuration/settings.md)
- [Qwen Code headless structured output](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Claude Agent ACP `0.61.0` source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [Claude Code CLI reference](https://learn.chatgpt.com/docs/cli/reference)
- [Pi RPC `0.80.10`](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [Gemini CLI `0.51.0` model configuration](https://github.com/google-gemini/gemini-cli/blob/v0.51.0/docs/cli/generation-settings.md)
- [Gemini CLI `0.52.0` model configuration](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/docs/cli/generation-settings.md)
- [Gemini CLI headless automation](https://geminicli.com/docs/cli/tutorials/automation/)
- [Kimi Code `0.29.2` CLI options](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/options.ts)
- [Kimi Code current CLI options](https://github.com/MoonshotAI/kimi-code/blob/main/apps/kimi-code/src/cli/options.ts)
- [Python Kimi CLI command reference](https://moonshotai.github.io/kimi-cli/en/reference/kimi-command.html)
- [Python Kimi CLI changelog](https://moonshotai.github.io/kimi-cli/en/release-notes/changelog.html)
- [OpenCode `1.14.48` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.18.4` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)

### Hosted providers

- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [Bedrock Converse](https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference.html)
- [Bedrock structured output](https://docs.aws.amazon.com/bedrock/latest/userguide/structured-output.html)
- [Bedrock Claude thinking](https://docs.aws.amazon.com/bedrock/latest/userguide/claude-messages-adaptive-thinking.html)
- [Anthropic Messages](https://platform.claude.com/docs/en/api/messages/create)
- [Anthropic effort](https://platform.claude.com/docs/en/build-with-claude/effort)
- [Anthropic structured outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)
- [DeepSeek Chat](https://api-docs.deepseek.com/api/create-chat-completion)
- [Gemini Live](https://ai.google.dev/api/live)
- [Kimi Code configuration](https://moonshotai.github.io/kimi-code/en/configuration/config-files)
- [Kimi Platform prompt guidance](https://platform.moonshot.ai/docs/guide/prompt-best-practice)
- [OpenAI Responses API](https://developers.openai.com/api/reference/resources/responses/methods/create)
- [OpenAI Realtime call configuration](https://developers.openai.com/api/reference/resources/realtime/subresources/calls/methods/accept)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI text controls](https://docs.x.ai/developers/model-capabilities/text/comparison)
- [xAI reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning)
- [xAI structured outputs](https://docs.x.ai/developers/model-capabilities/text/structured-outputs)

### Attached runtimes

- [llama.cpp exact server revision](https://github.com/ggml-org/llama.cpp/blob/f5525f7e7/tools/server/README.md)
- [Ollama Chat API](https://docs.ollama.com/api/chat)
- [Ollama thinking](https://docs.ollama.com/capabilities/thinking)
- [Ollama structured outputs](https://docs.ollama.com/capabilities/structured-outputs)

## First Tranche

Select seven cells across three existing adapter crates:

- OpenAI: background reasoning, background structured output, and Realtime
  output limit
- Ollama: reasoning and structured output
- OpenCode: reasoning and structured output

This covers hosted structured inference, hosted realtime configuration,
attached local inference, and a provider-owned HTTP harness. It avoids new
route identities, live credentials, heavy containers, ambient configuration
mutation, and the operator-held xAI route.

Qwen and Gemini harness configuration should follow in a separate host-scoped
configuration tranche. Bedrock, Anthropic, and the remaining direct routes
follow through model-capability and schema-dialect corpora.

## Missing Shared Rules

Card 085 should promote one narrow contract:

- request, planned, dispatched, provider-accepted, and effective control
  states stay separate
- model-conditioned controls require exact route/model capability evidence
- reasoning mappings are exact and never clamp or fall back silently
- structured-output enforcement source and schema dialect remain visible
- harness validation is not mislabeled provider-native enforcement
- unsupported or ignored controls fail before successful completion
- version milestones can add or remove a control without splitting provider
  identity

The existing request types, capability constraints, schema transport, version
milestones, and prepared-facade boundary remain sufficient. No generic
generation-parameter map is needed.

## Promotion

- Classified all 48 starting cells.
- Found 25 plausible conversions and no current matrix error.
- Selected the OpenAI, Ollama, and OpenCode tranche.
- Preserved the xAI operator hold.
- Left the broader feature-family matrix runway explicit after generation
  controls.
