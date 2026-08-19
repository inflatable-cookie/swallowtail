# 010 Readiness And Admission Contract

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../004-readiness-admission-contract-promotion.md`
Depends on: completed g04.003

## Goal

Write Contract 057 from Spec 011 without implementing the facade.

## Scope

1. Create `docs/contracts/057-route-readiness-and-connection-admission.md`.
2. Own addable-route catalog, credential-field descriptors, sign-in loop
   through host ports, store port, readiness refresh, authenticated-subject
   observation, config-field descriptors, and the bound model-presentation
   overlay.
3. Record crate placement and first-proof routes as later implementation
   notes.

## Out Of Scope

- seam amendments (card 011)
- spec archive (card 012)
- Rust types, adapters, or host-port implementations
- tag, registry, or GitHub Release mutation

## Acceptance Criteria

- [x] Contract 057 exists and is the owner of the named surfaces
- [x] 047 remains a selection snapshot
- [x] enablement and readiness stay independent
- [x] host ports cover URL open, loopback callback, and device-code display
      without embedding a browser, keychain, or OAuth client secret
- [x] no production code changes

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 011.

## Stop Conditions

- Stop if the draft would store raw secrets or require a Swallowtail server.
- Stop if crate placement or first-proof routes would reopen a settled
  operator decision.

## Evidence

Contract 057 is active. Crate placement and first-proof routes remain later
implementation notes. No Rust types were added.
