# Bounded Single-Turn Structured-Run Projection

Status: active
Owner: Tom
Updated: 2026-08-11

## Purpose

Permit an exact provider route to expose one bounded structured run even when
its native transport requires an operation-private session, connection,
process, or provider resource.

This contract does not create a generic prompt API or flatten structured runs
into interactive sessions.

## Operation Rule

One structured run is one consumer-authorized operation with:

- one immutable preflight plan
- one explicit `StructuredRunRequest`
- one returned `RunHandle`
- ordered events and optional callback exchange
- one terminal outcome
- explicit cancellation and deadline behavior
- awaited joined cleanup

The provider mechanism may use more than one internal request. It may open one
operation-private process, connection, session, conversation, or owned remote
resource when the exact driver contract requires it.

Internal lifecycle steps do not become consumer session authority. The run
exposes no reusable interactive handle, resume binding, or silent
continuation. An exact durable harness route may return one take-once
provider-session management binding after successful terminal completion when
Contract 038 independently qualifies its origin and actions. That binding does
not make the structured run resumable or interactive.

## Independent Role Qualification

An interactive, realtime, serving, or catalogue role does not imply
`StructuredRun`.

Every projected route must:

- register `DriverRole::StructuredRun`
- declare its exact execution layer and transport
- bind a separate capability and requirements set
- expose a typed prepared structured operation
- qualify its exact interface versions and behavior revisions
- pass the provider-neutral projection assertions plus provider fixtures

There is no generic adapter from `InteractiveSessionDriver` to
`StructuredRunDriver`. Shared helpers may coordinate provider-neutral handle
and cleanup mechanics, but provider request mapping, retention, callbacks,
failure, and lifecycle stay in the adapter.

## Request Mapping

The prepared operation derives only adapter-owned facts and immutable plan
echoes. The consumer still supplies content and every supported authority
input.

Each projected driver states which structured-run inputs it supports:

- working resource
- attachments
- tool declarations and callbacks
- structured-output schema
- maximum output tokens
- reasoning selection
- external network and search policy
- provider retention
- deadline

Unsupported fields or policy combinations fail before process, endpoint,
credential, session, connection, conversation, or model effects.

An adapter cannot emulate unsupported structured output by prompt injection,
execute consumer tools, infer a model, or reuse ambient session state.

Plain text remains a valid structured-run result when the exact route
qualifies only the bounded one-turn operation shape. JSON-looking text does not
become `StructuredOutput`: the request carries no descriptor, the capability
is absent, and the consumer owns all parsing and validation.

## Direct Request Projection

A resource-free hosted direct route may perform one buffered or streamed
inference request. It must preserve provider model, access, usage, rate, quota,
request, cancellation, storage, and failure semantics.

If the broader route also supports consumer-owned continuation, the
structured branch starts no tool loop and retains no adapter-private
continuation after close.

## Connection Projection

A connection-oriented text route may:

1. acquire one approved endpoint and credential lease
2. open one connection
3. submit one response request
4. stream its ordered events to terminal state
5. close and join the connection
6. release the credential last

The branch sends no previous-response reference, performs no reconnect,
reattachment, rollover, retry, or second turn, and exposes no connection or
session handle.

Realtime-media transports remain under Contract 026 unless a later exact
route independently qualifies a structured role. Merely closing a media
session after one response is not qualification.

## Harness Session Projection

An ACP, RPC, attached-server, or other harness route may:

1. start or attach through its exact qualified transport
2. create one operation-private session when required
3. start one prompt turn
4. relay only qualified callbacks and provider events
5. await the terminal turn result
6. cancel or close native work where supported
7. join process, connection, callback, resource, credential, and task work

Provider-native session close releases runtime resources. It does not imply
history deletion. Process exit does not imply transcript deletion.

The run keeps harness interaction as its execution layer. It does not become
direct model inference because the consumer submits only one prompt.

An exact tool-free harness text projection may omit a working resource and all
callbacks when its prepared plan advertises neither authority. Its invocation
must suppress every model-visible built-in, extension, and MCP tool surface;
the stream must confirm empty tools and MCP servers before assistant output.
The driver rejects provider tool calls, user/tool-result records, a second
assistant response, more than one reported turn, or missing terminal text.
The operation exposes no provider session binding even when the harness uses
one private non-persistent operation id internally.

Exact qualified harness progress does not create another response. A
provider-native cumulative token estimate may project as a content-free,
coalescible `ProgressSnapshot` only when its event family, session identity,
position, integer fields, bounds, and cumulative delta are all validated.
It is neither token usage nor readable reasoning. A provider-private thinking
envelope may be validated and discarded when required to reach the single
text response; thought text must never be accepted or exposed as reasoning
activity, output, observation, or a second response. Unknown system or
assistant shapes still fail closed.

The first mapping is the distinct `claude-code.response-only` route at exact
Claude Code `2.1.228`. It uses print mode, text stdin, stream JSON, empty tools,
safe mode, disabled slash commands and Chrome, strict empty MCP configuration,
disabled prompt suggestions, and no session persistence. It binds
provider-suppressed configuration and ambient-host isolation: the tool posture
removes model-visible filesystem authority but does not sandbox the harness
process or its local subscription authentication. It requires only Task,
Process, and Time host services and advertises no working-resource, callback,
tool, session, continuation, retry, fallback, or structured-output capability.
The existing `claude-code.headless` read-only Plan profile remains unchanged.

The maintained response-only compatibility segment contains only exact
`2.1.228`. Exact `2.1.227`, exact `2.1.229`, and every other version remain
incompatible with this route; no range or unverified-newer execution is
implied.

For exact Claude Code `2.1.228`, `system/thinking_tokens` is qualified only
after init and before assistant text. `estimated_tokens` and
`estimated_tokens_delta` are positive integers no greater than 1,000,000;
the total increases exactly by the delta from zero. Each valid frame emits one
content-free `ProgressSnapshot`. An exact assistant message containing one
`thinking` block with empty thinking text and a non-empty opaque signature may
follow those estimates and is discarded after session, model, message, role,
stop-reason, and ordering validation. At most
one such private-thinking record may precede the one text assistant record.
No thought text or estimate is promoted to portable reasoning or usage.

Provider-managed recovery and active-turn stream reattachment remain
prohibited unless the exact route independently satisfies Contract 042.
Neither durable session retention nor asynchronous prompt acceptance supplies
that authority. A qualified reattachment submits no second prompt and creates
no replacement session or turn.

The Claude Agent ACP projection is an ambient read-write conversion profile,
separate from its read-only interactive profile. Its immutable plan requires a
filesystem working resource with `ResourceAccess::ReadWrite`. Session creation
enables exactly `Read`, `Glob`, `Grep`, `Edit`, and `Write`; the driver then
requires and selects the provider's `acceptEdits` mode before the prompt.
Neither the working directory nor that provider mode creates a bounded
filesystem-containment claim under `AmbientHost`.

## Callback Projection

When the exact route supports consumer callbacks, `RunHandle::take_callbacks`
exposes the same bounded, correlated, exactly-once exchange required by
Contract 012.

The projection maps provider session and turn correlation into the runtime run
scope without discarding provider identity internally. Late, mismatched,
undeclared, timed-out, or abandoned callbacks retain their existing failure
semantics. Swallowtail never executes the tool or chooses a callback response.

Routes that cannot complete safely without an unavailable approval or question
exchange fail or stop according to their exact driver contract. They do not
auto-approve silently.

Claude Agent ACP keeps reject-and-stop as its default structured profile. Its
opt-in consumer-mediated profile binds the exact permission extension in the
immutable run plan and exposes only one-shot provider options through this
callback projection. It does not grant the adapter or host authority to choose
an approval.

## Retention

`OperationPolicy::provider_retention` must match the exact route:

- `Prohibited` when the operation creates no retained provider state
- `TemporaryAllowed` only for provider-required temporary execution state
- `DurableAllowed` when a harness or provider retains a session or transcript

Durable retention is valid for a structured run. It is not a resume, list,
archive, restore, delete, hard-delete, or secure-erasure claim.

Kimi local-server structured runs require `DurableAllowed`. Closing the run
does not delete the created Kimi thread. Archive is reported only when the
exact operation sends and confirms the native archive effect.

An exact harness route may expose two separate named structured profiles:

- durable retention without operation-owned deletion
- temporary retention with qualified operation-owned deletion

The temporary profile is opt-in. Its immutable plan must require
`ProviderTemporaryRetention` and
`OwnedRemoteResourceDeletion` for the exact resource kind. It may delete only
the operation-private session or transcript created by that run. It cannot
delete a consumer-selected persistent session, relabel close as deletion, or
change the durable profile.

Claude Agent ACP may qualify temporary cleanup by sending its exact native
close, then its already-qualified provider-data delete for the
operation-private session. Gemini CLI headless may qualify temporary cleanup
by joining the run process, invoking the separately qualified stored-
transcript delete role, and reconciling exact history absence. Gemini ACP
cannot borrow that CLI capability.

A successful durable Gemini CLI headless run may expose the take-once
management binding qualified by Contract 038. It is unavailable before
terminal completion. Failed, cancelled, timed-out, and temporary-cleanup runs
return no binding. A raw provider run reference remains insufficient.

## Cancellation, Deadline, And Cleanup

Cancellation and deadline target the active provider turn or response first,
then invalidate the operation-private session or connection.

Every terminal path:

- stops new callback and event delivery
- requests native interruption where qualified
- closes operation-private transport and session resources
- joins all owned work
- releases working resources, endpoints, and credentials in their contracted
  order

When a profile promises operation-owned deletion, the deletion attempt joins
before access and working-resource release. Terminal inference status and
deletion truth remain separate. Confirmed deletion records the exact owned
resource; failed or incomplete reconciliation records unconfirmed deletion
and degraded cleanup. Drop performs no provider deletion and cannot report
cleanup success.

Cancellation acknowledgement remains distinct from terminal cancellation.
Provider completion remains distinct from cleanup success. A cleanup failure
cannot turn a failed or timed-out operation into completion.

No projection creates a global executor or detached background task.

## Prepared Facade

Contract 037 applies unchanged. A solution facade may add a typed structured
branch beside interactive, realtime, catalogue, lifecycle, or serving
branches.

The facade keeps route and driver selection explicit. It does not select a
different provider surface merely because that surface also supports a
one-shot request.

Provider-specific headless, ACP, RPC, HTTP, and WebSocket branches retain
separate identifiers and compatibility claims even when one solution facade
groups them.

## Serving Boundary

An owned serving facade is not automatically a structured run.

The current llama.cpp owned prepared solution starts and stops one ephemeral
server. Its returned endpoint is consumed through the separate attached
inference route. The owned solution is therefore `Not applicable` for
structured run in the solution matrix.

A later cold-start composite operation would need separate artifact, serving,
inference, stop, failure, and cleanup qualification. This contract does not
authorize it implicitly.

## Conformance

The projection assertion pack proves:

- independent role and capability registration
- exact request-plan agreement
- unsupported input rejection before effects
- one provider turn or response only
- tool-free profiles confirm empty tool and MCP surfaces and reject any
  provider tool, user-result, or multi-turn record
- ordered events, callbacks, usage, and terminal outcome
- cancellation and deadline invalidation
- explicit retention agreement
- no reusable session or connection escape
- any post-run management binding is exact, independently qualified,
  take-once, and terminal-success-only
- joined cleanup under local and remote-authoritative host identities
- safe diagnostics without prompts, outputs, payloads, credentials, paths, or
  raw provider errors

Provider fixtures additionally prove their exact creation, prompt, terminal,
close, deletion or non-deletion, connection, and process behavior.

## Acceptance

- structured run remains one bounded consumer operation
- execution layer and native lifecycle remain exact
- provider sessions may be internal but never hidden in retention truth
- durable retention is permitted only by explicit policy
- close never implies delete
- unsupported request features fail before effects
- JSON-looking text is never promoted to schema enforcement or portable
  structured output without a descriptor and qualified capability
- realtime media and serving lifecycle do not inherit structured execution
- no provider, model, access, transport, or fallback choice is implicit
