# 071 Bedrock Catalogue SDK Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../021-bedrock-control-plane-catalogue-proof.md`

## Objective

Add the provider-neutral observation records fixed by Contract 020 and freeze
the exact generated Bedrock control-plane catalogue boundary selected by card
070.

## Scope

- provider-neutral bounded modality, lifecycle, streaming, inference-type,
  customization-type, and provider-extension observations
- version-pinned generated request, non-paginated output, model summary, and
  error types
- explicit region, endpoint audience, delegated credential provider, and one
  attempt
- bounded single-response projection and safe model evidence projection
- drift, malformed values, unknown enums, redaction, and no-fallback fixtures

## Acceptance Criteria

- [x] common observation records do not widen runtime capabilities or routes
- [x] fixtures compile against the exact selected SDK version
- [x] mutable catalogue evidence stays distinct from configured routes
- [x] provider payloads and access material stay out of diagnostics
- [x] default QA needs no AWS credential, account, or network

## Evidence

- `swallowtail-core` adds optional source-scoped catalogue observations for
  modalities, advertised streaming, inference types, customization types,
  lifecycle status and transitions, provider display name, and bounded
  provider-defined values
- optional repeated observations preserve absent versus observed-empty; an
  absent streaming field remains unknown while observed `false` stays false
- provider display names do not populate stable provider identity, observation
  metadata does not create a model route, and no observation becomes a runtime
  capability
- `swallowtail-adapter-bedrock` pins exact
  `aws-sdk-bedrock = 1.148.0` with default features disabled for this fixture
  batch
- generated fixtures cover the unfiltered request, single response collection,
  summaries, known and unknown enums, lifecycle timestamps, typed errors,
  explicit region and endpoint configuration, one attempt, bounds, and
  redaction
- the fixture projector rejects more than 1,024 entries, oversized or unsafe
  fields, and more than 32 values in one observation category; it never
  truncates and discards provider ARNs from common output
- 42 focused core and Bedrock tests pass
- focused warnings-denied clippy and workspace all-target checking pass
- `git diff --check` passes
- default validation used no AWS credential, account, endpoint, Marketplace
  action, or network request

## Validation

- focused adapter fixture tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- generated types expose an unresolved shared boundary

## Auto-Continuation

No. The corpus is stable. Card 072 now owns production mapping and conformance.
