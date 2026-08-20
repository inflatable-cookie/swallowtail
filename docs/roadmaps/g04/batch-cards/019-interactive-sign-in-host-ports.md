# 019 Interactive Sign-In Host Ports

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../007-sign-in-loop-and-host-ports.md`
Depends on: completed g04.006

## Goal

Add optional Contract 010 host ports for URL open, loopback callback, and
device-code display.

## Scope

1. New optional service kinds that do not collapse into Credential, Process,
   or Network.
2. Host-local test doubles.
3. Registration does not start sign-in.

## Out Of Scope

- the sign-in state machine (card 020)
- embedding a browser or keychain
- OAuth client secrets

## Acceptance Criteria

- [x] missing ports are observable
- [x] ports never return secret bytes
- [x] process-spawned login helpers stay process authority
- [x] 047 is unchanged

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 020.

## Stop Conditions

- Stop if the ports embed a browser or return tokens.
