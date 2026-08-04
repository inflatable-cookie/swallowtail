# 077 OpenAI Background Run Reconciliation

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../030-openai-background-run-reconciliation-and-detachment.md`
Depends on: card 076

## Goal

Restore one exact OpenAI response/cursor record and observe its current state,
bounded output, and usage through one read-only retrieve request.

## Scope

1. Emit exact response/cursor checkpoints after provider identity is known.
2. Add the prepared run-reconciliation surface.
3. Map exact active and terminal response states.
4. Reject foreign response, route, host, access, model, and malformed evidence.
5. Freeze request-method and no-side-effect corpus truth.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-openai`

## Stop Conditions

- stop if exact status requires polling, stream attachment, or another inference attempt
- stop if terminal output cannot remain bounded

## Auto-Continuation

Continue to card 078 when exact deterministic restart reconciliation passes.
