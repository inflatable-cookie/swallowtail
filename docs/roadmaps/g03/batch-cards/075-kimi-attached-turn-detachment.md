# 075 Kimi Attached-Turn Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../029-kimi-operation-checkpoint-reconciliation-and-detachment.md`
Depends on: card 074

## Goal

Close one qualified Kimi observer without aborting provider work, then
reconcile the same exact turn after restart.

## Scope

1. Add explicit prepared-session selection.
2. Exclude callbacks, owned servers, structured runs, and unverified versions.
3. Close and join only the WebSocket observer.
4. Return local `Detached` truth without synthetic provider terminal state.
5. Prove persisted checkpoint to exact active reconciliation.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-kimi`
- `effigy qa:docs`
- `git diff --check`

All passed. Focused validation ran 291 tests. Affected-package extraction
verified core, runtime, and Kimi packages.

## Auto-Continuation

Complete. Return to the g03 retained-operation evidence gate.
