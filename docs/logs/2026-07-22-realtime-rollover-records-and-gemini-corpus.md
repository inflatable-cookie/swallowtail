# Realtime Rollover Records And Gemini Corpus

Date: 2026-07-22

## Changed

- added an explicit planned-connection-rollover policy with disabled default
  and positive bounded opt-in
- added exact maximum-count capability agreement across requirements,
  configured instance, model route, immutable plan, and open request
- added pure-preflight rejection for disabled, absent, zero, mismatched, and
  unsupported rollover posture before effects
- added an idle-handoff, no-replay, exhaustion, failure, and credential-last
  cleanup assertion pack over the existing realtime profile
- froze a test-only Gemini Live raw-WebSocket corpus dated 2026-07-22
- added an adapter-private replaceable, redacted, zeroizing provider-handle
  fixture with no public or durable representation

## Evidence

The current raw Live API reference places response modalities, speech
configuration, and thinking configuration under `generationConfig`. Realtime
input configuration, session resumption, and output transcription remain
setup-level fields. The corpus follows the raw wire schema, not an SDK's
flattened configuration facade.

The frozen subset binds `v1beta`,
`models/gemini-3.1-flash-live-preview`, authorization API-key audience,
`Kore`, minimal thinking, disabled automatic activity detection,
`NO_INTERRUPTION`, PCM16 mono 16 kHz input, PCM16 mono 24 kHz output, two
turns, and one planned rollover.

Official evidence:

- <https://ai.google.dev/api/live>
- <https://ai.google.dev/gemini-api/docs/live-api/session-management>

## Validation

- focused core, runtime, testkit, Gemini, and OpenAI tests pass
- warnings-denied clippy passes for all affected packages and targets
- no credential, paid request, browser, device, or production WebSocket was
  used

## Next

Card 096 is ready: implement the separately registered Gemini Live bounded
WebSocket driver. Card 097 remains the in-bounds portability closeout.
