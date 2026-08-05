# Anthropic Direct Prepared Integration

Use the prepared facade for the provider-supported Anthropic Models and
Messages route. It binds the public API audience, dated protocol facade,
configured endpoint target, API-key lease source, host, access evidence, and
operation plans without exposing the key or approved endpoint value.

## Inputs That Stay Explicit

Preparation requires:

- one configured-instance identity and revision
- one execution host and host-approved endpoint target
- one provider-supported, pay-as-you-go public API-key access profile
- one opaque credential reference for `api.anthropic.com`
- observed or caller-asserted access evidence

Swallowtail does not discover an account, read an environment variable, choose
an endpoint, select a model, infer entitlement, or fall back to another route.
The host resolves the opaque endpoint and credential references only when an
operation starts.

Call `prepare_anthropic_direct` with `AnthropicPreparationInput` and the host
service composition. Preparation is local and performs no provider request or
credential acquisition. The result exposes its safe configured instance,
access provenance, service set, and low-level `AnthropicDirectDriver` escape
hatch.

## Catalogue Observation

`prepare_catalogue` accepts an `AnthropicCatalogueProfileInput` containing a
request identity and optional deadline. It derives a `ModelCatalog` plan and
request without a model route.

`AnthropicPreparedCatalogue::list_models` observes the exact configured
catalogue source. Returned entries are mutable, source-scoped evidence. Their
presence, token limits, and provider ids do not select a route or prove model
permission, entitlement, balance, quota, or request acceptance.

## One Inference Attempt

`prepare_inference_attempt` requires:

- one exact `AnthropicModelSelection`
- request identity and user content
- one explicit positive maximum-output-token bound
- an optional host-monotonic deadline

The result derives an offline, attached, non-retained structured-run request
and a route-bound direct-inference plan. `start_run` makes exactly one Messages
request. Provider errors, cancellation, deadline, usage, request correlation,
rate evidence, terminal outcome, and joined cleanup retain the low-level
driver semantics.

This first Anthropic subset is text-only. It declares neither `ToolCalls` nor
`DirectToolContinuation`. Swallowtail does not execute tools or turn a model
tool call into another request. Consumers needing a tool loop must select a
separately supported direct-session route; they cannot treat this structured
run as an implicit agent loop.

Every further Anthropic attempt requires another explicit prepared input and
`start_run` call. No usage observation, remaining capacity, provider error,
timer, catalogue result, or successful output authorizes retry.

## Direct Continuation And Restart

`prepare_session` binds one resource-free consumer-tool continuation session.
Its provider-required assistant and reasoning envelopes remain bounded,
adapter-private process memory. They are neither portable output nor durable
provider-session identity.

`AnthropicPreparedSession::prepare_working_state_restoration` therefore opens
one fresh session and returns `SessionReplaced`. It preserves the interrupted
consumer turn id but no prompt, transcript, tool result, provider-private
continuation, or terminal truth. Closing or losing the original session makes
its hidden continuation unrecoverable.

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for diagnostics and advanced low-level use.

See the compile-tested
[`prepared_direct` example](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs).
