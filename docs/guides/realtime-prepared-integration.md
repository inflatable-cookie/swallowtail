# Realtime Prepared Integration

Swallowtail exposes three separate prepared connection surfaces:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

| Route | Package and driver ID | Bound operation | Native shape |
| --- | --- | --- | --- |
| `xai.responses-websocket` | `swallowtail-adapter-xai`; `swallowtail.xai.websocket` | `XaiPreparedResponsesRun`; `XaiPreparedResponsesSession` | Responses WebSocket; one bounded response without continuation, or serial text turns with private continuation and billed cost |
| `openai.realtime` | `swallowtail-adapter-openai`; `swallowtail.openai.realtime` | `OpenAiPreparedRealtimeSession` | Realtime WebSocket; manual PCM input, audio/transcript output, native response cancellation |
| `gemini.live` | `swallowtail-adapter-gemini`; `swallowtail.gemini.live` | `GeminiPreparedLiveSession` | Gemini Live raw WebSocket; asymmetric PCM, local interruption, one provider-planned rollover, qualified thinking-level selection |

They share provider-neutral prepared evidence. They do not share a connection
constructor, turn method, cancellation claim, rollover policy, media format,
or model-selection rule.

Choose xAI for text Responses over one socket, OpenAI for the exact public
Realtime PCM profile and output-token control, and Gemini for the qualified
Live preview with planned rollover. Reject this guide when the application
needs HTTP/SSE request-response inference, browser/WebRTC/SIP transport,
durable provider sessions, callbacks, tools, or cross-process continuity.

## Provider Catalogue Branches

Each provider exposes a separate catalogue preparation path:

| Provider | Prepared constructor | Source |
| --- | --- | --- |
| xAI | `prepare_xai_models` | public `/v1/language-models` |
| OpenAI | `prepare_openai_models` | public `/v1/models` |
| Gemini | `prepare_gemini_models` | paginated Developer API `models.list` |

Each branch requires its own configured-instance revision, approved public API
endpoint, API-key access profile and evidence, execution host, request
identity, and optional deadline. `prepare_catalogue` returns a typed operation
whose `list_models` method performs bounded authenticated discovery.

These are provider catalogue branches, not Realtime, Live, or Responses route
capability probes. A listed id does not become an invocation route or prove
transport support, entitlement, billing readiness, or request acceptance.

## xAI Responses WebSocket

`prepare_xai_responses_websocket` binds one execution host, host-approved
`/v1/responses` WebSocket target, public xAI API-key profile, exact dated
facade, and access provenance.

`prepare_responses_session` requires an exact caller-selected route revision
and model identity. Swallowtail does not choose a Grok alias. The returned
interactive session retains `store=false`, one active text turn, private
connection-local continuation, provider usage and billed ticks, and
credential-last joined cleanup.

The optional prepared generation controls are exact and independent. For
`grok-4.5`, reasoning accepts `low`, `medium`, or `high`; for `grok-4.6`, it
also accepts `xhigh`. Both exact model ids accept a positive
`max_output_tokens` value through `2_147_483_647`. Use
`XaiRunProfileInput::with_reasoning_mode` or
`XaiSessionProfileInput::with_reasoning_mode`, and the corresponding
`with_maximum_output_tokens` builder, to select them. Aliases, Grok 4.5
`xhigh`, multi-agent effort, and other model ids are rejected before endpoint
or credential work. Omission preserves the existing request body.

For a session, the prepared selection is fixed on the opened handle and is
sent on the first turn, each serial continuation turn, and a fresh working
state replacement. `max_output_tokens` is a provider request bound that
includes reasoning tokens; it is not a client truncation or an exact text
length claim.

`prepare_responses_run` binds the same explicit route and access evidence to
one resource-free structured operation. It opens one connection, sends one
`store=false` `response.create` without a previous-response id, streams one
terminal response, reports usage and billed cost, then closes and joins the
connection. It exposes no provider run, session, or continuation binding.

For runs and text turns, take and drain events and terminal concurrently, then
close the operation. Usage and billed-cost evidence are provider observations,
not retry or balance authority. Cancellation, deadline, disconnect, and
cleanup remain distinct.

Cancellation, deadline, disconnect, provider failure, or connection lifetime
end invalidates the whole session. There is no reconnect, replay, provider
storage, or consumer resume binding.

`XaiPreparedResponsesSession::prepare_working_state_restoration` opens a fresh
interactive WebSocket session. It returns `SessionReplaced`, not connection
continuation, and carries no private response chain from the lost socket.

See the compile-tested
[`prepared_responses_websocket` example](../../crates/swallowtail-adapter-xai/examples/prepared_responses_websocket.rs).

## OpenAI Realtime

`prepare_openai_realtime` binds the public Realtime WebSocket audience,
standard public API-key access, exact `gpt-realtime-2.1` route, and the dated
GA facade.

`OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns` fixes:

- mono PCM16 input and output at 24 kHz
- maximum 32,768-byte chunks
- maximum two serial responses
- disabled planned rollover
- optional operation deadline

Call `with_maximum_output_tokens` for an exact positive maximum no greater
than 4,096. Omission leaves the provider profile's existing behavior; the
adapter never invents a default for the consumer.

The prepared session delegates append, commit, output audio, transcript,
usage, rate, request correlation, native response cancellation, connection
invalidation, and cleanup to the unchanged realtime driver. Consumers retain
capture, playback, conversion, pacing, privacy, and played-position truth.

Consumers must continuously drain media/session events while driving input
and response operations. Native response cancellation affects the active
response, while connection failure invalidates the session. Close joins local
socket, task, and credential work; terminal and cleanup truth stay separate.

`OpenAiPreparedRealtimeSession::prepare_working_state_restoration` returns
`RealtimeSessionReplaced` with one new media handle. It carries no audio,
transcript, response, buffer, cancellation, or terminal state from the lost
connection.

See the compile-tested
[`prepared_realtime_session` example](../../crates/swallowtail-adapter-openai/examples/prepared_realtime_session.rs).

## Gemini Live

`prepare_gemini_live` binds the project authorization API-key profile,
Generative Language `v1beta` WebSocket, exact
`gemini-3.1-flash-live-preview` model, and preview support authority.

`GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover` fixes:

- mono PCM16 input at 16 kHz
- mono PCM16 output at 24 kHz
- maximum 32,768-byte chunks
- maximum two serial responses
- exactly one provider-planned connection rollover
- optional operation deadline

`GeminiLiveSessionProfileInput::with_reasoning_mode` adds one optional
thinking level. The route admits exactly `minimal`, `low`, `medium`, and
`high`, and maps them to the qualified setup values `MINIMAL`, `LOW`,
`MEDIUM`, and `HIGH`. Every other value, including `off`, `default`, `xhigh`,
`max`, and any numeric budget, is rejected before endpoint, credential, or
socket work. Nothing is clamped, aliased, or substituted.

The selection is fixed at preparation and immutable for the session: the same
level is sent on the initial setup, on the one planned rollover setup, and on
a fresh working-state restoration. Omitting the selection keeps the route's
existing fixed setup bytes, which already carry `MINIMAL`, and claims no
reasoning capability. An explicit `minimal` selection serializes identically
but is a planned `ReasoningSelection` in the capability profile, plan, and
prepared evidence.

Swallowtail claims qualified dispatch only. The Live surface's setup
acknowledgement carries no fields, so provider acceptance, effective reasoning
depth, and thought-summary disclosure are not claimed or observable here.
Thought summaries, `includeThoughts`, and `thinkingBudget` remain out of
scope.

The thinking-capable behavior is qualified at its own exact opaque facade
point, `GEMINI_LIVE_FACADE_REVISION`. The point qualified before it is retained
as `GEMINI_LIVE_SUPERSEDED_FACADE_REVISION`; it is not a supported claim, and a
plan carrying it is rejected before endpoint, credential, or socket work.
Publish a new configured-instance revision when moving to the current point.

Rollover uses only the latest in-memory resumable handle after provider
`GoAway`, at an idle turn boundary, under the unchanged plan. It is not retry,
unexpected reconnect, stream reattachment, consumer resume, or durable
provider storage. Cancellation and deadline close locally with unconfirmed
provider cancellation truth.

Consumers drain audio, transcript, usage, lifecycle, rollover, and terminal
events continuously. They retain capture, playback, sample conversion,
backpressure, privacy, and played-position truth. Rollover failure invalidates
the connection rather than silently starting a fresh session.

`GeminiPreparedLiveSession::prepare_working_state_restoration` also returns a
fresh realtime handle with connection-state loss. The configured one-rollover
policy remains an in-session idle-boundary mechanism; it does not recover a
session after process loss.

See the compile-tested
[`prepared_live_session` example](../../crates/swallowtail-adapter-gemini/examples/prepared_live_session.rs).

## Common Preparation Boundary

Each preparation input requires:

- one configured-instance identity and revision
- one exact execution host and host-approved endpoint target
- the adapter-owned access profile with a consumer-selected credential
  reference
- observed or caller-asserted access evidence

The host binds approved endpoint, opaque credential, WebSocket/HTTP, task, and
time services. Swallowtail performs no login, endpoint discovery, model
selection, billing selection, microphone/speaker access, or route fallback.
Each route uses public API-key billing and its exact audience; Gemini uses its
project-authorization profile.

The three routes bind exact opaque facade revisions:

- `xai.responses-websocket-facade`
- `openai.realtime-facade`
- `gemini.live-facade`

They have no ordered or unverified-newer version range. Catalogue presence
cannot promote a model to any of these transports.

Preparation acquires no endpoint grant or credential and opens no socket.
Each prepared operation exposes its immutable plan, request, access evidence,
low-level driver, and `into_parts` escape hatch.

Live authentication, microphone capture, speakers, browser transports,
ephemeral client tokens, WebRTC, SIP, tools, automatic provider fallback, and
route selection remain downstream or separately gated.

The routes also expose no attachments, structured output, consumer callbacks,
working resources, public load/resume, reconciliation, provider-session
management, background execution, or cross-process stream reattachment. xAI
reports billed cost and exposes the qualified text reasoning/output controls;
OpenAI exposes the qualified Realtime output-token maximum and rejects a
realtime reasoning selection before any endpoint or credential work; Gemini
permits the planned rollover and the qualified thinking-level selection.

## Failures, Promotion, And Validation

Handle failures through portable classification and retain the exact route
diagnostic (`swallowtail.xai.*`, `swallowtail.openai.*`, or
`swallowtail.gemini.*`) for support. Do not parse WebSocket payloads, provider
prose, private continuation handles, credentials, or endpoint values.

Promotion requires exact provider facade/model evidence, immutable media,
access, and rollover binding, bounded WebSocket fixtures, cancellation and
connection-lifecycle tests, and route-matrix coverage.

Validate the three compile-tested examples without opening sockets:

```sh
effigy validate:focused swallowtail-adapter-xai swallowtail-adapter-openai swallowtail-adapter-gemini
effigy check:examples
```

Live API calls, microphone capture, audio playback, and allowance spend remain
separately operator-gated.
