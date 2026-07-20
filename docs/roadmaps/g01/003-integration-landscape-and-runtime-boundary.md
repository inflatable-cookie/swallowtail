# 003 Integration Landscape and Runtime Boundary

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

- [x] map the current harness, direct-model, protocol, and transport landscape
- [x] define the smallest host-neutral runtime boundary serving several
  materially different adapter shapes
- [x] settle ownership before introducing runtime traits, host ports,
  processes, transports, or provider adapters

## Governing Contracts

- Contract 001: Working Rules
- Contract 002: Repository Authority
- Contract 003: Portable Contract Kernel
- Contract 004: Runtime Ownership Boundary
- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- Contract 007: Model Artifact and Serving Boundary

## Execution Plan

- [x] inventory Nucleus interactive-session and Soundcheck structured-run
  requirements without copying product policy
- [x] inventory current official surfaces across target integration families
- [x] compare materially different harness, direct API, SDK, CLI, protocol,
  local-runtime, and remote-service shapes
- [x] compare credential mechanisms, entitlement and metering sources, endpoint
  audiences, and support authorities
- [x] decide host ownership, async posture, lifecycle, cancellation, redaction,
  and event delivery against that wider evidence
- [x] promote a testable runtime contract and compile implementation cards

## Cards

- `batch-cards/006-two-consumer-runtime-requirement-inventory.md` — completed
- `batch-cards/007-integration-family-and-transport-inventory.md` — completed
- `batch-cards/008-cross-adapter-runtime-decisions.md` — completed
- `batch-cards/009-runtime-contract-and-implementation-runway.md` — completed

## Acceptance Criteria

- [x] both consumer execution shapes have explicit common and distinct
  requirements
- [x] consumer product policy is excluded from shared mechanisms
- [x] local and remote execution-host topologies remain valid
- [x] integration family, driver, transport, instance, and model route remain
  distinct across the target inventory
- [x] execution layer, operation shape, and access-profile dimensions remain
  independent
- [x] runtime capability and failure behavior is testable across several
  materially different adapter shapes
- [x] implementation sequencing names clear stop conditions

## Stop Condition

No runtime crate or provider code is added during this planning milestone.
