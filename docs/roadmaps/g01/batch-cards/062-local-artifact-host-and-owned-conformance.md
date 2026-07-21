# 062 Local Artifact Host And Owned Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../019-owned-llama-cpp-serving-proof.md`

## Objective

Implement deterministic local model-artifact authority and a real Contract 018
owned-self-hosted conformance profile.

## Governing References

- `../../../contracts/010-execution-host-services-and-inputs.md`
- `../../../contracts/011-runtime-conformance-profiles.md`
- `../../../contracts/018-owned-ephemeral-model-serving-lifecycle.md`
- `061-model-artifact-and-owned-serving-records.md`

## Scope

- exact local-host artifact approvals and read-only lease resolution
- digest and regular-file checks without ownership transfer
- scoped dynamic endpoint registration and invalidation for driver-observed
  loopback addresses
- recording services for artifact acquire/release and endpoint publish/release
- owned profile ordering, mismatch, failure, and redaction assertions

## Out Of Scope

- model download, conversion, mutation, deletion, or licensing
- llama.cpp startup parsing or HTTP behavior
- resource scheduling and persistent serving

## Acceptance Criteria

- [x] only pre-approved artifacts materialize on the exact execution host
- [x] a digest mismatch fails before process start
- [x] release never deletes consumer-owned artifact material
- [x] only loopback endpoint observations can become scoped grants
- [x] endpoint invalidation precedes artifact release and follows process join
- [x] local and remote-authoritative fixtures preserve the same public contract

## Evidence

- `LocalProcessHost` now requires an explicit execution-host binding for
  artifact and serving-endpoint authority.
- exact artifact approvals bind opaque reference, full descriptor, host path,
  regular-file state, and lowercase SHA-256 content before issuing a lease.
- artifact leases are tracked by scope, host, and reference; release drops
  authority with `NotApplicable` cleanup and never removes the source file.
- dynamic endpoint publication accepts only exact nonzero IPv4 or IPv6
  loopback HTTP sockets, issues an opaque scoped reference, and removes it on
  awaited release.
- the network service authorizes a published endpoint only for its exact scope
  and audience and rejects it after release.
- the owned synthetic profile records artifact acquisition before process
  start and endpoint publication, then process stop and join before endpoint
  and artifact release. Local and remote-authoritative host fixtures retain
  the same public bindings.

## Validation

- focused local-host and testkit tests
- focused clippy and all-target compile for touched crates
- `cargo fmt --all -- --check`
- `git diff --check`

## Validation Result

- 50 focused local-host and testkit tests pass
- focused warnings-denied clippy and all-target compile pass
- full repository QA passes with 236 tests; two installed/live probes remain
  separately gated and ignored
- docs QA, Northstar QA, formatting, whitespace, and diff checks pass
- doctor remains at the inherited 19 findings, including 7 errors

## Stop Conditions

- safe local resolution requires model-management policy
- dynamic endpoint publication can cross host or serving scope

## Auto-Continuation

No. Card 063 is ready for the exact llama.cpp b10069 owned-serving driver.
