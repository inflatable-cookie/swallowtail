# 158 Run-Loop And Activity Projector Extraction

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 157

## Goal

Extract the run-loop scaffold and the ACP event-to-activity projector so the
turn, session, pump, handle, and lifecycle families and the five near-identical
ACP activity modules share one implementation.

## Scope

1. Extract a run-loop scaffold (pump, handle, cancellation, cleanup, terminal
   projection) into `swallowtail-runtime`, parameterized by the route's event
   translation and terminal rules.
2. Extract the ACP event-to-activity projection into
   `swallowtail-protocol-acp` or `swallowtail-runtime` (pair-identical at 67%
   across kimi, claude-agent, gemini, grok, cursor).
3. Migrate the run-loop family in adapter batches; migrate the ACP activity
   modules in one batch since they share the same wire.

## Out Of Scope

- activity profile or event-shape changes
- provider-specific event translation (stays adapter-local)
- public API changes

## Acceptance

- [ ] the run-loop scaffold and projector have focused tests
- [ ] migrated adapters pass focused and extracted-package proof with an
      unchanged public API baseline
- [ ] the run-loop and activity duplication shrinks by the measured amounts

## Stop Conditions

- stop if a migrated adapter changes event delivery, terminal, or activity
  fidelity

## Auto-Continuation

Yes, to card 159 after acceptance.

## Validation

- focused validation per migrated adapter; `effigy package:verify-affected`
  per batch
- `effigy check:examples` and `effigy qa:routes` after activity touches
