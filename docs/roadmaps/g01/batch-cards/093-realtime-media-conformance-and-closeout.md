# 093 Realtime Media Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../031-openai-realtime-media-direct-session-proof.md`

## Objective

Prove the OpenAI Realtime driver under both execution-host topologies and close
roadmap 031 without live provider access.

## Readiness Gate

Card 092 must implement the exact production driver and pass its deterministic
protocol and lifecycle fixtures.

## Scope

- eleventh provider-neutral profile against the production driver
- local and remote-authoritative host identities
- two-turn success, bounds, parallel rejection, provider failure, unknown
  event, format drift, disconnect, cancellation, deadline, and cleanup failure
- exact connection, task, blocking-work, timer, and credential join order
- full architecture, roadmap, front-door, and log closeout
- one meaningful full repository validation round

## Acceptance Criteria

- [x] both topologies preserve exact instance, host, audience, access, route,
      model, media format, lifecycle, and cleanup truth
- [x] two serial turns and one active response match the frozen transcript
- [x] cancellation acknowledgement, deadline, disconnect, and provider failure
      remain distinct
- [x] interrupted sessions cannot be reused
- [x] all prior profiles and production drivers remain passing
- [x] roadmap 031 closes with one sole next task

## Evidence

- the eleventh common profile now records its already-proved separation of
  usage, rate, quota, and request-correlation evidence; the production driver
  passes the profile without provider branches
- local and remote-authoritative fixtures preserve exact host, audience,
  access, route, model, PCM16 format, turn bound, and credential lifetime
- deterministic production tests cover two serial turns, active-response
  rejection, provider failure, unknown semantics, format drift, disconnect,
  confirmed and unconfirmed cancellation, deadline, and cleanup failure
- transport reset now retains disconnected runtime truth instead of becoming
  a provider failure
- response timers and tasks plus connection blocking work join before the sole
  credential release; injected release failure remains visible
- all 28 OpenAI adapter tests and focused warnings-denied clippy pass
- full `effigy qa` passes with 430 tests; three existing installed or live
  probes remain ignored by default
- Effigy doctor remains at the inherited 19 findings: 12 warnings and 7 errors

## Continuation

Roadmap 032 and card 094 own the post-realtime evidence checkpoint. No provider
or transport is preselected.

## Validation

- focused realtime-media and OpenAI tests
- focused warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- topology fixtures require provider-specific branches in the shared profile
- cleanup can release credentials before duplex work joins
- conformance implies device, playback, truncation, reconnect, or resume truth

## Auto-Continuation

No. Close the proof and return to the provider-coverage checkpoint.
