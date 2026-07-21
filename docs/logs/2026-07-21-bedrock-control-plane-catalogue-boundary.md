# 2026-07-21 Bedrock Control-Plane Catalogue Boundary

## Outcome

Selected the exact native Bedrock control-plane discovery subset and promoted
the missing shared catalogue-observation boundary. Card 070 is complete; card
071 is ready for provider-neutral records and generated SDK fixtures.

## Current Evidence

- the current official Rust control-plane crate is
  `aws-sdk-bedrock = 1.148.0`
- native `ListFoundationModels` is one `GET /foundation-models` request with a
  single model-summary collection and no pagination token
- its only optional filters are provider, output modality, inference type, and
  customization type; the first proof uses none
- model summaries expose model id, name, provider, input and output modalities,
  advertised streaming, inference types, customization types, and Bedrock
  lifecycle evidence
- the regional `bedrock` control plane, `bedrock-runtime`, and the separately
  documented `bedrock-mantle` OpenAI-compatible endpoint are distinct service
  and protocol audiences
- `bedrock:ListFoundationModels` permission and catalogue presence do not prove
  runtime IAM, Marketplace entitlement, provider prerequisites, regional
  invocation availability, quota, billing, readiness, or request acceptance

No AWS credential, account, service endpoint, Marketplace action, or paid
request participated in this evidence batch.

## Changed

- added Research 014 with current official and generated-SDK evidence
- added Contract 020 for source-scoped catalogue observations, identity and
  availability separation, unknown-enum handling, bounds, one-attempt
  lifecycle, and side-effect-free discovery
- corrected roadmap 021 and cards 071-072 from paginated-page language to one
  bounded non-paginated response
- made card 071 the sole ready continuation
- recorded the Mantle `/models` catalogue as a separate future boundary rather
  than silently treating it as native Bedrock discovery

## Decisions

- catalogue-advertised streaming is observation evidence, not a Swallowtail
  runtime `Capability`
- Bedrock lifecycle remains source-scoped and cannot become universal route or
  support state
- unknown generated enum values may survive only as bounded namespaced
  provider observations and never imply common support
- model ARN stays adapter-private in the first subset and cannot replace model
  id or route identity
- response entry and field bounds remain required even though the operation is
  not paginated
- catalogue discovery performs no inference, agreement, subscription,
  Marketplace, onboarding, or access mutation

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and 7 errors

## Remaining Risks

- live IAM authorization, identity refresh, regional catalogue behavior, and
  account-specific results remain unproved and separately gated
- Bedrock service lifecycle documentation can advance ahead of generated SDK
  enums; the unknown-value fixture must preserve that drift safely
- native Bedrock and Mantle catalogue contents can differ and must not be
  merged without a later route and policy decision
- the exact provider-neutral observation record shape remains implementation
  work for card 071

## Continuation Record

Card 071 is ready. Add bounded provider-neutral catalogue observations and the
exact generated `aws-sdk-bedrock = 1.148.0` fixture corpus. Card 072 remains in
bounds for the separately registered production catalogue driver after the
corpus is stable.
