# Alibaba Model Studio Prepared Integration

Use this facade for the frozen Singapore workspace-dedicated Conversations and
Responses route. It binds one configured workspace instance, exact regional
audience, general API-key access profile, and Qwen route. Conversation
ownership is selected explicitly: operation-owned delete-on-close or retained
provider state with separate cleanup authority.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The production route is `alibaba.conversations` in
`swallowtail-adapter-alibaba-model-studio`, driver ID
`swallowtail.alibaba-model-studio.conversations-responses`, over HTTPS/SSE.
Choose it for the exact Singapore workspace conversation or resource-free
Responses shapes. Reject it when the application needs another region,
Coding/Token Plan billing, tools, working resources, or archive/restore.

## Regional Access

`prepare_alibaba_model_studio` requires:

- one configured-instance identity and revision
- one execution host with the exact workspace endpoint approved under the
  adapter's opaque endpoint reference
- the Singapore workspace general API-key profile
- pay-as-you-go metering and provider support authority
- observed or caller-asserted access evidence

Region, workspace, deployment scope, endpoint, key, and model list remain one
binding. Keys from another region or workspace cannot substitute. Coding Plan,
Token Plan, savings-plan, legacy DashScope, and trial access do not satisfy
this profile.

Preparation performs no endpoint or credential work. Its result exposes the
safe configured instance, access provenance, service set, and low-level driver
escape hatch.

Operations bind the approved endpoint and opaque credential through host HTTP,
credential, task, and time services. No secret or endpoint value enters plans
or stable diagnostics. The route binds exact
`openai-conversations-responses` facade behavior; there is no ordered or
unverified-newer version inference.

## Deployable Model Catalogue

`prepare_alibaba_deployable_models` is a separate international
cloud-control-plane integration. It binds its own configured instance,
`dashscope-intl.aliyuncs.com` endpoint audience, general API-key access
profile, facade revision, and catalogue plan.

Its `prepare_catalogue` and `list_models` path traverses bounded base and
custom deployment-candidate pages and projects model names only. A returned
candidate does not prove deployment, entitlement, Singapore workspace
availability, Conversations compatibility, or successful invocation. The
catalogue branch is not an endpoint, access, region, or model fallback for the
conversation facade.

## Explicit Provider State

`prepare_conversation` requires:

- request identity
- exact route identity and revision
- exact `qwen3.7-plus-2026-05-26` model identity
- `DurableConversationDeleteOnClose`
- an optional host-monotonic open deadline

The retention value is consumer-visible authority, not a constructor default.
Preparation derives the resource-free session request and plan agreement from
that exact selection.

`prepare_retained_conversation` is a separate profile. It fixes
`DurableProviderSessionPreserved`, exposes no owned-resource deletion
capability, and returns both an exact resource-free resume binding and a
separate management binding. Ordinary close preserves provider state.

The retained profile's `load_session` retrieves exact conversation metadata,
follows bounded ascending item pages, and returns complete ordered replay
before the live handle becomes ready. Its
`prepare_working_state_restoration` maps that same load path to
`ProviderSessionContinuationRecovery`; it preserves the interrupted consumer
turn as unresolved. There is no replay-free resume or inferred terminal state.

## Resource-Free Structured Run

`prepare_run` binds the same exact workspace, route, and model but creates no
provider conversation. It accepts text content and an optional host-monotonic
deadline, then sends one streamed Responses request with `store=false`.
`conversation` and `previous_response_id` are omitted.

The run supports ordered text, usage, request correlation, cancellation, and
joined credential-last cleanup. Provider retention is prohibited. Tools,
attachments, structured output, reasoning overrides, output-token overrides,
working resources, retries, and background execution reject before endpoint
or credential effects.

Take the run event stream and terminal outcome immediately and poll them
concurrently, then close the run. A terminal response, error, usage record, or
timer does not authorize retry.

`open_session` creates one provider conversation and returns the normal
`InteractiveSessionHandle`. Each `start_turn` sends one synchronous streaming
Responses request against the same conversation. The first subset allows two
serial text turns and no tools, response storage, cache, background execution,
retry, reattachment, or resume.

For each turn, drain events and terminal concurrently, then close the turn.
Cancellation stops local transport without fabricating remote cancellation.
Close the session only after active turns finish; deletion and preservation
then follow the selected profile rather than consumer inference.

The delete-on-close profile exposes `prepare_working_state_restoration` as a
fresh replacement. It creates a new provider conversation under the same
prepared route and returns the interrupted consumer turn id without prior
messages or terminal truth. Ordinary close still deletes the replacement's
items and conversation. This path grants no retained load or management
authority.

## Inspection And Deletion

The provider conversation is not consumer memory. Closing the handle:

1. joins active local turn work
2. lists the complete bounded provider item inventory
3. deletes each discovered item
4. deletes the conversation separately
5. joins cleanup
6. releases the credential

Item deletion and conversation deletion retain separate provider truth.
Conversation deletion cannot mask an incomplete inventory or failed item
deletion. Cancellation and deadlines stop local transport but do not fabricate
remote response cancellation or confirmed deletion when remote state is
uncertain.

Retained cleanup is not granted by the resume binding. Call
`prepare_delete_retained_conversation` with the exact management binding. It
lists and deletes items before deleting the conversation, and distinguishes
failed-before-effect from unconfirmed-after-effect outcomes.

Prepared run and conversation branches expose `plan`, `request`, `evidence`,
and `low_level_driver` for diagnostics and advanced use.

See the compile-tested
[`prepared_provider_conversation` example](../../crates/swallowtail-adapter-alibaba-model-studio/examples/prepared_provider_conversation.rs).

## Failures, Unsupported Capabilities, And Promotion

Handle failures through portable classification and retain the exact
`swallowtail.alibaba_model_studio.*` diagnostic for support. Keep inference
terminal, item deletion, conversation deletion, management effect, and local
cleanup truth separate. Do not parse HTTP bodies, SSE frames, provider prose,
credentials, endpoint values, or conversation items in consumer code.

The route exposes no reasoning or output-token override, attachments,
structured output, tools, callbacks, working resource, external search,
background execution, stream reattachment, retry, replay-free resume,
archive, restore, native close, or billed cost. The separate deployable-model
catalogue grants no conversation-route authority.

Promotion requires exact regional workspace/facade evidence, immutable plan
and access binding, bounded response and lifecycle fixtures, effect-boundary
tests, and route-matrix coverage.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-alibaba-model-studio
effigy check:examples
```

No live workspace request, conversation creation, deletion, or credential use
is required.
