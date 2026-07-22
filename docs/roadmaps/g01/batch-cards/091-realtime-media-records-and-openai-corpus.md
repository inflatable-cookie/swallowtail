# 091 Realtime Media Records And OpenAI Corpus

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../031-openai-realtime-media-direct-session-proof.md`

## Objective

Realize the minimum provider-neutral realtime-media boundary and freeze the
exact OpenAI Realtime GA corpus before production transport work.

## Readiness Gate

Research 020 selects the route. Contract 026 fixes the operation, media,
consumer-ownership, cancellation, access, and lifecycle boundary. No unresolved
product decision remains in the bounded subset.

## Governing References

- Research 020
- Contracts 005, 006, 009, 011, 014, 016, 020, and 026
- roadmap 031
- official OpenAI Realtime WebSocket, conversations, event, and model evidence
  accessed 2026-07-22

## Scope

- separate realtime-media session role under interactive direct inference
- audio kind, direction, encoding, sample rate, channels, and exact format
- bounded redacted media chunks, stream identity, and monotonic sequence
- append, commit, response, output-audio, transcript, usage, rate, request,
  terminal, cancellation, deadline, and cleanup records
- pure requirements and preflight rejection before effects
- one new provider-neutral realtime direct-session conformance profile
- frozen OpenAI GA server-to-server WebSocket corpus for
  `gpt-realtime-2.1`
- no production network driver, WebRTC, SIP, ephemeral token, device, codec,
  provider credential, or paid request

## Ordered Work

1. Add the smallest core and runtime record set required by Contract 026.
2. Keep existing interactive-session and structured-run traits unchanged.
3. Add pure preflight fixtures for role, format, bounds, access, route, model,
   and host-service mismatches.
4. Add an eleventh provider-neutral profile for ordered media input/output,
   cancellation, terminal state, and joined cleanup.
5. Freeze bounded OpenAI client and server event fixtures, including malformed,
   unknown, format-drift, provider-error, cancel, and disconnect cases.
6. Run focused core, runtime, testkit, and adapter tests plus warnings-denied
   clippy before card 092 becomes ready.

## Acceptance Criteria

- [x] realtime media remains an interactive direct-inference role
- [x] text operation content and finite attachments do not carry media chunks
- [x] formats and maximum chunk bytes bind before provider effects
- [x] chunk bytes redact and sequences reject gaps, duplicates, and crossings
- [x] append, commit, active response, output, and terminal ordering are exact
- [x] consumer device, playback, conversion, and played-position ownership is
      absent from Swallowtail records
- [x] the OpenAI corpus contains no endpoint client, credential, audio device,
      or paid request
- [x] all ten existing conformance profiles remain unchanged

## Completion Evidence

- `RealtimeMediaSession` is separately registered under direct inference and
  interactive-session shape; existing operation traits are unchanged.
- exact model, input/output format, chunk, turn, access, and host-service
  requirements reject through pure preflight before provider effects.
- runtime records enforce redacted bounded chunks, exact stream and turn
  identity, monotonic sequences, serial responses, terminal reuse, and joined
  session lifecycle.
- the eleventh synthetic profile proves two serial turns plus cancellation,
  deadline, failure, disconnect, usage, rate, quota, request, and cleanup truth.
- the dated OpenAI corpus freezes the manual PCM WebSocket subset, safe failure
  cases, synthetic handshake evidence, and current client/server event shapes.
- focused core, runtime, testkit, and OpenAI tests pass; warnings-denied focused
  clippy passes.
- full repository QA passes with 416 tests; three installed or live probes
  remain separately gated and ignored.
- doctor remains at the inherited 19 findings: 12 warnings and seven errors.

## Evidence Required

- public provider-neutral record and trait tests
- pure preflight rejection matrix
- eleventh-profile success, cancellation, deadline, failure, and cleanup report
- exact OpenAI fixture field and event-order assertions
- raw audio, transcript, frame, provider-id, endpoint, and secret redaction
- focused test and warnings-denied clippy output

## Validation

- focused core, runtime, testkit, and OpenAI adapter tests
- focused warnings-denied clippy
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- records need a new execution layer or generic prompt method
- media bytes enter `OperationContent`, public diagnostics, or serialization by
  default
- the OpenAI route cannot use one exact format and model
- fixture work requires live provider state or a browser/device runtime

## Auto-Continuation

No. Confirm the shared records and corpus before production WebSocket work.
