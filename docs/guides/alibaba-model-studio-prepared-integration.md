# Alibaba Model Studio Prepared Integration

Use this facade for the frozen Singapore workspace-dedicated Conversations and
Responses route. It binds one configured workspace instance, exact regional
audience, general API-key access profile, Qwen route, durable provider
conversation, and delete-on-close lifecycle.

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

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for diagnostics and advanced use.

See the compile-tested
[`prepared_provider_conversation` example](../../crates/swallowtail-adapter-alibaba-model-studio/examples/prepared_provider_conversation.rs).
