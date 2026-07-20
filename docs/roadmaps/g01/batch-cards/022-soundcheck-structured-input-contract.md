# Soundcheck Structured-Input Contract

Status: completed
Owner: Tom
Roadmap: 007 Soundcheck Structured-Run Readiness
Updated: 2026-07-19

## Goal

Promote the exact provider-neutral runtime and host-service records needed to
represent Soundcheck's bounded structured-run inputs before implementation.

## Governing References

- `docs/architecture/consumer-runtime-evidence.md`
- Contracts 004 and 008-011
- `docs/roadmaps/g01/007-soundcheck-structured-run-readiness.md`

## Scope

- reasoning-mode requirement and selected operation value
- explicit harness network/search policy without implicit fallback
- schema and image attachment materialization leases usable by a driver without
  exposing raw consumer paths
- operation-scoped temporary working resource and cleanup ownership
- deadline observation and cancellation/timeout distinction
- model-catalog reasoning metadata required for explicit consumer selection
- conformance cases for every added field and authority boundary

## Out Of Scope

- Soundcheck source changes
- taxonomy schemas, prompts, validation, repair, ranking, or proposal apply
- Codex command construction
- hosted direct API behavior
- generic schema validation

## Acceptance Criteria

- every field has provider-neutral meaning or an explicit extension namespace
- unsupported values reject during preflight or before provider side effects
- leases expose only bounded host-authorized material and remain redacted
- consumer-owned temporary resources and Swallowtail-owned materializations have
  distinct cleanup authority
- model metadata remains mutable presentation/capability evidence, not routing
  policy
- contract and conformance updates are sufficient to implement cards 023-024
  without inventing new behavior

## Validation

- Rust compile probes for object-safe host leases and request records
- testkit rejection and redaction fixtures
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop if web/search or reasoning selection cannot be modeled without a
provider-specific common field. Use a declared Codex extension instead.

## Closeout

- `ReasoningMode` is provider-neutral catalog evidence and an exact capability
  constraint; defaults never select an operation value.
- every structured run now carries explicit provider-side network, external
  search, and optional reasoning policy.
- external search cannot imply network authority, and missing search,
  reasoning-mode, or schema-service support rejects before provider effects.
- working-resource leases distinguish consumer cleanup from operation-scoped
  temporary cleanup.
- attachment and schema file leases expose only host-authorized redacted
  materializations to drivers.
- deadline waits return explicit monotonic observations; cancellation and
  timed-out terminal states remain separate.
- public compile probes and recording fixtures cover the new fields, object-safe
  host ports, redaction, and cleanup authority without changing Soundcheck.
