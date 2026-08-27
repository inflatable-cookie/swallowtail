# 227 xAI Responses WebSocket Web Search Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.080 / 222

## Question

Which exact `xai.responses-websocket` model and operation profiles can dispatch
one bounded provider-owned `web_search` tool through explicit external-search
policy while preserving exact WebSocket response, citation, usage, billed-cost,
terminal, continuation, and cleanup truth?

## Method And Boundary

Official xAI documentation and the public OpenAPI document were retrieved on
2026-08-27. Retrieval was read-only and used no API key, account, catalogue,
prompt, provider request, paid search, or ambient configuration mutation. The
digestable docs corpus is each page's `.md` export. HTML bodies are Next.js
shells and are corroboration only. The OpenAPI document is the schema source
for request and completed-response fields.

The selected operation remains only `xai.responses-websocket`:

- driver `swallowtail.xai.websocket`;
- facade axis `xai.responses-websocket-facade`;
- exact facade `xai-responses-websocket-2026-04-23`;
- public API-key access;
- one-response structured run and serial connection-local interactive session;
- selected model ids `grok-4.5` and `grok-4.6`;
- `store=false`;
- current request body always includes `tools: []`.

Research 187 remains the exact reasoning and output-control source. Research
169 and the realtime prepared guide remain the search-free WebSocket lifecycle
source. Current documentation is a lead until frozen below. Adapter
implementation and fixtures were inspected and not changed.

No live provider operation, credential work, account inspection, or paid
search was used. Specimens below are secret-free documentation-shape sketches,
not captured provider responses.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 (bytes) |
| --- | --- | --- | --- |
| [WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode.md) | `response.create` uses the Responses create body minus `stream` and `background`; serial `previous_response_id`; `store=false`; event types/order match Responses streaming; first/follow-up specimens still send `tools: []` | 2026-08-27 | `b7697dc824320a4a603b6cdddbc145a1e26501698fcccb369c41d4f2b795ee06` (7684) |
| [Web search](https://docs.x.ai/developers/tools/web-search.md) | HTTP/Responses `tools: [{"type":"web_search"}]`; `grok-4.6` examples; optional filters and image switches | 2026-08-27 | `56ebf56c2cb292d485ccb692e9e8e33e1f06a7a87054bb609cc071f155a75d0a` (8635) |
| [Tool usage details](https://docs.x.ai/developers/tools/tool-usage-details.md) | server-side outputs not returned; Responses `web_search_call` items; `max_turns` turn bound; billing from successful `server_side_tool_usage` | 2026-08-27 | `9083de967676fa504b9ce45afe00daa5c80ff332bea09b9fe4b5422b75fafc08` (7998) |
| [Citations](https://docs.x.ai/developers/tools/citations.md) | Responses inline `[[N]](url)` default; `include: ["no_inline_citations"]` opt-out; `url_citation` annotations | 2026-08-27 | `bd31aa8b03b24e37389332318a083cef7c29ed5a33c319616af3714d87d3585d` (16918) |
| [Tools overview](https://docs.x.ai/developers/tools/overview.md) | built-in tools execute on xAI; HTTP `stream: true` examples listen only for `response.output_text.delta` | 2026-08-27 | `6a2ea3461dfe52c8aa12bf56aa5e815a735f4ab01b1ca18fe1bcbb8499cc0037` (5416) |
| [Grok 4.6](https://docs.x.ai/developers/grok-4-6.md) | exact id `grok-4.6`; Tools list includes web search | 2026-08-27 | `8dd4b88db17c741deb20f3a77077ed52dc6e87383c962f24212e9453850be908` (4350) |
| [Grok 4.6 model page](https://docs.x.ai/developers/models/grok-4.6.md) | capabilities list function calling, structured outputs, and reasoning only | 2026-08-27 | `6a4b7ae9823a3f7121dfe7e7e05fe1aa9da52714b2757449edc94b06565855f0` (810) |
| [Grok 4.5 model page](https://docs.x.ai/developers/models/grok-4.5.md) | exact id `grok-4.5`; no web-search capability claim | 2026-08-27 | `c82ac26b16887f1b1f947c1936a9f032bf921664ca5fdc16efcbb4f0bde5603b` (987) |
| [Models](https://docs.x.ai/developers/models.md) | realtime data needs search tools; aliases are not exact ids | 2026-08-27 | `d5a6e7caf787f2ffb4628ecb7314f5b4c15ae8e18af3550e510a3c4937167c17` (4386) |
| [Responses reference](https://docs.x.ai/developers/rest-api-reference/inference/chat.md) | `max_turns` on create; `max_tool_calls` on the response object; `tools` currently described as functions and web search | 2026-08-27 | `f38e8b59ae868969102bc0ca07fdcea6897d099fcf7d0e6b66b3e8e66cdf344c` (47229) |
| [Pricing](https://docs.x.ai/developers/pricing.md) | web search billed per successful invocation at $5 / 1k calls plus tokens | 2026-08-27 | `6232258ec935eab707481b9037881b46a9fed9f6921e7a84aec97d5b7bf47727` (8765) |
| [Streaming](https://docs.x.ai/developers/model-capabilities/text/streaming.md) | HTTP SSE chat-completions streaming only; no Responses `web_search_call` events | 2026-08-27 | `caace5bb5b0889b705e276c2c27c5812645ca467454fe944f49da004852184d9` (4937) |
| [OpenAPI](https://docs.x.ai/openapi.json) | `ModelRequest.tools` / `max_turns`; `ModelTool` `web_search`; `WebSearchCall`; `ModelUsage`; no WebSocket path and no search streaming-event schemas | 2026-08-27 | `7df26eaf5d7cfe7ed2aff4cfaf8acf05f1936937224804988f2513be2aa826c4` (218811) |

HTML shells for the same URLs without `.md` were retrieved the same day and
are not the hashed corpus. Research 187's 2026-08-22 HTML digests are stale
relative to this retrieval and are not reused.

## Frozen Official Semantics

### WebSocket composition

The WebSocket guide still states that every turn is a `response.create` whose
body is the Responses create body minus transport-only `stream` and
`background`. Responses are always streamed as socket events. Event types and
ordering are identical to the existing Responses streaming format. One
connection is serial. `store=false` continuation is connection-local
`previous_response_id`. A failed turn evicts that id.

That composition rule is exact for fields that already belong to the qualified
search-free body. It does not by itself freeze a web-search request or the
search event subset. Both official WebSocket specimens still send `"tools": []`.
The follow-up specimen adds `function_call_output` items, which are client-tool
results, not provider-owned search.

HTTP/Responses web-search examples use `tools: [{"type":"web_search"}]` on
`POST /v1/responses` with `grok-4.6`. OpenAPI `ModelTool` admits that object
with only `type` required. Optional filter, image, location, and
`external_web_access` fields exist; `external_web_access`,
`search_context_size`, and `user_location` are documented as OpenAI-compat
fields that reject the request if set. The smallest candidate tool object is
therefore `{"type":"web_search"}` with no other keys.

There is no official WebSocket specimen that combines `response.create` with
that tool object, `max_turns`, citations, or search output items. Independent
HTTP tool support plus the body-equivalence sentence is not route proof.

### Model and profile membership

| Exact model | Official search claim | Disposition |
| --- | --- | --- |
| `grok-4.6` | Get-started page lists web search among Tools; every retrieved web-search example uses this id | candidate only; not deliver-now |
| `grok-4.5` | Model page lists function calling, structured outputs, and reasoning; no web-search claim and no example | withheld |
| aliases (`grok-4.5-latest`, `grok-build-latest`, others) | aliases remain distinct names | withheld |
| any other model id | no exact Research 187 or 227 qualification | withheld |

The Grok 4.6 model-detail page omits web search from its capabilities list
while the get-started page includes it. That discrepancy is recorded; it is
not flattened into a family-wide claim and is not enough to deliver.

Structured run, first session turn, continuation turn, and fresh restoration
must be classified independently. All four use the same `response.create`
encoder today, with continuation adding only `previous_response_id` and
restoration dropping that id. Official search material does not prove those
four paths separately. A later binding would have to resend the same immutable
tool object on every eligible turn because the current encoder always includes
`tools`. That is a candidate encoding rule, not proof that search continuation
or restoration preserves citation and terminal truth.

### Provider-side bound

OpenAPI `ModelRequest.max_turns` is `integer | null`, format `int32`:
"Maximum number of agentic tool calling turns allowed for this request. If not
set, defaults to the server's global cap. This parameter will be ignored for
any non-agentic requests."

Tool-usage details: `max_turns` limits assistant/tool-call turns, not
individual tool calls. One turn may invoke multiple tools in parallel. Web
search usage category `SERVER_SIDE_TOOL_WEB_SEARCH` covers `web_search`,
`web_search_with_snippets`, `browse_page`, `open_page`, and
`open_page_with_find`. At the bound, the agent stops further tool calls and
writes a final response from information already gathered. Recommended quick
lookups are 1-2 turns.

`max_tool_calls` is a response-object field, not a create-body field. It is
not a request bound.

A fixed `max_turns: 1` is the smallest documented positive turn bound. Its
signed-int32 domain matches Research 187's `max_output_tokens` upper bound.
The global default cap is unnamed. Overflow, zero, and interaction with
`reasoning` / `max_output_tokens` are not documented. The field does not cap
search queries inside one turn. That is an exact candidate disposition, not a
closed deliver-now bound for this WebSocket route.

Omission of `max_turns` on a search-enabled request would accept the unnamed
server cap. Current search-free requests omit the field, which is ignored for
non-agentic bodies. Search-free omission must stay byte-equivalent with
`tools: []` and no `max_turns`.

### Filters, include, and other tools

Caller `allowed_domains` / `excluded_domains` (max 5, mutually exclusive),
`enable_image_search`, and `enable_image_understanding` are withheld.
`search_parameters` is the older live-search surface and is withheld.
X search, code execution, file/collection search, MCP, functions, and consumer
tools are withheld.

Responses inline citations default to on. Opt-out is
`include: ["no_inline_citations"]`. OpenAPI `include` is an open string array
named as `reasoning.encrypted_content` plus unspecified "tool-output options".
The closed include vocabulary for WebSocket is not frozen. A later binding
must not send `include` unless that list is closed. Default inline markdown in
assistant text is not the same as bounded portable citation delivery.

### Response grammar, citations, usage, billing, failure

OpenAPI `ModelOutput` admits `WebSearchCall` items with required `type` and
`action`. Action variants are `search` (query, optional sources), `open_page`
(url), and `find_in_page` (url, pattern). Status is an unconstrained string.
Tool-usage details say server-side tool outputs are not returned; the agent
uses them internally. Responses represent activity as `web_search_call` output
items, distinct from `function_call`.

Citations page: `response.citations` is an xAI SDK chat attribute. Responses
API inline citations are `[[N]](url)` in `output_text` plus `url_citation`
annotations with url and optional start/end/title. Enabling citations does not
force the model to cite. Image embeds require withheld `enable_image_search`.

`ModelUsage` requires token counts plus `num_server_side_tools_used` and
`num_sources_used`. Optional `server_side_tool_usage_details.web_search_calls`
and `cost_in_usd_ticks` exist. Pricing bills successful web-search invocations
separately from tokens. Failed attempts are not billed. Current route mapping
reads `input_tokens`, `output_tokens`, `total_tokens`, and `cost_in_usd_ticks`
from `response.completed` and ignores tool-usage details.

OpenAPI contains no WebSocket path and no `response.web_search_call.*` or
`response.output_item.*` event schemas. The streaming guide documents HTTP
chat-completions SSE only. Tools-overview Responses streaming examples handle
`response.output_text.delta` only. The current adapter fail-closes on unknown
events and requires `response.completed.output` to be exactly one assistant
`message`. A `web_search_call` item in that array is already malformed under
the qualified parser. Unknown search-progress events would fail the same way.

That is enough to keep dispatch, acceptance, invocation, result delivery,
citation delivery, usage, billing, and terminal status conceptually separate.
It is not enough to freeze the composed WebSocket event sequence, search-item
ordering versus text, citation annotation presence on streaming frames,
failure/quota/tool-drift shapes, or billed-search mapping for this route.

A model declining to search is ordinary provider choice. That fact cannot be
proved without a live turn and is not adapter failure in any later binding.

## Production Seam Audit

Inspected and unchanged:

- `crates/swallowtail-adapter-xai/src/protocol/request.rs` always emits
  `tools: []`. Reasoning and `max_output_tokens` are the only additive body
  fields. `max_turns` is absent.
- structured-run policy requires `ExternalSearchPolicy::Disabled` and
  `ExternalNetworkPolicy::Denied`, and rejects consumer tools before endpoint
  work.
- session open rejects consumer tools in session options. It does not itself
  inspect `ExternalSearchPolicy`; prepared sessions are resource-free from the
  plan and inherit the search-free offline policy.
- decoder allowlist is `response.created`, `response.in_progress`,
  `response.output_text.delta`, `response.output_text.done`,
  `response.completed`, and `error`. Unknown events fail closed.
- `completed_output` requires `output.len() == 1` and `type == "message"`.
- activity profile is `AssistantMessage` / `FinalAnswerText` /
  `FailClosed`.
- fixtures `xai-responses-websocket-2026-04-23` explicitly exclude tools and
  search. Completed fixtures carry `"annotations":[]` on a single message.
- prepared evidence carries reasoning and output-token selection only.
- realtime guide rejects the route when the application needs tools.

Omission therefore remains the current canonical body. Enabled search cannot
be added without a parser, completed-output, activity, policy, and fixture
revision that the frozen event grammar does not authorize.

## Secret-Free Specimens

These are non-live sketches. They are not admitted requests or captured
provider frames. Digests cover the compact JSON shown, with no trailing
newline.

### S1 — candidate WebSocket create shape

```json
{"type":"response.create","model":"grok-4.6","store":false,"input":[{"type":"message","role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"tools":[{"type":"web_search"}],"max_turns":1}
```

### S2 — current qualified omission

```json
{"type":"response.create","model":"grok-4.6","store":false,"input":[{"type":"message","role":"user","content":[{"type":"input_text","text":"<user-input>"}]}],"tools":[]}
```

### S3 — documented completed search-item shape

```json
{"type":"web_search_call","id":"ws_example","status":"completed","action":{"type":"search","query":"<omitted>","sources":[]}}
```

Query text, source contents, and raw provider bodies remain excluded from any
portable activity surface.

### S4 — documented citation annotation shape

```json
{"type":"url_citation","url":"https://example.invalid/source","start_index":0,"end_index":1,"title":"1"}
```

### S5 — documented Responses text-delta event name

```json
{"type":"response.output_text.delta","delta":"<assistant-output>"}
```

S5 shows the only search-related streaming event name frozen by official
Responses examples. It does not prove search-item events.

| Specimen | SHA-256 |
| --- | --- |
| S1 | `9dca1079ea179b0a94f67926c1a22c01581b2ad337e5bb7dd52dcd54947c6af8` |
| S2 | `7405b3472a05c6289155dec7864b736d5765166d8991378821945da2ce4849f9` |
| S3 | `e363adfcc8b0cb07780133560d7c9bde6ab8050e5e52647fab345b5619df16f7` |
| S4 | `5870e45090819c95cd6c7403eca1fa8eb94b36cd7e410ef86a64f96d2ab29729` |
| S5 | `2946492c3ef6935e558ac15ce7fa66e7dade23224ca56b004697548c052de538` |

## Compatibility Classification

| Combination or truth | Disposition | Reason |
| --- | --- | --- |
| `grok-4.6` structured run with `tools: [{"type":"web_search"}]` and `max_turns: 1` | evidence-gated | HTTP tool object and WS body-equivalence are documented; composed WebSocket request, search events, completed mixed output, and citation/terminal mapping are not. |
| `grok-4.6` first session turn with the same body | evidence-gated | Same request sketch; no independent session-turn proof. |
| `grok-4.6` continuation with `previous_response_id` and resent `web_search` | evidence-gated | WS follow-up specimen is client-tool `function_call_output`, not server-side search. |
| `grok-4.6` fresh restoration with search | evidence-gated | Restoration drops continuation and resends input; search-item continuity is unfrozen. |
| `grok-4.6` plus Research 187 reasoning and/or `max_output_tokens` plus search | evidence-gated | Controls are independent on the search-free route; composed search interaction is not documented. |
| `grok-4.5` any search row | withheld | No exact model-page or example claim. |
| Aliases and other model ids | withheld | Not exact route identities. |
| Search absent, `tools: []` | unchanged | Current qualified body, parser, activity, usage, billed cost, continuation, restoration, cancellation, deadline, invalidation, and cleanup remain the search-free claim. |
| `max_turns: 1` as the smallest positive request bound | evidence-gated candidate | Field and int32 format are exact; unnamed default cap, intra-turn query count, overflow, and WS effectiveness are not. |
| `max_tool_calls` as a request bound | withheld | Response field, not create-body. |
| Filters, image search/understanding, `include`, `search_parameters`, `tool_choice`, `external_web_access` | withheld | Not required for the smallest row; several compat fields reject if set. |
| X/image/code/file/MCP/function/consumer tools | withheld or not applicable | Outside the named candidate. |
| Account/organization search enablement | evidence-gated | No live account inspection is authorized. Public model support is not access qualification. |
| Host networking or consumer tool-result port | not applicable | Provider-owned search must keep `ExternalNetworkPolicy::Denied`. |

## Candidate Disposition

Deliver-now rows: none.

The public vocabulary contains one typed provider-owned tool,
`{"type":"web_search"}`, a positive int32 turn bound `max_turns`, and
completed-response item/citation/usage fields. Contracts 037, 041, and 044
already express enabled search intent, denied host networking, and bounded
activity. No shared-contract change is required to explain the stop.

The missing proof is the exact composed WebSocket event grammar and the
route-local parser/activity mapping that would have to consume it. The current
fail-closed decoder cannot admit `web_search_call` items or unknown search
events without inventing that grammar. Live provider work is out of scope, so
the set stays empty.

The opaque facade point `xai-responses-websocket-2026-04-23` remains unchanged.
No private behavior revision is assigned because no additive behavior is
admitted. Omission retains exact `tools: []` bytes.

## Decision

Card 222 is complete as an evidence stop. Cards 223 and 224 are blocked and
were not executed. The xAI adapter, fixtures, guide, matrices, changelog, and
API baseline remain unchanged by this lane.
