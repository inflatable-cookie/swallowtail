# 100 Alibaba Retained Conversation Recovery

Status: ready
Owner: Tom
Created: 2026-08-05
Milestone: `../037-retained-session-recovery-promotion.md`
Depends on: card 098

## Goal

Implement retained Alibaba conversation load/replay and prepared continuation
recovery under the separately qualified profile.

## Scope

1. Add a distinct prepared retained-conversation operation.
2. Retrieve exact conversation metadata and bounded ordered items before
   returning readiness.
3. Return one live session without interrupted-turn state inference.
4. Keep ordinary operation-owned delete-on-close behavior unchanged.
5. Prove exact binding, failure uncertainty, cancellation, and cleanup.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-alibaba-model-studio`
- `effigy package:verify-affected swallowtail-adapter-alibaba-model-studio`

## Stop Conditions

- stop if recovery requires silent retention or skipped ordinary cleanup
- stop if conversation retrieval cannot establish exact route ownership

## Auto-Continuation

Continue to card 101 when Alibaba passes independently.
