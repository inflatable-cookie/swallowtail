# 014 Lifecycle Store Port

Status: ready
Owner: Tom
Created: 2026-08-19
Milestone: `../005-connection-lifecycle-kernel.md`
Depends on: card 013

## Goal

Add the Contract 057 store trait and lifecycle roles in
`swallowtail-runtime`.

## Scope

1. Store interface for admitted instance records, secret *references*,
   enablement, optional labels, and overlay markers.
2. List and get by configured-instance id, including several instances of
   one family as distinct ids.
3. Enablement as a host preference independent of access-status dimensions
   and of 047 readiness.
4. The trait never requires raw secrets.

## Out Of Scope

- in-memory or JSON-file adapters (card 015)
- admission, sign-in, refresh, or overlay projection behavior
- SQLite, keychain, or product stores
- mutating 047 snapshots

## Acceptance Criteria

- [ ] runtime owns the store trait and depends on the new core records
- [ ] put/get/list round-trip references, labels, enablement, and overlay
      markers
- [ ] a disabled instance can still be stored with ready access dimensions
- [ ] an enabled instance can still be stored with not-ready access
      dimensions
- [ ] the trait API has no secret-bytes parameter or return

## Validation

- `effigy validate:focused swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 015.

## Stop Conditions

- Stop if the trait absorbs 047, 037 preparation, or credential leases.
- Stop if enablement is collapsed into `Ready` / `NotReady`.
