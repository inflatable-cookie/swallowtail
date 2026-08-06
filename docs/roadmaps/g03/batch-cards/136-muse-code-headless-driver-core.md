# 136 Muse Code Headless Driver Core

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../045-muse-code-headless-foundation.md`
Depends on: card 135

## Goal

Implement exact installed discovery and one bounded Muse event-JSONL structured
run without consumer or provider policy inference.

## Scope

1. Add the exact opaque release claim and version parser.
2. Add target-bound installed discovery against the versioned payload.
3. Encode the read-only `muse exec --json` command with exact bounds.
4. Decode correlated output, terminal, safe failure, and task lifecycle.
5. Join cancellation, deadline, event delivery, process exit, and cleanup.

## Acceptance

- [x] no launcher update or login action occurs
- [x] output and terminal records correlate to the admitted operation
- [x] task lifecycle degrades to truthful bounded activity without becoming a
      task-list or subagent claim
- [x] unsupported semantic records fail or remain explicitly namespaced
- [x] all process exits preserve separate provider, harness, host, runtime, and
      cleanup truth

## Validation

- `effigy validate:focused swallowtail-adapter-muse`
- deterministic driver and corpus suites only

## Stop Conditions

- stop if the process service cannot keep the prompt and working resource
  within the approved launch boundary
- stop if the event stream requires private session-log parsing

## Auto-Continuation

Completed. Continue to card 137; exact low-level discovery and execution are
ready for consumer-safe preparation.

## Completion

Added the package core, qualified-only opaque release claim, exact-payload
discovery, read-only ephemeral command, strict JSONL decoder, task and unknown
activity projection, cancellation, deadline, process-exit classification, and
joined cleanup. Fourteen Rust tests plus the five package-independent corpus
tests pass. Extracted-package proof also passes.
