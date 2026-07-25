# 2026-07-25 Realtime Prepared Facades

Status: complete

## Changed

The xAI, OpenAI, and Gemini adapters now expose separate prepared connection
facades over their unchanged realtime drivers.

xAI requires an explicit model route and retains serial text turns,
connection-private continuation, usage, billed ticks, and whole-session
invalidation. OpenAI binds `gpt-realtime-2.1`, fixed mono 24 kHz PCM, manual
input commit, native response cancellation, and no planned rollover. Gemini
binds `gemini-3.1-flash-live-preview`, mono 16 kHz input, mono 24 kHz output,
local interruption truth, and exactly one planned provider rollover.

Every prepared surface retains exact endpoint, access, configured-instance,
host, facade, model, format, turn, cancellation, and cleanup evidence. Bound
open delegates to the existing low-level driver. No shared realtime
constructor or universal turn API was added.

## Current Evidence

Current first-party documentation was rechecked before implementation:

- [OpenAI Realtime with WebSocket](https://developers.openai.com/api/docs/guides/realtime-websocket)
- [OpenAI Realtime conversations](https://developers.openai.com/api/docs/guides/realtime-conversations#handling-audio-with-websockets)
- [Gemini Live API](https://ai.google.dev/api/live)
- [Gemini Live session management](https://ai.google.dev/gemini-api/docs/live-api/session-management)
- [xAI Responses WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)

xAI now documents a separate voice Realtime route. This batch covers the
already contracted text Responses WebSocket route. Adding xAI voice would be a
new route with separate evidence and authority.

## Boundaries

- preparation opens no socket and acquires no endpoint or credential lease
- xAI model choice remains explicit; example aliases do not become defaults
- Gemini planned rollover is not retry, reconnect, stream reattachment,
  consumer resume, or durable provider state
- OpenAI and xAI do not inherit Gemini rollover behavior
- cancellation and disconnect truth remain provider-specific
- consumers retain capture, playback, conversion, pacing, privacy, and played
  position
- live credentials, devices, WebRTC, SIP, and ephemeral browser tokens remain
  separately gated

## Validation

- 85 tests pass across the three adapter packages
- the existing Gemini live authentication probe remains separately ignored
- all three packages pass warnings-denied all-target lint
- workspace formatting passes
- public API declarations match updated baselines
- `effigy doctor` still reports the known 19 oversized-file findings,
  including seven errors; this batch adds none

## Next

Card 032 adds separate prepared Bedrock Runtime and control-plane catalogue SDK
facades. Cards 032-036 remain in bounds.
