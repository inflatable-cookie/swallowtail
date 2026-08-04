# 028 Controlled Shutdown Active Operation Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.027
Vision tags: provider continuity, controlled shutdown, lifecycle truth
Contract refs: 009, 017, 042, 048, 049
Planning state: cards 071-072 completed

## Problem

Ordinary handle close may cancel active work even when an attached provider can
retain it and Swallowtail can reconcile the same bound session after restart.
Drop cannot stand in for joined detachment.

## Goals

- [x] add optional portable detachment control and local terminal truth
- [x] keep ordinary close and cancellation semantics unchanged
- [x] realize opt-in OpenCode read-only active-turn detachment
- [x] prove detach, close, restart binding, and reconciliation compose
- [x] publish exact gates for other route families

## Execution Plan

- [x] card 071: portable capability, scope, control, acknowledgement, terminal,
  and lifecycle tests
- [x] card 072: OpenCode prepared selection, driver detachment, corpus,
  composition acceptance, public guidance, and package proof
- [x] return to the g03 evidence gate with Kimi and retained-operation records
  left as explicit candidates

## Boundaries

- no fallback from requested detachment to provider cancellation
- no provider terminal inference
- no task, process, callback, credential, or resource leak
- no callback-bearing, structured-run, owned-process, or delete-on-close claim
- no authenticated provider work

## Acceptance Criteria

- [x] unsupported handles expose no detachment control
- [x] admitted detachment is idempotent and cancellation wins races
- [x] local terminal outcome is `Detached`
- [x] OpenCode sends no abort or deletion and joins the SSE attachment
- [x] the exact persisted binding reconciles the same provider session later
- [x] focused and affected-package validation pass

## Next Planning Checkpoint

Complete. The sole Next Task has returned to the g03 evidence gate. Kimi
local-server detachment still requires its separate exact cursor
reconciliation tranche first.
