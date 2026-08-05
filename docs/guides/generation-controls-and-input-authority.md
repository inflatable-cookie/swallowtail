# Generation Controls And Input Authority

Use this runbook when an operation needs output limits, reasoning, structured
output, attachments, tools, operator input, working-resource access, writes,
or search. Support is exact per route. A `Yes` on one route never promotes
another route from the same provider.

## Generation Controls

Swallowtail keeps requested, planned, dispatched, accepted, effective, and
observed controls distinct.

- Output-token limits use the route's exact positive bound. Omission preserves
  the route posture; it does not ask Swallowtail to invent a default.
- Reasoning selection uses the adapter's typed input and exact model/version
  mapping. Do not translate arbitrary UI labels into provider strings or
  emulate reasoning with prompt text.
- Structured output uses `StructuredOutputDescriptor` and a host-resolved
  schema where the route requires one. Schema dialect and enforcement source
  remain route evidence. JSON-looking text is not schema enforcement.

Read the selected route guide before constructing its profile input. Unsupported
controls fail during preparation or request admission; they are not silently
ignored, clamped, or moved to another model.

## Attachments

Attach finite `AttachmentDescriptor` values only in representations accepted
by the selected route. The host's `AttachmentService` resolves opaque
references and bounded leases. Role, media type, digest, representation,
maximum bytes, and cleanup authority remain explicit.

The consumer owns file picking and consent. Swallowtail does not turn a path,
URL, provider file id, or working-resource locator into another attachment
representation. Attachment cleanup authority does not imply deletion of the
consumer's source file or provider-owned data.

## Consumer Tools And Direct Continuation

Native consumer tools use bounded `ToolDeclaration` records and the route's
callback exchange. Correlate every request by callback and operation identity;
reply exactly once. Validate tool name and arguments against the declarations
prepared for that operation. Tool execution, side effects, authorization,
idempotency, and durable results remain consumer-owned.

Locally continued direct routes use `DirectToolExchange` instead. The
consumer authorizes each `DirectInferenceAttempt`, receives bounded
`DirectToolCall` values, executes declared tools, and returns typed
`DirectToolResult` values. `ProviderPrivateContinuationRecord` is ephemeral,
route-bound, redacted, and non-serializable. It is not a provider session or a
consumer transcript.

Provider-owned tool activity is observational. Do not execute it merely
because it appears in `ActivityObservation`.

## Permissions And Questions

Permission mediation and typed questions are callback features, not prompt
parsing.

- Permission requests expose provider-request evidence and the exact response
  vocabulary admitted by the route.
- Typed questions use `HarnessUserInputRequest`, stable question and option
  ids, `HarnessUserInputAnswer`, and `HarnessUserInputResponse`.
- Use `CallbackResponse::for_request`; never copy a provider-native request id
  into consumer code.
- Keep the event and callback streams draining while an operator UI waits.
- Respond once. Late responses after cancellation, timeout, terminal, or close
  must fail rather than resume another turn.

An unattended consumer must choose a route/profile whose permission posture
does not require a human. A studio may enable consumer-mediated permissions
only where the prepared route advertises them. Do not auto-approve because a
working resource is configured.

The compile-tested
[typed-question consumer](../../crates/swallowtail-runtime/examples/harness_user_input_consumer.rs)
shows stable-id response construction.

## Working Resources And Writes

`WorkingResourceRef` is opaque authority bound during preparation. The host's
`WorkingResourceService` resolves the approved resource and its lease.
`WorkingResourceIoService` separately handles bounded text reads and writes
through `WorkingResourceLocator`, `WorkingResourceReadRequest`, and
`WorkingResourceWriteRequest`.

Filesystem boundary, access mode, harness isolation, ambient read intent,
provider permissions, and write callbacks remain separate. A configured
working resource does not grant arbitrary filesystem access. A read-capable
route does not inherit writes. A write request must remain inside the host's
approved resource and bounds; the consumer must not apply provider prose as a
patch.

## External Search And Network Authority

External search requires both route support and
`ExternalSearchPolicy::Enabled` in the exact operation policy. Search is not
arbitrary network access, a consumer browser, or permission to fetch URLs
found in output. `ExternalSearchProgress` and search activity are evidence of
the admitted provider feature, not content to execute.

`ExternalNetworkPolicy` governs the route's network posture separately.
Enabling one does not silently enable the other.

## Ordering, Failure, And Cleanup

Prepare declarations, policies, schemas, attachments, and access before
dispatch. Drain events and callback/direct-tool exchanges concurrently with
terminal. On cancellation or terminal, stop accepting new effects, settle or
abandon callbacks through the typed exchange, and close the operation to join
leases and tasks.

Treat unsupported shape, invalid callback, schema mismatch, denied resource,
and search-policy mismatch through the
[portable failure guide](portable-failure-handling.md). Do not infer a class
from provider prose or tool display content.

## Route Applicability And Validation

The [feature matrix](provider-solution-feature-matrix.csv) is authoritative
for the three generation-control columns and seven input/authority columns.
The [integration guide map](integration-guide-map.md) links the exact route
guide and compiling example. Route guides name model/version restrictions and
unsupported features.

```sh
effigy check:examples
effigy qa:docs
effigy qa:routes
```

Deterministic validation uses fixtures and host doubles. Live tools, writes,
searches, prompts, authentication, and allowance spend remain separately
operator-gated.
