# 007 Soundcheck Structured-Run Readiness

Status: completed
Owner: Tom
Updated: 2026-07-19

## Purpose

Close the shared structured-run gaps exposed by Soundcheck, prove the expanded
Codex exec route, and produce a bounded downstream adoption handoff without
moving Soundcheck product policy into Swallowtail.

## Generation Runway

This milestone advances the g01 consumer-adoption runway. It proves the smaller
one-shot consumer before interactive-session adoption and before any runtime
stability claim.

## Contracts

- Contract 002: Repository Authority
- Contract 004: Runtime Ownership Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles

## Goals

- [x] Express Soundcheck-required structured inputs without product types.
- [x] Materialize schema, screenshot, working, and deadline authority through
      host-owned services.
- [x] Expand Codex exec only for capabilities it can prove.
- [x] Produce a reversible Soundcheck-owned adoption seam and validation plan.

## Execution Plan

- [x] Contract batch: card 022 promotes exact structured-run inputs and host
      leases required by the consumer evidence.
- [x] Host batch: card 023 realizes scoped local materialization and deadline
      support without accepting arbitrary paths.
- [x] Driver batch: card 024 expands Codex exec and model-catalog behavior under
      explicit capability and policy checks.
- [x] Handoff batch: card 025 proves parity fixtures and records the downstream
      Soundcheck replacement and rollback boundary.

## Cards

- `batch-cards/022-soundcheck-structured-input-contract.md` — completed
- `batch-cards/023-local-materialization-and-deadline-services.md` — completed
- `batch-cards/024-codex-exec-capability-expansion.md` — completed
- `batch-cards/025-soundcheck-adoption-handoff.md` — completed

## Acceptance Criteria

- [x] No Soundcheck taxonomy, prompt, repair, ranking, or application type
      enters a Swallowtail production crate.
- [x] Schema, image, reasoning, network/search, and deadline requirements fail
      before provider work when unsupported.
- [x] Temporary host materialization is scoped, redacted, and cleaned up.
- [x] The expanded exec driver passes the one-shot profile without weakening
      its common assertions.
- [x] The handoff names an incremental consumer seam and rollback path; it does
      not modify Soundcheck.

## Handoff Artifact

- [Soundcheck Adoption Handoff](soundcheck-adoption-handoff.md)

## Checkpoint Result

The structured-run input API is proven for the bounded Soundcheck shape. It is
not generalized into session behavior. Roadmap 008 separately promotes the
session-option, callback, and topology rules required by Nucleus.

## Stop Condition

Stop before implementation if reasoning/search configuration or materialized
schema/attachment access cannot be represented without provider-specific
fields in common runtime records. Promote a namespaced extension contract
instead of widening the common API by intuition.
