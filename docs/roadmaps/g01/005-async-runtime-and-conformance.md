# 005 Async Runtime and Conformance

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Build the object-safe async runtime skeleton, scoped lifecycle, host services,
and deterministic cross-shape conformance profiles.

## Contracts

- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles

## Cards

- `batch-cards/012-runtime-crate-and-role-traits.md` — completed
- `batch-cards/013-operation-handles-events-and-outcomes.md` — completed
- `batch-cards/014-host-services-inputs-and-recording-fixtures.md` — completed
- `batch-cards/015-synthetic-cross-shape-conformance.md` — completed
- `batch-cards/016-runtime-kernel-validation-and-promotion.md` — completed

## Exit Criteria

- dynamic role registration compiles without a global executor
- run, session, turn, and serving handles obey ordered event, terminal, cancel,
  and cleanup contracts
- capability-scoped host services preserve authority and redaction boundaries
- all five synthetic conformance profiles pass
- runtime architecture is promoted from plan to realized state

## Closeout

- The executor-neutral runtime separates structured runs, interactive
  sessions, and attached or owned serving lifecycles behind object-safe roles.
- Ordered bounded events, first-wins terminal outcomes, scoped cancellation,
  cleanup evidence, and capability-scoped host ports are realized.
- Five provider-free profile runners prove all common Contract 011 assertions
  plus their shape-specific authority boundaries.
- Full QA passes with 38 tests. The crate graph contains only core, runtime,
  testkit, and `futures-core` in the runtime layer.
