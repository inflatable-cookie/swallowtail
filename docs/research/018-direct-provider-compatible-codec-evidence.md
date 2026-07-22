# 018 Direct Provider Compatible-Codec Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Which direct provider should start practical breadth, and what part of its
OpenAI-compatible Chat Completions surface can be shared without flattening
provider identity, access, lifecycle, or capability semantics?

## Method

Official provider documentation was checked on 2026-07-21. The comparison uses
the realized hosted-direct, background, connection-scoped, catalogue, and
attached-runtime contracts. No credential, provider account, external request,
paid inference, or live model catalogue was used.

## Kimi Platform

Kimi Open Platform now redirects its international documentation to
`platform.kimi.ai` and names Kimi K3 as the current flagship. K3 uses model id
`kimi-k3`, always reasons, exposes `low`, `high`, and `max` effort, supports
streaming reasoning and output deltas, and has a one-million-token context
window. The Chat Completions route is `POST
https://api.moonshot.ai/v1/chat/completions` with bearer API-key access.

The dedicated Thinking Effort guide and the current request schema both expose
`low`, `high`, and `max`. One older embedded paragraph on the Chat Completions
page still says K3 accepts only `max`; that paragraph conflicts with the K3
guide, dedicated capability guide, and schema. The frozen corpus follows the
dedicated guide and schema and requires an explicit selection.

The direct API is stateless. Consumers return prior messages and tool results
explicitly; the provider does not retain chat history for this route. SSE ends
with `[DONE]`, and the terminal chunk may carry prompt, completion, total, and
cached token usage. The response echoes the requested model. A separate
authenticated `GET /v1/models` returns current model ids, context length, and
image, video, and reasoning flags.

Kimi Open Platform is pay-as-you-go and has no subscription plan. Kimi K3
requires a successful minimum top-up. Kimi Membership, Kimi Code, and regional
platform keys remain separate. The current catalogue documentation says keys
from `platform.kimi.ai` and `platform.kimi.com` are not interchangeable.

The provider's reconnection guide contains application retry examples. Those
examples do not grant Swallowtail retry authority. The first driver keeps one
attempt and leaves retry to the consumer.

Evidence, accessed 2026-07-21:

- [Kimi API overview](https://platform.kimi.ai/docs/api/overview)
- [Kimi K3](https://platform.kimi.ai/docs/guide/kimi-k3-quickstart)
- [Thinking effort](https://platform.kimi.ai/docs/guide/use-thinking-effort)
- [Chat Completions](https://platform.kimi.ai/docs/api/chat)
- [Model catalogue](https://platform.kimi.ai/docs/api/list-models)
- [Products and plans](https://platform.kimi.ai/docs/guide/product-plans)
- [Errors](https://platform.kimi.ai/docs/api/errors)
- [Automatic reconnection](https://platform.kimi.ai/docs/guide/auto-reconnect)

## DeepSeek

DeepSeek now documents both OpenAI- and Anthropic-compatible APIs. The current
direct models are `deepseek-v4-pro` and `deepseek-v4-flash`; the older
`deepseek-chat` and `deepseek-reasoner` aliases are scheduled for deprecation
on 2026-07-24. Chat Completions streams data-only SSE plus `[DONE]`, terminal
usage, cache-hit and cache-miss tokens, reasoning tokens, tool calls, and
provider-specific finish reasons. Keepalive comments may arrive while a
request remains open.

Compatibility is explicitly semantic, not only structural. Anthropic model
names are mapped to DeepSeek models, unsupported Anthropic fields may be
ignored, and reasoning-effort values may be mapped. An unsupported Anthropic
model name is automatically mapped to `deepseek-v4-flash`. A future stable
driver must therefore pin a native DeepSeek model id, validate returned model
identity, and own an explicit ignored-or-rejected field table. No current
catalogue operation is selected by this evidence.

Evidence, accessed 2026-07-21:

- [DeepSeek first API call](https://api-docs.deepseek.com/)
- [Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion)
- [Anthropic API compatibility](https://api-docs.deepseek.com/guides/anthropic_api)
- [Rate limits and isolation](https://api-docs.deepseek.com/quick_start/rate_limit/)
- [Thinking mode](https://api-docs.deepseek.com/guides/thinking_mode)

## Z.AI

Z.AI's general API uses bearer API keys and the OpenAI-compatible base
`https://api.z.ai/api/paas/v4`. Current examples select `glm-5.1`, expose
streaming, reasoning content, tool calls, usage, and provider business error
codes. The provider also publishes a native SDK. This evidence does not select
one over the other or claim a live catalogue endpoint.

The GLM Coding Plan is a separate subscription audience. It uses
`https://api.z.ai/api/coding/paas/v4` for OpenAI-compatible tools and
`https://api.z.ai/api/anthropic` for supported Anthropic clients. Provider
policy limits the plan to named supported tools and rejects SDK or unsupported
integration use. A Swallowtail direct driver must use the general API unless a
future provider-supported product integration names Swallowtail's audience.

Evidence, accessed 2026-07-21:

- [Z.AI general API quick start](https://docs.z.ai/guides/overview/quick-start)
- [OpenAI SDK compatibility](https://docs.z.ai/guides/develop/openai/python)
- [Chat Completions](https://docs.z.ai/api-reference/llm/chat-completion)
- [API errors](https://docs.z.ai/api-reference/api-code)
- [Coding Plan quick start](https://docs.z.ai/devpack/quick-start)
- [Coding Plan usage policy](https://docs.z.ai/devpack/usage-policy)

## Alibaba Model Studio And Qwen

Alibaba Model Studio supports OpenAI-compatible Chat Completions and Responses
plus an Anthropic-compatible Messages route. Pay-as-you-go API keys belong to
one user, workspace, and region. Base URLs vary by region and protocol, and
workspace authorization can narrow models, applications, IP addresses, and
rate limits. These dimensions must remain in configured-instance and access-
profile identity.

The Model Studio Coding Plan is a separate subscription key and endpoint. Its
documentation prohibits automated scripts, custom application backends, and
other non-interactive batch use. It is not a Swallowtail direct-inference
credential. The Anthropic-compatible Messages route explicitly has no
`/v1/models` endpoint.

Model Studio Responses adds provider-stored response ids, seven-day
continuation, conversations, built-in tools, and `store=true` by default.
Those are useful later provider-specific lifecycle proofs, not evidence that a
basic Chat Completions codec owns storage or continuation.

Evidence, accessed 2026-07-21:

- [API keys](https://help.aliyun.com/en/model-studio/get-api-key)
- [Workspace permissions](https://help.aliyun.com/en/model-studio/permission-management-overview)
- [OpenAI-compatible text generation](https://help.aliyun.com/en/model-studio/text-generation)
- [Anthropic-compatible Messages](https://help.aliyun.com/en/model-studio/anthropic-api-messages)
- [OpenAI-compatible Responses](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)
- [Coding Plan FAQ](https://help.aliyun.com/en/model-studio/coding-plan-faq)

## Compatible Codec Boundary

The reusable seam is smaller than a provider adapter:

- bounded SSE framing, comments, data records, and `[DONE]`
- bounded structural encoding and decoding for common Chat Completions request,
  chunk, choice, content-delta, finish, model, usage, and error envelopes
- no endpoint, credential, provider, model selection, capability, lifecycle,
  retry, storage, catalogue, billing, or support authority

The provider driver still owns:

- exact path, headers, access profile, audience, model route, and returned-model
  agreement
- supported and rejected request fields
- reasoning, tool, structured-output, multimodal, finish-reason, error, usage,
  rate, and quota semantics
- catalogue shape and its relationship to invocation
- cancellation truth, retry count, state, retention, and continuation

Unknown semantic fields cannot be discarded by the shared codec. It returns a
bounded structural unknown to the provider adapter or fails. The adapter's
dated corpus decides whether that field is accepted, namespaced, or rejected.
No raw payload becomes a stable runtime API.

Existing llama.cpp Chat Completions framing and the Kimi K3 corpus provide the
two independent fixtures needed to justify a shared
`swallowtail-protocol-openai-chat` package. llama.cpp keeps its deployment-
specific request and semantic rejection rules.

## Comparison

| Candidate | Breadth value | Main provider-specific pressure | Result |
| --- | --- | --- | --- |
| Kimi Platform K3 | current flagship, stateless direct stream, authenticated catalogue, reasoning and usage | platform-key audience, mandatory reasoning, model-specific fixed parameters | select |
| DeepSeek V4 | two compatible formats, cache and reasoning usage, tools | model alias mapping, ignored fields, keepalive, imminent alias retirement | later mapping |
| Z.AI general API | OpenAI-compatible plus native SDK, GLM reasoning and tools | general versus Coding Plan audience, SDK choice, business error semantics | later mapping |
| Alibaba/Qwen | regional workspace breadth, three compatible surfaces | region/workspace keys, separate plans, stored Responses lifecycle | later mapping |

## Decision

Select one Kimi Platform K3 text-only structured direct run plus its
authenticated model catalogue. This is Kimi Open Platform API-key,
pay-as-you-go access at `api.moonshot.ai`, not Kimi Membership or Kimi Code
access. No provider, model, endpoint, key, reasoning mode, output bound, retry,
or fallback becomes implicit.

The first run pins `kimi-k3`, requires an explicit `low`, `high`, or `max`
reasoning selection and positive output-token bound, uses one streaming Chat
Completions attempt, preserves reasoning progress separately from final output,
and validates the returned model. It excludes tools, structured output,
multimodal input, files, batch, balance, official tools, web search, Partial
Mode, provider state, automatic reconnection, and retry.

Promote the shared boundary in Contract 024. Roadmap 028 owns the common codec,
Kimi corpus, production driver, and conformance. DeepSeek, Z.AI, and Alibaba
remain later provider adapters over the same structural codec only after their
own dated corpora prove the exact semantic mapping.
