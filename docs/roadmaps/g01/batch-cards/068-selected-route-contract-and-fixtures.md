# 068 Selected Route Contract And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../020-post-portability-coverage-expansion.md`

## Objective

Promote the embedded-SDK boundary selected by card 067 and freeze the exact
Bedrock Runtime `ConverseStream` SDK corpus.

## Scope

- Contract 019 embedded SDK, explicit configuration, delegated credential-
  provider, one-attempt, and private executor rules
- `swallowtail-adapter-bedrock` fixture-first crate pinned to
  `aws-sdk-bedrockruntime = 1.136.0`
- typed text delta, stop, usage, throttling, model-stream failure, unknown
  semantic variant, cancellation, redaction, and bounded projection fixtures
- exact Bedrock Runtime endpoint audience, region, route, model, credential,
  support, and cloud-account billing pins
- explicit exclusion of catalogue, tools, guardrails, prompt resources,
  cross-region profiles, ambient AWS configuration, and live authentication

## Acceptance Criteria

- [x] the selected route and authority are fixed by card 067
- [x] shared rules contain no provider identity branch
- [x] deterministic fixtures fail closed on version and capability drift
- [x] live credentials and external inference remain outside default QA

## Evidence

- Contract 019 keeps SDK identity, concrete executor, endpoint, credential,
  configured instance, cloud service, gateway, model provider, route, and model
  separate without adding an AWS branch to common runtime code
- `swallowtail-adapter-bedrock` links the exact provider-supported
  `aws-sdk-bedrockruntime = 1.136.0` crate with default network/runtime features
  disabled for the fixture batch
- the manifest pins Bedrock Runtime audience, configured-instance region,
  delegated cloud identity, cloud-account billing, provider support, one
  attempt, consumer-owned output bound, exact route, bounded projection, and
  all first-subset exclusions
- generated SDK builders produce the success corpus; the decoder preserves
  text, stop, and token-usage order and rejects missing, reordered, unknown,
  and unsupported semantic events
- generated `ConverseStreamError` values prove safe throttling and mid-stream
  model-failure classification without retaining raw provider messages
- an exact generated `RetryConfig` fixture proves maximum attempts is one
- six focused tests pass without an endpoint, AWS configuration, credential,
  account, or inference request
- full repository QA passes with 277 tests; three installed/live probes remain
  gated and ignored by default
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors; no new finding was added
- card 069 has no remaining protocol or access-policy decision and may proceed

## Validation

- focused fixture tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- card 067 is incomplete
- the selected route needs unresolved product policy
- stable fixtures require live credentials or paid inference

## Auto-Continuation

Continue to card 069 only when the production boundary is unambiguous.
