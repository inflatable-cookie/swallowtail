# 182 Command Code Headless Driver Core

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../059-command-code-headless-foundation.md`
Depends on: card 181

## Goal

Implement exact installed discovery and one bounded Command Code NDJSON
structured run without consumer or provider policy inference.

## Scope

1. Add the exact npm release claim and version parser for `1.15.1`.
2. Add target-bound installed discovery against `command-code --version`.
3. Encode the read-only `-p --output-format json` command with stdin prompt.
4. Decode thinking, text, tool, usage, terminal, safe failure, and unknowns.
5. Join cancellation, deadline, event delivery, process exit, and cleanup.

## Acceptance

- [x] no login, taste onboarding, or auto-update action occurs
- [x] output and terminal records correlate to the admitted operation
- [x] exit `10` / credit failure maps to portable `QuotaExhausted`
- [x] unsupported semantic records fail or remain explicitly namespaced
- [x] all process exits preserve separate provider, harness, host, runtime, and
      cleanup truth

## Validation

- `effigy validate:focused swallowtail-adapter-command-code`
- deterministic driver and corpus suites only

## Stop Conditions

- stop if the process service cannot keep the prompt and working resource
  within the approved launch boundary
- stop if the event stream requires private session-file parsing

## Auto-Continuation

Continue to card 183 once exact low-level discovery and execution are ready.
