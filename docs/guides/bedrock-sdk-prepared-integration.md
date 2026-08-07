# Bedrock SDK Prepared Integration

Amazon Bedrock uses two separate prepared surfaces:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

| Route | Driver ID and operation | Transport and SDK |
| --- | --- | --- |
| `bedrock.runtime` | `swallowtail.amazon-bedrock.direct`; `BedrockPreparedInferenceAttempt` | Rust SDK EventStream; `aws-sdk-bedrockruntime = 1.136.0` |
| `bedrock.catalogue` | `swallowtail.amazon-bedrock.catalogue`; `BedrockPreparedCatalogue` | Rust SDK control plane; `aws-sdk-bedrock = 1.148.0` |

They do not share a configured instance, driver, access profile, endpoint
audience, plan, request, or operation method.

Both routes live in `swallowtail-adapter-bedrock`. Choose Runtime for one
explicit regional model attempt and Catalogue for control-plane observation.
Reject Bedrock when the application needs a public model API, automatic global
routing, ambient AWS discovery, a reusable session, or provider lifecycle
management.

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

Use `prepare_bedrock` with `BedrockFacadePreparationInput` to bind the shared
execution host and exact cloud-client configuration. Then select `runtime` or
`catalogue` with a route-specific instance, approved regional target, access
profile, and evidence. The older branch-specific preparation functions remain
public escape hatches; the normal facade never infers a branch.

The host supplies blocking-work, task, time, network, and credential services
plus the explicit credential-provider composition. The adapter constructs the
route-specific SDK client with retries disabled. Preparation makes no SDK
request and does not inspect AWS environment variables, profiles, shared
files, container or instance metadata, account billing, IAM policy, or model
entitlement.

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

Take and drain streaming events and terminal concurrently, then close the
run. Output, usage, rate/request evidence, SDK error, cancellation, deadline,
EventStream cleanup, and credential release remain distinct. Cancellation
releases local request work without claiming provider-native interruption. No
result or error authorizes retry.

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

Runtime binds exact `amazon-bedrock.runtime-rust-sdk` and Runtime service API
revisions. Catalogue binds exact `amazon-bedrock.control-plane-rust-sdk` and
control-plane service API revisions. These are opaque exact claims, not an
ordered or unverified-newer range.

Prepared operations expose `plan`, `request`, `low_level_driver`, and
`into_parts`. Advanced consumers may still assemble and call the low-level
drivers directly.

Cross-region inference profiles, global routing, guardrails, tools, prompt
resources, attachments, ambient AWS configuration, automatic SDK retry, route
selection from catalogue results, and live AWS authentication tests remain
excluded.

Both routes also expose no structured output, reasoning selection,
working-resource access, callbacks, background execution, retained sessions,
reconciliation, or provider management. Owned cloud resources are not created.

## Failures, Promotion, And Validation

Handle failures through portable classification and retain the exact
`swallowtail.bedrock.*` diagnostic for support. Never parse SDK debug values,
raw EventStream records, provider prose, endpoint targets, or credential
provider internals.

Promotion requires exact SDK and service-model revisions, regional target and
access binding, bounded client fixtures, operation lifecycle tests, and
route-matrix coverage. Catalogue presence alone cannot promote Runtime support.

```sh
effigy validate:focused swallowtail-adapter-bedrock
effigy check:examples
```

The linked Runtime and Catalogue examples compile without AWS credentials or
network calls. Live AWS access and billable inference remain operator-gated.
