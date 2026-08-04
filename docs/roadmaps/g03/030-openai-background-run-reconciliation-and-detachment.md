# 030 OpenAI Background Run Reconciliation And Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.029
Vision tags: provider continuity, exact recovery, controlled shutdown
Contract refs: 021, 042, 048, 049
Planning state: cards 076-078 completed

## Problem

The OpenAI background route can recover one dropped SSE attachment in process,
but it deletes terminal responses and carries no durable response/cursor record
across consumer restart. The session reconciliation vocabulary cannot honestly
represent a structured run with no provider session.

## Goals

- [x] persist an exact route-bound provider run and opaque cursor
- [x] reconcile one OpenAI background response without creating or controlling work
- [x] detach one explicitly selected background run without cancel or delete
- [x] keep ordinary close, cancellation, deadline, and terminal deletion unchanged

## Execution Plan

- [x] card 076: portable run checkpoint and reconciliation kernel
- [x] card 077: OpenAI exact response reconciliation and corpus
- [x] card 078: OpenAI structured-run detachment and restart proof

## Boundaries

- no provider session manufactured around a structured run
- no raw response id or cursor admission downstream
- no create, retry, prompt, stream attach, cancel, delete, callback, or session
  request during reconciliation
- no detach before a recoverable checkpoint
- no authenticated provider work

## Acceptance Criteria

- [x] persisted checkpoints reject corruption, drift, foreign runs, and oversize
- [x] reconciliation returns exact state plus bounded terminal output and usage
- [x] explicit detach joins locally and sends no cancel or delete
- [x] ordinary terminal cleanup still deletes the provider response
- [x] focused and affected-package validation pass

## Next Planning Checkpoint

Complete. Continue with g03.031 card 079 to qualify ACP retained-history
reconciliation evidence without mislabelling history as a live turn.
