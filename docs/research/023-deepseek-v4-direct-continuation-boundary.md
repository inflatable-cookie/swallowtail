# 023 DeepSeek V4 Direct Continuation Boundary

Status: promoted
Owner: Tom
Updated: 2026-07-22

## Question

What exact DeepSeek V4 route should prove direct tool continuation, and what
shared boundary must exist before implementation?

## Method

Official DeepSeek API, guide, change-log, pricing, rate, cache, error, and
privacy material was accessed on 2026-07-22. No account, API key, balance
request, paid inference, external mutation, SDK installation, or live model
call was used.

The audit compares DeepSeek's OpenAI and Anthropic facades against Contracts
006, 008-011, 014, 020, 024, 025, and 029. The target is integration
mechanism, not a consumer default or general provider router.

## Current Surface

DeepSeek exposes exact `deepseek-v4-flash` and `deepseek-v4-pro` routes at
`https://api.deepseek.com`. Both support thinking and non-thinking modes,
tools, a 1M context window, and a documented maximum 384K output. The
authenticated `GET /models` surface reports both ids.

The mutable `deepseek-chat` and `deepseek-reasoner` aliases map to V4 Flash
during transition and retire on 2026-07-24 at 15:59 UTC. They are not eligible
qualification points.

Access uses one DeepSeek Open Platform bearer API key. Usage draws from granted
or topped-up balance. HTTP 401, 402, 429, 500, and 503 evidence keeps
authentication, balance, account concurrency, provider failure, and overload
separate. The current account concurrency observation is 500 for V4 Pro and
2500 for V4 Flash. No remaining-capacity or reset evidence is documented.

Evidence:

- [API start](https://api-docs.deepseek.com/)
- [change log](https://api-docs.deepseek.com/updates/)
- [models and pricing](https://api-docs.deepseek.com/quick_start/pricing/)
- [model catalogue](https://api-docs.deepseek.com/api/list-models/)
- [error codes](https://api-docs.deepseek.com/quick_start/error_codes/)
- [rate and isolation](https://api-docs.deepseek.com/quick_start/rate_limit/)

## Facade Decision

Select the OpenAI-format `POST /chat/completions` facade. Do not append `/v1`
in the first configured endpoint.

The Anthropic facade is provider-supported but unsuitable for the first exact
proof. It automatically maps unsupported model names to V4 Flash, maps Claude
model prefixes, and ignores multiple headers and request fields. Those
behaviors conflict with exact route, version, and unsupported-input failure.
They may support a later separately qualified driver or facade revision.

The OpenAI facade already fits the bounded structural codec. Reuse stops at
JSON and SSE structure. DeepSeek retains all request-field, continuation,
reasoning, model, error, cache, usage, and lifecycle authority.

Evidence:

- [Anthropic compatibility and model mapping](https://api-docs.deepseek.com/guides/anthropic_api/)
- [OpenAI-format Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)

## Model Decision

Pin `deepseek-v4-pro` for the first corpus and driver. This is a fixture and
qualification anchor, not a consumer default.

Official thinking/tool continuation examples use V4 Pro. It exercises the
strongest documented route while avoiding the retiring aliases. V4 Flash has
the same advertised high-level mechanism but remains a separate model route;
it needs its own returned-model, usage, limit, and corpus qualification before
the driver claims it.

## Continuation Pressure

Thinking mode returns `reasoning_content` beside assistant content and tool
calls. When an assistant message performs a tool call, DeepSeek requires that
reasoning content and the full assistant message on later requests. Omission
causes HTTP 400. The requirement continues into later user turns, not only the
immediate tool-result request.

This is not consumer-visible reasoning and not provider conversation storage.
It is provider-private continuation material needed to reconstruct later
stateless requests. A one-attempt structured run cannot carry the complete
lifecycle. A connection-scoped session is also wrong: each attempt uses a new
HTTP request and the continuation survives connection close.

The correct operation is a resource-free `InteractiveSession` over
`DirectModelInference` with adapter-private local history. The consumer starts
user turns, declares tools, executes tool calls, supplies correlated results,
and explicitly authorizes every further inference attempt. Swallowtail stores
and replays only provider-required private fields for that scoped session. It
does not choose a tool, execute it, or run an automatic agent loop.

Evidence:

- [thinking mode and required replay](https://api-docs.deepseek.com/guides/thinking_mode/)
- [tool calls and consumer execution](https://api-docs.deepseek.com/guides/tool_calls/)

## Field Ownership And Redaction

| Field or state | Owner and visibility | Rule |
| --- | --- | --- |
| system/user content | consumer-owned, transport-private | supplied explicitly; redacted from diagnostics |
| tool declarations | consumer-owned | bounded schemas; no provider or Swallowtail tool injection |
| assistant text | provider output, consumer-visible | ordered normalized event and final output evidence |
| tool call id, name, arguments | provider output, consumer-visible | bounded and correlated; consumer validates and executes |
| tool result | consumer-owned, transport-private | exact call correlation; redacted from diagnostics |
| `reasoning_content` | provider-private, adapter-held | never a public event, result, callback payload, diagnostic, or serialized binding |
| exact assistant replay envelope | adapter-private mechanism | replay only to the same session, facade, route, model, and access binding |
| attempt usage and cache token counts | provider evidence, consumer-visible | typed per-attempt observations; never retry authority |
| context-cache contents | provider-owned and unavailable | no observation, read, delete, or persistence claim |

Provider-private continuation uses bounded zeroizing memory. Session close,
failure, cancellation, deadline, route mismatch, or cleanup invalidates it.
There is no resume, export, durable serialization, cross-provider use, or
consumer transcript substitution in the first proof.

## Request Semantics

The first proof sends explicit `thinking.type=enabled`,
`reasoning_effort=high`, `max_tokens`, `stream`, exact model, messages, and
tools where applicable. It sends no temperature, top-p, penalty, logprob,
JSON-output, prefix, FIM, or beta strict-mode fields.

Official DeepSeek pages currently conflict on `tool_choice`: the main API
reference documents it, while the official Oh My Pi integration guide says V4
thinking mode rejects it. The first request therefore expresses provider-
automatic tool selection in the Swallowtail plan but omits the wire field. The
adapter does not silently inherit policy; omission is the frozen mapping.

The first tool-bearing attempt is non-streaming. Official guides freeze a
complete assistant message with reasoning and tool calls, but do not provide a
complete streamed tool-call assembly contract. Final-answer attempts use SSE,
including keepalive comments, terminal usage, and `[DONE]`.

Evidence:

- [Chat Completions fields and streaming](https://api-docs.deepseek.com/api/create-chat-completion/)
- [official Oh My Pi compatibility notes](https://api-docs.deepseek.com/quick_start/agent_integrations/oh_my_pi/)

## Bounded First Proof

- exact dated facade revision `deepseek-openai-chat-2026-07-22`
- exact endpoint `https://api.deepseek.com`; no `/v1`, beta, Anthropic, proxy,
  gateway, or alternate-provider facade
- one provider-supported Open Platform API-key lease and usage-billed balance
- authenticated `GET /models` observation plus exact V4 Pro route
- resource-free direct interactive session; no working directory
- maximum two user turns, three provider inference attempts, eight declared
  functions, one returned tool call, and one correlated tool result
- first tool-bearing attempt non-streaming; later final attempts streaming
- maximum 8,192 output tokens per attempt, 64 KiB tool arguments, 64 KiB tool
  result, 256 KiB private continuation field, 1 MiB private session history or
  encoded record, and 4,096 SSE records per attempt
- one active turn and one active provider request
- one host-monotonic deadline per turn, including tool-result wait and every
  authorized attempt in that turn
- no retry, reconnection, fallback, automatic tool execution, or hidden next
  attempt

The exact fixture proves one tool call and result in turn one, a final answer,
then a second user turn whose request still contains the required private
continuation. A second tool call or a fourth provider attempt fails the
session rather than widening the loop.

## Cache And Privacy Posture

DeepSeek context caching is enabled by default. Request boundaries can persist
cache prefixes to disk; unused entries usually clear within hours to days.
Cache hits are best-effort and reported through hit and miss token counts.
The selected API exposes no first-proof cache deletion authority.

The route must therefore require explicit acceptance of provider-managed
cache and data-processing posture. This is separate from local private
continuation, consumer transcript storage, provider conversation retention,
and response storage. Swallowtail reports the posture; the consumer decides
whether the route is acceptable. It cannot be an implicit fallback.

DeepSeek's current privacy policy also places provider data processing and
cross-border disclosure outside Swallowtail's authority. No library capability
claim replaces the consumer's privacy notice or route policy.

Evidence:

- [context caching](https://api-docs.deepseek.com/guides/kv_cache/)
- [current privacy policy](https://cdn.deepseek.com/policies/en-US/deepseek-privacy-policy.html?locale=en_US)

## Failure And Cleanup

- HTTP 400 continuation failure remains invalid provider request, not auth or
  retry evidence
- 401 is credential rejection; 402 is insufficient balance; 429 is account
  concurrency; 500 and 503 are provider failure or overload
- `length`, `content_filter`, `tool_calls`, and
  `insufficient_system_resource` remain distinct finish evidence
- local cancellation or deadline closes and joins the active HTTP work but
  cannot claim remote inference stop
- cancellation, deadline, disconnect, malformed continuation, overflow, model
  mismatch, and unknown semantic fields end the session in the first proof
- close abandons pending tool input, joins network work, zeroizes private
  history, then releases endpoint and credential authority

## Promotion

- durable boundary: Contract 030
- conformance: add a twelfth locally continued direct-session profile rather
  than weaken hosted-direct, connection-scoped, or provider-conversation
  profiles
- records and corpus: card 103
- production driver and closeout: card 104
- V4 Flash, Anthropic facade, streamed tool-call assembly, strict beta tools,
  multiple tool rounds, persistent resume, and live auth remain later work

