# Bedrock SDK Prepared Integration

Amazon Bedrock uses two separate prepared surfaces:

| Route | Preparation | Bound operation | SDK |
| --- | --- | --- | --- |
| Runtime inference | `prepare_bedrock_runtime` | `BedrockPreparedInferenceAttempt` | `aws-sdk-bedrockruntime = 1.136.0` |
| Control-plane catalogue | `prepare_bedrock_catalogue` | `BedrockPreparedCatalogue` | `aws-sdk-bedrock = 1.148.0` |

They do not share a configured instance, driver, access profile, endpoint
audience, plan, request, or operation method.

## Explicit Cloud Client

Both preparation inputs require `BedrockCloudClientConfig`. It contains:

- one exact `BedrockRegion`
- one already-selected `BedrockCredentialProvider`

The consumer or execution host chooses how to construct that provider. It may
represent a named profile, workload identity, helper, or another authorized
source. Swallowtail does not search the AWS environment, shared files,
profiles, container metadata, instance metadata, or region chain.

The credential provider object stays opaque. Stable diagnostics and prepared
evidence expose no keys, tokens, SDK request objects, endpoint values, or raw
provider payloads.

## Runtime

Build the runtime access profile with
`bedrock_runtime_access_profile(credential_reference)`. Then construct
`BedrockRuntimePreparationInput` with the instance identity, revision,
execution host, host-approved endpoint target, matching access evidence, and
cloud-client configuration.

`prepare_bedrock_runtime` performs no network, credential, SDK, or model work.
It returns `BedrockRuntimePreparedIntegration`.

`prepare_inference_attempt` additionally requires:

- one route id and revision
- one exact Bedrock model id
- one underlying provider id
- text content
- one positive maximum-output-token bound
- an optional host-monotonic deadline

The returned `BedrockPreparedInferenceAttempt::start_run` delegates to the
unchanged `ConverseStream` driver. One operation remains one SDK attempt with
SDK retries disabled. Cancellation and deadline join the operation-private
executor before credential release.

See the compile-tested
[`prepared_runtime` example](../../crates/swallowtail-adapter-bedrock/examples/prepared_runtime.rs).

## Catalogue

Build the independent catalogue access profile with
`bedrock_catalogue_access_profile(credential_reference)`. Construct
`BedrockCataloguePreparationInput` with its own configured instance,
host-approved control-plane target, access evidence, and cloud-client
configuration.

`prepare_catalogue` accepts only a request id and optional deadline. The
resulting `BedrockPreparedCatalogue::list_models` delegates one unfiltered,
non-paginated `ListFoundationModels` request to the unchanged catalogue
driver.

The prepared catalogue plan has no model route or model id. Returned catalogue
observations cannot construct or mutate a Runtime route. Catalogue presence
does not prove IAM invocation permission, model entitlement, provider
prerequisites, regional availability, quota, rate, billing, or request
acceptance.

See the compile-tested
[`prepared_catalogue` example](../../crates/swallowtail-adapter-bedrock/examples/prepared_catalogue.rs).

## Evidence And Escape Hatch

Runtime and catalogue evidence separately retain:

- adapter driver and role
- configured instance, revision, target, and execution host
- access status and provenance
- exact region
- exact SDK crate and package version
- exact service-operation facade
- immutable preflight plan

Prepared operations expose `plan`, `request`, `low_level_driver`, and
`into_parts`. Advanced consumers may still assemble and call the low-level
drivers directly.

Cross-region inference profiles, global routing, guardrails, tools, prompt
resources, attachments, ambient AWS configuration, automatic SDK retry, route
selection from catalogue results, and live AWS authentication tests remain
excluded.
