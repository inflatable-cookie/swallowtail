# 009 Async Operation Lifecycle

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Define an executor-neutral, dynamically dispatchable lifecycle for structured
runs, interactive sessions, and serving instances.

## Async And Dependency Posture

Runtime traits are object-safe role traits using explicit boxed `Send` futures
rather than native `async fn` methods. The public shape is equivalent to:

```rust
type BoxFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + 'a>>;
```

Event delivery uses a boxed `futures_core::Stream` boundary. The initial
`swallowtail-runtime` dependency policy permits:

- `swallowtail-core`
- `futures-core` for the standard ecosystem stream trait

It does not expose Tokio, async-std, smol, a transport client, an SDK, or
`async-trait` in the public contract. Provider adapters may use additional
private dependencies without leaking their concrete runtime types.

Drivers and host services are `Send + Sync`. Returned operation futures,
streams, and handles are `Send`. A future local-only adapter must use an
explicit later contract rather than weakening this boundary silently.

Swallowtail does not create a global executor. The execution host drives
futures and supplies scoped task and blocking-work services under Contract
010.

## Role Traits

The runtime defines separate object-safe roles for:

- discovery
- structured-run start
- interactive-session open or resume
- serving-instance attach or start

Role methods consume a successful immutable preflight plan. They do not repeat
routing or silently alter the selected instance, route, access profile,
ownership, or host.

## Scoped Handles

### Structured Run

A structured-run request carries opaque operation content, an opaque
working-resource reference, and an explicit `OperationPolicy`. The policy
separately selects provider-side external-network permission, external search,
and an optional reasoning mode. It has no implicit values from model metadata
or provider configuration. Swallowtail transports the content and selected
policy; it does not author, persist, interpret, or apply product policy to it.

A run start returns an owned run handle with:

- Swallowtail request and run ids
- ordered event stream
- idempotent run cancellation
- one terminal-outcome future
- explicit awaited close or cleanup

### Interactive Session

A session open or resume request carries an opaque working-resource reference.
The driver may bind that resource at process or session scope through the host;
it never receives an arbitrary consumer path.

A session open or resume returns an owned session handle. It can start turns,
close the session, and expose an optional resume binding. That binding keeps
the opaque provider session reference with the configured-instance, execution-
host, model-route, and model identities allowed to resume it. One active turn
is the default. Concurrent turns require an explicit parameterized capability.

Each turn has its own turn id, ordered event stream, cancellation control,
terminal outcome, and optional opaque provider turn reference.

A turn request uses the same opaque operation-content boundary. Session
drivers may bind working resources at session scope rather than repeating them
on every turn.

Resume carries the binding returned by the prior session. It requires the same
configured instance, execution host, model route, model, and provider
reference. The provider's resume response must repeat the requested provider
session reference. Migration requires a declared provider extension and
separate validation.

### Serving Instance

A serving handle records whether it is external attached,
host-owned ephemeral, or host-owned persistent. Stop is available only for an
owned instance under host authority. Closing an attached handle never stops
the external service.

## Task Ownership

Every spawned or blocking task belongs to a discovery probe, run, session,
turn, or owned serving-instance scope. Close joins the scope after child
resource cleanup. No task detaches beyond its owner.

Dropping a handle is not successful cleanup. Drop may request best-effort
cancellation but cannot block; the parent host task scope remains responsible
for joining leaked child work and reporting cleanup failure.

## Events

- sequence numbers are monotonic within one run or turn
- a start event precedes provider progress or output
- normalized semantic events are never silently dropped
- external-search activity and safe reasoning summaries remain distinct
  provider-neutral semantic event kinds rather than consumer-side parsing
- only event kinds declared coalescible may be replaced under pressure
- the runtime-facing stream is bounded
- adapters may use a separately bounded ingress buffer when provider reads
  cannot pause safely
- non-coalescible overflow terminates the operation with a runtime failure
- exactly one terminal outcome completes the handle
- terminal outcome is independent from the last progress event
- normalized output content may be carried by semantic events and the terminal
  outcome
- output deltas remain distinct from completed output so consumers do not need
  provider-specific heuristics to assemble a message
- late provider events are quarantined as internal diagnostics
- replay is consumer persistence policy, not a runtime guarantee

## Terminal And Cleanup Outcomes

Terminal state distinguishes:

- completed
- cancelled
- timed out
- provider failed
- host failed
- runtime failed

A completed provider result does not erase a cleanup failure. Terminal outcome
therefore carries a separate cleanup outcome: clean, degraded, failed, or not
applicable. Internal details remain redacted by default.

Provider completion never means consumer validation, acceptance, or product
state mutation.

## Cancellation And Deadlines

Cancellation scopes are run, active turn, whole session, and separately
authorized owned serving instance.

- requests are idempotent
- acknowledgement is distinct from terminal cancellation
- deadlines use host monotonic time and propagate to child work
- bounded model-catalogue requests may carry the same host-monotonic deadline;
  expiry closes and joins the owned provider connection
- a time-service wait resolves to a `DeadlineObservation` carrying the exact
  deadline and monotonic observation time
- only a deadline observation may select the timed-out terminal state;
  operator or consumer cancellation remains cancelled even when cleanup uses
  the same stop mechanics
- native provider interruption is preferred
- cancellation may stop local consumption only when the provider cannot be
  cancelled; the terminal detail must preserve that distinction safely
- graceful cleanup precedes forceful cleanup
- forceful process or connection termination requires ownership and host
  authority
- external processes and services are never force-stopped by generic cleanup
- timeout, cancellation, provider failure, host failure, and cleanup failure
  remain distinguishable

## Callback Exchange

Tools, permissions, filesystem callbacks, terminal callbacks, and steering are
separate parameterized capabilities. Callback ids remain distinct from request,
run, session, turn, and provider ids.

The consumer owns declarations and execution authority. Swallowtail transports
correlated calls and responses. Waiting callbacks remain cancellable and
deadline-bound. Unknown callbacks follow explicit extension policy.

A driver that does not claim callback exchange rejects a provider-initiated
request safely and terminates the affected operation. It does not guess an
approval, fabricate a tool result, or leave the provider waiting indefinitely.

## Compile Evidence

On Rust 1.96.0 with edition 2024:

- a dynamic role registry using explicit boxed `Send` futures, boxed event
  streams, dynamic handles, and optional host-service trait objects compiled
- the equivalent role trait using native `async fn` failed dynamic dispatch
  with `E0038` because the trait was not dyn compatible

Contract 009 adopts the proven object-safe shape. Final method and type names
remain implementation-card decisions within this boundary.

## Acceptance

- role traits can be stored behind `Arc<dyn Trait>`
- handles and futures cross a host worker boundary
- every operation produces ordered events and exactly one terminal outcome
- prompt and output bodies are transportable but redacted from default
  formatting and diagnostics
- cancellation and deadline paths join cleanup
- attached services are not stopped
- no provider-specific runtime type leaks into core or runtime public APIs
