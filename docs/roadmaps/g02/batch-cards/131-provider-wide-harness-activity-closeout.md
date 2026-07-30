# 131 Provider-Wide Harness Activity Closeout

Status: completed
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

- [x] every production harness route has one exact profile
- [x] no profile relies on consumer provider parsing
- [x] no unknown semantic event silently disappears
- [x] version milestones and unverified-newer behavior are explicit
- [x] all harness packages and public APIs compile
- [x] exact gaps remain evidence-labelled

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

## Completion Evidence

- the machine-checked inventory accounts for 13 canonical production harness
  routes and 18 positive ordinary prepared-operation profiles
- every structured-run and interactive-session profile is available through
  public prepared evidence; there is no unexplained whole-profile gap
- catalogue and provider-session-management operations retain exact
  not-applicable classifications where the route does not expose those roles
- provider-specific adapter tests assert the public prepared profile without
  asking consumers to decode provider events
- the public guide records exact route fidelity, thinness, and non-ordinary
  operation boundaries
- the complete workspace regression and required formatting, Rust, docs,
  route, and public-API gates pass
- no live credential, executable, account, model request, paid inference, or
  consumer repository was used
