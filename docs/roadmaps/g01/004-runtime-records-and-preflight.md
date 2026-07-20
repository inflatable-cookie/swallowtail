# 004 Runtime Records and Preflight

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Implement the pure identity, access, requirement, capability-constraint, and
preflight records needed to reject impossible executions before side effects.

## Contracts

- Contract 003: Portable Contract Kernel
- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 007: Model Artifact and Serving Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 011: Runtime Conformance Profiles

## Cards

- `batch-cards/010-runtime-identity-access-and-requirements.md` — completed
- `batch-cards/011-side-effect-free-preflight-and-fixtures.md` — completed

## Exit Criteria

- pure public records cover configured instance, route, access, ownership,
  host-service, and parameterized requirements
- preflight produces an immutable bound plan or dimensional safe failure
- testkit proves rejection occurs before recorded side effects
- core remains free of async runtime, transport, provider, secret, and consumer
  dependencies

## Closeout

- Core now owns the safe runtime identity, access, driver, instance, route,
  requirement, capability-constraint, and preflight vocabulary.
- Preflight is a pure function with no process, network, SDK, credential, or
  host-resolution ports.
- Testkit evidence covers every required rejection dimension, exact successful
  bindings, and stale instance revisions before recorded provider work.
- `effigy qa` and diff hygiene pass with 15 tests.
