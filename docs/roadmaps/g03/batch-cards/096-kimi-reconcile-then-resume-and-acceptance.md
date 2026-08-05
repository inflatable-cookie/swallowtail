# 096 Kimi Reconcile Then Resume And Acceptance

Status: planned
Owner: Tom
Created: 2026-08-05
Milestone: `../036-reconciliation-then-attachment-composition.md`
Depends on: card 095

## Goal

Map Kimi local-server exact-turn reconciliation to replay-free resume, then
close public and package acceptance for the composed workflow.

## Scope

1. Prepare exact reconciliation and resume bindings before provider work.
2. Resume only after exact eligible settled-turn evidence.
3. Report no replay for Kimi local-server attachment.
4. Update public guidance across all three composed routes.
5. Run focused and affected-package verification.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-codex swallowtail-adapter-opencode swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-codex swallowtail-adapter-opencode swallowtail-adapter-kimi`

## Stop Conditions

- stop if Kimi resume can race an active or uncertain provider turn
- stop if public guidance hides replay-bearing versus replay-free attachment

## Auto-Continuation

Continue to card 097 after g03.036 closes.
