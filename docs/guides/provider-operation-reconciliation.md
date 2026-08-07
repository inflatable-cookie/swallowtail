# Provider Operation Reconciliation

Use this path when a consumer persisted a turn as active, then lost its
Swallowtail runtime handle through process exit or crash. New to the shared
vocabulary? Read [Key Concepts](key-concepts.md) first.

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

For a consumer-selected settled session, Codex app-server, OpenCode HTTP, and
Kimi local server can compose reconciliation with an independently prepared
attachment through `prepare_settled_session_restoration`. The sequence retains
read-only reconciliation as its first phase and never attaches after active,
waiting, or unknown evidence, or after a reconciliation failure. Codex and
OpenCode then load bounded replay; Kimi resumes without replay. See the
[working-state restoration guide](working-state-restoration.md).

Provider-owned runs use a separate sequence. Persist the emitted
`ProviderRunCheckpoint` against the exact prepared run plan, restore it only
through the same prepared route, and call that route's bounded
`prepare_run_reconciliation` operation. A run checkpoint is not a session
binding and grants no prompt, retry, callback, cancellation, management, or
cleanup authority.

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

## OpenAI Background

`OpenAiBackgroundPreparedIntegration::prepare_run_reconciliation` restores one
exact background response checkpoint and performs one bounded response retrieval.
See the [OpenAI background guide](openai-background-prepared-integration.md)
for checkpoint persistence, optional controlled detachment, and exact terminal
mapping.

## Anthropic Managed Agents

`AnthropicManagedPreparedIntegration::prepare_run_reconciliation` restores one
exact Managed Agents run checkpoint and performs bounded session plus paginated
persisted-event reads. A separate
`prepare_recovered_cleanup` operation accepts only the separately emitted
owned-resource cleanup binding. Reconciliation cannot answer a waiting
callback or clean resources; cleanup cannot interrupt active or ambiguous
work. See the
[Anthropic Managed Agent guide](anthropic-managed-agent-prepared-integration.md)
for the full persistence and cleanup sequence.

## Route Availability

`codex.app-server` and `opencode.http` implement session reconciliation.
`openai.background` and `anthropic.managed-agent` implement exact provider-run
reconciliation. Kimi local server implements exact-turn reconciliation. Do not
derive capability from provider family, session load support, durable
retention, or another transport.

This guide intentionally does not add another column to the main provider
feature CSV. Recovery support has several evidence strengths; collapsing them
to `Yes` or `No` would hide the promotion work.

Consumers that need one restart execution API should use the
[working-state restoration facade](working-state-restoration.md). It wraps
these read-only mappings and the separately qualified Claude Agent ACP and
Kimi ACP continuation-recovery paths without changing reconciliation truth.

## Example And Validation

The compile-tested
[Anthropic Managed Agent example](../../crates/swallowtail-adapter-anthropic/examples/prepared_managed_agent.rs)
shows the recoverable-run, reconciliation, and recovered-cleanup constructors
together. The [working-state restoration guide](working-state-restoration.md)
shows the prepared facade and its outcome match with an inline example.

Validate without provider work:

```sh
effigy check:examples
effigy qa:docs
```

Live reconciliation and cleanup against provider state remain separately
operator-gated probes, never deterministic acceptance.
