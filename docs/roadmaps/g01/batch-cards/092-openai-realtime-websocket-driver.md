# 092 OpenAI Realtime WebSocket Driver

Status: completed
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

- [x] exact preflight plan survives connection, turns, and cleanup
- [x] invalid formats, bounds, access, routes, models, and inputs reject before
      provider effects
- [x] one active response and maximum two successful responses are enforced
- [x] only frozen client frames are sent and unknown server semantics fail
- [x] cancellation and deadline send at most one native cancel and end the
      session after acknowledgement or uncertainty
- [x] reader, writer, timer, connection, and blocking work join before
      credential release
- [x] raw endpoint, credential, audio, transcript, provider ids, and frames
      remain out of stable diagnostics

## Evidence

- `OpenAiRealtimeDriver` is separately registered from background Responses
  and binds the public API-key audience, exact `gpt-realtime-2.1` route, fixed
  PCM16 24 kHz mono formats, 32 KiB chunks, and two serial responses.
- one blocking worker owns the WebSocket reader and writer; response tasks,
  connection work, and credential release remain explicitly joined.
- the current GA schema confirms `response_id` is optional on
  `response.cancel`; the frozen and production frame omit it so cancellation
  racing `response.created` remains sendable.
- four deterministic driver tests cover both host identities, serial turns,
  exact handshake and frames, evidence ordering, native cancel, deadline,
  unknown semantics, pre-effect rejection, redaction, and cleanup order.
- all 23 OpenAI adapter tests pass; focused warnings-denied clippy passes;
  doctor remains at the inherited 19 findings with no new oversized-file debt.

## Continuation

Card 093 remains in bounds for provider-neutral production conformance and
roadmap 031 closeout. This card does not auto-continue.

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
