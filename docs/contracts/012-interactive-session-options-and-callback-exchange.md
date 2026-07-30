# 012 Interactive Session Options And Callback Exchange

Status: active
Owner: Tom
Updated: 2026-07-30

## Purpose

Define the provider-neutral session setup and correlated callback boundary
needed by interactive harness drivers without moving consumer tools or product
authority into Swallowtail.

## Session Setup

An open or resumed interactive session may carry:

- optional opaque developer instructions
- one optional exact reasoning mode
- one optional exact harness mode
- zero or more bounded tool declarations

The selected model route remains part of the successful preflight plan. A
session request cannot change it. Selecting a different model requires a new
preflight and session unless a later contract adds an explicit model-switch
capability.

Catalog reasoning metadata is evidence only. A selected reasoning mode must be
present as an exact `ReasoningSelection` constraint in the plan. It is never
silently filled from catalog or provider defaults.

The first portable harness mode is `HarnessMode::Plan`. A selected mode must
be present as an exact `HarnessModeSelection` constraint in the plan. It
constrains harness behavior only: it grants no filesystem, network, tool,
permission, credential, or isolation authority. Absence means the qualified
route default, not `Plan`.

Each tool declaration contains a consumer-owned name, optional description,
and bounded input-schema transport data. Swallowtail and its drivers may
validate, materialize, and translate this data. They do not interpret the
schema, execute the tool, or persist consumer records.

A session with tools requires `ToolCalls`. Its plan records maximum tool count,
maximum inline schema bytes, and accepted schema dialect. Unsupported counts,
dialects, reasoning modes, or schema forms fail before provider work.

Resume re-declares the effective session options. A driver either applies and
validates them or rejects the resume before provider work; it cannot silently
reuse unknown provider-side settings.

## Callback Records

Every callback request contains:

- a Swallowtail callback id
- the owning runtime turn id
- the matching turn-event sequence
- an optional host-monotonic deadline
- either a declared tool call, portable harness user input, or a namespaced
  provider extension

Tool-call arguments and callback result bodies are bounded opaque bytes and
redacted by default. Provider wire envelopes remain adapter-private.

A callback response repeats the callback and owning turn ids and carries
either a bounded success body, a typed harness user-input response, or an
explicit failure kind. Failure kinds distinguish unknown declarations,
unsupported callback kinds, consumer execution failure, cancellation, and
timeout. Optional provider-facing detail remains bounded and redacted.

A portable user-input request contains ordered bounded questions with stable
question and option ids. It can represent single choice, multiple choice,
free text, an optional other value, secret text, and optional provider
auto-resolution timing. The response uses the same stable ids and carries
selected options, text, skipped answers, or a callback failure. Drivers
validate cardinality and membership before provider translation.

Callback ids are distinct from request, run, session, turn, provider, product,
task, and receipt ids. Runtime identities and callback bodies do not reveal
their values through default formatting.

## Exchange And Ordering

One turn may expose one callback exchange containing a bounded request stream
and a response port. Taking the exchange is one-shot, like taking the event
stream and terminal outcome.

For each callback:

1. the driver validates the callback kind, declared tool name, active turn,
   bounds, and deadline
2. it enqueues the callback request before publishing the matching
   `CallbackRequested` event
3. the request and event carry the same callback id and event sequence
4. the consumer responds exactly once through the response port
5. the driver correlates the response before translating it to provider wire

The response-port result confirms transport acceptance, not provider or turn
completion. Callback requests, turn events, callback response acceptance, and
the terminal turn outcome remain independently observable.

Only the active turn may own new callbacks. A callback id cannot be reused in
one session. A response with the wrong turn id, unknown id, duplicate id, or
abandoned id fails explicitly and is never forwarded.

## Cancellation And Deadlines

A callback deadline uses the host monotonic clock and cannot outlive its turn
or session deadline. A turn waiting for a callback remains cancellable.

Turn cancellation, timeout, terminal completion, or close abandons every
pending callback before cleanup completes. Pending request streams close and
late responses fail explicitly. Cancellation produces cancelled state; only a
host deadline observation produces timed-out state.

A callback response does not keep an otherwise terminal turn alive. An
unanswered callback cannot leave the provider waiting indefinitely: the
driver sends an available native cancellation or error response, then applies
the owning turn's cleanup contract.

## Extensions And Unsupported Requests

Tool calls and losslessly representable harness user-input requests are common
callback kinds. Approval, permission, and any question schema that cannot be
represented without losing provider semantics remain namespaced extensions
and require a matching preflight namespace. A declared extension has one exact
handling mode:

- reject
- observe and stop without granting response authority
- exchange through the correlated callback response port

A consumer may answer common user input, handle an exchangeable extension, or
explicitly reject either through callback failure. Swallowtail never guesses
provider meaning. Provider-specific request and response helpers may live in
an adapter while the common runtime keeps extension payloads opaque.

An undeclared tool, undeclared extension namespace, malformed callback, or
callback received by a driver without callback exchange fails the affected
turn safely. The driver does not fabricate approval, filesystem, terminal,
steering, or tool results.

## Authority Boundary

The consumer owns:

- tool names, descriptions, schemas, and instructions
- callback dispatch and execution
- product authorization, mutation, validation, receipts, and persistence
- any approval, filesystem, terminal, or steering policy

Swallowtail owns only bounded declaration transport, correlation, lifecycle,
redaction, and explicit failure. No generic tool executor belongs in core,
runtime, testkit, host, or adapter crates.

## Acceptance

- exact session options, including harness mode, are checked against preflight
- declarations and callback payloads are bounded and redacted
- callback request and event ordering is deterministic
- mismatched, duplicate, late, unknown, and unsupported callbacks fail
  explicitly
- callback waits end on response, cancellation, timeout, terminal state, or
  close
- no consumer product type or execution authority crosses this boundary
