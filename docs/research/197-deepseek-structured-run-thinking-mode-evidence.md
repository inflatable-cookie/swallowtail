# 197 DeepSeek Structured-Run Thinking-Mode Evidence

Status: promoted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.050 / 139

## Question

Can exact production route `deepseek.continuation`, model
`deepseek-v4-pro`, and the current OpenAI Chat Completions facade bind explicit
non-thinking mode for one-request structured runs while preserving exact
enabled reasoning and enabled-only direct continuation?

## Evidence Plan

- freeze current official Chat Completions, Thinking Mode, Tool Calls, and
  Models/Pricing pages with retrieval/page dates and specimen digests
- enumerate exact thinking values, omission/default behavior,
  `reasoning_effort` composition, response fields, and unknown-value posture
- freeze current prepared input, plan/evidence, driver, request/response,
  cache, cancellation, deadline, failure, cleanup, and continuation truth
- classify one-request structured runs and direct continuation separately
- distinguish selected, planned, dispatched, accepted, effective, and observed
  thinking states without exposing private reasoning
- choose exact compatibility revisions and publish a deliver-now table or an
  honest empty set

No live/provider acceptance or effective-mode claim is made by this record. No
credential, account/balance inspection, provider request, paid work, or live
DeepSeek call is authorized.

## Frozen Official Evidence

The four official pages were fetched without credentials on 2026-08-23 at
18:36:18 GMT. The response headers reported the following same-day
`Last-Modified` values; no separate publication date was present in the page
body.

| Surface | URL | Last-Modified | ETag | Complete-body SHA-256 |
| --- | --- | --- | --- | --- |
| Chat Completions API | <https://api-docs.deepseek.com/api/create-chat-completion/> | `2026-08-23 01:46:35 GMT` | `6226506ef61a9fb9faeccb6ec6c6b9e8` | `e8fa7d52637f5b652e98468181ce36fa0941248c26e7bc4bf9b4f21493f9269d` |
| Thinking Mode | <https://api-docs.deepseek.com/guides/thinking_mode/> | `2026-08-23 01:46:34 GMT` | `edaf231975de6a3f3182a9abd94e34cc` | `03f7530f0fe66dcfd244d33b87b87bd9e3604126c53877e32f6fb6dadbd7939e` |
| Tool Calls | <https://api-docs.deepseek.com/guides/tool_calls/> | `2026-08-23 01:46:35 GMT` | `70de065904d3e2724b5a72201d640df7` | `8b1ade2f6479b233a78a1acbcf8225bb521af7d03a0d7077137d4e3b7c1800c0` |
| Models and Pricing | <https://api-docs.deepseek.com/quick_start/pricing/> | `2026-08-23 01:46:42 GMT` | `08d1b81ab658e4c3a0d32b8217362826` | `d321546b99bc77060c1716c86228810e84ccfee6c157a3ee5aee5296a3cdec51` |

The current Chat Completions schema names exact model values including
`deepseek-v4-pro`, `thinking.type` values `enabled|disabled`, default
`enabled`, and `reasoning_effort` values `low|high|max`. It describes
`reasoning_content` as thinking-mode-only response content. The Thinking Mode
guide independently lists the OpenAI-format thinking toggle and effort field,
and its V4 Pro examples use the existing explicit enabled/high composition.
The Tool Calls guide has a separate non-thinking V4 Pro example that sends
model, messages, and tools without a reasoning-effort field. The Models and
Pricing page lists V4 Pro on `https://api.deepseek.com` with both non-thinking
and thinking modes and tool calls.

These sources establish the dispatch fields and profile applicability. They do
not prove provider acceptance, effective mode, quality, latency, price, cache
effect, or the behavior of an arbitrary request that combines disabled mode
with an effort field. The admitted adapter subset therefore defines the exact
disabled request as `thinking: {"type":"disabled"}` with
`reasoning_effort` absent. This is a fail-closed local mapping, not a claim
that DeepSeek accepts or rejects every other composition.

## Frozen Repository Evidence

The current route is exact `deepseek.continuation`, driver
`swallowtail.deepseek.direct`, model route/model `deepseek-v4-pro`, endpoint
`https://api.deepseek.com`, path `/chat/completions`, facade
`deepseek-openai-chat-2026-07-22`, and private behavior
`deepseek.v4-thinking-tools-v1`. The existing protocol manifest
`crates/swallowtail-adapter-deepseek/tests/fixtures/deepseek-openai-chat-2026-07-22/protocol.json`
has SHA-256
`04a174652debf22a863885b955aff9da21383b1ebb3a4c0e74eec9ee22330112` and
freezes the three-attempt shape, cache acceptance without read/delete
authority, and `/v1` exclusion.

The source and deterministic tests freeze these boundaries:

- `prepared_profile/input.rs` requires a portable reasoning value for the
  existing run and session constructors; `prepared_profile/plan.rs` retains
  route-local evidence alongside the immutable plan; `selection.rs` puts the
  existing `ReasoningSelection` requirement on both profiles.
- `protocol/request.rs` currently emits the exact enabled/high-style fields;
  `protocol/tests.rs`, prepared-facade tests, and driver tests prove the
  enabled `low|high|max` bytes and pre-network mismatch rejection.
- `protocol/stream.rs` and `protocol/response.rs` keep
  `reasoning_content` adapter-private and bounded. It never becomes a public
  event, terminal output, diagnostic, or durable binding.
- The existing continuation fixture set covers initial tool request, private
  tool response, tool-result request, final stream, later-user request and
  final stream, model/protocol drift, provider failure, cancellation, deadline,
  cleanup, cache, and restoration. Its enabled bytes remain historical proof;
  this lane does not rewrite them.

## Exact Deliver-Now Disposition

| Selection or state | Structured run | Direct continuation | Wire / evidence disposition |
| --- | --- | --- | --- |
| Exact `deepseek-v4-pro` / current facade | Deliver now | Preserve existing path | No model, route, facade, claim, or private-behavior revision change |
| Existing portable `low|high|max` | Deliver now | Deliver now | Keep `ReasoningSelection`, `reasoning_effort` exact, and explicit `thinking.enabled`; existing bytes remain authoritative |
| Adapter-local `DeepSeekThinkingMode::disabled()` | Deliver now | Withhold | Explicit `thinking.type=disabled`; no portable `ReasoningSelection`; omit `reasoning_effort` |
| Input-level omission of the new adapter-local mode | Deliver now | Deliver now | Preserve the existing enabled request and constructors byte-for-byte |
| Disabled mode with a portable effort, `ReasoningMode("none")`, `medium`, `xhigh`, alias, or unknown value | Reject | Reject | Fail before endpoint or credential work; no aliasing or Responses-API spelling |
| Disabled mode with tools, continuation, session restoration, or private replay | Withhold | Withhold | Existing Contract 030 proof remains enabled-only |

For disabled structured runs, the prepared input, immutable evidence, and
configured low-level driver retain the typed adapter-local selection while the
shared request policy and plan carry no `ReasoningSelection`. The request
encoder emits only the exact disabled thinking object and the existing model,
content, stream, output, tools-empty, and cache-safe boundaries. The response
parser accepts ordinary content, usage, finish, model, and request evidence;
it does not infer effective non-thinking from an absent private field. A
non-empty `reasoning_content` under the selected disabled profile is protocol
drift and fails closed without exposing the field.

Cache acceptance remains
`ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority`. Disabled
mode changes no cache read, deletion, retention, billing, retry, endpoint,
credential, cancellation, deadline, or cleanup claim.

## Compatibility Verdict

The current dated OpenAI Chat Completions facade and
`deepseek.v4-thinking-tools-v1` private behavior remain sufficient. No new
facade point, claim revision, model-route revision, Contract 029 currentness
change, shared capability, portable reasoning value, or Contract 030 change is
needed. Cards 140-141 may bind and accept only the one disabled structured-run
row above, then leave the shared closeout delta for the orchestrator.

## Implementation Evidence

Cards 140-141 implemented the admitted row without changing the route,
facade, shared capability vocabulary, or continuation boundary. The public
`DeepSeekThinkingMode::disabled()` type is available only through the
structured-run input constructor. Prepared evidence retains that selection;
the disabled plan has no `ReasoningSelection`, and the request policy has no
portable reasoning value.

The disabled request emits `thinking: {"type":"disabled"}` with
`reasoning_effort` absent. The existing constructor still emits the previous
enabled effort ladder and `thinking: {"type":"enabled"}`. Structured disabled
response parsing accepts ordinary content, finish, usage, and model evidence
without private reasoning; a non-null `reasoning_content` field is rejected as
protocol drift. Continuation parsing and replay retain the existing enabled
non-empty private-continuation invariant.

Deterministic prepared-facade and protocol fixtures cover the admitted disabled
path, no portable selection, omitted effort, response composition, private
reasoning drift, enabled effort preservation, continuation replay, and
pre-network rejection. These tests prove local selection and dispatch
agreement only. They do not prove provider acceptance, effective mode,
quality, latency, price, cache effect, or any private-reasoning observation.
