# 036 Scoped Endpoint And Credential Grants

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../011-hosted-transport-foundations.md`

## Objective

Realize Contract 014 scope- and audience-bound endpoint grants and credential
leases without adding concrete HTTP behavior.

## Governing References

- Contract 008
- Contract 010
- Contract 014
- Research 004

## Scope

- operation scope and endpoint audience on network authorization
- redacted driver-only authorized endpoint value
- scope, credential reference, and audience on secret/delegated leases
- explicit awaited credential release
- host-service registry and recording-fixture updates

## Out Of Scope

- HTTP client or provider wire types
- credential persistence or sign-in
- provider driver

## Implementation Steps

1. Add redacted authorized endpoint and exact grant bindings.
2. Bind credential variants to scope, reference, and audience.
3. Add awaited release to the credential host port.
4. Update recording services and hosted-profile assertions.
5. Migrate callers cleanly; add no compatibility shim.

## Acceptance Criteria

- [x] endpoint values appear only through an explicit driver accessor
- [x] endpoint and credential formatting is redacted
- [x] scope and audience mismatches are representable and tested
- [x] credential release is recorded and awaited
- [x] runtime remains executor- and transport-client-neutral

## Validation

- focused runtime and testkit tests
- `git diff --check`

## Evidence Required

- redaction tests
- recording-host acquire/authorize/release order
- hosted profile without process service

## Evidence

- `AuthorizedEndpoint` is non-empty, redacted, and exposed only through
  `as_driver_value` on a scope/audience-bound `NetworkGrant`.
- secret and delegated credential leases retain scope, reference, and audience;
  the credential service owns explicit awaited release.
- recording fixtures prove authorize, acquire, release, and zero process start.

## Stop Conditions

- endpoint URI enters core or public diagnostics
- credential cleanup can only occur in `Drop`
- change requires a generic network byte transport

## Auto-Continuation

Yes. Continue to card 037 when focused validation passes.
