# 183 Command Code Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../059-command-code-headless-foundation.md`
Depends on: card 182

## Goal

Expose consumer-safe preparation for one exact read-only Command Code headless
run with explicit model selection and no hidden defaults.

## Scope

1. Add provider-owned local Command Code account access without credential
   extraction.
2. Bind exact model, read-only resource, retention prohibited, ambient
   configuration, and host services into immutable evidence.
3. Expose `prepare_command_code_headless` → `prepare_run` → `start_run`.
4. Keep a low-level driver escape hatch without inventing catalogue or
   interactive roles.

## Acceptance

- [x] preparation fails closed on version drift, missing model, or write
      authority
- [x] activity-affecting options are immutable prepared evidence
- [x] no default model or effort is invented
- [x] deterministic prepared-facade tests pass without credentials

## Validation

- `effigy validate:focused swallowtail-adapter-command-code`

## Stop Conditions

- stop if preparation would require Swallowtail to lease Command Code
  credentials
- stop if the facade collapses Provider API into the harness route

## Auto-Continuation

Continue to card 184 once the prepared facade is ready for package acceptance.
