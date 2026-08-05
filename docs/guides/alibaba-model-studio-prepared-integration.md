# Alibaba Model Studio Prepared Integration

Use this facade for the frozen Singapore workspace-dedicated Conversations and
Responses route. It binds one configured workspace instance, exact regional
audience, general API-key access profile, and Qwen route. Conversation
ownership is selected explicitly: operation-owned delete-on-close or retained
provider state with separate cleanup authority.

## Regional Access

`prepare_alibaba_model_studio` requires:

- one configured-instance revision
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

`open_session` creates one provider conversation and returns the normal
`InteractiveSessionHandle`. Each `start_turn` sends one synchronous streaming
Responses request against the same conversation. The first subset allows two
serial text turns and no tools, response storage, cache, background execution,
retry, reattachment, or resume.

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
