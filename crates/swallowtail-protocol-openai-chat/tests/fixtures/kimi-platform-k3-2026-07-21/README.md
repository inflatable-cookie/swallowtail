# Kimi Platform K3 Chat Fixture

Synthetic wire evidence for the Kimi Open Platform K3 direct API observed on
2026-07-21. Default QA reads these files only. It does not use an API key,
make a network request, inspect an account, fetch a live catalogue, or perform
paid inference.

## Frozen Boundary

- audience: `https://api.moonshot.ai`
- access: bearer API key issued by `platform.kimi.ai`
- metering: pay as you go
- catalogue: authenticated `GET /v1/models`
- inference: `POST /v1/chat/completions`
- exact model route: `kimi-k3`
- request: one text message, explicit `reasoning_effort`, positive
  `max_completion_tokens`, streaming usage
- lifecycle: one stateless attempt, local connection cancellation only

The catalogue body is synthetic source-scoped evidence. Presence of `kimi-k3`
does not prove account entitlement, top-up, quota, or invocation readiness.

## K3 Rules

K3 always reasons. The dedicated Thinking Effort guide and current API schema
support `low`, `high`, and `max`, with `max` as the default. One older embedded
paragraph on the Chat Completions page still says only `max`; this corpus uses
the dedicated guide and schema and requires the caller to choose explicitly.

K3 fixes `temperature=1.0`, `top_p=0.95`, `n=1`, `presence_penalty=0`, and
`frequency_penalty=0`; the provider says to omit them. This fixture does so.

Kimi Membership, Kimi Code, other regional platform keys, subscription
metering, tools, structured output, multimodal input, files, batch, balance,
official tools, web search, Partial Mode, provider history, automatic
reconnection, retry, and fallback are outside this route.

## Sources

- <https://platform.kimi.ai/docs/guide/kimi-k3-quickstart>
- <https://platform.kimi.ai/docs/guide/use-thinking-effort>
- <https://platform.kimi.ai/docs/api/chat>
- <https://platform.kimi.ai/docs/api/list-models>
- <https://platform.kimi.ai/docs/api/errors>
