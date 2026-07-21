# 072 Bedrock Catalogue Driver And Conformance

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../021-bedrock-control-plane-catalogue-proof.md`

## Objective

Implement the separate Bedrock control-plane model-catalogue driver and prove
its exact hosted topology and access boundaries.

## Scope

- separately registered SDK-native model-catalogue driver
- exact host-approved control-plane endpoint, region, audience, access profile,
  delegated provider, response bound, deadline, and one attempt
- local and remote-authoritative deterministic conformance
- cancellation, non-paginated response bounds, errors, drift, redaction,
  credential release, and joined private executor cleanup
- roadmap and front-door closeout

## Acceptance Criteria

- [x] inference and catalogue service bindings remain independent
- [x] catalogue output never becomes implicit entitlement or route selection
- [x] default QA remains offline and credential-free
- [x] full QA passes or failures are recorded honestly

## Evidence

- a distinct `swallowtail.amazon-bedrock.catalogue` descriptor registers only
  the model-catalogue role over `rust-sdk-control-plane`
- `BedrockCatalogueBinding` independently fixes configured instance, access
  profile, credential reference, execution host, AWS region, and delegated
  provider
- production work uses one host-approved regional endpoint, one unfiltered
  `ListFoundationModels` call, one SDK attempt, and an operation-private Tokio
  executor inside joined blocking work
- deadline expiry signals the private executor, waits for completion, then
  releases the delegated credential; the catalogue role exposes no separate
  caller cancellation handle
- local and remote-authoritative deterministic fixtures prove exact endpoint,
  source-scoped projection, no model route or provider identity inference,
  drift rejection, provider failure, redaction, cleanup, and join behavior
- catalogue and core modules were split before closeout; doctor returned from
  22 findings to the inherited 19-finding baseline
- full `effigy qa` passes with 291 tests; three installed or live probes remain
  gated and ignored by default

## Validation

- focused adapter and conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- implementation requires a new product policy or shared contract

## Auto-Continuation

No. Return to the roadmap 021 checkpoint.
