# 102 Interactive Crash Recovery Kernel

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../038-provider-wide-interactive-crash-recovery.md`
Depends on: card 090

## Goal

Realize exact attachment recovery and fresh-session replacement in the common
runtime without weakening load, resume, or reconciliation.

## Scope

1. Add explicit method and outcome variants.
2. Add a separate low-level attachment-recovery operation.
3. Permit durable bindings to encode exact no-model preparation.
4. Preserve existing persisted-binding bytes for model-bound routes.
5. Add provider-free exact-once, context-loss, mismatch, and failure tests.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`

## Stop Conditions

- stop if model-less binding requires a synthetic model identity
- stop if attachment can be represented as complete load replay
- stop if replacement needs a prompt or consumer transcript

## Auto-Continuation

Continue to card 103 when common conformance passes.

## Outcome

- added attachment-recovery and fresh-replacement methods and outcomes
- added an explicit low-level attachment operation with a safe unsupported
  default
- extended durable session bindings to preserve exact no-model preparation
- retained the existing model-bound persistence fingerprint domain and format
- focused common validation passed: 276 tests
