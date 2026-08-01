# 049 Provider Session Catalogue And Import Records

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../019-provider-session-catalogue-and-import-foundation.md`
Depends on: card 048

## Goal

Add the provider-neutral records, capabilities, requirements, plans, and
request validation needed to keep session discovery separate from import
authority.

## Scope

1. Add distinct catalogue and import capabilities, driver roles, and operation
   shapes.
2. Add bounded discovery scope, cursor, candidate, optional display content,
   compatibility, and availability records.
3. Add immutable catalogue and import plans plus effect-free request
   constructors.
4. Bind exact instance, host, access, version, model, resource, and policy.
5. Prove raw provider ids and candidates cannot satisfy load or resume.

## Out Of Scope

- runtime drivers or provider adapters
- serialization, consumer persistence, polling, synchronization, or UI
- archive, restore, delete, fork, export, or active-handle control

## Acceptance Criteria

- [x] catalogue and import remain independent capabilities and shapes
- [x] constructors enforce all bounds and attachment dimensions
- [x] candidate content is absent from `Debug` and safe diagnostics
- [x] cursors and candidates reject cross-plan reuse
- [x] import plans require load and resume compatibility explicitly
- [x] focused core and runtime tests pass
- [x] card 050 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 050 after focused acceptance.

## Evidence

- separate catalogue/import capabilities, roles, operation shapes, and
  cancellation scopes are public
- bounded scope, cursor, candidate, display, activity, availability, and
  catalogue-limit records are realized
- immutable plans bind exact source and future attachment evidence
- typed requests reject cursor, candidate, resource, continuation-capability,
  cancellation-scope, and plan drift before effects
- `effigy validate:focused swallowtail-core swallowtail-runtime` passed 163 tests
- `git diff --check` passed
