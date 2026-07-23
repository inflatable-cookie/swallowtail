# 2026-07-23 Harness Configuration Posture Records

## Changed

- Promoted Contract 033.
- Added `Ambient`, `ProviderSuppressed`, and `HostScoped` harness configuration
  records.
- Bound posture independently in configured instances, operation requirements,
  immutable plans, and runtime request policy.
- Kept absent posture as unmigrated state rather than an ambient alias.
- Added pure preflight rejection for direct inference and exact posture
  mismatch.
- Kept `HostScoped` non-executable until a separate opaque host lease and
  capability-scoped service exist.
- Added provider-neutral assertions over the existing structured-CLI and
  long-lived RPC harness profiles.

## Boundaries

- no configuration parsing, discovery, mutation, or deletion
- no temporary home, copied credentials, container, or provider implementation
- no implication from isolation, credentials, retention, or working resources
- no fallback between configuration postures
- no Codex branch in shared records or conformance

## Validation

- 66 focused core, runtime, and testkit tests pass
- workspace all-target check passes
- workspace warnings-denied clippy passes
- docs QA and diff checks pass

## Next

Card 117 freezes the January-to-April Codex exec and app-server corpora,
including configuration and retention behavior segments. Production driver
claims remain unchanged.
