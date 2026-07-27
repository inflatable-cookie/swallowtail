# 047 Provider Session Management Runtime Role

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../015-provider-session-management-foundation.md`

## Objective

Add one low-level scoped runtime role and shared prepared evidence for archive,
restore, or deletion of a bound inactive provider session.

## Governing Refs

- Contract 038
- Contracts 008-010, 017, 029, and 037
- card 046

## Scope

1. Add side-effect-free management plan validation.
2. Add separate typed archive, restore, and delete requests over one common
   driver role.
3. Require exact binding, action, capability, compatibility, host, target,
   access, deadline, and promised-result agreement before effects.
4. Represent the dispatch effect boundary and unconfirmed destructive result.
5. Reuse scoped host services, cancellation, deadlines, request/rate evidence,
   and joined access release.
6. Add shared prepared-operation evidence without a central provider router.
7. Leave provider-native active close in handle cleanup.

## Acceptance Criteria

- [x] inactive target is explicit without a global session registry
- [x] unsupported and drifted requests stop before provider effects
- [x] cancellation and deadline before dispatch cause no effect
- [x] loss after dispatch returns unconfirmed truth
- [x] no retry, fallback, provider search, or active-handle discovery appears
- [x] low-level role and adapter-local prepared use remain separately usable

## Evidence

- `ProviderSessionManagementPlan` fixes preflight, binding, action, initial
  state, scope, inactive evidence, cancellation posture, and deadline.
- archive, restore, and delete have separate request types and driver methods;
  request-plan or host-service drift fails before adapter effects.
- the common outcome retains the exact binding, effect truth, provider request
  reference, rate evidence, and safe diagnostic without strengthening
  unconfirmed deletion.
- `PreparedProviderSessionManagementEvidence` reuses the provider-neutral
  prepared-operation record without adding a central router.
- the role has no registry, retry, fallback, active-handle discovery, consumer
  state, or provider-native close method.
- 49 core and 66 runtime tests pass; focused Clippy and all-target workspace
  compilation pass.

## Validation

- focused runtime tests
- core/runtime public API checks
- `effigy check:rust`
- `git diff --check`

## Stop Conditions

- the role requires consumer confirmation or persistence
- one generic state setter would replace typed actions
- active-session discovery needs process-global state
- joined cleanup cannot preserve destructive uncertainty

## Auto-Continuation

Yes after card 046 completes. Continue to card 048.
