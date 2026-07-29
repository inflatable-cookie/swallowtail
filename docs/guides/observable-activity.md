# Observable Activity

Swallowtail carries provider-visible agent work on the existing ordered run or
turn event stream. It does not add a transcript API, second event bus, global
recorder, or consumer-specific facade.

## Inspect Before Effects

Every prepared ordinary structured run or interactive turn exposes an
`ObservableActivityProfile` through `PreparedOperationEvidence`.

The profile reports:

- `Available` with exact per-kind lifecycle, content, disclosure, and
  correlation limits
- `Unavailable` when the operation remains usable but has no qualified
  portable activity
- `NotApplicable` for catalogue, serving, realtime-media, and other
  non-ordinary operation roles

`StreamingEvents` means ordered bounded delivery. `ObservableActivity` means
qualified semantic activity. Requiring one does not imply the other.

Consumers that need selected activity kinds may add exact
`ObservableActivity` constraints to preflight. Consumers that can render a
thinner timeline may accept the inspected profile and handle only emitted
events.

## Consume One Stream

Read `RuntimeEventKind::Activity` beside the existing operation events:

```rust
match event.kind() {
    RuntimeEventKind::Activity(activity) => {
        let id = activity.activity_id();
        let phase = activity.phase();
        let kind = activity.kind();
        let content = activity.content();
        // Project into consumer-owned transient or durable presentation state.
    }
    RuntimeEventKind::CallbackRequested(callback_id) => {
        // Settle through the existing callback exchange.
    }
    RuntimeEventKind::DirectToolCallAvailable(tool_call_id) => {
        // Continue through the existing direct-tool exchange.
    }
    RuntimeEventKind::OutputAvailable => {
        // Final operation output remains a separate event.
    }
    _ => {}
}
```

An activity correlation points at a callback, direct-tool call, or provider
request. It does not replace that exchange or duplicate its body. A completed
final assistant activity and final operation output may contain the same task
text, but they remain separate ordered events.

Only provider-intended readable summaries use `ReasoningSummary`. Hidden
reasoning, provider-private continuation state, and raw provider envelopes are
not portable activity.

Codex app-server prepared sessions publish an available profile. Qualified
events retain native item ownership, lifecycle, readable summaries, plans,
command output and status, file diffs, tool and request correlations, hooks,
and bounded namespaced unknowns. Versions before Codex `0.105.0` may report
`ActivityAssistantPhase::ProviderUnspecified`; consumers should render that
identity without labeling it commentary or final answer.

Codex exec publishes a separate, thinner profile:

| Activity | App-server | Exec |
| --- | --- | --- |
| assistant message | complete lifecycle | completion only; final answer |
| reasoning summary | complete lifecycle | completion only |
| command, MCP tool, search | complete lifecycle | start and completion |
| file change | complete lifecycle | completion only |
| plan | update/completion or complete lifecycle by version | unavailable |
| todo/task | no separate task item | start, replacement updates, completion |
| collaboration | complete lifecycle from its qualified milestone | start and completion from `0.92.0` |
| dynamic tool, image, review, compaction, hook | qualified per app-server version | unavailable |
| unknown semantic activity | bounded namespaced preservation | bounded namespaced preservation |

Exec keeps final operation output separate from the completed final-assistant
activity. Its prepared profile does not inherit richer app-server kinds merely
because both drivers use the same executable.

## Unknown And Newer Events

A qualified route either:

- preserves a safely identified unknown semantic item as
  `ActivityKind::Unknown` with a bounded namespace
- fails closed

It never converts an unknown semantic event into empty generic progress.

Permitted unverified-newer execution retains the last qualified activity
profile. Newly observed fields do not widen the guarantee until evidence
promotes a new behavior milestone.

## Conformance

Adapter tests can reuse:

- `ObservableActivityTraceFixture`
- `ObservableActivityFixtureCase`
- `assert_observable_activity_trace`
- `assert_observable_activity_contract`

The assertion pack covers complete, update-and-completion, completion-only,
unavailable, assistant, reasoning-summary, unknown, callback, direct-tool,
redaction, bounds, ordering, and unverified-newer behavior.

Provider adapters must pass their decoded profile and existing runtime events
to the shared trace assertion. They must not expose native payloads through
the public fixture.

## Consumer Ownership

Swallowtail owns portable identity, lifecycle, ordering, bounds, profile truth,
and redacted formatting. The application owns persistence, grouping, collapsed
tool rows, labels, review state, retention, deletion, and UI.

See [Contract 044](../contracts/044-observable-agent-activity-and-disclosure.md)
for the durable boundary.
