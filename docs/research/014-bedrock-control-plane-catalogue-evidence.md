# 014 Bedrock Control-Plane Catalogue Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

What exact Bedrock model-catalogue surface should Swallowtail prove after the
Bedrock Runtime driver, and which discovery facts need a durable shared
boundary before generated fixtures or production work begin?

## Method

Official AWS API, endpoint, IAM, model-access, model-lifecycle, and model-
catalogue documentation was checked on 2026-07-21. The current Rust registry
release and its generated source were inspected locally with `cargo search`
and `cargo info`.

No AWS credential source, provider account, Marketplace subscription, or live
service endpoint was used.

## Exact Native Catalogue Surface

The bounded native control-plane operation is `ListFoundationModels` through
the separate Amazon Bedrock client:

- official Rust crate: `aws-sdk-bedrock = 1.148.0`
- request: `GET /foundation-models`
- service endpoint audience: regional Bedrock control plane, distinct from
  Bedrock Runtime
- response: one `modelSummaries` collection
- pagination: none; the request and response expose no token or paginator
- optional filters: provider, customization type, inference type, and output
  modality
- first proof: no filters, so Swallowtail observes the complete response for
  the one explicitly bound regional service

The generated request input has only those four filters. The generated output
has no continuation token. Roadmap language describing a page or bounded
pagination is therefore wrong for this operation and must be removed.

Evidence:

- [ListFoundationModels API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListFoundationModels.html)
- [AWS regional Bedrock endpoints](https://docs.aws.amazon.com/general/latest/gr/bedrock.html)
- [`aws-sdk-bedrock` crate](https://crates.io/crates/aws-sdk-bedrock)

## Generated Model Evidence

The current generated `FoundationModelSummary` exposes:

- required model ARN and model id
- optional model name and provider name
- input and output modalities
- response-streaming support
- customization types
- inference types
- model lifecycle

Known generated modalities are text, image, and embedding. Known inference
types are on-demand and provisioned. Known customization types are fine-
tuning, continued pre-training, and distillation. Generated enums are non-
exhaustive and can carry service values newer than the pinned SDK.

The generated lifecycle status currently recognizes active and legacy. The
service lifecycle documentation also describes end-of-life and public extended
access periods. It warns that Bedrock lifecycle can differ from the underlying
model provider's lifecycle. Swallowtail must therefore preserve lifecycle as
source-scoped catalogue evidence rather than force it into a universal support
or routability enum.

The model ARN is not needed as the public model identity in the first proof.
It remains generated provider evidence and cannot replace the model id, model
route, configured instance, endpoint, or provider identity.

Evidence:

- [Bedrock model lifecycle](https://docs.aws.amazon.com/bedrock/latest/userguide/model-lifecycle.html)
- [Getting information about foundation models](https://docs.aws.amazon.com/bedrock/latest/userguide/models-get-info.html)

## Availability And Access Meaning

Catalogue presence means only that the bound regional Bedrock control plane
reported an entry at observation time. It does not prove:

- IAM authorization for runtime inference
- account or AWS Marketplace entitlement
- completion of a third-party provider form or agreement
- model availability through one runtime inference type or profile
- quota, rate, capacity, billing readiness, or request acceptance
- support by a selected Swallowtail facade
- a configured model route or consumer routing preference

`ListFoundationModels` is a separate IAM list action and does not use a
resource ARN in its authorization table. The first driver needs that exact
control-plane permission posture. It does not call `InvokeModel`, any
Marketplace operation, or any model-provider onboarding surface.

AWS documents account-specific access behavior for third-party models. A first
invocation can initiate Marketplace subscription work and some models require
additional provider prerequisites. None of that behavior is part of catalogue
discovery, and a catalogue result cannot claim it succeeded.

Evidence:

- [Amazon Bedrock IAM actions](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrock.html)
- [Bedrock model access](https://docs.aws.amazon.com/bedrock/latest/userguide/model-access.html)
- [Bedrock identity-policy examples](https://docs.aws.amazon.com/bedrock/latest/userguide/security_iam_id-based-policy-examples.html)

## Two Current Catalogue Audiences

AWS now documents two inference-endpoint catalogue shapes:

- native Bedrock `ListFoundationModels` through the regional `bedrock` control
  plane
- OpenAI-compatible `GET /models` through the distinct `bedrock-mantle`
  endpoint

The Mantle surface is not an alias for the native control plane. It has a
different endpoint and protocol authority and may expose a different catalogue
shape. Roadmap 021 proves only the native `aws-sdk-bedrock` operation. A future
Mantle route needs its own evidence and driver identity.

Evidence:

- [Bedrock inference endpoint catalogues](https://docs.aws.amazon.com/bedrock/latest/userguide/models-get-info.html)
- [Bedrock service endpoints](https://docs.aws.amazon.com/bedrock/latest/userguide/endpoints.html)

## Smallest Safe Projection

The first provider-neutral projection needs more than the current model name,
description, reasoning, and token-limit fields. It should add source-scoped
catalogue observations for:

- input and output modalities
- lifecycle status and optional lifecycle timestamps
- provider-advertised response streaming
- inference types
- customization types
- bounded provider-defined values newer than the common vocabulary

These observations do not become runtime `Capability` claims. For example,
Bedrock reporting streaming support does not prove that a selected Swallowtail
facade implements streaming for that model. Unknown generated enum values are
retained as bounded, namespaced observations and never imply common support.
Absent optional fields remain unknown.

The response is still bounded even without pagination. The adapter must set a
fixed maximum entry count and bounded observation-string lengths, fail safely
on overflow, use one SDK attempt, and keep the outer operation deadline and
cancellation authority through projection and cleanup.

## Decision

Proceed with native Bedrock `ListFoundationModels` through
`aws-sdk-bedrock = 1.148.0`.

Use a separately registered control-plane catalogue driver, one exact regional
endpoint audience, one exact configured access profile and delegated AWS
credential provider, no ambient AWS configuration, no request filters, one
service call, one SDK attempt, and deterministic generated-type fixtures.

Do not reuse the Bedrock Runtime endpoint grant or runtime driver. Do not add a
route, select a model, invoke a model, trigger Marketplace behavior, or infer
entitlement from the result.

## Promotion

- durable observation and availability rules: Contract 020
- fixture and implementation sequence: g01 roadmap 021 and cards 071-072
- currentness and boundary closeout: 2026-07-21 Bedrock Control-Plane
  Catalogue Boundary log
