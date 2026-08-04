# 049 Controlled Shutdown Active Operation Detachment

Status: active
Owner: Tom
Updated: 2026-08-04

## Purpose

Allow a consumer to end one local run or turn attachment during controlled
shutdown without asking a separately retained provider operation to stop.

## Separate Dispositions

Completion, cancellation, timeout, failure, and detachment remain distinct.

Detachment:

- closes only the qualified local observation attachment
- sends no provider prompt, retry, callback response, interrupt, abort,
  deletion, archive, import, load, or resume request
- joins every local task and releases every operation-scoped lease
- leaves later provider state observation to Contract 048 reconciliation

`TerminalStatus::Detached` is local attachment truth. It does not prove that
provider work remained active, completed, failed, or stopped during cleanup.

## Admission

Detachment is unavailable unless the exact preflight plan carries
`Capability::ActiveOperationDetachment` with the matching run or active-turn
scope. Configured-instance visibility does not select the capability.

The prepared profile must opt in before dispatch and must bind:

- an externally retained provider session or operation
- exact route, configured instance, host, model, resource, and access posture
- a durable restart binding persisted before the active operation can be lost
- a qualified reconciliation path for the same provider state

Unsupported, owned-foreground, delete-on-close, callback-bearing, raw-id,
unbound, or non-reconcilable operations expose no detachment control.

## Handle Control

An admitted handle exposes `OperationDetachmentControl` separately from
`CancellationControl`.

- `request` is idempotent
- `Requested` and `AlreadyRequested` acknowledge local disposition only
- a terminal handle rejects detachment
- cancellation already in progress rejects detachment
- cancellation wins a concurrent race
- requesting detachment does not itself claim cleanup

After acknowledgement, the consumer awaits the ordinary consuming `close`.
Close skips provider cancellation only for the admitted detached state, joins
the local attachment, and returns its existing `CleanupOutcome`. A failed or
degraded cleanup does not become successful detachment.

Calling ordinary close without first requesting detachment retains its existing
route semantics, including cancellation where already contracted. Dropping a
handle remains unsuccessful cleanup.

## Event And Callback Boundary

Detachment completes the local terminal future once with `Detached`. It emits
no synthetic completed, failed, or cancelled activity for provider work whose
state is unresolved.

No callback exchange crosses the boundary. A route with an admitted provider
request, unanswered permission, question, tool callback, or other consumer
decision must reject detachment unless a later contract defines durable exact
callback recovery.

Consumers persist the local operation as detached/unresolved, not completed or
cancelled. On restart they use Contract 048 before permitting another turn on
the same consumer thread.

## Cleanup And Ownership

No reader, poller, deadline, callback, credential, process, cleanup, or host
task detaches with the provider work. Every local task joins before access and
working-resource release.

Attached provider services remain running. Owned processes and foreground
servers are excluded unless a later profile proves transfer to another
explicit owner. Detachment never manufactures an owner.

## First Production Mapping

`opencode.http` implements active-turn detachment for explicitly selected
read-only interactive sessions across qualified `1.14.48..=1.18.10` segments.
The profile requires durable provider-session preservation, a persisted
`SessionResumeBinding`, and the existing session-scoped reconciliation path.

The driver stops and joins the SSE attachment without issuing `/abort`, prompt,
delete, callback, load, resume, import, or status work. Callback-enabled
sessions and structured runs do not expose detachment. The attached OpenCode
server and provider session remain external.

## Conformance

Portable and route tests cover:

- optional control visibility only on admitted plans
- idempotent request acknowledgement
- cancellation and terminal rejection
- cancellation-winning race semantics
- one `Detached` local terminal outcome
- no provider abort, retry, callback answer, or deletion
- no synthetic provider-terminal activity
- joined stream, task, attachment, resource, endpoint, and credential cleanup
- exact durable binding preservation followed by same-session reconciliation
- unchanged ordinary close cancellation
