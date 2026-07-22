# 027 Planned Connection Rollover And Realtime Continuity

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Allow one realtime-media session to cross a provider-planned connection
lifetime boundary without silently becoming retry, reconnect, consumer resume,
durable provider storage, or route fallback.

## Identity And Meaning

Planned connection rollover means:

1. the provider warns that the current connection will end
2. the provider has supplied a resumable handle for the current session state
3. the driver opens one replacement connection under the same immutable plan
4. the replacement setup carries the latest handle
5. provider setup confirms continuity before later input is accepted

The runtime session, configured instance, execution host, endpoint audience,
access profile, credential lease, route, model, media formats, and turn sequence
do not change.

Rollover is not:

- reconnect after unexpected transport loss
- stream reattachment to a running response
- inference retry or replay
- `Capability::Resume` or a `SessionResumeBinding`
- durable provider conversation retention
- provider-managed recovery
- a new route, model, endpoint, credential, or topology selection

An unexpected disconnect remains terminal unless a separate later contract and
operation policy authorize recovery.

## Explicit Capability And Policy

Planned rollover is opt-in. Card 095 must realize one exact capability and one
request policy with disabled as the default and a positive bounded count as the
only enabled posture. Requirements, configured instance, model route,
open-session request, and immutable preflight plan must agree before endpoint,
credential, or connection work.

The first proof permits exactly one rollover and at most two connections. A
second rollover request, missing bound, mismatched bound, or disabled request
fails closed. Existing realtime drivers retain disabled rollover and unchanged
behavior.

The policy is specific to connection replacement inside one live operation. It
must not reuse `StreamReattachmentPolicy`, provider-recovery policy, or
interactive consumer resume.

## Provider Handle Boundary

A provider resumption handle is opaque adapter-owned transport state. It is not
a credential, runtime session id, provider request id, model route, consumer
resume binding, or durable transcript reference.

Handles are non-serializable, redacted from `Debug`, `Display`, diagnostics,
events, and outcomes, and cleared when replaced or when the operation ends.
Only the latest handle marked resumable may be used. A documented provider
validity window does not authorize persistence after the operation, external
storage, later reopen, or exposure to a consumer.

If planned rollover is required without a current resumable handle, the session
ends. The driver cannot create a fresh provider session and present it as
continuity.

## Safe Handoff

The first proof rolls over only at an idle media-turn boundary. A provider
warning received during an active response may mark rollover pending, but the
response must reach its normal terminal boundary before handoff. If the old
connection ends or its bounded rollover window expires first, the response and
session fail.

Handoff order is:

1. stop accepting another input commit on the old connection
2. retain only the latest resumable handle and provider time-left evidence
3. open the replacement through the same host-approved endpoint grant and
   credential lease
4. send the exact setup with the handle and await setup completion
5. switch subsequent input to the replacement connection
6. close and join the old connection work
7. continue with the next runtime turn and event sequence

No accepted input or emitted output is replayed. Provider connection-local
sequences restart only where the provider protocol requires it; runtime event,
media-stream, and turn identities remain monotonic and unambiguous.

Failure to connect, authenticate, configure, or confirm the replacement is
terminal. There is no fresh-session fallback or second attempt.

## Access And Secret Transport

Contract 014 applies to every connection. One operation-scoped endpoint grant
and credential lease may authorize the bounded replacement connection only
when the exact host, endpoint reference, audience, profile, and scope still
match.

When a provider protocol carries a credential in the WebSocket query, the host
grant remains the credential-free approved endpoint. The driver may construct
the provider-required authenticated request only inside private transport
work. The resulting URL, query, credential bytes, handshake headers, and raw
failure payloads are secret material and never stable endpoint or diagnostic
evidence.

Authorization API keys, standard API keys, ephemeral tokens, workload
identity, and delegated authentication remain distinct access profiles even
when more than one uses the `ApiKey` credential mechanism. A driver cannot
infer key kind from secret bytes or substitute another profile after rejection.

Provider-supported preview maturity is not `ExperimentalObserved` integration
authority. Preview identity stays explicit in the protocol facade, configured
instance policy, exact model route, fixture corpus, and live-probe gate.

## Cancellation, Deadline, And Cleanup

Cancellation or deadline during rollover closes both old and replacement
connections, joins all readers, writers, timers, tasks, and blocking work, and
then releases the credential. No handoff worker may detach.

When the selected provider has no native response-cancel request, connection
close proves only local transport stop. Provider cancellation remains
unconfirmed. Cancellation, deadline, planned rollover failure, unexpected
disconnect, provider failure, and protocol failure remain distinct terminal
causes.

Normal close joins both historical and current connection work before the sole
credential release. Best-effort drop is not joined cleanup evidence.

## First Gemini Live Subset

The first proof binds:

- provider-supported Gemini Live preview and raw server-to-server WebSocket
- exact `v1beta` `BidiGenerateContent` audience and model resource
  `models/gemini-3.1-flash-live-preview`
- a project-bound authorization API-key access profile and project billing
- exact PCM16 mono 16 kHz input and PCM16 mono 24 kHz output
- exact `Kore` voice, minimal thinking, output transcription, no tools, and no
  system instruction
- disabled automatic activity detection and explicit no-interruption handling
- activity start, bounded audio chunks, and activity end for each manual turn
- two serial successful turns, one active response, and one rollover between
  completed turns
- the latest private resumable handle, `GoAway` time-left evidence, replacement
  setup completion, cumulative usage, and joined two-connection cleanup
- local-close cancellation and deadline with unconfirmed provider stop

It excludes standard keys, ephemeral tokens, `v1alpha`, client-to-server
browser access, automatic VAD, barge-in, text, image or video input, tools,
function calls, context compression, durable handle storage, consumer resume,
unexpected reconnect, replay, retry, fallback, aliases, devices, playback,
resampling, transcoding, and default live authentication.

## Conformance

Deterministic fixtures must prove:

- exact preview facade, route, model, audience, authorization-key profile,
  formats, voice, thinking, activity, rollover, host, and turn bounds
- no endpoint, credential, connection, or handle effect before valid preflight
- disabled rollover for every existing realtime route
- one completed turn, provider warning, latest-handle replacement, confirmed
  successor setup, second completed turn, and no replay
- handle replacement, redaction, clearing, and absence from public bindings
- missing or non-resumable handle, active-response timeout, rollover exhaustion,
  replacement failure, unexpected disconnect, and unknown event behavior
- asymmetric format validation, usage evidence, and terminal ordering
- cancellation, deadline, old/new connection failure, and cleanup failure
- every old and current connection worker joined before credential release
- unchanged prior profiles and production drivers

Live authentication and paid inference remain separately gated.

## Acceptance

- provider-planned rollover is explicit and positively bounded
- consumer resume and unexpected reconnect do not become implicit
- provider handles remain private, replaceable, and operation-scoped
- no input or output is replayed across handoff
- the plan and access boundary cannot change across connections
- preview maturity and provider support authority remain independently visible
- cancellation and local close do not claim remote provider stop
- no worker, connection, handle, endpoint, or credential survives joined close
