# 092 OpenAI Realtime WebSocket Driver

Status: ready
Owner: Tom
Updated: 2026-07-22
Milestone: `../031-openai-realtime-media-direct-session-proof.md`

## Objective

Implement the exact OpenAI Realtime GA server-to-server WebSocket driver over
card 091's frozen records and corpus.

## Readiness Gate

Card 091 must complete with stable provider-neutral records, an eleventh
profile, pure preflight, and a dated offline corpus. Any contract or provider
currentness gap returns to card 091 authority before network work.

## Scope

- separate descriptor and realtime-media session driver in
  `swallowtail-adapter-openai`
- public API-key profile, public Realtime endpoint, API usage billing, and exact
  `gpt-realtime-2.1` route
- one session-scoped endpoint and credential lease
- server-to-server WebSocket, exact session configuration, bounded audio
  append/commit, and maximum two serial responses
- ordered audio, transcript, usage, rate, request, and terminal mapping
- native response cancel followed by session-ending joined cleanup
- no changes to the existing background Responses driver
- deterministic loopback only; live authentication remains gated

## Acceptance Criteria

- [ ] exact preflight plan survives connection, turns, and cleanup
- [ ] invalid formats, bounds, access, routes, models, and inputs reject before
      provider effects
- [ ] one active response and maximum two successful responses are enforced
- [ ] only frozen client frames are sent and unknown server semantics fail
- [ ] cancellation and deadline send at most one native cancel and end the
      session after acknowledgement or uncertainty
- [ ] reader, writer, timer, connection, and blocking work join before
      credential release
- [ ] raw endpoint, credential, audio, transcript, provider ids, and frames
      remain out of stable diagnostics

## Validation

- focused OpenAI production-driver tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- the driver needs WebRTC, a browser, a device, or live authentication
- background Responses behavior changes
- interrupted conversation state would remain reusable without played-position
  truth
- connection or credential work can detach from the session

## Auto-Continuation

No. Prove production behavior before topology closeout.
