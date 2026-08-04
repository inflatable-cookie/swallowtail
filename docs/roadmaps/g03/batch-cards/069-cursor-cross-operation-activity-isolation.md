# 069 Cursor Cross-Operation Activity Isolation

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../026-portable-activity-key-and-cross-operation-isolation.md`
Depends on: card 068

## Goal

Prove the Cursor ACP route safely retains repeated provider-backed and minted
activity ids across different runtime turns when consumers use `ActivityKey`.

## Scope

1. Add deterministic two-turn explicit-message-id reuse coverage.
2. Add deterministic two-turn absent-message-id fallback reuse coverage.
3. Verify standalone activity/provider values repeat while composite keys do
   not.
4. Reconcile observable-activity guidance, roadmap, and closeout evidence.

## Out Of Scope

- live Cursor, authentication, provider prompts, or installed probes
- consumer repository or database changes
- provider identity rewriting or transcript deduplication policy

## Acceptance Criteria

- [x] repeated explicit provider message ids remain operation-isolated
- [x] repeated fallback activity ids remain operation-isolated
- [x] public guidance uses the composite key
- [x] focused runtime/Cursor and affected-package verification pass
- [x] the sole Next Task returns to the g03 evidence gate

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-runtime swallowtail-adapter-cursor`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

Complete. The sole Next Task has returned to the g03 evidence gate.
