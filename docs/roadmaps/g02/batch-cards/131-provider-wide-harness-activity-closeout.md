# 131 Provider-Wide Harness Activity Closeout

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../038-non-acp-harness-activity-coverage.md`
Depends on: card 130

## Goal

Close observable-activity coverage across every production harness route.

## Scope

1. Build a machine-checked harness-route activity inventory.
2. Verify every positive profile through a public prepared operation.
3. Classify exact unavailable and not-applicable activity dimensions.
4. Run all harness adapter, compatibility, facade, lifecycle, and callback
   regressions.
5. Update public harness guidance.
6. Select direct-inference applicability as the next lane.

## Out Of Scope

- direct inference implementation
- consumer edits
- package candidate replacement
- live breadth testing

## Acceptance Criteria

- [ ] every production harness route has one exact profile
- [ ] no profile relies on consumer provider parsing
- [ ] no unknown semantic event silently disappears
- [ ] version milestones and unverified-newer behavior are explicit
- [ ] all harness packages and public APIs compile
- [ ] exact gaps remain evidence-labelled

## Validation

- all harness adapter tests
- `effigy format:check`
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy qa:routes`
- `effigy package:api`

## Stop Conditions

- Do not close with an unexplained harness activity gap.
- Keep one route paused rather than weakening provider-wide truth.

## Auto-Continuation

Continue to card 132 only after roadmap g02.038 closes.

