# 100 Alibaba Retained Conversation Recovery

Status: complete
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

## Completion

- [x] added a separate prepared retained-conversation profile with preserved
      provider state and no owned-resource deletion capability
- [x] retained open issues exact resource-free resume and management bindings
- [x] load validates the binding, retrieves exact metadata, follows bounded
      ascending pages, and returns complete ordered replay before readiness
- [x] loaded sessions continue through the existing exact turn path without
      inferring interrupted-turn state
- [x] ordinary retained close joins local work, releases access, and sends no
      provider deletion request
- [x] the existing operation-owned profile still deletes items before its
      conversation on close
- [x] explicit retained cleanup requires a separate management binding and
      preserves failed-before-effect versus unconfirmed-after-effect truth
- [x] stale, foreign, missing, malformed, oversized, and deadline-bound loads
      return no usable handle
- [x] focused adapter and runtime validation passed without authenticated
      provider work

Card 101 is ready for public route, facade, package, and remaining-gate
acceptance.
