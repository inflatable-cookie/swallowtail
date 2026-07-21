# 063 llama.cpp Owned Serving Driver

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../019-owned-llama-cpp-serving-proof.md`

## Objective

Implement the exact llama.cpp b10069 host-owned ephemeral lifecycle through
the existing bounded protocol facade.

## Governing References

- `../../../contracts/018-owned-ephemeral-model-serving-lifecycle.md`
- `../../../research/008-owned-llama-cpp-serving-lifecycle-evidence.md`
- `062-local-artifact-host-and-owned-conformance.md`

## Scope

- deterministic b10069 startup-output, readiness, mismatch, exit, and stop
  fixtures
- exact offline, loopback, port-zero, alias, no-UI, and no-agent launch mapping
- bounded startup parsing and dynamic endpoint publication
- health, build, single-route, and exclusion checks through reused protocol
  parsing
- graceful stop, owned escalation, process join, endpoint invalidation, and
  artifact release

## Out Of Scope

- installed or live model test in default QA
- b9910 attached-driver upgrade
- downloads, router mode, tools, UI, public listeners, API keys, or persistence

## Acceptance Criteria

- [x] no owned handle appears before exact readiness
- [x] malformed, duplicate, non-loopback, or mismatched startup evidence fails
      and joins cleanup
- [x] launch arguments contain only host-resolved executable and artifact
      material plus the contracted flags
- [x] catalogue and runs reuse the bounded llama.cpp facade without flattening
      owned and attached lifecycle
- [x] default diagnostics expose no artifact path, endpoint value, pid, or raw
      process output

## Evidence

- the production driver registers a distinct b10069 process-plus-HTTP identity
  while the b9910 attached identity retains no stop authority
- launch uses only the host-resolved executable and artifact plus `--alias`,
  loopback port zero, offline, no-UI, and no-agent flags
- bounded stderr parsing remains supervised through health, build, and route
  readiness, catching late duplicate records and child exit before handoff
- deterministic fixtures cover success, malformed and non-loopback endpoints,
  duplicate startup records, early exit, build mismatch, graceful stop, forced
  stop, process join, endpoint release, artifact release, and redaction
- 25 focused adapter tests and all 254 repository tests pass; focused warnings-
  denied clippy passes; doctor remains at the inherited 19 findings

## Validation

- focused adapter fixture and driver tests
- focused clippy and all-target compile for touched crates
- `cargo fmt --all -- --check`
- `git diff --check`

## Stop Conditions

- current b10069 behavior differs from the promoted fixture boundary
- a live artifact becomes necessary for deterministic lifecycle proof
- cleanup requires authority over an external or persistent service

## Auto-Continuation

No. Mark card 064 ready only after the owned driver passes deterministic
failure and success fixtures.
