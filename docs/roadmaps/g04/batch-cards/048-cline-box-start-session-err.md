# 048 Cline Box Start-Session Err

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../017-cline-stable-clippy-result-large-err.md`
Depends on: none

## Goal

Box the `ClineAcpDriver::start_session` Err pair so Clippy 1.98.0
`result_large_err` is quiet.

## Scope

1. Change the return type to
   `Result<ClineSessionHandle, Box<(RuntimeFailure, ResourceLease)>>`.
2. Update every `return Err((error, resource))` and the
   `map_err(|_| (malformed(), resource.clone()))` site.
3. Update the `open_session` match so it still destructures the pair and
   releases the lease on failure.
4. Do not allow the lint. Do not change `RuntimeFailure` or
   `ResourceLease`.

## Out Of Scope

- proving workspace clippy (card 049)
- DeepSeek or g04.016
- other `result_large_err` allows in test servers

## Acceptance Criteria

- [ ] `start_session` no longer returns a 128-byte Err tuple by value
- [ ] `open_session` still releases the working-resource lease on
      `start_session` failure
- [ ] no files outside `swallowtail-adapter-cline` change

## Validation

- `effigy validate:focused swallowtail-adapter-cline`
- `git diff --check`

## Auto-Continuation

Yes, into card 049.

## Stop Conditions

- Stop if session-start behavior would change beyond the Err type.
- Stop if another adapter is edited.
