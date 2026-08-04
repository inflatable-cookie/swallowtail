# 072 OpenCode Active-Turn Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../028-controlled-shutdown-active-operation-detachment.md`
Depends on: card 071

## Goal

Prove controlled detach, joined local cleanup, durable binding preservation,
and later reconciliation on the first qualified route.

## Scope

1. Add explicit prepared-profile selection for read-only sessions.
2. Bind durable provider state, exact detachment capability, and existing
   reconciliation support.
3. Stop the active SSE attachment without `/abort` and join it locally.
4. Return one local `Detached` terminal outcome without synthetic terminal
   provider activity.
5. Prove the persisted binding reconciles the same OpenCode session.
6. Publish route classification and consumer sequence.

## Acceptance Criteria

- [x] default and callback-enabled profiles expose no detachment
- [x] selected profile exposes active-turn detachment only
- [x] repeated detachment is idempotent
- [x] ordinary close still cancels
- [x] detach close sends no abort, prompt, delete, or callback response
- [x] all local work joins and attached server/session remain preserved
- [x] deterministic restart reconciliation observes the same session
- [x] roadmap, architecture, contract, guide, and closeout reconcile

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-opencode`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

Complete. The sole Next Task has returned to the g03 evidence gate.
