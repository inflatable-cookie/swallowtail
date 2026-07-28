# Bounded Single-Turn Structured-Run Projection

Status: active
Owner: Tom
Updated: 2026-07-27

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
exposes no reusable interactive handle, resume binding, provider-session
management binding, or silent continuation.

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
- ordered events, callbacks, usage, and terminal outcome
- cancellation and deadline invalidation
- explicit retention agreement
- no reusable session, connection, or management binding escape
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
- realtime media and serving lifecycle do not inherit structured execution
- no provider, model, access, transport, or fallback choice is implicit
