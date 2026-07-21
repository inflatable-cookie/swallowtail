# 2026-07-21 Bedrock Catalogue Driver And Conformance

## Outcome

Completed the separately registered Amazon Bedrock control-plane model-
catalogue route through the official AWS SDK for Rust.

## Changed

- added `swallowtail.amazon-bedrock.catalogue` over the distinct
  `rust-sdk-control-plane` transport with only the model-catalogue role
- added an independent binding for configured instance, access profile,
  credential reference, execution host, AWS region, and delegated credential
  provider
- configured `aws-sdk-bedrock = 1.148.0` directly with one exact approved
  endpoint, no ambient AWS chain, one unfiltered request, and one SDK attempt
- ran the SDK on an operation-private Tokio executor inside joined host
  blocking work
- projected bounded model, modality, streaming, inference, customization, and
  lifecycle observations without creating provider identity, entitlement,
  runtime capability, or model routes
- split the new core and adapter catalogue modules before they became new
  oversized-file debt

## Lifecycle And Authority

Catalogue deadlines signal the private executor and wait for it to finish
before delegated credential release. The model-catalogue role has no separate
public caller cancellation handle, so the driver does not claim one. Runtime
and catalogue endpoints, access profiles, descriptors, routes, permissions,
and SDK clients remain independent.

Default tests use injected SDK execution and generated AWS types. No live AWS
credential, account, network request, or paid inference participated.

## Validation

- 23 Bedrock tests pass, including five production catalogue-driver lifecycle
  cases, generated control-plane fixtures, projection drift, and the distinct
  Runtime conformance profile
- focused warnings-denied clippy passes
- full `effigy qa` passes with 291 tests; three installed or live probes remain
  gated and ignored by default
- `effigy doctor` returned to the inherited 19 findings: 12 warnings and 7
  errors
- `git diff --check` passes

## Remaining Risks

- live AWS IAM, credential refresh, regional catalogue variation, throttling,
  quota, account access, billing, and provider-side request acceptance remain
  unproved and separately gated
- local deadline cancellation cannot prove that AWS did not already receive a
  signed request
- generated SDK and provider catalogue drift require a new frozen corpus
- catalogue presence remains observation only; invocation readiness still
  needs a separately selected runtime route and access state

## Continuation Record

Roadmap 021 and cards 070-072 are complete. Roadmap 022 and card 073 own an
evidence-only post-SDK coverage checkpoint. No next provider is preselected.
