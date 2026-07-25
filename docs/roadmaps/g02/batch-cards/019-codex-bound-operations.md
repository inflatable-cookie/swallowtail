# 019 Codex Bound Operations

Status: completed
Owner: Tom
Created: 2026-07-25
Completed: 2026-07-25
Milestone: `../007-provider-wide-facade-contract-and-foundation.md`

## Objective

Complete Codex as the reference facade by binding prepared values directly to
their matching low-level operations.

## Governing Refs

- Contracts 009-013, 029, 032-034, and 037
- cards 008-010 and 018
- Codex prepared-integration guide

## Scope

1. Add separate typed entry points for structured exec, catalogue observation,
   read-only session, and bounded-workspace session.
2. Accept explicit operation content at start/open time.
3. Reuse the existing Codex drivers and lifecycle unchanged.
4. Expose safe prepared evidence and low-level parts.
5. Update the Codex guide and compile-tested examples.

## Acceptance Criteria

- [x] consumers do not instantiate a Codex driver or matching request manually
- [x] exec and app-server remain separate
- [x] read-only and bounded-workspace authority remain separate
- [x] version, access provenance, configuration posture, and plan stay visible
- [x] cancellation, deadlines, callbacks, interruption, and cleanup are
      unchanged

## Validation

- focused Codex facade and low-level suites
- local and remote-authoritative fixtures
- public docs and examples
- `effigy test --plan`-selected validation

## Execution Evidence

- prepared catalogue delegates `list_models` to `ModelCatalogDriver`
- prepared exec delegates `start_run` to `StructuredRunDriver`
- prepared sessions delegate `open_session` and validated `resume_session` to
  `InteractiveSessionDriver`
- every bound method carries the immutable plan, explicit request, and
  caller-supplied host services into the existing role
- `low_level_driver`, `plan`, `request`, and `into_parts` remain public
- local and remote-authoritative prepared tests execute every profile
- compile-tested guidance shows the bound normal path
- all nine prepared-profile tests and full repository QA pass
- the held candidate baseline is unchanged; card 036 owns its replacement

## Auto-Continuation

No. g02.007 is closed. Card 020 is ready.
