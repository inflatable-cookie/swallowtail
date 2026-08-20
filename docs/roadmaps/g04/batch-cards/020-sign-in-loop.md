# 020 Sign-In Loop

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../007-sign-in-loop-and-host-ports.md`
Depends on: card 019

## Goal

Own start, poll, complete, cancel, and timeout for interactive OAuth, device
OAuth, and delegated CLI login through the new host ports.

## Scope

1. Library-owned loop over host ports.
2. `SignInAction` remains an advertisement, not permission to execute.
3. Distinct from ACP `authenticate` and Contract 017 delegated login.
4. Deterministic tests with mock ports. No live provider.

## Out Of Scope

- API-key collection and missing-port fail-closed (card 021)
- live OAuth
- first-proof Anthropic subscription
- 047 subject or token fields

## Acceptance Criteria

- [x] start, poll, complete, cancel, and timeout are explicit
- [x] ACP authenticate is not called
- [x] 017 delegated login is not this loop
- [x] success does not change mechanism, account, audience, or billing

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `git diff --check`

## Auto-Continuation

Yes, into card 021.

## Stop Conditions

- Stop if live provider work is required to pass tests.
- Stop if authenticate and login collapse into one role.
