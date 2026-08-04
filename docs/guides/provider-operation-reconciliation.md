# Provider Operation Reconciliation

Use this path when a consumer persisted a turn as active, then lost its
Swallowtail runtime handle through process exit or crash.

Do not send another prompt first. Do not import the session again. Do not infer
completion from an idle provider session.

## Consumer Sequence

1. Persist the consumer `RuntimeTurnId` and
   `PersistedSessionResumeBinding` before provider dispatch can be forgotten.
2. On restart, prepare the same route and restore the binding against the
   ordinary matching session plan.
3. If the prepared route advertises `ProviderSessionReconciliation`, prepare a
   bounded reconciliation request for that binding and runtime turn.
4. Replace the consumer's provider-history snapshot only when the returned
   replay is complete. Merge conservatively when it is incomplete.
5. Interpret state with its attribution:
   - exact provider turn plus terminal state is provider terminal evidence
   - session-scoped `Active` means provider work still exists
   - session-scoped `InactiveUnresolved` stops live presentation but proves no
     terminal class
   - `Unknown` keeps uncertainty visible
6. Use a separately qualified reattachment or cancellation operation if the
   operator chooses control. Reconciliation itself exposes none.

## OpenCode

`OpenCodePreparedIntegration::prepare_session_reconciliation` accepts
`OpenCodeSessionReconciliationInput`. The input carries:

- a new request id for the observation
- the same explicit `OpenCodeModelSelection`
- the restored `SessionResumeBinding`
- the original consumer `RuntimeTurnId`
- `ProviderSessionReconciliationBounds`
- an optional deadline

Call `OpenCodePreparedSessionReconciliation::reconcile`. The outcome is always
`InterruptedTurnAttribution::ProviderSession`; OpenCode currently exposes no
exact prompt or turn id for `prompt_async`. Valid states are therefore
`Active`, `InactiveUnresolved`, or `Unknown`.

The driver performs only health, exact session lookup, session status, and
bounded message reads. It does not prompt, abort, delete, load, resume, import,
or answer callbacks.

## Codex App-Server

`CodexPreparedIntegration::prepare_session_reconciliation` accepts
`CodexSessionReconciliationInput`. Supply the exact `TurnRef` exposed by the
original turn handle when it was persisted. The result then uses
`ExactProviderTurn` attribution and can report provider terminal truth. If the
provider turn reference was not captured, the same operation remains
session-scoped and cannot report a terminal state.

The driver performs one exact `thread/read` with turns included. Missing exact
turns fail closed. It sends no turn start, interrupt, thread resume, archive,
restore, or delete request.

## Route Availability

`codex.app-server` and `opencode.http` are currently production-qualified.
Research 099 records the actionable gate for every other route. Do not derive capability from
provider family, session load support, durable retention, or another transport.

This guide intentionally does not add another column to the main provider
feature CSV. Recovery support has several evidence strengths; collapsing them
to `Yes` or `No` would hide the promotion work.
