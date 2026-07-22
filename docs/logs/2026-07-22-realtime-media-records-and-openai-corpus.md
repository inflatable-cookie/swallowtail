# 2026-07-22 Realtime Media Records And OpenAI Corpus

## Changed

- added a separately registered `RealtimeMediaSession` role under direct model
  inference and interactive-session shape
- added exact audio kind, direction, PCM encoding, rate, channels, model,
  chunk, and turn requirements with pure preflight rejection before effects
- added resource-free open requests, redacted zeroized chunks, stream and turn
  identity, append/commit ordering, response handles, output audio, transcript,
  provider observations, terminal state, cancellation, deadline, and close
- added an eleventh provider-neutral profile for two serial manual media turns,
  interruption, failure, disconnect, and joined credential/task cleanup
- froze the 2026-07-22 OpenAI Realtime GA server WebSocket subset for exact
  model `gpt-realtime-2.1` and mono 24 kHz PCM16 input and output
- completed card 091 and advanced card 092 to ready

## Frozen Boundary

- public OpenAI API, API-key credential lease, API billing, and
  `wss://api.openai.com/v1/realtime` only
- `session.update`, bounded `input_audio_buffer.append`, commit,
  `response.create`, native response cancel, audio output, transcripts, usage,
  rate limits, request correlation, and terminal response truth
- maximum 32 KiB Swallowtail chunk, two successful serial turns, one active
  response, no reconnect, resume, replay, retry, storage, or playback truncation
- malformed, unknown, format-drift, provider-error, cancel, and disconnect
  fixtures fail or terminate explicitly

## Boundary

Realtime media does not widen `OperationContent`, attachments, structured runs,
or existing text sessions. Consumers retain capture, playback, codecs,
resampling, buffering, privacy, and played-position truth. Swallowtail accepts
already encoded bounded input and emits ordered output plus provider evidence.
The OpenAI protocol code remains test-only until card 092 adds the production
WebSocket driver. The sixteen existing production routes remain unchanged.

## Validation

- focused core, runtime, testkit, and OpenAI tests pass
- focused warnings-denied clippy passes
- full repository QA passes with 416 tests; three installed or live probes
  remain separately gated and ignored
- the ten earlier profiles remain present; the suite now contains eleven
- doctor remains at the inherited 19 findings: 12 warnings and seven errors
- no credential, endpoint client, audio device, browser, external request, or
  paid inference was used

## Continuation

Card 092 is the sole ready task. Implement the production OpenAI Realtime
server WebSocket driver over the frozen records and corpus, using deterministic
loopback transport. Live authentication remains separately gated.
