# 005 Soundcheck Prepared Facade Adoption

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: g02.003 and g02.004 migration evidence
Vision tags: consumer adoption, structured runs, model catalogue
Contract refs: 002, 004, 008-010, 013, 023, 029, 032-033, 036-037
Planning state: cards 013-014 completed; card 015 ready

## Problem

Soundcheck compiles against the current low-level API while omitting runtime
version and configuration bindings. Its duplicated host and preflight helpers
therefore prove neither current compatibility nor a usable release handoff.

## Goals

- [x] Migrate model catalogue to the prepared app-server catalogue path.
- [x] Migrate bounded structured operations to the prepared Codex exec path.
- [x] Preserve prompts, schemas, screenshots, reasoning, search, progress,
      cancellation, validation, review, and proposal application.
- [x] Delete superseded host and preflight construction.
- [x] Add deterministic runtime preparation evidence; retain live acceptance
      behind its separate gate.
- [x] Record exact consumer-owned rollback.

## Non-Goals

- [ ] Do not move Soundcheck taxonomy or product workflow into Swallowtail.
- [ ] Do not merge app-server catalogue with exec structured runs.
- [ ] Do not change search, reasoning, schema, screenshot, or timeout policy.
- [ ] Do not add direct-provider fallback.
- [ ] Do not publish or release Soundcheck as part of adoption.

## Execution Plan

### Batch 5.1 — Catalogue And Structured-Run Migration

- [x] Execute card 013 only with consumer-repository authority.
- [x] Replace manual catalogue and exec planning with prepared paths.
- [x] Keep existing request, progress, terminal, and error projections.

### Batch 5.2 — Simplification And Acceptance

- [x] Execute card 014 after functional parity.
- [x] Remove duplicated host/preflight setup and stale tests.
- [x] Prove deterministic runtime preparation before gated installed or
      authenticated checks.

## Acceptance Criteria

- [x] Soundcheck uses prepared app-server catalogue and exec run paths
- [x] exact installed version is observed and bound before provider work
- [x] configuration posture and operation policy remain inspectable
- [x] schema, attachment, reasoning, search, cancellation, and timeout behavior
      remains unchanged
- [x] product validation and proposal application stay downstream
- [x] compile-only success can no longer hide invalid runtime preparation
- [x] rollback restores the prior pinned integration

## Risks And Mitigations

- Risk: exec and catalogue assumptions flatten. Mitigation: keep separate
  prepared objects and compatibility claims.
- Risk: optional search or screenshots lose host requirements. Mitigation:
  derive required capabilities and services from explicit request inputs.
- Risk: cleanup behavior changes behind a smaller API. Mitigation: retain
  terminal and cleanup evidence in focused tests.

## Evidence Requirements

- before/after ownership map and line-count delta
- deterministic catalogue and structured-run preparation fixtures
- schema, attachment, reasoning, search, cancellation, deadline, and cleanup
  tests
- gated installed catalogue and bounded structured-run probes where authorized
- Soundcheck Effigy health and QA
- consumer log and rollback instructions

## Decision Gate

Nucleus has proved the prepared interactive facade and the Codex
structured-run path remains contract-ready. Card 013 is ready under the
operator's prior direction to migrate both consumers. Editing, committing,
pushing, or releasing Soundcheck remains under Soundcheck authority.

Cards 013-014 are complete. Soundcheck passes health, 106 Rust tests, 13 Vitest
tests, the locked app check, and normal QA. Installed and authenticated Codex
probes remain separately gated. Card 015 is ready for packaged cross-consumer
runtime proof.
