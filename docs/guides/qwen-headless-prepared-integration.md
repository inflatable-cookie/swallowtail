# Qwen Headless Prepared Integration

Use `swallowtail-adapter-qwen` for the installed `qwen` harness. The production
route is `qwen.headless`, driver ID `swallowtail.qwen.headless`, over Qwen's
structured CLI stream JSON protocol.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

Choose it for an authenticated model catalogue, one bounded prompt, or a
turn-scoped interactive continuation that privately reuses Qwen's exact
session ID. Reject it when the application needs writes, callbacks,
attachments, unsupported generation controls, public load/resume, or provider-session
management.

## Operator Prerequisites

Preparation requires configured-instance and execution-host identity, one
host-approved executable target, explicit environment, the
maintainer-supported `qwen-code/delegated-harness-auth` profile with matching
evidence, and bounded discovery cancellation and deadline. Operations bind the
task, process, time, credential, and read-only working-resource services in
their immutable plan.

Swallowtail does not install Qwen Code, search `PATH`, log in, choose a
provider, model, credential, workspace, billing route, sandbox, or fallback.
The delegated credential is an opaque scoped lease. Ambient harness
configuration and `--safe-mode` do not prove host containment.

Qualified versions are `0.19.11..=0.20.1`, `0.21.0..=0.21.15`, and
`0.22.0..=0.22.1`; the 0.21+ segments have the image-only catalogue
filter. Later stable releases may remain visible `UnverifiedNewer` under
the latest qualified behavior. The unpublished `0.20.2` and `0.21.16`
gaps, older versions, and prereleases do not prepare.

## Prepare And Catalogue

Build `QwenPreparationInput` and `QwenPreparationProbe`, then call
`prepare_qwen_headless`. Preparation probes only the approved target and sends
no model prompt. Inspect `evidence()` and the configured instance before
deriving an operation.

Model discovery is separate: call `prepare_qwen_catalogue`, then
`list_models`. One ephemeral safe-mode control child verifies
`can_get_available_models`, calls `get_available_models`, projects bounded
provider/model evidence, and joins. Catalogue evidence does not select or
invoke a model.

## Structured Run

Create `QwenRunProfileInput` with request identity, exact provider/model route,
prompt content, read-only working resource, and mandatory host deadline. Call
`prepare_run`, inspect its plan and request, then `start_run`.

The plan binds prompt bytes over stdin, partial stream-JSON output, the frozen
read-only and excluded-tool sets, and a 60-second native wall time. Omitted
turn/tool budgets keep the current command bytes: at most 16 tool calls and at
most 24 Qwen session turns **per child**. Provider retention is allowed, but
the run exposes no reusable provider identity.

Take the event stream and terminal outcome immediately and poll them
concurrently. Drain assistant, tool, usage, and terminal events, then close the
run. Cancellation or deadline stops and joins the child. Terminal status and
cleanup remain separate.

### Exact Reasoning Selection

Reasoning selection is qualified only at package `0.21.15`, provider
`alibaba-modelstudio`, and models `qwen3.8-max` or `qwen3.8-max-preview`.
Each model admits only the canonical values `low`, `medium`, `high`, `xhigh`,
and `max`. Other Qwen models, upstream aliases, and other package points stay
withheld. An omitted selection keeps the existing text-stdin command and
request shape.

Supply the selection through the typed prepared input. The prepared plan,
request policy, evidence, and driver must retain the same exact value. The
selected child switches to `--input-format stream-json`, sends a private
`initialize` control request, requires `can_set_effort`, sends `set_effort`
with the canonical value, and requires an exact `applied: true` response with
no override before sending the user message. The bounded child has started by
this point; a substituted tier or higher-priority ambient provider knob is
rejected before the user message/provider prompt is sent.

This is an in-memory operation-private control exchange. It does not automate
`/effort`, persist `model.reasoningEffort`, mutate user/project settings, or
create a synthetic configuration root. It proves requested, planned,
dispatched, and Qwen-control accepted values only; it does not claim
provider-effective or response-observed reasoning. The exact evidence is in
[Research 189](../research/189-qwen-headless-reasoning-effort-evidence.md).

### Exact Turn And Tool Budgets

Turn and tool budgets are qualified only at package `0.21.15`. They are
adapter-local caller-decreasing limits for one Qwen child: session turns
`1..=24` and tool calls `0..=16`. Other package points, raised values,
unlimited `-1`, turn budget `0`, fractions, and aliases stay withheld or
invalid. An omitted field keeps the current `--max-session-turns 24` or
`--max-tool-calls 16` bytes. Wall time stays `60s`.

Supply a selected value through `with_session_turn_budget` /
`with_tool_call_budget` on `QwenRunProfileInput` or
`QwenSessionProfileInput`, using `QwenSessionTurnBudget` (`1..=24`) and
`QwenToolCallBudget` (`0..=16`). Inspect the admitted pair on
`QwenPreparedEvidence::budgets` as `QwenHeadlessBudgets`. The prepared
evidence and every spawned child — structured run, first turn, resumed turn,
and fresh replacement — must emit the same integers. Counters are
process-local and reset on every child, including `--resume`. They do not
reduce Swallowtail's separate interactive session bound of 24 host turns,
prove that the provider completed less work, or become portable generation
controls.

Zero tool calls can still produce assistant text. The first tool request
aborts before dispatch. Overrun is a process exit: 53 for the turn cap and 55
for a native run budget (tool-call or wall-time). This route's `stream-json`
output writes those failures as stderr plus exit code, not a stronger
semantic stream event. Swallowtail already classifies those exits as
`swallowtail.qwen.headless.native_turn_limit` and
`swallowtail.qwen.headless.native_budget`. Exact evidence is in
[Research 198](../research/198-qwen-headless-turn-and-tool-budget-evidence.md).

## Interactive Continuation

Create `QwenSessionProfileInput` with request identity, exact model, working
resource, and optional deadline. Call `prepare_session`, then `open_session`.
The session supports at most 24 turns and starts one joined Qwen child per
turn.

After a clean first turn, Swallowtail privately retains only the exact
provider session ID from Qwen's result and passes it to the next child with
`--resume`. A failed, cancelled, timed-out, or mismatched turn does not advance
that state. The ID never becomes a public `SessionResumeBinding`, load, or
resume operation.

When the prepared session has an admitted reasoning selection, every child —
first, resumed, and fresh replacement — repeats the same private initialize /
`set_effort` handshake before its user message. The setting is not recovered
from ambient configuration or provider session state.

For every turn, drain events and terminal concurrently, then close the turn.
Active-turn interruption stops only that child. Closing the session joins
local work and preserves any Qwen-owned state.

`prepare_working_state_restoration` opens a fresh context-losing replacement
after process loss. It does not recover the private session ID, transcript, or
interrupted turn.

The compile-tested
[`prepared_qwen_headless` example](../../crates/swallowtail-adapter-qwen/examples/prepared_qwen_headless.rs)
covers run and session preparation.

## Failures, Unsupported Capabilities, And Promotion

Handle failures through portable classification and retain the exact
`swallowtail.qwen.*` diagnostic for support. Never parse stderr, stream JSON,
Qwen state, tool display, or provider prose in consumer code to infer auth,
retry, terminal, or cleanup truth.

The route exposes no output-token limit outside the qualified Qwen reasoning
selection above, structured output, attachments, consumer tools, permissions,
questions, writes, external
search, public continuation, provider-session catalogue/import, reconciliation,
archive/restore/delete, native close, or public child control.

Promotion requires exact published-version and protocol evidence, immutable
prepared-plan binding, bounded fixtures, operation lifecycle tests, and
route-matrix coverage. A CLI flag or retained local file alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-qwen
effigy check:examples
```

No provider prompt, credential use, login mutation, or state-file access is
required.
