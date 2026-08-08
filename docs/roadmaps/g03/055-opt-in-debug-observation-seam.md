# 055 Opt-In Debug Observation Seam

Status: completed
Owner: Tom
Created: 2026-08-08
Depends on: g03.047, g03.041
Vision tags: consumer stability, safe diagnostics, compatibility maintenance
Contract refs: 003, 004, 009, 010, 044, 051, 053
Planning state: cards 169-171 completed

## Problem

Harness and provider failures often surface only as a safe code and redacted
message. Bounded safe excerpts help on some Codex paths, but hosts still lack
a general opt-in channel for restricted wire, lifecycle, process, and
classification context. The `DiagnosticObserver` skeleton and
`Diagnostic.internal_detail` exist; adapters almost never emit through them.

## Goals

- [x] realize structured `DebugObservation` records and fail-soft emit helpers
      in runtime under Contract 053
- [x] evolve `DiagnosticObserver` without breaking ordinary integration
- [x] prove Codex app-server emits correlated debug observations on a known
      malformed-inbound failure while keeping its exact safe diagnostic
- [x] document host opt-in wiring for consumers and operators

## Execution Plan

- [x] Execute card 169 (runtime observation vocabulary and emit path).
- [x] Execute card 170 (Codex proof emissions).
- [x] Execute card 171 (guide, example host, failure-guide cross-link).

## Boundaries

- no second public event stream or global telemetry bus
- no widening of `SafeDiagnostic` into raw wire dumps
- no change to classification, terminal truth, cleanup, or route selection
- no consumer-repo commits; Swallowtail owns the seam and guidance only
- no tag, release, registry publication, or live provider work in this
  milestone unless a later card explicitly requires the active API baseline
  check

## Acceptance Criteria

- [x] Contract 053 acceptance items covered by runtime and Codex fixtures
- [x] focused runtime and Codex package validation passes
- [x] public API stays within the operator-authorized candidate baseline
      policy for this lane
- [x] guides show opt-in host registration and state that ordinary apps need
      not register an observer

## Next Planning Checkpoint

Return to the g03 evidence gate. The `v0.3.0` source candidate remains a
separate operator-authorized release step.
