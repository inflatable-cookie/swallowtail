# 120 Observable Activity Capability And Prepared Evidence

Status: planned
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

- [ ] a prepared consumer can inspect route fidelity without starting work
- [ ] a consumer may require exact activity constraints before effects
- [ ] thinner profiles remain usable without silent promotion
- [ ] the profile is derived from driver, operation, transport, and version
- [ ] unverified-newer evidence cannot widen a guaranteed profile
- [ ] prepared APIs require no per-provider event configuration
- [ ] low-level roles remain available

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

Continue to card 121 after capability, preparation, and public-API checks pass.

