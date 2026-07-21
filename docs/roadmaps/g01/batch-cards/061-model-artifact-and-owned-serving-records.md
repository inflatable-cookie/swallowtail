# 061 Model Artifact And Owned Serving Records

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../019-owned-llama-cpp-serving-proof.md`

## Objective

Realize Contract 018's provider-neutral artifact authority and owned-serving
handoff without provider or host implementation.

## Governing References

- `../../../contracts/007-model-artifact-and-serving-boundary.md`
- `../../../contracts/008-runtime-registration-and-preflight.md`
- `../../../contracts/009-async-operation-lifecycle.md`
- `../../../contracts/010-execution-host-services-and-inputs.md`
- `../../../contracts/018-owned-ephemeral-model-serving-lifecycle.md`

## Scope

- safe model-artifact identity and opaque host reference
- read-only, scope-bound, execution-host artifact lease and explicit release
- required host-service registration and preflight facts
- owned start deadline accessor and artifact binding
- redacted host-scoped serving endpoint binding on the owned handle
- pure record, mismatch, redaction, and zero-side-effect fixtures

## Out Of Scope

- filesystem-backed artifact resolution
- process launch, startup parsing, health probes, or provider mapping
- persistent ownership, downloads, or artifact mutation

## Acceptance Criteria

- [x] model artifacts do not reuse attachment records or cleanup authority
- [x] reference, scope, execution host, access, digest, and artifact identity
      remain inspectable without exposing materialized values
- [x] artifact and endpoint host services are independently optional and
      preflight-addressable
- [x] start requests expose their deadline and exact artifact binding
- [x] owned handles expose only a redacted same-host endpoint reference
- [x] mismatches fail before process or network attempts
- [x] existing attached and synthetic profile APIs remain source-coherent

## Evidence

- `swallowtail-core` now separates safe artifact identity and digest metadata
  from one opaque host artifact reference and binds it into preflight plans.
- `swallowtail-runtime` adds read-only artifact leases, independently optional
  artifact and serving-endpoint host ports, and redacted endpoint bindings.
- owned start now carries an explicit scope, exact artifact binding, and one
  required monotonic deadline; common validation rejects host, ownership,
  artifact, or service drift before effects.
- the owned handle exposes execution-host identity and a scoped endpoint
  reference without becoming an inference API.
- the synthetic owned profile no longer materializes an attachment as a fake
  model. It proves missing-artifact preflight rejection, substitution rejection
  before all recorded effects, artifact acquire/release, endpoint publication,
  redaction, and owned stop behavior.

## Validation

- focused core, runtime, and testkit tests
- focused clippy and all-target compile for touched crates
- `cargo fmt --all -- --check`
- `git diff --check`

## Evidence Required

- public record and trait tests
- recording-host zero-side-effect assertions
- redaction assertions for artifact and endpoint material
- affected profile results

## Validation Result

- 81 focused core, runtime, and testkit tests pass
- full repository QA passes with 230 tests; the Gemini and OpenCode installed
  probes remain separately gated and ignored
- warnings-denied clippy, all-target workspace compile, formatting, docs QA,
  Northstar QA, whitespace, and diff checks pass
- doctor is unchanged at the inherited 19 findings, including 7 errors

## Stop Conditions

- one record would collapse artifact, deployment, route, or endpoint identity
- the endpoint binding requires ambient registration or a global serving map
- a provider-specific readiness field would enter common records

## Auto-Continuation

No. Card 062 is ready for concrete local-host authority and expanded lifecycle
ordering fixtures.
