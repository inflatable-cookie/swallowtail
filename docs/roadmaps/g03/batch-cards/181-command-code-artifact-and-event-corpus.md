# 181 Command Code Artifact And Event Corpus

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../059-command-code-headless-foundation.md`
Depends on: Research 116

## Goal

Freeze exact Command Code `1.15.1` artifact, command, NDJSON, correlation,
terminal, and failure evidence before production Rust behavior exists.

## Scope

1. Record npm package, launcher, and payload digests from Research 116.
2. Freeze version, help, no-tool success, tool success, credit-failure, and
   unknown-event fixtures with private content redacted.
3. Define stream rules for event wrappers, result terminal, usage, thinking,
   text, tools, and unknown types.
4. Name the exact qualified-only compatibility and protocol-facade revisions.

## Acceptance

- [x] fixtures contain no credentials, account identifiers, private paths,
      prompts, thinking bodies, or tool input/result bodies
- [x] random identities are consistently sanitized without weakening
      correlation evidence
- [x] `run_end.result.nextState` is recorded as non-ingestible private state
- [x] malformed, oversized, post-terminal, and mismatched-model records fail
      safely
- [x] unknown event types remain namespaced observations

## Validation

- focused package fixture/parser tests introduced by this card
- `effigy qa:northstar`

## Stop Conditions

- stop if NDJSON cannot bind one command, session, run, and terminal
- stop if sanitized fixtures cannot retain exact lifecycle meaning

## Auto-Continuation

Continue to card 182 once the shared fixture tree is ready for the Rust driver.
