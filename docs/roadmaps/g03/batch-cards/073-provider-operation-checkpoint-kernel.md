# 073 Provider Operation Checkpoint Kernel

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../029-kimi-operation-checkpoint-reconciliation-and-detachment.md`
Depends on: card 072

## Goal

Persist one exact route-bound provider operation and event position without
exposing provider cursor structure to consumers.

## Scope

1. Add typed provider operation and persisted checkpoint records.
2. Bind session, runtime turn, provider turn, opaque cursor, and attachment.
3. Carry qualified checkpoints on runtime events.
4. Admit the checkpoint into exact reconciliation agreements.
5. Freeze corruption, version, size, and attachment failures.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`

Passed in the cards 073-075 validation batch.

## Auto-Continuation

Completed with cards 074-075 in the same validation batch.
