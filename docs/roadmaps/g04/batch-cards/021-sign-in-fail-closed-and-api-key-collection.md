# 021 Sign-In Fail-Closed And API-Key Collection

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../007-sign-in-loop-and-host-ports.md`
Depends on: card 020

## Goal

Fail closed when a required host port is missing, and collect API keys
through field descriptors into opaque credential references.

## Scope

1. Missing required port fails the loop that needs it.
2. API-key collection uses credential-field descriptors.
3. Complete materializes a `CredentialRef`; Contract 014 still owns leases.
4. Field descriptors never carry secret bytes.

## Out Of Scope

- live key verification against a provider
- readiness refresh
- overlay projection
- first-proof Anthropic Messages wiring

## Acceptance Criteria

- [ ] missing URL, loopback, or device-code port fails the matching loop
- [ ] API-key collection stores a reference, not the secret, in the 057 store
- [ ] 014 acquire/release still owns leases
- [ ] 047 still has no tokens

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

No. Compile readiness refresh, subject observation, and overlay projection
after g04.007 closes.

## Stop Conditions

- Stop if the store or 047 receives secret bytes.
- Stop if field descriptors replace scoped leases.
