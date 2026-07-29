# 120 Observable Activity Capability And Prepared Evidence

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../035-observable-agent-activity-kernel.md`
Depends on: card 119

## Goal

Describe exact activity fidelity before effects without adding consumer
configuration burden or another operation facade.

## Scope

1. Add a separate observable-activity capability.
2. Add immutable route activity profiles covering:
   - activity kinds
   - lifecycle fidelity
   - content streams
   - disclosure strength
   - correlation
   - unknown-event posture
3. Add exact capability constraints for consumers that require selected
   activity semantics.
4. Attach the derived profile to prepared operation evidence.
5. Preserve binary `StreamingEvents` as delivery truth.
6. Add exact failure-before-effects conformance.

## Out Of Scope

- provider-native event-name requirements
- profile inference from events after execution starts
- consumer persistence or presentation preferences
- adapter semantic rollout

## Acceptance Criteria

- [x] a prepared consumer can inspect route fidelity without starting work
- [x] a consumer may require exact activity constraints before effects
- [x] thinner profiles remain usable without silent promotion
- [x] the profile is derived from driver, operation, transport, and version
- [x] unverified-newer evidence cannot widen a guaranteed profile
- [x] prepared APIs require no per-provider event configuration
- [x] low-level roles remain available

## Result

- Added `ObservableActivity` beside unchanged binary `StreamingEvents`.
- Added exact activity kind, lifecycle, content-stream, disclosure,
  correlation, unknown-event, and interface-behavior profile evidence.
- Rich lifecycle and disclosure profiles satisfy explicitly thinner
  requirements without changing the inspected maximum fidelity.
- Prepared operation evidence now retains transport identity and one immutable
  available, unavailable, or not-applicable activity profile.
- Available profiles must match immutable preflight capability evidence and
  the qualified interface behavior revision.
- Unverified-newer interfaces inherit the latest qualified behavior revision;
  they cannot select a wider profile from the observed version.
- Existing adapters require no migration and claim no activity semantics yet.

## Validation

- `cargo test -p swallowtail-core`
- `cargo test -p swallowtail-runtime`
- focused prepared-operation tests
- `effigy package:api`
- `effigy check:rust`

## Stop Conditions

- Stop if the profile collapses operation or transport identity.
- Stop if a profile value would claim content not proved by fixtures.

## Auto-Continuation

Continue to card 121. Capability, preparation, workspace, and public-API checks
pass.
