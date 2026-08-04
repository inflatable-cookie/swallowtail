# 2026-08-04 Provider Recovered Resource Cleanup Kernel

Roadmap: `../roadmaps/g03/033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Card: `../roadmaps/g03/batch-cards/084-provider-recovered-resource-cleanup-kernel.md`

## Changed

- added `WaitingForProviderInput` as distinct non-terminal run truth
- added a versioned, bounded, integrity-checked cleanup binding that cannot be
  restored from a run checkpoint or against a different prepared route
- bound one runtime run and provider run to typed owned-resource kinds plus an
  adapter-private exact-resource record
- added the separate recovered-cleanup capability, operation shape, driver role,
  cancellation scope, immutable plan/request, prepared evidence, and outcome
- kept active or unknown resources intact through explicit rejected effect truth
- kept partial and after-effect uncertainty distinct from complete cleanup
- froze corruption, oversize, version, drift, cross-operation, deadline,
  cancellation-scope, active-resource, and partial-effect coverage

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime` — 194 tests passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime` — both
  extracted packages compiled
- `cargo fmt --all`

No authenticated provider work, provider request, resource deletion, or
consumer edit ran.

## Next Move

Execute card 085. Emit the Anthropic checkpoint and cleanup binding before work
can be lost, then map bounded exact reconciliation and inactive-only cleanup.
