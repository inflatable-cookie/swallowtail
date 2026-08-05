# 095 Codex And OpenCode Reconcile Then Load

Status: planned
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
