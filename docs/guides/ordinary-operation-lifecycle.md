# Ordinary Operation Lifecycle

Use this runbook after selecting and preparing one exact route. It covers the
shared lifecycle of structured runs and interactive sessions without
flattening their provider-specific operations.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Choose The Operation Shape

`StructuredRunDriver` starts one bounded operation with
`StructuredRunRequest` and returns a `RunHandle`. Use it when the route owns a
single prompt-to-terminal shape.

`InteractiveSessionDriver` opens, loads, or resumes a session and returns an
`InteractiveSessionHandle`. `start_turn` accepts one `TurnRequest` and returns
a `TurnHandle`. Use it for serial turns or native continuation. `LoadedSession`
adds bounded ordered replay; `resume_session` returns no replay.

Do not emulate a missing structured run by opening a hidden session, or a
missing session by replaying prompts through runs. Check the selected route in
the [feature matrix](provider-solution-feature-matrix.csv) and use its prepared
entry point.

## Start And Drain

For a run or turn:

1. Allocate the consumer-unique request, run, session, and turn identities
   required by the adapter input.
2. Prepare the exact operation. Preparation must finish before provider work.
3. Start the run, or open/load/resume the session and start one turn.
4. Immediately take the event stream once with `take_events`.
5. If supported, take the callback exchange or direct-tool exchange once.
6. Take the terminal future once with `take_terminal_outcome`.
7. Drain events, callbacks, direct tools, and terminal concurrently. Never
   wait for terminal while leaving a bounded semantic event stream unread.
8. Persist semantic events and checkpoint or cleanup bindings before
   acknowledging them as durable application state.
9. After terminal, finish callback work, drain remaining events, and call
   `close` on the run or turn. Close the session separately when no further
   turn will start.

`take_*` methods transfer ownership and may return `None` after the first
call. A consumer must not assume a second observer can attach.

Events are ordered by `RuntimeEvent::sequence`. `Semantic` delivery cannot be
dropped. `ProgressSnapshot` and `Keepalive` are coalescible. Preserve activity
identity and correlation as described by the
[observable activity guide](observable-activity.md).

## Output, Usage, And Cost

Incremental output arrives through event content. The terminal outcome may
also carry final output. Treat `OutputAvailable` as operation output, not as
proof of terminal completion.

`RuntimeEventKind::ProviderObservation` carries route-qualified usage, billed
cost, rate-limit, quota, and finish evidence. Evidence may be absent or
partial. Token usage is not billed cost; billed ticks are not account balance;
rate-limit evidence is not retry permission. Keep the route's source and
semantics intact.

Only `TerminalOutcome` settles the operation. Interpret `TerminalStatus`,
optional provider-cancellation truth, remote-resource deletion truth, and
`CleanupOutcome` independently.

## Cancellation And Interruption

Call the handle's `CancellationControl` only after consumer authorization.
Cancellation is scoped to that handle. It does not imply provider
cancellation unless `provider_cancellation` says so, and it does not replace
session interruption, provider-native abort, detachment, child control, or
remote deletion.

`Completed`, `Cancelled`, `TimedOut`, `Detached`, provider input observation,
and the three failure sources are distinct terminal states. A deadline is not
the same as a confirmed provider cancellation. If controlled detachment is
available, follow the
[detachment guide](provider-operation-detachment.md); ordinary close must not
be reinterpreted as detach.

## Callbacks And Waiting

`RuntimeEventKind::CallbackRequested` identifies a request on the separately
taken `CallbackExchange`. Correlate with `CallbackId` and the exact run or turn
operation identity. Build responses with `CallbackResponse::for_request` so
the provider request cannot be answered for another operation.

Permission requests, typed questions, provider requests, and local direct
tool calls have different payloads and authority. A waiting callback is not a
failure and does not complete the operation. Cancellation, timeout,
termination, or close abandons unanswered callbacks exactly once.

See [Generation Controls And Input Authority](generation-controls-and-input-authority.md)
for callback and tool admission.

## Terminal And Cleanup

Always retain both terminal and cleanup truth. `close` joins operation-owned
tasks, streams, leases, processes, credentials, and connections according to
the route. It may return `Clean`, `Degraded`, `Failed`, or `NotApplicable`.
Cleanup failure must not overwrite a successful or failed terminal result.

Closing a session does not generally archive or delete provider state.
Provider-native close, archive, restore, delete, owned remote cleanup, and
owned serving lifecycle require separately qualified operations. See
[Provider State And Resource Lifecycle](provider-state-and-resource-lifecycle.md).

## Persistence And Restart

Persist only public opaque bindings or checkpoints emitted by the route. A
provider session reference, turn reference, activity id, model label, or raw
provider token is not a replacement. If the process loses an active handle,
do not infer terminal state or replay the prompt. Follow
[Working-State Restoration](working-state-restoration.md).

The consumer owns transcript storage, event deduplication, UI state, retry and
fallback policy, and durable correlation between its thread and Swallowtail's
opaque records. Swallowtail owns the live operation lifecycle and joined
cleanup.

## Examples And Validation

Every route row in the [integration guide map](integration-guide-map.md)
links a compiling prepared example for its normal operation shape. The
[typed-question example](../../crates/swallowtail-runtime/examples/harness_user_input_consumer.rs)
shows callback correlation without a provider process.

```sh
effigy check:examples
effigy qa:routes
```
