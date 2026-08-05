# 2026-08-05 Alibaba Retained Conversation Recovery

Roadmap: `../roadmaps/g03/037-retained-session-recovery-promotion.md`
Card: 100

## Changed

- added a distinct prepared retained Alibaba conversation profile
- added exact resource-free open and load bindings
- retrieved exact conversation metadata before ascending item pagination
- enforced 10-page, 1,000-item, and 4 MiB aggregate replay bounds plus
  cross-page duplicate and cursor-progress checks
- returned complete ordered replay with one live continuation handle
- preserved retained conversations on ordinary close
- kept the existing operation-owned conversation delete-on-close path
  unchanged
- added separate prepared provider-session deletion using an exact management
  binding, the same bounded cursor walk, and item-before-conversation ordering
- classified missing and access-denied provider resources without exposing
  provider payloads

## Current State

`AlibabaModelStudioPreparedRetainedConversation` can open a retained
conversation, expose its resume and management bindings, load that exact
conversation after restart, replay its bounded transcript, and continue with
the existing turn contract. `AlibabaModelStudioPreparedDelete` is the only new
destructive path. A persisted resume binding alone still grants no deletion
authority.

Missing, foreign, malformed, oversized, stale, timed-out, and uncertain loads
return no handle. Retained close sends no deletion request. Management
cancellation before dispatch reports failed-before-effect; a failure after an
item deletion reports unconfirmed-after-effect and does not delete the parent
conversation.

No authenticated provider work, external request, conversation mutation,
prompt, or paid inference ran.

## Validation

- Alibaba package tests passed
- card-required focused runtime and Alibaba validation passed: 174 tests
- affected Alibaba package verification passed
- `git diff --check`

## Next Move

Execute card 101. Publish final retained-session facade and route truth, retain
Pi and other blocked candidates honestly, and close g03.037.
