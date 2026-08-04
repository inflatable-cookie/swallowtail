# 029 Kimi Operation Checkpoint, Reconciliation, And Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.028
Vision tags: provider continuity, exact recovery, controlled shutdown
Contract refs: 017, 042, 048, 049
Planning state: cards 073-075 completed

## Problem

Kimi local-server live reattachment retains an exact cursor only in process.
A crash or controlled detach therefore cannot prove the same provider turn
after restart.

## Goals

- [x] persist an exact portable operation checkpoint on qualified events
- [x] reconcile one exact Kimi turn through a finite retained event snapshot
- [x] detach an attached Kimi observer without provider abort
- [x] keep owned servers, callbacks, and unverified versions excluded

## Execution Plan

- [x] card 073: portable checkpoint record, persistence, event projection, and
  agreement correlation
- [x] card 074: Kimi finite cursor reconciliation, corpus, and restart proof
- [x] card 075: Kimi attached-turn detachment and detach-to-reconcile proof

## Boundaries

- no raw cursor parsing downstream
- no prompt replay, resume, callback answer, abort, or management side effect
- no idle-session terminal inference
- no owned foreground server detachment
- no authenticated provider work

## Acceptance Criteria

- [x] checkpoint restoration is exact and attachment-bound
- [x] terminal reconciliation requires exact retained turn evidence
- [x] stale, foreign, corrupt, and discontinuous checkpoints fail closed
- [x] explicit detach joins locally and sends no abort
- [x] ordinary close and cancellation remain unchanged
- [x] focused and affected-package validation pass

## Next Planning Checkpoint

Complete. Return to the g03 retained-operation evidence gate.
