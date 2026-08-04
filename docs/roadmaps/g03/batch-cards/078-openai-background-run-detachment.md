# 078 OpenAI Background Run Detachment

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../030-openai-background-run-reconciliation-and-detachment.md`
Depends on: card 077

## Goal

Close one explicitly selected OpenAI background SSE observer without cancel or
delete, then reconcile the same exact provider run after restart.

## Scope

1. Add explicit prepared-profile detachment selection.
2. Expose structured-run detachment only after checkpoint availability.
3. Close and join the local stream while preserving the response.
4. Return local `Detached` truth without provider terminal claims.
5. Prove detach-to-reconcile and unchanged ordinary cleanup.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-openai`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-openai`
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- stop if detach must retain a credential lease or local task
- stop if cancellation cannot win a concurrent disposition race

## Auto-Continuation

Complete g03.030 and return to the retained-operation evidence gate.
