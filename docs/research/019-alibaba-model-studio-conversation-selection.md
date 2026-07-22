# 019 Alibaba Model Studio Conversation Selection

Status: promoted
Owner: Tom
Updated: 2026-07-22

## Question

Which remaining direct provider adds the most architectural information after
the Kimi Platform proof without treating compatible syntax, a coding plan,
region choice, workspace authority, or provider retention as implicit?

## Method

Official DeepSeek, Z.AI, and Alibaba Cloud documentation was checked on
2026-07-22 against Swallowtail's realized direct, session, retention,
catalogue, deletion, and compatible-codec boundaries. No credential, account,
workspace, provider request, paid inference, or live catalogue was used.

## DeepSeek Currentness

DeepSeek now exposes exact `deepseek-v4-pro` and `deepseek-v4-flash` routes
through OpenAI Chat Completions and Anthropic-compatible APIs. The legacy
`deepseek-chat` and `deepseek-reasoner` aliases still map to V4 Flash but retire
on 2026-07-24 at 15:59 UTC. A supported `GET /models` currently reports the two
exact V4 ids.

The native Chat Completions route has useful semantic pressure. Thinking
defaults on, `high` and `max` are the native effort values, compatible `low`
and `medium` map to `high`, and `xhigh` maps to `max`. In thinking mode,
temperature and penalty fields are accepted but ignored. A future adapter must
construct only its frozen subset, reject unsupported consumer inputs before
effects, and pin an exact V4 model rather than accept a retiring alias or
provider mapping.

DeepSeek remains a stateless HTTP/SSE direct route over the compatible-chat
codec. Its cache-hit, cache-miss, reasoning-token, model-catalogue, balance,
keepalive, and failure evidence remain worthwhile later, but they do not add a
new lifecycle after Kimi Platform.

Evidence, accessed 2026-07-22:

- [DeepSeek API start](https://api-docs.deepseek.com/)
- [DeepSeek V4 change log](https://api-docs.deepseek.com/updates/)
- [Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Thinking mode](https://api-docs.deepseek.com/guides/thinking_mode/)
- [Model list](https://api-docs.deepseek.com/api/list-models/)
- [Error codes](https://api-docs.deepseek.com/quick_start/error_codes/)

## Z.AI Currentness

Z.AI's general API currently uses bearer API keys at
`https://api.z.ai/api/paas/v4` and names `glm-5.1` as the latest flagship
route. Chat Completions supports standard SSE ending in `[DONE]`, explicit
thinking enablement, reasoning deltas, usage, cached tokens, tools, structured
output, and provider business errors. Preserved thinking is provider-specific:
`thinking.clear_thinking=false` requires complete unmodified historical
reasoning blocks.

The GLM Coding Plan remains a separate subscription audience at the coding or
Anthropic endpoints. Its policy limits subscription benefits to supported
tools and explicitly warns against SDK or unsupported third-party integration
use. A Swallowtail direct driver may use only the general API unless the
provider later supports its product audience explicitly.

The current first-party Python and Java SDKs add no Rust-native SDK boundary.
The bounded general route would therefore be another compatible HTTP/SSE
mapping with distinct reasoning and error semantics, not a new transport or
lifecycle.

Evidence, accessed 2026-07-22:

- [Z.AI quick start](https://docs.z.ai/guides/overview/quick-start)
- [Chat Completion](https://docs.z.ai/api-reference/llm/chat-completion)
- [GLM-5.1](https://docs.z.ai/guides/llm/glm-5.1)
- [Streaming messages](https://docs.z.ai/guides/capabilities/streaming)
- [Coding Plan usage policy](https://docs.z.ai/devpack/usage-policy)

## Alibaba Model Studio Currentness

Alibaba Model Studio now recommends workspace-dedicated production domains.
Each region has its own endpoint, key, and model list. A workspace-dedicated
key is scoped to one user, one workspace, and one region, and inherits callable
functions and throttling from workspace permissions. Those identities cannot
be replaced by a generic DashScope key or another regional endpoint.

The Singapore workspace endpoint
`https://{WorkspaceId}.ap-southeast-1.maas.aliyuncs.com` has one International
deployment scope and supports exact model `qwen3.7-plus-2026-05-26` through
OpenAI-compatible Responses and Conversations APIs. Standard model calls are
pay-as-you-go by default. The separate Model Studio Coding Plan uses a plan key
and plan endpoint and prohibits custom application backends, automated scripts,
and non-interactive batch use. It is not a Swallowtail direct credential.

Responses is synchronous only; `background` is unsupported. Streaming carries
provider sequence numbers, reasoning summaries, output deltas, completed
output, terminal usage, and `response.completed`. Only documented parameters
are processed; other OpenAI parameters are ignored. The first driver must
therefore construct a strict provider-owned request and reject unsupported
inputs before network or credential work.

Responses can bind one provider conversation. Completed input and output are
added to that conversation automatically. The separate Conversations API can
create, retrieve, list items, delete individual items, and delete the
conversation. Deleting a conversation explicitly does not delete its message
items. That makes item and conversation deletion separate truth, not one
cleanup flag.

Evidence, accessed 2026-07-22:

- [Regions and access domains](https://help.aliyun.com/en/model-studio/regions/)
- [Workspace permissions](https://help.aliyun.com/en/model-studio/permission-management-overview)
- [Responses API](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)
- [Conversations API](https://help.aliyun.com/en/model-studio/openai-compatible-conversations)
- [Response deletion](https://help.aliyun.com/en/model-studio/delete-a-response)
- [Model pricing](https://help.aliyun.com/en/model-studio/model-pricing)
- [Coding Plan FAQ](https://help.aliyun.com/en/model-studio/coding-plan-faq)

## Comparison

| Candidate | New information | Main pressure | Result |
| --- | --- | --- | --- |
| Alibaba Model Studio Conversations + Responses | region/workspace-bound access, direct interactive provider state, item inventory, separate deletion truth | explicit durable retention, session policy, deletion races, strict ignored-field table | select |
| DeepSeek V4 | exact retiring-alias boundary, model catalogue, cache and reasoning usage | compatible effort mapping and ignored fields | later stateless mapping |
| Z.AI GLM-5.1 general API | current GLM reasoning, preserved-thinking rules, business errors | general versus Coding Plan authority | later stateless mapping |

## Decision

Select one resource-free direct-inference interactive session over Alibaba
Model Studio's Singapore workspace-dedicated Conversations and Responses APIs.
This is a representative proof route, not a default region, provider, model,
endpoint, workspace, or billing choice.

The first subset binds:

- one opaque operator-approved Singapore workspace endpoint and exact
  workspace/region audience
- one general Model Studio API key with pay-as-you-go metering and provider
  support authority
- exact route `qwen3.7-plus-2026-05-26`; no alias or model fallback
- one driver-owned provider conversation created when the local session opens
- two maximum serial text turns through synchronous streaming Responses
- explicit `conversation`, `stream=true`, `store=false`,
  `reasoning.effort=none`, no tools, and no session-cache header
- provider sequence, text, completed-output, returned-model, usage, request,
  failure, and unknown-event validation
- bounded item listing, deletion of every discovered conversation item, then
  conversation deletion before credential release

The subset excludes model catalogue, DashScope legacy domains, trial domains,
other regions or workspaces, deployment-scope choice, Coding Plan, Token Plan,
savings-plan policy, aliases, previous-response continuation, response
storage/retrieval/deletion, Conversations resume, metadata mutation, item
creation outside turns, built-in or custom tools, files, multimodal input,
search, MCP, code execution, session cache, background execution, retry,
reattachment, and fallback.

## Required Promotion

Contract 025 must settle:

- direct interactive sessions over request-scoped HTTP/SSE rather than one
  persistent connection
- explicit provider-conversation retention and delete-on-close policy at
  session open
- runtime session, provider conversation, provider item, provider response,
  turn, stream, workspace, region, route, and model identity
- driver-owned remote conversation and aggregate item deletion truth
- complete bounded item inventory before conversation deletion
- local-only cancellation when no native response cancel is documented
- remote turn and cleanup races without claiming confirmed deletion
- session-scoped endpoint and credential lifetime through remote cleanup
- no resume, response storage, context cache, retry, or fallback in the first
  proof

No provisional product spec is required. The representative Singapore route
selects no consumer default, data-residency policy, or routing preference.

## Promotion

- provider-owned direct conversation boundary: Contract 025
- implementation sequence: g01 roadmap 029 and cards 088-089
- deferred compatible-chat routes: DeepSeek V4 and Z.AI GLM-5.1 general API
