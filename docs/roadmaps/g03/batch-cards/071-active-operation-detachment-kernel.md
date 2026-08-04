# 071 Active Operation Detachment Kernel

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../028-controlled-shutdown-active-operation-detachment.md`
Depends on: card 070

## Goal

Add an optional handle control that ends local observation without asserting or
requesting provider termination.

## Scope

1. Add the detachment capability and exact run/turn scope.
2. Add object-safe control and idempotent acknowledgement.
3. Expose optional control from run and turn handles.
4. Add `TerminalStatus::Detached` as local, non-provider-terminal truth.
5. Freeze unsupported, idempotent, terminal, and cancellation boundaries.

## Acceptance Criteria

- [x] detachment is distinct from cancellation
- [x] unsupported handles remain unchanged through a default `None`
- [x] no consuming unsupported path can leak a handle
- [x] terminal outcome can represent local detachment without provider failure
- [x] portable tests pass

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`

## Auto-Continuation

Completed with card 072 in the same validation batch.
