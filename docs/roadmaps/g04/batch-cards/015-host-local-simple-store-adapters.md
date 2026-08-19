# 015 Host-Local Simple Store Adapters

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../005-connection-lifecycle-kernel.md`
Depends on: card 014

## Goal

Ship optional in-memory and JSON-file store adapters in
`swallowtail-host-local`.

## Scope

1. In-memory adapter for tests and small apps.
2. JSON-file adapter that persists store records to a host-owned path.
3. Tests that JSON on disk contains no secret bytes.
4. Additive public API snapshots under
   `release-baselines/public-api-unreleased/` for packages this kernel
   changes. Do not rewrite `public-api-0.3.3`.

## Out Of Scope

- consumer SQLite or keychain stores
- sign-in, catalog, or first-proof adapters
- a default on-disk path chosen by Swallowtail
- GitHub Release, registry, or tag mutation

## Acceptance Criteria

- [x] both adapters implement the runtime store trait
- [x] JSON-file adapter refuses to write secret bytes
- [x] multiple instances of one family round-trip as distinct ids
- [x] `public-api-0.3.3` files are unchanged
- [x] no production adapter crate changes

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local`
- `effigy package:api`
- `git diff --check`

## Auto-Continuation

No. g04.005 closes. g04.006 stays planned until this kernel exists.

## Stop Conditions

- Stop if the JSON adapter becomes a product database or writes secrets.
- Stop if `public-api-0.3.3` snapshots are edited.
