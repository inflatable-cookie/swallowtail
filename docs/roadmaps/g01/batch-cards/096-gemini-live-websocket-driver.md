# 096 Gemini Live WebSocket Driver

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../033-gemini-live-realtime-portability-proof.md`

## Objective

Implement the exact Gemini Live preview raw-WebSocket driver over card 095's
frozen records and corpus.

## Readiness Gate

Card 095 must complete with stable bounded-rollover records, pure preflight,
provider-neutral assertions, and a dated offline corpus. Any currentness or
contract gap returns to card 095 before network work.

## Scope

- separate Live descriptor and realtime-media driver in
  `swallowtail-adapter-gemini`
- provider-supported preview facade, exact `v1beta` audience, authorization-key
  profile, project billing, and `gemini-3.1-flash-live-preview`
- one session-scoped endpoint grant and credential lease
- raw WebSocket setup, exact asymmetric PCM formats, `Kore`, minimal thinking,
  output transcription, no tools, and no system instruction
- manual activity start/audio/activity end and two serial responses
- maximum one idle-boundary rollover using the latest private handle
- cumulative usage, output audio, transcript, terminal, unknown, warning, and
  provider-failure mapping
- local-close cancellation and deadline with joined two-connection cleanup
- no changes to the Gemini CLI ACP driver
- deterministic loopback only; live authentication remains gated

## Acceptance Criteria

- [x] exact preflight plan survives both connections, turns, and cleanup
- [x] invalid access, preview, route, model, format, activity, or rollover
      posture rejects before provider effects
- [x] only frozen client frames are sent and setup completes before input
- [x] one successful rollover carries the latest handle with no replay
- [x] missing handle, exhausted rollover, replacement failure, and unexpected
      disconnect fail closed without a fresh session
- [x] cancellation and deadline never claim native provider stop
- [x] both connection workers and timers join before credential release
- [x] authenticated URL, credential, handle, audio, transcript, ids, and frames
      remain out of stable diagnostics

## Completion Evidence

- a separate `swallowtail.gemini.live` realtime-media descriptor binds the
  exact preview facade, model, authorization-key profile, asymmetric formats,
  manual activity, and maximum-one rollover plan
- production transport uses one host-approved endpoint grant and one secret
  lease; query authentication exists only inside private WebSocket work
- the first response retains the latest private handle and `GoAway` warning,
  confirms replacement setup, switches input, then joins the old worker
- deterministic loopback proves two turns with two setups and no input replay
- missing handle, replacement failure, exhausted rollover, disconnect,
  cancellation, deadline, redaction, and credential-last cleanup pass offline
- focused Gemini tests pass; doctor remains at the inherited 19 findings with
  no new oversized file

## Validation

- focused Gemini production-driver tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- production needs `v1alpha`, ephemeral client credentials, a browser, or a
  device
- Gemini CLI ACP behavior changes
- rollover must occur by replaying input or exposing a durable handle
- connection or credential work can detach from the operation

## Auto-Continuation

No. Prove the exact driver before topology conformance.

Card 097 is ready. It owns unchanged-profile, dual-topology, failure-matrix,
and full-repository closeout evidence.
