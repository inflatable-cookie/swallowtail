# 026 Realtime Media Direct Session Boundary

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Add bounded duplex media communication to direct interactive sessions without
turning binary streams into prompt strings, hiding provider lifecycle, or
taking device and playback ownership from consumers.

## Execution And Operation Shape

Realtime media remains `DirectModelInference` plus `InteractiveSession`. It is
not a third execution layer and does not imply a provider-owned agent loop.

A separately registered realtime-media session role exposes the additional
input and output exchange. Existing interactive-session and structured-run
roles do not gain an optional catch-all media method. A driver may register
both roles only when each exact transport and capability set is independently
described and conformed.

The role is not a generic `send_prompt`, audio device API, or arbitrary byte
tunnel. Preflight binds the supported media directions and formats before
endpoint, credential, or connection work.

## Media Identity And Format

Provider-neutral media records distinguish:

- media kind, initially audio
- encoding or codec identity
- sample rate
- channel count
- input and output direction
- exact configured format versus observed provider format

Format identity belongs to the configured route and session. A model modality
catalogue observation cannot silently populate it. The first proof accepts
only one exact mono PCM input format and one exact mono PCM output format. It
performs no format negotiation, resampling, transcoding, channel mixing, or
fallback.

## Chunk And Commit Boundary

Input and output media use bounded opaque byte chunks. Each chunk carries:

- its exact runtime session and media-stream identity
- one monotonic stream sequence
- direction and format identity
- non-empty bytes below the preflight-bound maximum

Chunk bytes are non-serializable by default, redacted from `Debug`, `Display`,
diagnostics, and terminal outcomes, and cleared when ownership ends. Base64 is
a provider wire encoding, not a stable runtime payload.

The first input flow is explicit append then commit. Commit closes one logical
input segment and authorizes one provider response. Empty commits, chunks after
commit, wrong stream, wrong format, non-monotonic sequence, or over-limit bytes
fail before a provider frame.

## Session And Response Serialization

One session permits one active provider response. A second commit or response
start while another response is active fails before another provider frame.
Input chunk transport may be incremental, but the first proof does not accept
new input while provider output is active.

Each completed response belongs to one runtime media turn and emits ordered:

- response start
- output-audio deltas
- optional transcript deltas and completed transcript
- provider usage, rate, quota, and request-correlation observations
- response terminal state

Unknown semantic events cannot be discarded. Raw provider frames, item ids,
response ids, audio, transcripts, errors, credentials, and endpoints remain
outside stable diagnostics.

## Consumer And Host Ownership

The consumer owns:

- microphone, file, telephony, or other capture source
- speaker, file, or other playback sink
- user consent and privacy indication
- buffering, pacing, backpressure presentation, and jitter policy
- resampling, encoding, decoding, echo cancellation, and device selection
- how much output audio was actually played
- product instructions, tool policy, conversation intent, and acceptance

Swallowtail owns only the bounded provider exchange, lifecycle, normalized
events, cancellation transport, safe evidence, and joined cleanup. The
execution host owns endpoint and credential authorization plus the task,
blocking-work, and time services it grants.

No driver may infer played position from bytes received. Provider conversation
truncation that needs playback position requires an explicit later consumer
input and contract.

## Connection, Cancellation, And Deadline

Contract 016 connection and lease ownership applies. The first realtime proof
has no reconnect, resume, replay, or durable provider-session claim.

The selected provider exposes native response cancellation. Consumer
cancellation and deadline expiry therefore:

1. stop further local input and output delivery
2. send one native response-cancel request when the connection remains usable
3. await bounded provider acknowledgement or record it as unconfirmed
4. close the session connection
5. join reader, writer, timer, and blocking work
6. release the credential last

Cancellation and deadline remain distinct terminal causes. Native provider
acknowledgement does not prove playback truncation or durable provider-state
deletion. Any interrupted first-proof response ends the session so later turns
cannot inherit playback-inconsistent conversation state.

Ordinary response completion keeps the connection available for the bounded
next serial turn. Provider failure, protocol failure, output-format drift,
unexpected disconnect, or connection lifetime end invalidates the session.

## First OpenAI Realtime Subset

The first provider proof binds:

- provider-supported GA Realtime WebSocket at `api.openai.com/v1/realtime`
- public OpenAI API-key access, API usage billing, and public-API audience
- exact model `gpt-realtime-2.1`
- server-to-server WebSocket only
- resource-free direct interactive execution
- exact mono PCM input and output formats
- manual input append and commit
- maximum two successful serial responses and one active response
- output audio, transcript, final usage, rate, and request evidence
- native response cancellation followed by session close

It excludes ChatGPT, Codex, WebRTC, SIP, ephemeral client secrets, workload
identity, automatic voice activity detection, simultaneous barge-in input,
playback truncation, images, text turns, tools, MCP, stored prompts, safety-id
policy, browser media, retry, reconnect, resume, storage, aliases, and model or
route fallback.

Omitting the optional provider safety identifier creates no identifier. A
later consumer-supplied privacy-preserving identifier needs an explicit input
boundary; Swallowtail does not derive it from runtime or account identity.

## Conformance

Deterministic fixtures must prove:

- exact role, execution layer, operation shape, endpoint, audience, credential,
  model, format, and execution-host binding
- no endpoint, credential, connection, or media effect before valid preflight
- chunk byte and sequence bounds plus redaction
- append-before-commit ordering and one active response
- two successful serial media turns with ordered audio and transcript output
- usage, rate, quota, request, provider failure, and unknown-event truth
- native cancellation, deadline, cancellation-acknowledgement uncertainty, and
  session-ending interruption
- disconnect, format drift, connection close, and cleanup failure
- reader, writer, timer, connection, blocking work, and credential join order
- unchanged behavior for all ten existing profiles

Live authentication and paid inference remain separately gated.

## Acceptance

- media bytes never become prompt strings or diagnostics
- realtime media stays an interactive direct-inference capability
- existing text roles do not silently widen
- format and chunk bounds are explicit before provider work
- consumers retain device, playback, conversion, and privacy policy
- native response cancellation does not imply playback truncation
- interrupted responses cannot leave a reusable but inconsistent session
- no credential, model, transport, route, reconnect, or modality fallback is
  implicit
