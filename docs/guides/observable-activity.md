# Observable Activity

Swallowtail carries provider-visible agent work on the existing ordered run or
turn event stream. It does not add a transcript API, second event bus, global
recorder, or consumer-specific facade.

## Inspect Before Effects

Every prepared ordinary structured run or interactive turn exposes an
`ObservableActivityProfile` through `PreparedOperationEvidence`.

The
[provider-solution activity matrix](provider-solution-activity-matrix.md)
compiles every production route and operation shape into one consumer-facing
inventory. It does not replace exact prepared evidence.

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
        let provider_label = activity.label();
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

`label()` is optional bounded provider display metadata. It may refine between
observations and is carried forward when a later native update omits it. It is
not part of activity identity or a content stream. Tool payload remains in
`content()`; consumers should not parse its first line as a label. Applications
still own their final presentation labels and may replace the provider value.

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

## Harness Route Profiles

All 13 production harness routes expose `Available` activity for every
ordinary structured-run or interactive-session profile they support. There
is no provider-wide promise that a given kind or lifecycle phase exists:
missing kinds remain `Unavailable` inside that exact route profile.

| Route | Ordinary prepared profiles | Selected fidelity and exact thinness |
| --- | --- | --- |
| `codex.app-server` | interactive session | version-segmented native item lifecycle; older provider-unspecified assistant items remain identity-only |
| `codex.exec` | structured run | completion-oriented assistant and reasoning; command, search, MCP, task, warning, collaboration, and unknown activity only where qualified |
| `claude-agent.acp` | structured run, interactive session | ACP assistant, readable thought, plan, and provider-tool lifecycle; raw tool bodies excluded |
| `gemini-cli.acp` | interactive session | assistant and provider-tool lifecycle; operational thought-channel warnings do not become reasoning summaries |
| `kimi-code.acp` | interactive session | assistant, readable thought, plan, and provider-tool lifecycle |
| `pi.rpc` | structured run, interactive session | native message, readable-thinking, provider-tool, compaction, retry, warning, and unknown lifecycle |
| `qwen.headless` | structured run, interactive session | qualified partial-message lifecycle for assistant, readable-thinking, and provider-tool records |
| `kimi-code.local-server` | structured run, interactive session | cursor-admitted turn, step, assistant, thought, tool, shell, subagent, task, compaction, retry, warning, and unknown lifecycle |
| `opencode.http` | structured run, interactive session | range-segmented SSE message, reasoning, tool, step, warning, and unknown lifecycle; exact `1.14.51` remains thinner |
| `anthropic.managed-agent` | structured run | authoritative persisted completions only; provider and MCP tools are provider-owned, custom tools remain callbacks |
| `claude-code.headless` | structured run | completion-only assistant and provider-tool records; no selected partial or readable-reasoning channel |
| `gemini-cli.headless` | structured run | assistant updates plus completion-only correlated tool records; tool parameters and output excluded |
| `kimi-code.headless` | structured run | completion-only assistant and correlated tool records; retry is namespaced activity and resume prose is metadata |

Catalogue and provider-session-management operations remain `NotApplicable`
to ordinary agent activity. They retain their existing typed catalogue,
binding, and effect evidence. No harness run or turn is left with an
unexplained whole-profile `Unavailable` result.

The machine-checked inventory lives in the testkit fixture
`provider-wide-harness-activity.json`. Public prepared evidence, not this
table or provider event parsing in a consumer, is the runtime source of truth.

## Text Direct Route Profiles

All 14 selected ordinary text-inference profiles expose `Available` activity
through their prepared operation evidence. They report provider-visible model
output, not a harness work log.

| Route | Ordinary prepared profiles | Exact activity |
| --- | --- | --- |
| `anthropic.messages` | structured run, interactive session | complete assistant lifecycle; optional structured provider web-search lifecycle; session consumer-tool lifecycle correlated with the direct-tool exchange |
| `kimi-platform.chat-completions` | structured run | complete assistant lifecycle and update-plus-completion readable reasoning summary |
| `deepseek.chat-completions` | structured run, interactive session | assistant updates and completion; session consumer-tool completion correlated with the direct-tool exchange |
| `alibaba-model-studio.chat-completions` | structured run, interactive session | complete assistant lifecycle |
| `openai.background-responses` | structured run | complete assistant lifecycle |
| `xai.responses` | structured run, interactive session | complete assistant lifecycle |
| `amazon-bedrock.runtime` | structured run | complete assistant lifecycle |
| `ollama.attached` | structured run, interactive session | assistant updates and completion |
| `llama-cpp.attached` | structured run | complete assistant lifecycle without server ownership |

Streamed final-answer deltas use `FinalAnswerText`. The final
`OutputAvailable` event remains separate. Anthropic provider search and
consumer tools disclose identity and lifecycle only; their arguments, results,
and exchange bodies remain on existing typed surfaces.

Kimi Platform exposes only the K3 client-visible thought channel selected by
its qualified route. DeepSeek reasoning continuation and xAI encrypted
thinking remain private. Billed cost, usage, request correlation, rate,
quota, cache, retention, cancellation, recovery, cleanup, and model residency
remain separate evidence.

The machine-checked applicability inventory lives in
`direct-activity-applicability.json`. Adapter conformance checks the positive
profiles against decoded runtime traces. Adapter prepared-facade conformance
also checks all 13 catalogue, inventory, realtime-media, and serving-only
operations as `NotApplicable`. Realtime media events remain on their dedicated
session surfaces.

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

Swallowtail owns portable identity, provider labels, lifecycle, ordering,
bounds, profile truth, and redacted formatting. The application owns
persistence, grouping, collapsed tool rows, presentation labels, review state,
retention, deletion, and UI.

See [Contract 044](../contracts/044-observable-agent-activity-and-disclosure.md)
for the durable boundary.
