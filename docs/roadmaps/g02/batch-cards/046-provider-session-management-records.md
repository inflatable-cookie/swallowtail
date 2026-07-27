# 046 Provider Session Management Records

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../015-provider-session-management-foundation.md`

## Objective

Add the provider-neutral identities, capabilities, actions, and outcomes needed
to manage one bound inactive provider session.

## Governing Refs

- Research 036
- Contract 038
- Contracts 003, 008-010, 017, and 029
- contracted provider-session architecture section

## Scope

1. Inventory existing session, resume, compatibility, access, and deletion
   records before adding types.
2. Add independent archive, restore, delete, and provider-native-close
   capabilities.
3. Add an opaque management binding that does not require load or resume
   support.
4. Add exact action, lifecycle state, deletion strength, descendant scope,
   effect truth, and compatibility evidence records.
5. Keep driver-owned `OwnedRemoteResourceDeletion` unchanged.
6. Add construction, equality, validation, and redaction tests.

## Acceptance Criteria

- [x] raw provider ids cannot construct management authority accidentally
- [x] binding identity includes driver, transport, instance, host, version,
      target, access, and resource scope where applicable
- [x] archive, restore, delete, and provider-native close are independent
- [x] history removal, provider data deletion, and hard deletion cannot
      substitute
- [x] applied, already absent, before-effect failure, and after-effect
      uncertainty remain distinct
- [x] debug and diagnostic forms expose no raw provider reference or secret

## Evidence

- `swallowtail-core` owns independent lifecycle capabilities, typed actions,
  lifecycle state, deletion strength, affected scope, effect truth, and exact
  interface compatibility evidence.
- `swallowtail-runtime` owns an opaque management binding minted only from a
  driver descriptor, configured instance, access evidence, and explicit
  binding origin.
- binding construction rejects driver drift, access drift, absent or
  incompatible interface versions, and routes without archive, restore, or
  delete capability.
- `OwnedRemoteResourceDeletion` and `RemoteResourceDeletionOutcome` are
  unchanged.
- 49 core and 63 runtime tests pass; focused Clippy passes with warnings
  denied; all 23 workspace crates and all targets compile.

## Validation

- focused core tests
- runtime compile check against additive records
- `effigy format:check`
- `git diff --check`

## Stop Conditions

- a record needs consumer thread or UI state
- the binding requires a provider-specific enum
- existing owned-resource deletion would need semantic widening
- an arbitrary list result would gain authority

## Auto-Continuation

Yes. Continue to card 047 after focused validation passes.
