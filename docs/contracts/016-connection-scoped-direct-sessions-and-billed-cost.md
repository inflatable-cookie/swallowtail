# 016 Connection-Scoped Direct Sessions And Billed Cost

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Define a resource-free interactive direct-inference session over a long-lived
network connection and preserve provider-authoritative billed cost without
turning it into consumer accounting or routing policy.

## Direct-Inference Session Shape

An interactive session may use `DirectModelInference` when the provider offers
a connection-scoped sequence of model turns without a provider-owned agent
loop. The operation remains an `InteractiveSession`; its execution layer does
not become harness interaction merely because the provider retains response
state or executes provider-side tools.

A direct-inference session may omit a working resource when its selected
requirements declare no filesystem or working-resource capability. Common
session request and access records must represent that absence directly. They
cannot manufacture a placeholder resource or reuse a read-only workspace
profile whose filesystem boundary is irrelevant.

Tools, workspace access, approvals, attachments, schemas, provider-side search,
provider-side network, storage, and background execution remain independent
capabilities. Absence is unsupported, not inherited from another session
driver.

## Connection And Lease Ownership

Preflight binds one configured instance, execution host, WebSocket endpoint
reference, endpoint audience, credential reference, access profile, model
route, and model identity before connection work.

Opening the session acquires one endpoint grant and one credential lease for
the session scope. The handle retains both until the connection and every
reader/writer task are closed and joined. Credential release is awaited after
connection cleanup. A turn cannot replace the endpoint, credential, audience,
execution host, route, or model.

The driver owns WebSocket framing and protocol correlation. The host owns only
the approved endpoint, credential delivery, blocking or async work authority,
time observations, and cleanup services it explicitly supplies. A generic byte
transport is not required.

## Turn Serialization And Continuation

The first connection-scoped direct session permits one active turn. A second
turn request while another is active fails before a provider frame is sent;
Swallowtail does not rely on a provider queue and does not silently multiplex.

Each successful provider response may yield an opaque continuation reference.
The driver, not the consumer, attaches the latest valid reference to the next
turn on the same connection. It remains bound to the runtime session, provider
connection, configured instance, route, model, and prior successful response.

A provider failure that invalidates continuation also invalidates the local
chain. Swallowtail cannot retry from an older continuation point, branch a
chain, or start a fresh chain without an explicit new consumer operation.

Provider response ids, connection ids, and runtime session or turn ids remain
separate. Raw ids stay out of stable diagnostics.

## Reconnect, Resume, And Storage

Connection recovery is not implicit session resume.

The first xAI proof fixes `store=false` and claims no durable provider session.
When the socket closes, reaches its provider lifetime limit, is cancelled, or
loses its connection-local continuation, the runtime session ends. It exposes
no `SessionResumeBinding`.

The driver cannot:

- enable provider storage to make recovery easier
- reconnect and replay consumer prompts
- rehydrate from a provider response id
- retry the failed turn
- copy context from another connection

A consumer may open a new session and provide new complete context under a new
operation. That is not transparent recovery of the old session.

## Cancellation, Deadline, And Close

Provider documentation for the first xAI text Responses WebSocket does not
define a turn-cancel message. Cancellation or deadline therefore closes the
owned socket, stops local reads and writes, joins all work, releases the
credential, and ends the whole session. It must not report native provider
cancellation or preserve the connection for another turn.

Provider connection-limit errors, protocol errors, disconnect, consumer
cancellation, deadline expiry, credential cleanup failure, and ordinary close
remain distinct terminal or cleanup evidence. Only one terminal outcome wins.

No detached keepalive, reconnect, reader, writer, timer, or credential work may
survive session close.

## Streaming And Failure

Contract 014 streaming rules apply to every turn. The driver additionally
must:

- correlate every known event to the exact active provider response
- preserve event order inside the active turn
- fail on an event for another or absent response
- treat provider error events after connection success as provider failure
- reject unknown semantic content that could make final output incomplete
- keep raw frames, headers, prompts, outputs, and provider messages private

Connection keepalive is transport evidence, not model progress. A completed
provider response ends one turn, not the connection-scoped session.

## Provider-Authoritative Billed Cost

A provider may report the exact amount charged for one inference attempt. This
is a billed-cost observation, distinct from:

- token or tool usage
- catalogue price metadata
- a rate or quota limit
- account balance or entitlement
- an estimate calculated by Swallowtail or a consumer
- a cumulative session or application total

A portable billed-cost observation contains:

- an integer amount in the provider's exact smallest reported scale
- currency identity
- scale or denominator needed for exact conversion
- evidence that the provider, not Swallowtail, declared the amount billed
- the exact operation, turn, route, access profile, and provider attempt scope

Floating-point currency is not a stable record. For xAI,
`cost_in_usd_ticks` is recorded as an integer with currency `USD` and denominator
`10,000,000,000` ticks per USD.

Streaming cost observations follow the provider's cumulative semantics. The
latest observation replaces an earlier value for the same attempt. Swallowtail
does not sum chunks. It also does not sum turns into a session total; aggregation,
budget enforcement, presentation, forecasting, and accounting remain consumer
policy.

Missing billed cost means unknown. It cannot be reconstructed from catalogue
prices. Billed cost never authorizes retry, fallback, model selection, or a
billing-boundary change.

## First xAI Subset

The first provider proof binds:

- provider-supported xAI Responses WebSocket at one exact host-approved `wss`
  endpoint
- xAI public API bearer-key access, prepaid or usage-billed API credits, the
  `api.x.ai` endpoint audience, and provider support authority
- direct-model interactive session execution
- one exact configured model route; no alias or model fallback
- `store=false`
- text input and text output only
- one active turn and connection-local latest-response continuation
- final token usage and provider-billed cost

It excludes model catalogue HTTP, provider and client tools, external search,
attachments, structured output, warmup, background mode, persistent storage,
automatic retry, reconnect, resume, and live authentication from default QA.

## Conformance

Deterministic fixtures must prove:

- no resource, network, or credential effect before successful preflight
- exact WebSocket endpoint, audience, credential, route, model, and host binding
- session-scoped endpoint and credential lifetime
- first and chained turns with one active turn
- no consumer-supplied continuation id
- ordered output, final usage, and billed-cost replacement
- provider error, invalid continuation, lifetime limit, disconnect,
  cancellation, deadline, and close
- no reconnect, retry, storage, resume binding, or detached work
- endpoint, credential, frame, content, response-id, and cost-source redaction

Live authentication remains separately gated.

## Acceptance

- direct-model interactive sessions need no fake workspace
- a long-lived connection does not become a harness or durable provider session
- continuation stays opaque, connection-bound, and driver-owned
- cancellation and deadline truthfully end the session when no native cancel
  exists
- billed cost remains exact provider evidence, not consumer accounting policy
- no route, credential, storage, retry, reconnect, or support fallback is
  implicit
