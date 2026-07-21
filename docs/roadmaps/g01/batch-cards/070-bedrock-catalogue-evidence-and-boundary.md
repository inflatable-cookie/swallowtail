# 070 Bedrock Catalogue Evidence And Boundary

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../021-bedrock-control-plane-catalogue-proof.md`

## Objective

Revalidate the current Bedrock control-plane catalogue surface and decide the
smallest exact production boundary before adding another SDK client.

## Governing References

- `../../../contracts/005-integration-identity-and-transport-diversity.md`
- `../../../contracts/006-execution-layer-and-access-boundary.md`
- `../../../contracts/014-hosted-transport-credential-and-evidence-boundary.md`
- `../../../contracts/019-embedded-sdk-and-cloud-client-boundary.md`
- `../../../contracts/020-model-catalogue-observation-and-availability-boundary.md`
- `../../../research/013-sdk-native-and-post-portability-route-evidence.md`
- `../../../research/014-bedrock-control-plane-catalogue-evidence.md`

## Scope

- exact maintained `aws-sdk-bedrock` version and generated
  `ListFoundationModels` types
- control-plane endpoint, region, service audience, permissions, request shape,
  filters, retry, timeout, and credential-provider behavior
- provider, lifecycle, modality, streaming, and customization evidence
- catalogue presence versus IAM, access, region, quota, readiness, and route
  selection
- smallest contract or research delta required before fixtures

## Out Of Scope

- production driver or live AWS calls
- default region, provider, model, route, account, or billing selection
- runtime inference or reuse of its endpoint grant

## Acceptance Criteria

- [x] current official and generated-SDK evidence fixes one bounded catalogue
      subset
- [x] runtime and control-plane drivers retain distinct service and access
      bindings
- [x] catalogue evidence cannot imply invocation entitlement or select a route
- [x] ambient AWS configuration and SDK retries remain prohibited
- [x] card 071 can freeze exact generated types without a policy decision

## Evidence

- Research 014 checks current official AWS API, endpoint, IAM, model-access,
  lifecycle, and catalogue documentation plus the generated Rust SDK source
- the selected boundary is official `aws-sdk-bedrock = 1.148.0` and native
  `ListFoundationModels` against one explicit regional control-plane audience
- the request has four optional filters but no pagination; the first proof
  sends no filters and bounds the single returned collection
- Contract 020 keeps lifecycle, modalities, advertised streaming, inference,
  and customization as source-scoped observations rather than capability,
  entitlement, or route claims
- generated unknown enum values remain bounded provider observations and do
  not imply common support
- `bedrock:ListFoundationModels` access remains separate from runtime
  inference, Marketplace, provider onboarding, and account-specific model
  access
- the distinct `bedrock-mantle` OpenAI-compatible `/models` catalogue remains
  outside roadmap 021
- no AWS credential, account, endpoint, Marketplace action, or paid request was
  used
- `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` pass
- `effigy doctor` retains the inherited 19 oversized-file findings: 12
  warnings and 7 errors

## Validation

- source and authority review
- docs QA
- `git diff --check`

## Stop Conditions

- current evidence leaves service or permission authority ambiguous
- catalogue semantics require product routing policy

## Auto-Continuation

No. Card 071 now owns the provider-neutral observation records and exact
generated SDK corpus.
