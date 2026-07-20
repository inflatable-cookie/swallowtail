# Local Materialization And Deadline Services

Status: completed
Owner: Tom
Roadmap: 007 Soundcheck Structured-Run Readiness
Updated: 2026-07-19

## Goal

Implement the Contract 010 local-host services required for bounded schema,
attachment, temporary-resource, and deadline use.

## Scope

- approved schema and attachment materialization
- temporary-file or bounded-byte leases
- operation-scoped cleanup and ownership evidence
- monotonic deadline service and cancellable waits
- limits, redaction, and deterministic fixtures

## Out Of Scope

- provider protocol
- arbitrary paths or URLs
- Soundcheck scratch-directory ownership
- schema meaning or validation

## Acceptance Criteria

- only approved opaque references materialize
- cleanup joins all host-owned temporary work
- deadline expiry remains distinct from operator cancellation
- raw paths and materialized content remain absent from safe output

## Validation

- deterministic local-host fixtures
- applicable Contract 011 assertions
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop if card 022 does not settle lease access and cleanup ownership.

## Closeout

- the local host copies only approved attachment and schema sources within
  host-configured byte limits
- materialized file references expose driver values explicitly while default
  formatting remains redacted
- temporary working resources are opaque, operation-scoped, and accepted by
  local processes only inside their owning scope
- explicit resource, attachment, and schema release methods finish removal
  before returning cleanup evidence; consumer-owned resources remain intact
- monotonic deadline waits resolve with observations, and dropping a wait
  cancels and joins its worker
- deterministic fixtures cover authority rejection, bounds, redaction,
  cross-scope rejection, real process use, cleanup, expiry, and cancellation
