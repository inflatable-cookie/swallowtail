# 048 Cline Box Start-Session Err

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../017-cline-stable-clippy-result-large-err.md`
Depends on: none

## Goal

Box every production ACP `start_session` Err pair of
`(RuntimeFailure, ResourceLease)` so Clippy 1.98.0 `result_large_err`
is quiet.

## Scope

1. Apply the Cline boxing to Goose, Copilot CLI, Gemini, Kiro, and Deep
   Agents: `Result<Handle, Box<(RuntimeFailure, ResourceLease)>>`.
2. Update every `return Err((error, resource))` and each
   `map_err(|_| (malformed(), resource.clone()))` site.
3. Update each `open_session` match so it still destructures the pair and
   releases the lease on failure.
4. Do not allow the lint. Do not change `RuntimeFailure` or
   `ResourceLease`.

## Out Of Scope

- proving workspace clippy (card 049)
- DeepSeek or g04.016
- other `result_large_err` allows in test servers

## Acceptance Criteria

- [ ] no production `start_session` returns a 128-byte Err tuple by value
- [ ] each `open_session` still releases the working-resource lease on
      `start_session` failure
- [ ] no DeepSeek or g04.016 files change

## Validation

- `git diff --check`
- `cargo test --locked -p swallowtail-adapter-goose -p swallowtail-adapter-copilot-cli -p swallowtail-adapter-gemini -p swallowtail-adapter-kiro -p swallowtail-adapter-deepagents`

## Auto-Continuation

Yes, into card 049.

## Stop Conditions

- Stop if session-start behavior would change beyond the Err type.
- Stop if a seventh adapter is edited that does not have this Err pair.
- Stop if the lint is allowed instead of boxed.
