# Anthropic Direct Prepared Integration

Use `swallowtail-adapter-anthropic` route `anthropic.messages`, driver ID
`swallowtail.anthropic.direct`, for the provider-supported Models and Messages
API over HTTPS/SSE. Choose it for catalogue observation, one bounded Messages
attempt, or a small resource-free consumer-tool continuation. Reject it when
the application needs Claude subscription access, provider-hosted agent state,
durable continuation, a working resource, or provider-session management.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Add The Connection

`anthropic.messages` currently exports an addable descriptor. Topology is
**hosted**. It is not `ExecutionLayer`. Follow
[connection lifecycle](connection-lifecycle.md) before the prepared facade.

1. Assemble `AddableRouteCatalog` from
   `anthropic_messages_addable_route_descriptor`. The row is `Available` when
   the host exposes the Credential service; otherwise
   `Unavailable(HostService)`.
2. `admit_instance` writes the configured instance. Admission does not
   prepare.
3. Collect the secret API-key field `api_key` as `CredentialRef`. The
   advertised environment name `ANTHROPIC_API_KEY` is a name, not a resolved
   value. Host stores the bytes. The library-owned API-key loop does not
   use URL-open, loopback, or device-code ports.
4. Config field `endpoint` is an opaque `ApiEndpoint` `ConfigFieldRef`.
5. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement stays
   independent of 047 `Ready` / `NotReady`.
6. `observe_authenticated_subject` is `Absent`. Do not probe Messages for
   identity.
7. Overlay may mark catalogue rows whose `provider_id` is `anthropic`.
   Overlay copies readiness and cannot change it.
8. Build `AnthropicPreparationInput::from_admitted` from the admitted record,
   then call `prepare_anthropic_direct`. The constructor selects the stored
   `endpoint` and `api_key` refs; the host resolves them at its boundary.

The compile-tested
[`connection_lifecycle` example](../../crates/swallowtail-adapter-anthropic/examples/connection_lifecycle.rs)
shows catalog through prepare. The canonical route-map example remains
[`prepared_direct`](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs).
Hosted interactive OAuth is not a realized path on this route.

## Operator Prerequisites

Preparation requires one admitted instance, revision, public API-key
pay-as-you-go profile, and matching observed or caller-asserted access
evidence. `from_admitted` selects the stored endpoint and credential refs; the
host supplies their resolution plus HTTP, task, time, and optional attachment
services required by the prepared operation.

Swallowtail does not read environment variables, discover an account or
endpoint, choose a model, infer entitlement or billing, or fall back to Claude
Agent, Claude Code, Bedrock, or another compatible API. Credential and endpoint
values remain outside plans and stable diagnostics.

The route has no ordered version axis. It binds exact
`anthropic-2023-06-01` facade behavior. A different date or compatible JSON
shape requires separate qualification; there is no unverified-newer inference.

## Prepare And Observe Models

Call `prepare_anthropic_direct` with `AnthropicPreparationInput` and the exact
host services. Preparation is local: it checks host, target, access evidence,
and service availability without acquiring a credential or making a provider
request. The result exposes its safe configured instance, access provenance,
services, and `AnthropicDirectDriver` escape hatch.

`prepare_catalogue` accepts `AnthropicCatalogueProfileInput`; the returned
`list_models` operation observes the exact configured source. Entries and
token limits do not select a route or prove model permission, balance, quota,
capacity, billing, or invocation success.

## One Inference Attempt

`AnthropicInferenceAttemptInput` requires request identity, exact
`AnthropicModelSelection`, user content, one positive maximum-output-token
bound, and optional deadline. Optional qualified inputs are:

- at most one input PNG with declared size no greater than one MiB
- `AnthropicWebSearchInput` with one to ten bare allowed domains; the provider
  tool is fixed to at most two uses
- `with_reasoning_mode` for the promoted exact effort row: model
  `claude-opus-4-7` with `low`, `medium`, `high`, `xhigh`, or `max`
- `with_thinking_mode(AnthropicThinkingMode::adaptive())` for the promoted
  adaptive omitted-display row on the same model

Effort is the portable `ReasoningMode` selection for this route. Preparation
rejects another model, value, or profile before endpoint authorization. It emits
only the provider-owned `output_config: {"effort": "<exact>"}` field; it does
not add `thinking`, choose a default, clamp a value, or infer effort from
output text. Omission retains the existing request body and means no effort
selection.

Adaptive thinking is a separate adapter-local control. Preparation admits it
only for exact `claude-opus-4-7`. The request then includes
`thinking: {"display":"omitted","type":"adaptive"}`. Omission of that selection
keeps current request bytes: no `thinking` object. The two controls compose
without defaults, clamps, or shared confirmation. `adaptive` is not an effort
value. Summarized display, manual `thinking.type=enabled`, and
`budget_tokens` stay out.

The structured pump accepts omitted thinking and start-complete redacted
blocks only to validate stream order. It emits no thought text, signature,
redacted payload, or `ReasoningSummary` activity, and it retains no private
continuation after terminal. A `thinking_delta` under this omitted-display
selection fails closed.

The host materializes the opaque attachment. Web search is provider-owned
external network access, not a consumer tool or general network grant.

Call `prepare_inference_attempt`, inspect `plan()`, `request()`, and
`evidence()`, then `start_run`. One start makes exactly one Messages request.
Take and drain events and the terminal outcome concurrently, then close the
run. Usage, request correlation, rate evidence, cancellation, deadline,
provider failure, and cleanup remain distinct. No result, error, timer, usage,
or catalogue observation authorizes an automatic retry.

The structured role has no consumer tool loop, structured output, provider
retention, background execution, or continuation. Its optional effort and
adapter-local thinking selections are independent of output-token limits,
attachments, search, cancellation, and model identity.

## Direct Tool Continuation And Restart

`AnthropicSessionProfileInput` binds an exact model and one to eight declared
consumer JSON Schema tools. Add the same exact effort selection and, when using
`claude-opus-4-7`, the same adapter-local adaptive thinking selection. `prepare_session`
returns a resource-free interactive profile supporting two user turns and one
exact correlated tool call/result continuation. Swallowtail relays the tool
exchange but never selects or executes a tool.

Session effort and thinking are fixed at preparation. The same
`output_config.effort` and omitted-display `thinking` object are sent on the
initial request, every correlated continuation attempt, every later turn, and a
fresh working-state restoration. There is no per-turn raw override.

When adaptive thinking is selected, the first-assistant thinking and redacted
blocks are captured in bounded zeroizing memory and replayed unmodified, in
order, immediately before the correlated `tool_use` block. Adaptive skip with
no thinking block is valid. A consumer tool result remains the only
continuation authority. Thought text, signatures, and redacted data never
enter events, activity, output, callbacks, evidence, or diagnostics.

Call `open_session`, then start turns through the direct-continuation
interface. Drain events, the tool exchange, and terminal concurrently. A next
provider request occurs only after the consumer supplies the exact correlated
result. Cancellation, omission, or rejection sends no continuation attempt.
Close each turn and the session to join local HTTP, task, and credential work.

Provider-required assistant and tool envelopes stay bounded in adapter memory.
They are not portable output or durable provider-session identity.
`prepare_working_state_restoration` therefore opens a fresh session and
returns `SessionReplaced`. It preserves only the interrupted consumer turn ID,
not prompt, transcript, tool result, private continuation, or terminal truth.
Restoration repeats the prepared thinking selection and recovers no private
blocks.

The route exposes no load/resume binding, reconciliation, archive, restore,
delete, native close, provider-session catalogue/import, billed cost, or
working-resource authority.

## Failures, Promotion, And Validation

Handle failures through portable classification and retain the exact
`swallowtail.anthropic.*` diagnostic for support. Do not parse SSE records,
HTTP bodies, provider prose, tool text, endpoint values, or credentials in
consumer code.

Promotion requires an exact Anthropic facade and model surface, immutable
prepared-plan and access binding, bounded protocol/attachment fixtures,
lifecycle tests, and route-matrix coverage. Documentation or compatible syntax
alone is insufficient.

Keep the effort claim states separate:

- **requested:** the consumer supplied `ReasoningMode` during preparation
- **planned:** the immutable plan and prepared evidence carry one exact
  `ReasoningSelection` constraint
- **dispatched:** the Messages request contains the exact `output_config.effort`
- **accepted:** a successful provider response may establish request
  acceptance, subject to the route's normal response handling
- **effective:** not claimed; no response-text inference establishes provider
  reasoning allocation

Keep the thinking claim states separate:

- **requested:** the consumer supplied `AnthropicThinkingMode::adaptive()`
- **planned:** prepared evidence carries that adapter-local selection; the
  shared plan does not grow a thinking capability
- **dispatched:** the Messages request contains
  `thinking: {"display":"omitted","type":"adaptive"}`
- **accepted:** a successful response fixture proves parser acceptance only
- **effective:** not claimed; blocks, tokens, and prose do not establish
  thinking depth

The compile-tested
[`prepared_direct` example](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs)
covers catalogue, run, and session preparation. Validate without network work:

```sh
effigy validate:focused swallowtail-adapter-anthropic
effigy check:examples
```

Live API calls and allowance spend are separately operator-gated.
