# 022 Readiness Refresh

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../008-readiness-refresh-subject-and-updates.md`
Depends on: completed g04.007

## Goal

Refresh Contract 006 / 008 access dimensions for one admitted instance
without changing enablement or mutating a 047 snapshot.

## Scope

1. Re-observe credential, entitlement, endpoint authorization, runtime
   readiness, and support authority.
2. Write `AccessStatus` onto the admitted instance record.
3. Leave enablement untouched.
4. Leave 047 as an immutable snapshot the consumer replaces.

## Out Of Scope

- subject observation (card 023)
- overlay projection
- probing unrelated instances
- inventing an aggregate ready boolean
- live provider probes

## Acceptance Criteria

- [x] a disabled instance can refresh to ready access dimensions
- [x] an enabled instance can refresh to not-ready access dimensions
- [x] enablement is unchanged by refresh
- [x] no 047 type is written or mutated

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 023.

## Stop Conditions

- Stop if refresh writes enablement or a 047 snapshot.
- Stop if refresh probes more than the named instance.
