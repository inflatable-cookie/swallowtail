# 095 Codex And OpenCode Reconcile Then Load

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../036-reconciliation-then-attachment-composition.md`
Depends on: card 094

## Goal

Compose exact Codex and OpenCode reconciliation with their existing bounded
load/replay paths.

## Scope

1. Prepare both route-bound operations before dispatch.
2. Map only exact settled or inactive reconciliation outcomes to load.
3. Preserve ordered replay and the complete preceding observation.
4. Reject active, waiting, unknown, stale, foreign, and cross-operation state.
5. Prove first- and second-phase failure and cleanup independently.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-codex swallowtail-adapter-opencode`

## Stop Conditions

- stop if either adapter needs prompt, retry, cancellation, or import authority
- stop if session-scoped OpenCode truth is presented as exact turn truth

## Auto-Continuation

Continue to card 096 when both mappings pass.

## Outcome

Completed 2026-08-05. Codex app-server and OpenCode HTTP now compose their
prepared reconciliation with the existing managed load path. Immutable route
binding is checked before dispatch. Active Codex evidence remains observation
only; inactive OpenCode evidence proceeds to bounded replay-bearing load.
Focused validation passed 403 tests across the runtime and both adapters.
