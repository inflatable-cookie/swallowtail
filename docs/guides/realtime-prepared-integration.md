# Realtime Prepared Integration

Swallowtail exposes three separate prepared connection surfaces:

| Route | Prepared integration | Bound operation | Native shape |
| --- | --- | --- | --- |
| xAI Responses WebSocket | `XaiPreparedIntegration` | `XaiPreparedResponsesSession` | serial text turns with private continuation and billed cost |
| OpenAI Realtime | `OpenAiRealtimePreparedIntegration` | `OpenAiPreparedRealtimeSession` | manual PCM input, audio/transcript output, native response cancellation |
| Gemini Live | `GeminiLivePreparedIntegration` | `GeminiPreparedLiveSession` | asymmetric PCM, local interruption, one provider-planned rollover |

They share provider-neutral prepared evidence. They do not share a connection
constructor, turn method, cancellation claim, rollover policy, media format,
or model-selection rule.

## xAI Responses WebSocket

`prepare_xai_responses_websocket` binds one execution host, host-approved
`/v1/responses` WebSocket target, public xAI API-key profile, exact dated
facade, and access provenance.

`prepare_responses_session` requires an exact caller-selected route revision
and model identity. Swallowtail does not choose a Grok alias. The returned
interactive session retains `store=false`, one active text turn, private
connection-local continuation, provider usage and billed ticks, and
credential-last joined cleanup.

Cancellation, deadline, disconnect, provider failure, or connection lifetime
end invalidates the whole session. There is no reconnect, replay, provider
storage, or consumer resume binding.

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

The prepared session delegates append, commit, output audio, transcript,
usage, rate, request correlation, native response cancellation, connection
invalidation, and cleanup to the unchanged realtime driver. Consumers retain
capture, playback, conversion, pacing, privacy, and played-position truth.

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

Rollover uses only the latest in-memory resumable handle after provider
`GoAway`, at an idle turn boundary, under the unchanged plan. It is not retry,
unexpected reconnect, stream reattachment, consumer resume, or durable
provider storage. Cancellation and deadline close locally with unconfirmed
provider cancellation truth.

See the compile-tested
[`prepared_live_session` example](../../crates/swallowtail-adapter-gemini/examples/prepared_live_session.rs).

## Common Preparation Boundary

Each preparation input requires:

- one configured-instance revision
- one exact execution host and host-approved endpoint target
- the adapter-owned access profile with a consumer-selected credential
  reference
- observed or caller-asserted access evidence

Preparation acquires no endpoint grant or credential and opens no socket.
Each prepared operation exposes its immutable plan, request, access evidence,
low-level driver, and `into_parts` escape hatch.

Live authentication, microphone capture, speakers, browser transports,
ephemeral client tokens, WebRTC, SIP, tools, automatic provider fallback, and
route selection remain downstream or separately gated.
