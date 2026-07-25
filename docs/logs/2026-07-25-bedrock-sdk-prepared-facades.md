# 2026-07-25 Bedrock SDK Prepared Facades

Status: complete

## Changed

`swallowtail-adapter-bedrock` now exposes separate prepared integrations and
bound operations for Runtime `ConverseStream` and control-plane
`ListFoundationModels`.

Both preparation inputs require `BedrockCloudClientConfig`: one exact region
and one already-selected opaque delegated credential provider. Runtime and
catalogue retain different drivers, configured instances, endpoint audiences,
access profiles, SDK crates, service interfaces, plans, requests, and
operation methods.

Runtime preparation requires an exact route, model, underlying provider,
content, and output bound. Catalogue preparation remains route-free. A
catalogue result cannot become a Runtime route or claim invocation authority.

## Current Evidence

Current first-party documentation still confirms the boundary:

- [AWS SDK for Rust credential providers](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html)
- [AWS SDK for Rust region configuration](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/region.html)
- [AWS SDK for Rust endpoint configuration](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/endpoints.html)
- [Bedrock ConverseStream](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html)
- [Bedrock ListFoundationModels](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListFoundationModels.html)

The SDK documents ambient credential and region chains when callers do not
configure them explicitly. Swallowtail continues to prohibit those chains
inside the driver and accepts only explicit configuration. No contract or
fixture delta was required.

## Boundaries

- `aws-sdk-bedrockruntime = 1.136.0` and
  `aws-sdk-bedrock = 1.148.0` remain separate exact package axes
- `ConverseStream` and `ListFoundationModels` remain separate service axes
- the opaque provider object never enters stable diagnostics or core records
- preparation performs no network, credential, SDK, catalogue, or inference
  work
- SDK retries remain disabled; one operation performs at most one request
- cancellation and deadline join private SDK work before credential release
- catalogue access does not imply Runtime IAM, Marketplace, model access,
  regional availability, rate, quota, billing, or request acceptance
- live AWS authentication and paid inference remain separately gated

## Validation

- 25 Bedrock tests pass
- full repository QA passes
- both generated SDK fixture suites pass
- local and remote-authoritative prepared plans retain exact host identity
- low-level Runtime and catalogue lifecycle suites retain joined
  credential-last cleanup
- warnings-denied all-target lint and both examples pass
- general `1.93`, Bedrock `1.94.1`, and current-stable checks pass
- docs checks and public API declaration checks pass
- `effigy doctor` retains the known 19 oversized-file findings, including
  seven errors; this batch adds none

## Next

Card 033 adds distinct llama.cpp attached-runtime and owned-serving facades.
Cards 033-036 remain in bounds.
