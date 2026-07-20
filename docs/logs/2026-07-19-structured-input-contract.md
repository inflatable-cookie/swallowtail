# Structured Input Contract

Date: 2026-07-19
Status: recorded

## Result

g01 card 022 is complete.

- model catalogs can report supported and default reasoning modes without
  selecting consumer policy
- structured runs carry explicit provider-side network, external-search, and
  optional reasoning selections
- reasoning modes and external search are exact preflight capabilities
- working-resource leases distinguish consumer-owned resources from
  operation-scoped temporary resources
- attachment and schema services can return redacted operation-scoped file
  leases usable by drivers
- deadline waits return monotonic observations and do not collapse timeout into
  cancellation
- recording fixtures prove object safety, rejection before provider effects,
  redaction, and cleanup authority

No Soundcheck source changed. Concrete local filesystem and deadline behavior
remains card 023.

## Evidence

- Contracts 008-011
- `crates/swallowtail-core/src/model.rs`
- `crates/swallowtail-runtime/src/operation_policy.rs`
- `crates/swallowtail-runtime/src/host_traits.rs`
- `crates/swallowtail-testkit/tests/runtime_preflight.rs`
- `crates/swallowtail-testkit/tests/runtime_skeleton.rs`

## Next Lane

Card 023 implements the scoped services in `swallowtail-host-local` with real
cleanup and deterministic deadline behavior.
